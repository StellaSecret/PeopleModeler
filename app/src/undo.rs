use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use peoplemodeler_core::models::{Person, Prediction, Relationship};

use crate::db;

#[derive(Serialize, Deserialize)]
struct Snapshot {
    persons: Vec<Person>,
    predictions: Vec<Prediction>,
    relationships: Vec<Relationship>,
}

static UNDO_STACK: Mutex<Vec<Snapshot>> = Mutex::new(Vec::new());
static MAX_UNDO: usize = 20;

pub fn push_snapshot() {
    let snap = Snapshot {
        persons: db::all_persons(),
        predictions: db::all_predictions(),
        relationships: db::all_relationships(),
    };
    if let Ok(mut stack) = UNDO_STACK.lock() {
        stack.push(snap);
        if stack.len() > MAX_UNDO {
            stack.remove(0);
        }
    }
}

pub fn undo() -> bool {
    let snap = if let Ok(mut stack) = UNDO_STACK.lock() {
        stack.pop()
    } else {
        return false;
    };
    let Some(snap) = snap else { return false };
    for p in &snap.persons {
        db::save_person_quiet(p);
    }
    for p in &snap.predictions {
        db::save_prediction_quiet(p);
    }
    for r in &snap.relationships {
        db::save_relationship_quiet(r);
    }
    true
}

pub fn can_undo() -> bool {
    UNDO_STACK.lock().map(|s| !s.is_empty()).unwrap_or(false)
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn ensure_db() {
        crate::db::init();
    }

    fn drain_stack() {
        for _ in 0..=MAX_UNDO {
            if !can_undo() {
                break;
            }
            let _ = undo();
        }
    }

    #[test]
    fn can_undo_false_initially() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        assert!(!can_undo());
    }

    #[test]
    fn push_then_can_undo() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        push_snapshot();
        assert!(can_undo());
        drain_stack();
    }

    #[test]
    fn undo_returns_false_when_empty() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        assert!(!undo());
    }

    #[test]
    fn undo_returns_true_after_push() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        push_snapshot();
        assert!(undo());
    }

    #[test]
    fn push_snapshot_enforces_max_undo() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        for _ in 0..25 {
            push_snapshot();
        }
        let len = UNDO_STACK.lock().unwrap().len();
        assert_eq!(len, MAX_UNDO, "expected {MAX_UNDO}, got {len}");
        drain_stack();
    }

    #[test]
    fn push_exactly_max_undo_stays_at_max() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        for _ in 0..MAX_UNDO {
            push_snapshot();
        }
        let len = UNDO_STACK.lock().unwrap().len();
        assert_eq!(
            len, MAX_UNDO,
            "after exactly MAX_UNDO pushes, expected {MAX_UNDO}, got {len}"
        );
        drain_stack();
    }

    #[test]
    fn push_one_past_max_undo_stays_at_max() {
        let _g = TEST_LOCK.lock().unwrap();
        ensure_db();
        drain_stack();
        for _ in 0..=MAX_UNDO {
            push_snapshot();
        }
        let len = UNDO_STACK.lock().unwrap().len();
        assert_eq!(
            len, MAX_UNDO,
            "after MAX_UNDO+1 pushes, expected {MAX_UNDO}, got {len}"
        );
        drain_stack();
    }
}

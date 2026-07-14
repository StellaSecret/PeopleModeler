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

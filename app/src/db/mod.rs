use std::sync::OnceLock;

use peoplemodeler_core::models::Person;
use peoplemodeler_core::models::Prediction;
use peoplemodeler_core::models::Relationship;

use crate::undo;

static DB: OnceLock<Box<dyn StorageBackend + Send + Sync>> = OnceLock::new();

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    {
        migrate_from_bulk();
        DB.set(Box::new(WebStorage)).ok();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        DB.set(Box::new(SqliteStorage::new())).ok();
    }
}

fn db() -> &'static dyn StorageBackend {
    DB.get()
        .map(|b| b.as_ref())
        .expect("Storage not initialized. Call db::init() first.")
}

trait StorageBackend: Send + Sync {
    fn load_all_persons(&self) -> Vec<Person>;
    fn load_person(&self, id: &str) -> Option<Person>;
    fn save_person(&self, person: &Person);
    fn delete_person(&self, id: &str);
    fn load_all_predictions(&self) -> Vec<Prediction>;
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction>;
    fn save_prediction(&self, prediction: &Prediction);
    fn delete_prediction(&self, id: &str);
    fn load_all_relationships(&self) -> Vec<Relationship>;
    fn save_relationship(&self, relationship: &Relationship);
    fn delete_relationship(&self, id: &str);
}

pub fn all_persons() -> Vec<Person> {
    db().load_all_persons()
}
pub fn person(id: &str) -> Option<Person> {
    db().load_person(id)
}
pub fn save_person(person: &Person) {
    undo::push_snapshot();
    db().save_person(person);
}
pub(crate) fn save_person_quiet(person: &Person) {
    db().save_person(person);
}
pub(crate) fn save_prediction_quiet(prediction: &Prediction) {
    db().save_prediction(prediction);
}
pub(crate) fn save_relationship_quiet(relationship: &Relationship) {
    db().save_relationship(relationship);
}
pub fn delete_person(id: &str) {
    undo::push_snapshot();
    db().delete_person(id);
}
pub fn all_predictions() -> Vec<Prediction> {
    db().load_all_predictions()
}
pub fn predictions_for_person(person_id: &str) -> Vec<Prediction> {
    db().load_predictions_for_person(person_id)
}
pub fn save_prediction(prediction: &Prediction) {
    undo::push_snapshot();
    db().save_prediction(prediction);
}
pub fn delete_prediction(id: &str) {
    undo::push_snapshot();
    db().delete_prediction(id);
}
pub fn all_relationships() -> Vec<Relationship> {
    db().load_all_relationships()
}
pub fn save_relationship(relationship: &Relationship) {
    undo::push_snapshot();
    db().save_relationship(relationship);
}
pub fn delete_relationship(id: &str) {
    undo::push_snapshot();
    db().delete_relationship(id);
}

#[cfg(target_arch = "wasm32")]
trait Identifiable {
    fn id(&self) -> &str;
}
#[cfg(target_arch = "wasm32")]
impl Identifiable for Person {
    fn id(&self) -> &str {
        &self.id
    }
}
#[cfg(target_arch = "wasm32")]
impl Identifiable for Prediction {
    fn id(&self) -> &str {
        &self.id
    }
}
#[cfg(target_arch = "wasm32")]
impl Identifiable for Relationship {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(target_arch = "wasm32")]
fn upsert<T: Clone + PartialEq>(vec: &mut Vec<T>, item: &T)
where
    T: Identifiable,
{
    if let Some(i) = vec.iter().position(|x| x.id() == item.id()) {
        vec[i] = item.clone();
    } else {
        vec.push(item.clone());
    }
}

// ---- Web Storage (WASM) ----
#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;

#[cfg(target_arch = "wasm32")]
fn person_key(id: &str) -> String { format!("person_{id}") }

#[cfg(target_arch = "wasm32")]
fn prediction_key(id: &str) -> String { format!("pred_{id}") }

#[cfg(target_arch = "wasm32")]
fn relationship_key(id: &str) -> String { format!("rel_{id}") }

#[cfg(target_arch = "wasm32")]
fn store_individual<T: serde::Serialize>(key: &str, val: &T) {
    use base64::Engine;
    let json = serde_json::to_string(val).expect("serialize");
    let enc = crate::crypto::encrypt(json.as_bytes());
    let b64 = base64::engine::general_purpose::STANDARD.encode(&enc);
    let _ = gloo_storage::LocalStorage::set(key, &b64);
}

#[cfg(target_arch = "wasm32")]
fn load_individual<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    use base64::Engine;
    let Ok(b64) = gloo_storage::LocalStorage::get::<String>(key) else { return None };
    if b64.is_empty() { return None; }
    let enc = base64::engine::general_purpose::STANDARD.decode(&b64).ok()?;
    let dec = crate::crypto::decrypt(&enc)?;
    let json = String::from_utf8(dec).ok()?;
    serde_json::from_str(&json).ok()
}

#[cfg(target_arch = "wasm32")]
fn load_all_individual<T: serde::de::DeserializeOwned>(prefix: &str) -> Vec<(String, T)> {
    let Some(window) = web_sys::window() else { return vec![] };
    let Ok(Some(storage)) = window.local_storage() else { return vec![] };
    let Ok(len) = storage.length() else { return vec![] };
    let mut results = Vec::new();
    for i in 0..len {
        if let Ok(Some(k)) = storage.key(i) {
            if k.starts_with(prefix) {
                if let Some(v) = load_individual::<T>(&k) {
                    results.push((k, v));
                }
            }
        }
    }
    results
}

#[cfg(target_arch = "wasm32")]
fn remove_individual(key: &str) {
    gloo_storage::LocalStorage::delete(key);
}

/// Migrate from old bulk-encrypted format to individual-key storage.
/// Called once during init().
#[cfg(target_arch = "wasm32")]
fn migrate_from_bulk() {
    use base64::Engine;

    // Persons
    let old_b64: Option<String> = gloo_storage::LocalStorage::get("pm_persons").ok();
    if let Some(ref b64) = old_b64 {
        if !b64.is_empty() {
            if let Ok(enc) = base64::engine::general_purpose::STANDARD.decode(b64) {
                if let Some(dec) = crate::crypto::decrypt(&enc) {
                    if let Ok(json) = String::from_utf8(dec) {
                        if let Ok(persons) = serde_json::from_str::<Vec<Person>>(&json) {
                            for p in &persons {
                                store_individual(&person_key(&p.id), p);
                            }
                        }
                    }
                }
            }
        }
        gloo_storage::LocalStorage::delete("pm_persons");
    }

    // Predictions
    let old_b64: Option<String> = gloo_storage::LocalStorage::get("pm_predictions").ok();
    if let Some(ref b64) = old_b64 {
        if !b64.is_empty() {
            if let Ok(enc) = base64::engine::general_purpose::STANDARD.decode(b64) {
                if let Some(dec) = crate::crypto::decrypt(&enc) {
                    if let Ok(json) = String::from_utf8(dec) {
                        if let Ok(preds) = serde_json::from_str::<Vec<Prediction>>(&json) {
                            for p in &preds {
                                store_individual(&prediction_key(&p.id), p);
                            }
                        }
                    }
                }
            }
        }
        gloo_storage::LocalStorage::delete("pm_predictions");
    }

    // Relationships
    let old_b64: Option<String> = gloo_storage::LocalStorage::get("pm_relationships").ok();
    if let Some(ref b64) = old_b64 {
        if !b64.is_empty() {
            if let Ok(enc) = base64::engine::general_purpose::STANDARD.decode(b64) {
                if let Some(dec) = crate::crypto::decrypt(&enc) {
                    if let Ok(json) = String::from_utf8(dec) {
                        if let Ok(rels) = serde_json::from_str::<Vec<Relationship>>(&json) {
                            for r in &rels {
                                store_individual(&relationship_key(&r.id), r);
                            }
                        }
                    }
                }
            }
        }
        gloo_storage::LocalStorage::delete("pm_relationships");
    }
}

#[cfg(target_arch = "wasm32")]
static PERSONS_CACHE: OnceLock<std::sync::Mutex<Option<Vec<Person>>>> = OnceLock::new();
#[cfg(target_arch = "wasm32")]
static PREDICTIONS_CACHE: OnceLock<std::sync::Mutex<Option<Vec<Prediction>>>> = OnceLock::new();
#[cfg(target_arch = "wasm32")]
static RELATIONSHIPS_CACHE: OnceLock<std::sync::Mutex<Option<Vec<Relationship>>>> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
fn with_persons_cache<F, R>(f: F) -> R
where
    F: FnOnce(&mut Option<Vec<Person>>) -> R,
{
    let lock = PERSONS_CACHE.get_or_init(|| std::sync::Mutex::new(None));
    let mut guard = lock.lock().unwrap();
    f(&mut guard)
}

#[cfg(target_arch = "wasm32")]
fn with_preds_cache<F, R>(f: F) -> R
where
    F: FnOnce(&mut Option<Vec<Prediction>>) -> R,
{
    let lock = PREDICTIONS_CACHE.get_or_init(|| std::sync::Mutex::new(None));
    let mut guard = lock.lock().unwrap();
    f(&mut guard)
}

#[cfg(target_arch = "wasm32")]
fn with_rels_cache<F, R>(f: F) -> R
where
    F: FnOnce(&mut Option<Vec<Relationship>>) -> R,
{
    let lock = RELATIONSHIPS_CACHE.get_or_init(|| std::sync::Mutex::new(None));
    let mut guard = lock.lock().unwrap();
    f(&mut guard)
}

#[cfg(target_arch = "wasm32")]
struct WebStorage;

#[cfg(target_arch = "wasm32")]
impl StorageBackend for WebStorage {
    fn load_all_persons(&self) -> Vec<Person> {
        with_persons_cache(|cache| {
            if let Some(ref cached) = *cache {
                return cached.clone();
            }
            let items = load_all_individual::<Person>("person_");
            let persons: Vec<Person> = items.into_iter().map(|(_, p)| p).collect();
            *cache = Some(persons.clone());
            persons
        })
    }
    fn load_person(&self, id: &str) -> Option<Person> {
        // Try cache first
        let cached = with_persons_cache(|cache| cache.clone());
        if let Some(ref persons) = cached {
            if let Some(p) = persons.iter().find(|p| p.id == id) {
                return Some(p.clone());
            }
        }
        // Direct single-key lookup
        load_individual(&person_key(id))
    }
    fn save_person(&self, person: &Person) {
        store_individual(&person_key(&person.id), person);
        with_persons_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, person);
            *cache = Some(all);
        });
    }
    fn delete_person(&self, id: &str) {
        remove_individual(&person_key(id));
        with_persons_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|p| p.id != id);
            }
        });
    }
    fn load_all_predictions(&self) -> Vec<Prediction> {
        with_preds_cache(|cache| {
            if let Some(ref cached) = *cache {
                return cached.clone();
            }
            let items = load_all_individual::<Prediction>("pred_");
            let preds: Vec<Prediction> = items.into_iter().map(|(_, p)| p).collect();
            *cache = Some(preds.clone());
            preds
        })
    }
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction> {
        self.load_all_predictions()
            .into_iter()
            .filter(|p| p.person_id == person_id)
            .collect()
    }
    fn save_prediction(&self, prediction: &Prediction) {
        store_individual(&prediction_key(&prediction.id), prediction);
        with_preds_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, prediction);
            *cache = Some(all);
        });
    }
    fn delete_prediction(&self, id: &str) {
        remove_individual(&prediction_key(id));
        with_preds_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|p| p.id != id);
            }
        });
    }
    fn load_all_relationships(&self) -> Vec<Relationship> {
        with_rels_cache(|cache| {
            if let Some(ref cached) = *cache {
                return cached.clone();
            }
            let items = load_all_individual::<Relationship>("rel_");
            let rels: Vec<Relationship> = items.into_iter().map(|(_, r)| r).collect();
            *cache = Some(rels.clone());
            rels
        })
    }
    fn save_relationship(&self, relationship: &Relationship) {
        store_individual(&relationship_key(&relationship.id), relationship);
        with_rels_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, relationship);
            *cache = Some(all);
        });
    }
    fn delete_relationship(&self, id: &str) {
        remove_individual(&relationship_key(id));
        with_rels_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|r| r.id != id);
            }
        });
    }
}

// ---- SQLite Storage (Native) ----
#[cfg(not(target_arch = "wasm32"))]
struct SqliteStorage {
    conn: std::sync::Mutex<rusqlite::Connection>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SqliteStorage {
    fn new() -> Self {
        #[cfg(target_os = "android")]
        let path = {
            let dir = "/data/data/com.stellasecret.peoplemodeler/files";
            let _ = std::fs::create_dir_all(dir);
            format!("{dir}/peoplemodeler.db")
        };
        #[cfg(not(target_os = "android"))]
        let path = "peoplemodeler.db".to_string();
        let conn = rusqlite::Connection::open(&path).expect("Failed to open SQLite database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS persons (id TEXT PRIMARY KEY, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS predictions (id TEXT PRIMARY KEY, person_id TEXT NOT NULL, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS relationships (id TEXT PRIMARY KEY, data TEXT NOT NULL);",
        )
        .expect("Failed to initialize SQLite schema");
        Self {
            conn: std::sync::Mutex::new(conn),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StorageBackend for SqliteStorage {
    fn load_all_persons(&self) -> Vec<Person> {
        let Ok(conn) = self.conn.lock() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare("SELECT data FROM persons") else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).ok())
        }) else {
            return Vec::new();
        };
        rows.filter_map(|r| r.ok().and_then(|x| x)).collect()
    }
    fn load_person(&self, id: &str) -> Option<Person> {
        let Ok(conn) = self.conn.lock() else {
            return None;
        };
        conn.query_row("SELECT data FROM persons WHERE id = ?1", [id], |row| {
            let data: String = row.get(0)?;
            serde_json::from_str(&data)
                .map_err(|_| rusqlite::Error::ToSqlConversionFailure(Box::new(std::fmt::Error)))
        })
        .ok()
    }
    fn save_person(&self, person: &Person) {
        let Ok(conn) = self.conn.lock() else { return };
        let Ok(data) = serde_json::to_string(person) else {
            return;
        };
        let _ = conn.execute(
            "INSERT OR REPLACE INTO persons (id, data) VALUES (?1, ?2)",
            [&person.id, &data],
        );
    }
    fn delete_person(&self, id: &str) {
        let Ok(conn) = self.conn.lock() else { return };
        let _ = conn.execute("DELETE FROM persons WHERE id = ?1", [id]);
        let _ = conn.execute("DELETE FROM predictions WHERE person_id = ?1", [id]);
    }
    fn load_all_predictions(&self) -> Vec<Prediction> {
        let Ok(conn) = self.conn.lock() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare("SELECT data FROM predictions") else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).ok())
        }) else {
            return Vec::new();
        };
        rows.filter_map(|r| r.ok().and_then(|x| x)).collect()
    }
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction> {
        let Ok(conn) = self.conn.lock() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare("SELECT data FROM predictions WHERE person_id = ?1") else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([person_id], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).ok())
        }) else {
            return Vec::new();
        };
        rows.filter_map(|r| r.ok().and_then(|x| x)).collect()
    }
    fn save_prediction(&self, prediction: &Prediction) {
        let Ok(conn) = self.conn.lock() else { return };
        let Ok(data) = serde_json::to_string(prediction) else {
            return;
        };
        let _ = conn.execute(
            "INSERT OR REPLACE INTO predictions (id, person_id, data) VALUES (?1, ?2, ?3)",
            [&prediction.id, &prediction.person_id, &data],
        );
    }
    fn delete_prediction(&self, id: &str) {
        let Ok(conn) = self.conn.lock() else { return };
        let _ = conn.execute("DELETE FROM predictions WHERE id = ?1", [id]);
    }
    fn load_all_relationships(&self) -> Vec<Relationship> {
        let Ok(conn) = self.conn.lock() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare("SELECT data FROM relationships") else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).ok())
        }) else {
            return Vec::new();
        };
        rows.filter_map(|r| r.ok().and_then(|x| x)).collect()
    }
    fn save_relationship(&self, relationship: &Relationship) {
        let Ok(conn) = self.conn.lock() else { return };
        let Ok(data) = serde_json::to_string(relationship) else {
            return;
        };
        let _ = conn.execute(
            "INSERT OR REPLACE INTO relationships (id, data) VALUES (?1, ?2)",
            [&relationship.id, &data],
        );
    }
    fn delete_relationship(&self, id: &str) {
        let Ok(conn) = self.conn.lock() else { return };
        let _ = conn.execute("DELETE FROM relationships WHERE id = ?1", [id]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::{
        BehaviorResponse, BehavioralPattern, BehaviorTrigger, Bias, BiasType, Motivation, MotivationType,
        OceanScores, Person, Prediction, RelationType, Relationship, RepScores, Tag,
    };

    fn test_db() -> SqliteStorage {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS persons (id TEXT PRIMARY KEY, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS predictions (id TEXT PRIMARY KEY, person_id TEXT NOT NULL, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS relationships (id TEXT PRIMARY KEY, data TEXT NOT NULL);",
        )
        .unwrap();
        SqliteStorage {
            conn: std::sync::Mutex::new(conn),
        }
    }

    fn sample_person(id: &str) -> Person {
        Person {
            id: id.into(),
            name: "Test Person".into(),
            role: "Engineer".into(),
            context: "test".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![Tag { name: "tag1".into(), color: None }],
            notes: "some notes".into(),
            motivations: vec![Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            }],
            biases: vec![Bias {
                r#type: BiasType::Confirmation,
                intensity: 6,
                evidence: String::new(),
            }],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            ocean: OceanScores::default(),
            confidence: 5,
            log: vec![],
            created_at: 100,
            updated_at: 200,
        }
    }

    fn sample_prediction(person_id: &str) -> Prediction {
        Prediction {
            id: "pred-1".into(),
            person_id: person_id.into(),
            context: "meeting".into(),
            predicted_outcome: "will agree".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 300,
            resolved_at: None,
            resolved: false,
        }
    }

    fn sample_relationship() -> Relationship {
        Relationship {
            id: "rel-1".into(),
            source_id: "src-1".into(),
            target_id: "tgt-1".into(),
            r#type: RelationType::WorksWith,
            notes: String::new(),
            created_at: 400,
        }
    }

    // --- Person CRUD ---

    #[test]
    fn test_save_and_load_person() {
        let db = test_db();
        let p = sample_person("p1");
        db.save_person(&p);
        let loaded = db.load_person("p1").unwrap();
        assert_eq!(loaded.name, "Test Person");
        assert_eq!(loaded.id, "p1");
        assert_eq!(loaded.role, "Engineer");
        assert_eq!(loaded.motivations.len(), 1);
        assert_eq!(loaded.biases.len(), 1);
    }

    #[test]
    fn test_update_person() {
        let db = test_db();
        let mut p = sample_person("p-upd");
        db.save_person(&p);
        p.name = "Updated Name".into();
        db.save_person(&p);
        let loaded = db.load_person("p-upd").unwrap();
        assert_eq!(loaded.name, "Updated Name");
    }

    #[test]
    fn test_delete_person() {
        let db = test_db();
        let p = sample_person("p-del");
        db.save_person(&p);
        assert!(db.load_person("p-del").is_some());
        db.delete_person("p-del");
        assert!(db.load_person("p-del").is_none());
    }

    #[test]
    fn test_load_nonexistent_person() {
        let db = test_db();
        assert!(db.load_person("no-such-id").is_none());
    }

    #[test]
    fn test_all_persons() {
        let db = test_db();
        assert!(db.load_all_persons().is_empty());
        db.save_person(&sample_person("a"));
        db.save_person(&sample_person("b"));
        db.save_person(&sample_person("c"));
        let all = db.load_all_persons();
        assert_eq!(all.len(), 3);
        let ids: Vec<&str> = all.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"a"));
        assert!(ids.contains(&"b"));
        assert!(ids.contains(&"c"));
    }

    #[test]
    fn test_person_with_all_fields_roundtrip() {
        let db = test_db();
        let p = Person {
            id: "full".into(),
            name: "Full Person".into(),
            role: "Manager".into(),
            context: "pro".into(),
            avatar_emoji: "🎯".into(),
            tags: vec![
                Tag { name: "alpha".into(), color: None },
                Tag { name: "beta".into(), color: Some("#ff0".into()) },
            ],
            notes: "detailed notes".into(),
            motivations: vec![
                Motivation { r#type: MotivationType::Power, intensity: 9, notes: "driven".into() },
                Motivation { r#type: MotivationType::Learning, intensity: 7, notes: "curious".into() },
            ],
            biases: vec![
                Bias { r#type: BiasType::Anchoring, intensity: 8, evidence: "first impressions".into() },
                Bias { r#type: BiasType::LossAversion, intensity: 6, evidence: "risk averse".into() },
            ],
            rep_scores: RepScores {
                hardworker_lazy: Some(9),
                honest_deceitful: Some(7),
                ..RepScores::default()
            },
            behavioral_patterns: vec![BehavioralPattern {
                trigger: peoplemodeler_core::models::BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                intensity: 6,
            }],
            ocean: OceanScores {
                openness: Some(8),
                conscientiousness: Some(7),
                extraversion: Some(6),
                agreeableness: Some(5),
                neuroticism: Some(4),
            },
            confidence: 7,
            log: vec![],
            created_at: 1000,
            updated_at: 2000,
        };
        db.save_person(&p);
        let loaded = db.load_person("full").unwrap();
        assert_eq!(loaded.name, "Full Person");
        assert_eq!(loaded.tags.len(), 2);
        assert_eq!(loaded.tags[1].color.as_deref(), Some("#ff0"));
        assert_eq!(loaded.motivations.len(), 2);
        assert_eq!(loaded.biases.len(), 2);
        assert_eq!(loaded.rep_scores.hardworker_lazy, Some(9));
        assert_eq!(loaded.behavioral_patterns.len(), 1);
        assert_eq!(loaded.ocean.openness, Some(8));
        assert_eq!(loaded.confidence, 7);
        assert_eq!(loaded.created_at, 1000);
    }

    // --- Prediction CRUD ---

    #[test]
    fn test_save_and_load_prediction() {
        let db = test_db();
        let p = sample_prediction("p1");
        db.save_prediction(&p);
        let all = db.load_all_predictions();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].person_id, "p1");
    }

    #[test]
    fn test_predictions_for_person() {
        let db = test_db();
        db.save_prediction(&Prediction { id: "pred-1".into(), ..sample_prediction("p1") });
        db.save_prediction(&Prediction { id: "pred-2".into(), person_id: "p1".into(), ..sample_prediction("p1") });
        db.save_prediction(&Prediction { id: "pred-3".into(), ..sample_prediction("p2") });
        let for_p1 = db.load_predictions_for_person("p1");
        assert_eq!(for_p1.len(), 2);
        let for_p2 = db.load_predictions_for_person("p2");
        assert_eq!(for_p2.len(), 1);
        let for_p3 = db.load_predictions_for_person("p3");
        assert!(for_p3.is_empty());
    }

    #[test]
    fn test_delete_prediction() {
        let db = test_db();
        db.save_prediction(&sample_prediction("p1"));
        assert_eq!(db.load_all_predictions().len(), 1);
        db.delete_prediction("pred-1");
        assert!(db.load_all_predictions().is_empty());
    }

    #[test]
    fn test_delete_person_cascades_to_predictions() {
        let db = test_db();
        db.save_person(&sample_person("p-cascade"));
        db.save_prediction(&sample_prediction("p-cascade"));
        db.save_prediction(&Prediction {
            id: "pred-c2".into(),
            person_id: "p-cascade".into(),
            ..sample_prediction("p-cascade")
        });
        assert_eq!(db.load_all_predictions().len(), 2);
        db.delete_person("p-cascade");
        assert!(db.load_person("p-cascade").is_none());
        assert_eq!(db.load_all_predictions().len(), 0);
    }

    // --- Relationship CRUD ---

    #[test]
    fn test_save_and_load_relationship() {
        let db = test_db();
        let r = sample_relationship();
        db.save_relationship(&r);
        let all = db.load_all_relationships();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, "rel-1");
        assert_eq!(all[0].r#type, RelationType::WorksWith);
    }

    #[test]
    fn test_delete_relationship() {
        let db = test_db();
        db.save_relationship(&sample_relationship());
        assert_eq!(db.load_all_relationships().len(), 1);
        db.delete_relationship("rel-1");
        assert!(db.load_all_relationships().is_empty());
    }

    // --- Edge cases ---

    #[test]
    fn test_empty_db() {
        let db = test_db();
        assert!(db.load_all_persons().is_empty());
        assert!(db.load_all_predictions().is_empty());
        assert!(db.load_all_relationships().is_empty());
    }

    #[test]
    fn test_upsert_same_prediction_id() {
        let db = test_db();
        let mut p = sample_prediction("p1");
        db.save_prediction(&p);
        p.context = "updated context".into();
        p.predicted_outcome = "updated outcome".into();
        db.save_prediction(&p);
        let all = db.load_all_predictions();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].context, "updated context");
    }

    #[test]
    fn test_delete_nonexistent() {
        let db = test_db();
        db.delete_person("no-one");
        db.delete_prediction("no-one");
        db.delete_relationship("no-one");
        // Should not panic
    }

    #[test]
    fn test_save_load_json_equivalent() {
        let db = test_db();
        let original = Person {
            id: "json-eq".into(),
            name: "JSON Compare".into(),
            role: "Tester".into(),
            context: "testing".into(),
            avatar_emoji: "🧪".into(),
            tags: vec![Tag { name: "verify".into(), color: None }],
            notes: "json roundtrip".into(),
            motivations: vec![Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: "test".into(),
            }],
            biases: vec![Bias {
                r#type: BiasType::Confirmation,
                intensity: 5,
                evidence: "checked".into(),
            }],
            rep_scores: RepScores {
                hardworker_lazy: Some(7),
                honest_deceitful: Some(6),
                authoritative_submissive: Some(5),
                reliable_flaky: None,
                humble_arrogant: None,
                calm_reactive: None,
                diplomatic_blunt: None,
                generous_selfish: None,
            },
            behavioral_patterns: vec![BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                intensity: 6,
            }],
            ocean: OceanScores {
                openness: Some(9),
                conscientiousness: Some(8),
                extraversion: Some(7),
                agreeableness: Some(6),
                neuroticism: Some(5),
            },
            confidence: 6,
            log: vec![],
            created_at: 10,
            updated_at: 20,
        };
        db.save_person(&original);
        let loaded = db.load_person("json-eq").unwrap();
        let orig_json = serde_json::to_value(&original).unwrap();
        let loaded_json = serde_json::to_value(&loaded).unwrap();
        assert_eq!(orig_json, loaded_json);
    }

    #[test]
    fn test_save_all_persons_json_match() {
        let db = test_db();
        db.save_person(&sample_person("a1"));
        db.save_person(&sample_person("b1"));
        let all = db.load_all_persons();
        let a1 = db.load_person("a1").unwrap();
        let b1 = db.load_person("b1").unwrap();
        assert!(all.iter().any(|p| serde_json::to_value(p).unwrap() == serde_json::to_value(&a1).unwrap()));
        assert!(all.iter().any(|p| serde_json::to_value(p).unwrap() == serde_json::to_value(&b1).unwrap()));
    }

    #[test]
    fn test_prediction_roundtrip_json() {
        let db = test_db();
        let p = sample_prediction("p1");
        db.save_prediction(&p);
        let loaded = db.load_all_predictions();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            serde_json::to_value(&p).unwrap(),
            serde_json::to_value(&loaded[0]).unwrap()
        );
    }

    #[test]
    fn test_relationship_roundtrip_json() {
        let db = test_db();
        let r = sample_relationship();
        db.save_relationship(&r);
        let loaded = db.load_all_relationships();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            serde_json::to_value(&r).unwrap(),
            serde_json::to_value(&loaded[0]).unwrap()
        );
    }
}

use std::sync::OnceLock;

use peoplemodeler_core::models::Person;
use peoplemodeler_core::models::Prediction;
use peoplemodeler_core::models::Relationship;
use peoplemodeler_core::models::Team;

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
    fn save_person(&self, person: &Person) -> Result<(), String>;
    fn delete_person(&self, id: &str) -> Result<(), String>;
    fn load_all_predictions(&self) -> Vec<Prediction>;
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction>;
    fn save_prediction(&self, prediction: &Prediction) -> Result<(), String>;
    fn delete_prediction(&self, id: &str) -> Result<(), String>;
    fn load_all_relationships(&self) -> Vec<Relationship>;
    fn save_relationship(&self, relationship: &Relationship) -> Result<(), String>;
    fn delete_relationship(&self, id: &str) -> Result<(), String>;
    fn load_all_teams(&self) -> Vec<Team>;
    fn load_team(&self, id: &str) -> Option<Team>;
    fn save_team(&self, team: &Team) -> Result<(), String>;
    fn delete_team(&self, id: &str) -> Result<(), String>;
}

pub fn all_persons() -> Vec<Person> {
    db().load_all_persons()
}
pub fn person(id: &str) -> Option<Person> {
    db().load_person(id)
}
pub fn save_person(person: &Person) -> Result<(), String> {
    undo::push_snapshot();
    db().save_person(person)
}
pub(crate) fn save_person_quiet(person: &Person) {
    let _ = db().save_person(person);
}
pub(crate) fn save_prediction_quiet(prediction: &Prediction) {
    let _ = db().save_prediction(prediction);
}
pub(crate) fn save_relationship_quiet(relationship: &Relationship) {
    let _ = db().save_relationship(relationship);
}
pub fn delete_person(id: &str) -> Result<(), String> {
    undo::push_snapshot();
    db().delete_person(id)
}
pub fn all_predictions() -> Vec<Prediction> {
    db().load_all_predictions()
}
pub fn predictions_for_person(person_id: &str) -> Vec<Prediction> {
    db().load_predictions_for_person(person_id)
}
pub fn save_prediction(prediction: &Prediction) -> Result<(), String> {
    undo::push_snapshot();
    db().save_prediction(prediction)
}
pub fn delete_prediction(id: &str) -> Result<(), String> {
    undo::push_snapshot();
    db().delete_prediction(id)
}
pub fn all_relationships() -> Vec<Relationship> {
    db().load_all_relationships()
}
pub fn save_relationship(relationship: &Relationship) -> Result<(), String> {
    undo::push_snapshot();
    db().save_relationship(relationship)
}
pub fn delete_relationship(id: &str) -> Result<(), String> {
    undo::push_snapshot();
    db().delete_relationship(id)
}
pub fn all_teams() -> Vec<Team> {
    db().load_all_teams()
}
pub fn team(id: &str) -> Option<Team> {
    db().load_team(id)
}
pub fn save_team(team: &Team) -> Result<(), String> {
    db().save_team(team)
}
pub fn delete_team(id: &str) -> Result<(), String> {
    db().delete_team(id)
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
impl Identifiable for Team {
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
fn person_key(id: &str) -> String {
    format!("person_{id}")
}

#[cfg(target_arch = "wasm32")]
fn prediction_key(id: &str) -> String {
    format!("pred_{id}")
}

#[cfg(target_arch = "wasm32")]
fn relationship_key(id: &str) -> String {
    format!("rel_{id}")
}

#[cfg(target_arch = "wasm32")]
fn team_key(id: &str) -> String {
    format!("team_{id}")
}

#[cfg(target_arch = "wasm32")]
fn store_individual<T: serde::Serialize>(key: &str, val: &T) -> Result<(), String> {
    use base64::Engine;
    let json = serde_json::to_string(val).map_err(|e| e.to_string())?;
    let enc = crate::crypto::encrypt(json.as_bytes());
    let b64 = base64::engine::general_purpose::STANDARD.encode(&enc);
    gloo_storage::LocalStorage::set(key, &b64).map_err(|e| format!("WASM store error [{key}]: {e}"))
}

#[cfg(target_arch = "wasm32")]
fn load_individual<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    use base64::Engine;
    let Ok(b64) = gloo_storage::LocalStorage::get::<String>(key) else {
        return None;
    };
    if b64.is_empty() {
        return None;
    }
    let enc = base64::engine::general_purpose::STANDARD
        .decode(&b64)
        .ok()?;
    let dec = crate::crypto::decrypt(&enc)?;
    let json = String::from_utf8(dec).ok()?;
    serde_json::from_str(&json).ok()
}

#[cfg(target_arch = "wasm32")]
fn load_all_individual<T: serde::de::DeserializeOwned>(prefix: &str) -> Vec<(String, T)> {
    let Some(window) = web_sys::window() else {
        return vec![];
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return vec![];
    };
    let Ok(len) = storage.length() else {
        return vec![];
    };
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
                                let _ = store_individual(&person_key(&p.id), p);
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
                                let _ = store_individual(&prediction_key(&p.id), p);
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
                                let _ = store_individual(&relationship_key(&r.id), r);
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
static TEAMS_CACHE: OnceLock<std::sync::Mutex<Option<Vec<Team>>>> = OnceLock::new();

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
fn with_teams_cache<F, R>(f: F) -> R
where
    F: FnOnce(&mut Option<Vec<Team>>) -> R,
{
    let lock = TEAMS_CACHE.get_or_init(|| std::sync::Mutex::new(None));
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
    fn save_person(&self, person: &Person) -> Result<(), String> {
        store_individual(&person_key(&person.id), person)?;
        with_persons_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, person);
            *cache = Some(all);
        });
        Ok(())
    }
    fn delete_person(&self, id: &str) -> Result<(), String> {
        remove_individual(&person_key(id));
        with_persons_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|p| p.id != id);
            }
        });
        Ok(())
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
    fn save_prediction(&self, prediction: &Prediction) -> Result<(), String> {
        store_individual(&prediction_key(&prediction.id), prediction)?;
        with_preds_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, prediction);
            *cache = Some(all);
        });
        Ok(())
    }
    fn delete_prediction(&self, id: &str) -> Result<(), String> {
        remove_individual(&prediction_key(id));
        with_preds_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|p| p.id != id);
            }
        });
        Ok(())
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
    fn save_relationship(&self, relationship: &Relationship) -> Result<(), String> {
        store_individual(&relationship_key(&relationship.id), relationship)?;
        with_rels_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, relationship);
            *cache = Some(all);
        });
        Ok(())
    }
    fn delete_relationship(&self, id: &str) -> Result<(), String> {
        remove_individual(&relationship_key(id));
        with_rels_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|r| r.id != id);
            }
        });
        Ok(())
    }
    fn load_all_teams(&self) -> Vec<Team> {
        with_teams_cache(|cache| {
            if let Some(ref cached) = *cache {
                return cached.clone();
            }
            let items = load_all_individual::<Team>("team_");
            let teams: Vec<Team> = items.into_iter().map(|(_, t)| t).collect();
            *cache = Some(teams.clone());
            teams
        })
    }
    fn load_team(&self, id: &str) -> Option<Team> {
        let cached = with_teams_cache(|cache| cache.clone());
        if let Some(ref teams) = cached {
            if let Some(t) = teams.iter().find(|t| t.id == id) {
                return Some(t.clone());
            }
        }
        load_individual(&team_key(id))
    }
    fn save_team(&self, team: &Team) -> Result<(), String> {
        store_individual(&team_key(&team.id), team)?;
        with_teams_cache(|cache| {
            let mut all = cache.clone().unwrap_or_default();
            upsert(&mut all, team);
            *cache = Some(all);
        });
        Ok(())
    }
    fn delete_team(&self, id: &str) -> Result<(), String> {
        remove_individual(&team_key(id));
        with_teams_cache(|cache| {
            if let Some(ref mut all) = *cache {
                all.retain(|t| t.id != id);
            }
        });
        Ok(())
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
        conn.busy_timeout(std::time::Duration::from_secs(120))
            .expect("Failed to set SQLite busy timeout");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS persons (id TEXT PRIMARY KEY, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS predictions (id TEXT PRIMARY KEY, person_id TEXT NOT NULL, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS relationships (id TEXT PRIMARY KEY, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS teams (id TEXT PRIMARY KEY, data TEXT NOT NULL);",
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
    fn save_person(&self, person: &Person) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let data = serde_json::to_string(person).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO persons (id, data) VALUES (?1, ?2)",
            [&person.id, &data],
        )
        .map_err(|e| format!("DB write error [save_person {}]: {e}", person.id))?;
        Ok(())
    }
    fn delete_person(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM persons WHERE id = ?1", [id])
            .map_err(|e| format!("DB write error [delete_person {id}]: {e}"))?;
        conn.execute("DELETE FROM predictions WHERE person_id = ?1", [id])
            .map_err(|e| format!("DB write error [delete_person predictions {id}]: {e}"))?;
        Ok(())
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
    fn save_prediction(&self, prediction: &Prediction) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let data = serde_json::to_string(prediction).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO predictions (id, person_id, data) VALUES (?1, ?2, ?3)",
            [&prediction.id, &prediction.person_id, &data],
        )
        .map_err(|e| format!("DB write error [save_prediction {}]: {e}", prediction.id))?;
        Ok(())
    }
    fn delete_prediction(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM predictions WHERE id = ?1", [id])
            .map_err(|e| format!("DB write error [delete_prediction {id}]: {e}"))?;
        Ok(())
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
    fn save_relationship(&self, relationship: &Relationship) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let data = serde_json::to_string(relationship).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO relationships (id, data) VALUES (?1, ?2)",
            [&relationship.id, &data],
        )
        .map_err(|e| {
            format!(
                "DB write error [save_relationship {}]: {e}",
                relationship.id
            )
        })?;
        Ok(())
    }
    fn delete_relationship(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM relationships WHERE id = ?1", [id])
            .map_err(|e| format!("DB write error [delete_relationship {id}]: {e}"))?;
        Ok(())
    }
    fn load_all_teams(&self) -> Vec<Team> {
        let Ok(conn) = self.conn.lock() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare("SELECT data FROM teams") else {
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
    fn load_team(&self, id: &str) -> Option<Team> {
        let Ok(conn) = self.conn.lock() else {
            return None;
        };
        conn.query_row("SELECT data FROM teams WHERE id = ?1", [id], |row| {
            let data: String = row.get(0)?;
            serde_json::from_str(&data)
                .map_err(|_| rusqlite::Error::ToSqlConversionFailure(Box::new(std::fmt::Error)))
        })
        .ok()
    }
    fn save_team(&self, team: &Team) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let data = serde_json::to_string(team).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO teams (id, data) VALUES (?1, ?2)",
            [&team.id, &data],
        )
        .map_err(|e| format!("DB write error [save_team {}]: {e}", team.id))?;
        Ok(())
    }
    fn delete_team(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM teams WHERE id = ?1", [id])
            .map_err(|e| format!("DB write error [delete_team {id}]: {e}"))?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use peoplemodeler_core::models::{
        BehaviorResponse, BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation,
        MotivationType, OceanScores, Person, Prediction, RelationType, Relationship, RepScores,
        Tag, Team,
    };

    fn test_db() -> SqliteStorage {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS persons (id TEXT PRIMARY KEY, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS predictions (id TEXT PRIMARY KEY, person_id TEXT NOT NULL, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS relationships (id TEXT PRIMARY KEY, data TEXT NOT NULL);
              CREATE TABLE IF NOT EXISTS teams (id TEXT PRIMARY KEY, data TEXT NOT NULL);",
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
            tags: vec![Tag {
                name: "tag1".into(),
                color: None,
            }],
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
            styles: vec![],
            values: vec![],
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
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
            strength: 5,
            notes: String::new(),
            created_at: 400,
        }
    }

    // --- Person CRUD ---

    #[test]
    fn test_save_and_load_person() {
        let db = test_db();
        let p = sample_person("p1");
        let _ = db.save_person(&p);
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
        let _ = db.save_person(&p);
        p.name = "Updated Name".into();
        let _ = db.save_person(&p);
        let loaded = db.load_person("p-upd").unwrap();
        assert_eq!(loaded.name, "Updated Name");
    }

    #[test]
    fn test_delete_person() {
        let db = test_db();
        let p = sample_person("p-del");
        let _ = db.save_person(&p);
        assert!(db.load_person("p-del").is_some());
        let _ = db.delete_person("p-del");
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
        let _ = db.save_person(&sample_person("a"));
        let _ = db.save_person(&sample_person("b"));
        let _ = db.save_person(&sample_person("c"));
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
                Tag {
                    name: "alpha".into(),
                    color: None,
                },
                Tag {
                    name: "beta".into(),
                    color: Some("#ff0".into()),
                },
            ],
            notes: "detailed notes".into(),
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Power,
                    intensity: 9,
                    notes: "driven".into(),
                },
                Motivation {
                    r#type: MotivationType::Learning,
                    intensity: 7,
                    notes: "curious".into(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::Anchoring,
                    intensity: 8,
                    evidence: "first impressions".into(),
                },
                Bias {
                    r#type: BiasType::LossAversion,
                    intensity: 6,
                    evidence: "risk averse".into(),
                },
            ],
            rep_scores: RepScores {
                hardworker_lazy: Some(9),
                honest_deceitful: Some(7),
                ..RepScores::default()
            },
            behavioral_patterns: vec![BehavioralPattern {
                trigger: peoplemodeler_core::models::BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                notes: String::new(),
            }],
            styles: vec![],
            values: vec![],
            ocean: OceanScores {
                openness: Some(8),
                conscientiousness: Some(7),
                extraversion: Some(6),
                agreeableness: Some(5),
                neuroticism: Some(4),
            },
            resilience: None,
            risk_appetite: None,
            confidence: 7,
            log: vec![],
            created_at: 1000,
            updated_at: 2000,
        };
        let _ = db.save_person(&p);
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
        let _ = db.save_prediction(&p);
        let all = db.load_all_predictions();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].person_id, "p1");
    }

    #[test]
    fn test_predictions_for_person() {
        let db = test_db();
        let _ = db.save_prediction(&Prediction {
            id: "pred-1".into(),
            ..sample_prediction("p1")
        });
        let _ = db.save_prediction(&Prediction {
            id: "pred-2".into(),
            person_id: "p1".into(),
            ..sample_prediction("p1")
        });
        let _ = db.save_prediction(&Prediction {
            id: "pred-3".into(),
            ..sample_prediction("p2")
        });
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
        let _ = db.save_prediction(&sample_prediction("p1"));
        assert_eq!(db.load_all_predictions().len(), 1);
        let _ = db.delete_prediction("pred-1");
        assert!(db.load_all_predictions().is_empty());
    }

    #[test]
    fn test_delete_person_cascades_to_predictions() {
        let db = test_db();
        let _ = db.save_person(&sample_person("p-cascade"));
        let _ = db.save_prediction(&sample_prediction("p-cascade"));
        let _ = db.save_prediction(&Prediction {
            id: "pred-c2".into(),
            person_id: "p-cascade".into(),
            ..sample_prediction("p-cascade")
        });
        assert_eq!(db.load_all_predictions().len(), 2);
        let _ = db.delete_person("p-cascade");
        assert!(db.load_person("p-cascade").is_none());
        assert_eq!(db.load_all_predictions().len(), 0);
    }

    // --- Relationship CRUD ---

    #[test]
    fn test_save_and_load_relationship() {
        let db = test_db();
        let r = sample_relationship();
        let _ = db.save_relationship(&r);
        let all = db.load_all_relationships();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, "rel-1");
        assert_eq!(all[0].r#type, RelationType::WorksWith);
    }

    #[test]
    fn test_delete_relationship() {
        let db = test_db();
        let _ = db.save_relationship(&sample_relationship());
        assert_eq!(db.load_all_relationships().len(), 1);
        let _ = db.delete_relationship("rel-1");
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
        let _ = db.save_prediction(&p);
        p.context = "updated context".into();
        p.predicted_outcome = "updated outcome".into();
        let _ = db.save_prediction(&p);
        let all = db.load_all_predictions();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].context, "updated context");
    }

    #[test]
    fn test_delete_nonexistent() {
        let db = test_db();
        let _ = db.delete_person("no-one");
        let _ = db.delete_prediction("no-one");
        let _ = db.delete_relationship("no-one");
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
            tags: vec![Tag {
                name: "verify".into(),
                color: None,
            }],
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
                fair_favoritism: None,
                trusting_suspicious: None,
                assertive_passive: None,
                empathetic_detached: None,
                adaptable_rigid: None,
            },
            behavioral_patterns: vec![BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                notes: String::new(),
            }],
            styles: vec![],
            values: vec![],
            ocean: OceanScores {
                openness: Some(9),
                conscientiousness: Some(8),
                extraversion: Some(7),
                agreeableness: Some(6),
                neuroticism: Some(5),
            },
            resilience: None,
            risk_appetite: None,
            confidence: 6,
            log: vec![],
            created_at: 10,
            updated_at: 20,
        };
        let _ = db.save_person(&original);
        let loaded = db.load_person("json-eq").unwrap();
        let orig_json = serde_json::to_value(&original).unwrap();
        let loaded_json = serde_json::to_value(&loaded).unwrap();
        assert_eq!(orig_json, loaded_json);
    }

    #[test]
    fn test_save_all_persons_json_match() {
        let db = test_db();
        let _ = db.save_person(&sample_person("a1"));
        let _ = db.save_person(&sample_person("b1"));
        let all = db.load_all_persons();
        let a1 = db.load_person("a1").unwrap();
        let b1 = db.load_person("b1").unwrap();
        assert!(
            all.iter()
                .any(|p| serde_json::to_value(p).unwrap() == serde_json::to_value(&a1).unwrap())
        );
        assert!(
            all.iter()
                .any(|p| serde_json::to_value(p).unwrap() == serde_json::to_value(&b1).unwrap())
        );
    }

    #[test]
    fn test_prediction_roundtrip_json() {
        let db = test_db();
        let p = sample_prediction("p1");
        let _ = db.save_prediction(&p);
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
        let _ = db.save_relationship(&r);
        let loaded = db.load_all_relationships();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            serde_json::to_value(&r).unwrap(),
            serde_json::to_value(&loaded[0]).unwrap()
        );
    }

    // --- Team CRUD ---

    fn sample_team(id: &str) -> Team {
        Team {
            id: id.into(),
            name: "Test Team".into(),
            icon: "🎯".into(),
            member_ids: vec!["p1".into(), "p2".into()],
            created_at: 500,
        }
    }

    #[test]
    fn test_save_and_load_team() {
        let db = test_db();
        let t = sample_team("t1");
        let _ = db.save_team(&t);
        let loaded = db.load_team("t1").unwrap();
        assert_eq!(loaded.name, "Test Team");
        assert_eq!(loaded.member_ids, vec!["p1", "p2"]);
    }

    #[test]
    fn test_update_team() {
        let db = test_db();
        let mut t = sample_team("t-upd");
        let _ = db.save_team(&t);
        t.name = "Renamed".into();
        t.member_ids.push("p3".into());
        let _ = db.save_team(&t);
        let loaded = db.load_team("t-upd").unwrap();
        assert_eq!(loaded.name, "Renamed");
        assert_eq!(loaded.member_ids.len(), 3);
    }

    #[test]
    fn test_delete_team() {
        let db = test_db();
        let t = sample_team("t-del");
        let _ = db.save_team(&t);
        assert!(db.load_team("t-del").is_some());
        let _ = db.delete_team("t-del");
        assert!(db.load_team("t-del").is_none());
    }

    #[test]
    fn test_all_teams() {
        let db = test_db();
        assert!(db.load_all_teams().is_empty());
        let _ = db.save_team(&sample_team("a"));
        let _ = db.save_team(&sample_team("b"));
        let all = db.load_all_teams();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_team_roundtrip_json() {
        let db = test_db();
        let t = sample_team("t-json");
        let _ = db.save_team(&t);
        let loaded = db.load_all_teams();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            serde_json::to_value(&t).unwrap(),
            serde_json::to_value(&loaded[0]).unwrap()
        );
    }

    // --- Public dispatch function tests (catches "replace fn -> default" mutants) ---

    fn init_global_db() {
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            if super::DB.get().is_none() {
                let _ = std::fs::remove_file("peoplemodeler.db");
                super::init();
            }
        });
    }

    fn cleanup_id(id: &str) {
        let _ = super::delete_person(id);
        let _ = super::delete_prediction(id);
        let _ = super::delete_relationship(id);
    }

    #[test]
    fn dispatch_all_persons_returns_saved() {
        init_global_db();
        cleanup_id("dsp-ap-1");
        let p = sample_person("dsp-ap-1");
        let _ = super::save_person(&p);
        let all = super::all_persons();
        assert!(all.iter().any(|x| x.id == "dsp-ap-1"));
    }

    #[test]
    fn dispatch_person_returns_saved() {
        init_global_db();
        cleanup_id("dsp-p-1");
        assert!(super::person("dsp-p-1").is_none());
        let p = sample_person("dsp-p-1");
        let _ = super::save_person(&p);
        let loaded = super::person("dsp-p-1");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Test Person");
    }

    #[test]
    fn dispatch_person_nonexistent_returns_none() {
        init_global_db();
        cleanup_id("no-such-id-dispatch");
        assert!(super::person("no-such-id-dispatch").is_none());
    }

    #[test]
    fn dispatch_delete_person_removes() {
        init_global_db();
        cleanup_id("dsp-dp-1");
        let p = sample_person("dsp-dp-1");
        let _ = super::save_person(&p);
        assert!(super::person("dsp-dp-1").is_some());
        let _ = super::delete_person("dsp-dp-1");
        assert!(super::person("dsp-dp-1").is_none());
    }

    #[test]
    fn dispatch_save_person_quiet_works() {
        init_global_db();
        cleanup_id("dsp-spq-1");
        assert!(super::person("dsp-spq-1").is_none());
        let p = sample_person("dsp-spq-1");
        super::save_person_quiet(&p);
        assert!(super::person("dsp-spq-1").is_some());
    }

    #[test]
    fn dispatch_all_predictions_returns_saved() {
        init_global_db();
        let _ = super::delete_prediction("dsp-apred-1");
        let pred = Prediction {
            id: "dsp-apred-1".into(),
            person_id: "dsp-apred-p1".into(),
            context: "dispatch test".into(),
            predicted_outcome: "will work".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 500,
            resolved_at: None,
            resolved: false,
        };
        let _ = super::save_prediction(&pred);
        let all = super::all_predictions();
        assert!(all.iter().any(|x| x.id == "dsp-apred-1"));
    }

    #[test]
    fn dispatch_predictions_for_person_filters() {
        init_global_db();
        let _ = super::delete_prediction("dsp-pfp-1");
        let _ = super::delete_prediction("dsp-pfp-2");
        let p1 = Prediction {
            id: "dsp-pfp-1".into(),
            person_id: "dsp-pfp-person".into(),
            context: "test".into(),
            predicted_outcome: "yes".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 600,
            resolved_at: None,
            resolved: false,
        };
        let p2 = Prediction {
            id: "dsp-pfp-2".into(),
            person_id: "dsp-pfp-other".into(),
            context: "test".into(),
            predicted_outcome: "no".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 601,
            resolved_at: None,
            resolved: false,
        };
        let _ = super::save_prediction(&p1);
        let _ = super::save_prediction(&p2);
        let filtered = super::predictions_for_person("dsp-pfp-person");
        assert!(filtered.iter().any(|x| x.id == "dsp-pfp-1"));
        assert!(!filtered.iter().any(|x| x.id == "dsp-pfp-2"));
    }

    #[test]
    fn dispatch_delete_prediction_removes() {
        init_global_db();
        let _ = super::delete_prediction("dsp-dpred-1");
        let pred = Prediction {
            id: "dsp-dpred-1".into(),
            person_id: "dsp-dpred-p1".into(),
            context: "test".into(),
            predicted_outcome: "y".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 700,
            resolved_at: None,
            resolved: false,
        };
        let _ = super::save_prediction(&pred);
        assert!(
            super::all_predictions()
                .iter()
                .any(|x| x.id == "dsp-dpred-1")
        );
        let _ = super::delete_prediction("dsp-dpred-1");
        assert!(
            !super::all_predictions()
                .iter()
                .any(|x| x.id == "dsp-dpred-1")
        );
    }

    #[test]
    fn dispatch_save_prediction_quiet_works() {
        init_global_db();
        let _ = super::delete_prediction("dsp-spredq-1");
        assert!(
            !super::all_predictions()
                .iter()
                .any(|x| x.id == "dsp-spredq-1")
        );
        let pred = Prediction {
            id: "dsp-spredq-1".into(),
            person_id: "p1".into(),
            context: "test".into(),
            predicted_outcome: "y".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 800,
            resolved_at: None,
            resolved: false,
        };
        super::save_prediction_quiet(&pred);
        assert!(
            super::all_predictions()
                .iter()
                .any(|x| x.id == "dsp-spredq-1")
        );
    }

    #[test]
    fn dispatch_all_relationships_returns_saved() {
        init_global_db();
        let _ = super::delete_relationship("dsp-arel-1");
        let r = Relationship {
            id: "dsp-arel-1".into(),
            source_id: "s1".into(),
            target_id: "t1".into(),
            r#type: RelationType::WorksWith,
            strength: 5,
            notes: String::new(),
            created_at: 900,
        };
        let _ = super::save_relationship(&r);
        let all = super::all_relationships();
        assert!(all.iter().any(|x| x.id == "dsp-arel-1"));
    }

    #[test]
    fn dispatch_delete_relationship_removes() {
        init_global_db();
        let _ = super::delete_relationship("dsp-drel-1");
        let r = Relationship {
            id: "dsp-drel-1".into(),
            source_id: "s2".into(),
            target_id: "t2".into(),
            r#type: RelationType::WorksWith,
            strength: 3,
            notes: String::new(),
            created_at: 1000,
        };
        let _ = super::save_relationship(&r);
        assert!(
            super::all_relationships()
                .iter()
                .any(|x| x.id == "dsp-drel-1")
        );
        let _ = super::delete_relationship("dsp-drel-1");
        assert!(
            !super::all_relationships()
                .iter()
                .any(|x| x.id == "dsp-drel-1")
        );
    }

    #[test]
    fn dispatch_save_relationship_quiet_works() {
        init_global_db();
        let _ = super::delete_relationship("dsp-srelq-1");
        assert!(
            !super::all_relationships()
                .iter()
                .any(|x| x.id == "dsp-srelq-1")
        );
        let r = Relationship {
            id: "dsp-srelq-1".into(),
            source_id: "s3".into(),
            target_id: "t3".into(),
            r#type: RelationType::WorksWith,
            strength: 4,
            notes: String::new(),
            created_at: 1100,
        };
        super::save_relationship_quiet(&r);
        assert!(
            super::all_relationships()
                .iter()
                .any(|x| x.id == "dsp-srelq-1")
        );
    }

    #[test]
    fn dispatch_all_teams_returns_saved() {
        init_global_db();
        let t = sample_team("dsp-ateam-1");
        let _ = super::save_team(&t);
        let all = super::all_teams();
        assert!(all.iter().any(|x| x.id == "dsp-ateam-1"));
    }

    #[test]
    fn dispatch_team_returns_saved() {
        init_global_db();
        let t = sample_team("dsp-team-1");
        let _ = super::save_team(&t);
        let loaded = super::team("dsp-team-1");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Test Team");
    }

    #[test]
    fn dispatch_team_nonexistent_returns_none() {
        init_global_db();
        assert!(super::team("no-such-team-dispatch").is_none());
    }

    #[test]
    fn dispatch_delete_team_removes() {
        init_global_db();
        let t = sample_team("dsp-dteam-1");
        let _ = super::save_team(&t);
        assert!(super::team("dsp-dteam-1").is_some());
        let _ = super::delete_team("dsp-dteam-1");
        assert!(super::team("dsp-dteam-1").is_none());
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_dispatch_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use peoplemodeler_core::models::{
        Bias, BiasType, Motivation, MotivationType, OceanScores, Person, Prediction, RelationType,
        Relationship, RepScores, Tag, Team,
    };

    fn init_db() {
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(super::init);
    }

    fn make_person(id: &str) -> Person {
        Person {
            id: id.into(),
            name: format!("Person {id}"),
            role: "Tester".into(),
            context: "test".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![Tag {
                name: "tag".into(),
                color: None,
            }],
            notes: String::new(),
            motivations: vec![Motivation {
                r#type: MotivationType::Achievement,
                intensity: 5,
                notes: String::new(),
            }],
            biases: vec![Bias {
                r#type: BiasType::Confirmation,
                intensity: 3,
                evidence: String::new(),
            }],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            confidence: 5,
            log: vec![],
            created_at: 100,
            updated_at: 200,
        }
    }

    fn make_prediction(id: &str, person_id: &str) -> Prediction {
        Prediction {
            id: id.into(),
            person_id: person_id.into(),
            context: "ctx".into(),
            predicted_outcome: "out".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 300,
            resolved_at: None,
            resolved: false,
        }
    }

    fn make_relationship(id: &str) -> Relationship {
        Relationship {
            id: id.into(),
            source_id: "s1".into(),
            target_id: "t1".into(),
            r#type: RelationType::WorksWith,
            strength: 5,
            notes: String::new(),
            created_at: 400,
        }
    }

    fn make_team(id: &str) -> Team {
        Team {
            id: id.into(),
            name: format!("Team {id}"),
            icon: "🎯".into(),
            member_ids: vec![],
            created_at: 500,
        }
    }

    // --- Person CRUD via public dispatch ---

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_and_load_person() {
        init_db();
        let p = make_person("w1");
        super::save_person(&p).unwrap();
        let loaded = super::person("w1").unwrap();
        assert_eq!(loaded.name, "Person w1");
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_all_persons_contains_saved() {
        init_db();
        let p = make_person("w2");
        super::save_person(&p).unwrap();
        let all = super::all_persons();
        assert!(all.iter().any(|x| x.id == "w2"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_person_nonexistent_is_none() {
        init_db();
        assert!(super::person("w-no-such").is_none());
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_delete_person_removes() {
        init_db();
        let p = make_person("w3");
        super::save_person(&p).unwrap();
        super::delete_person("w3").unwrap();
        assert!(super::person("w3").is_none());
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_person_quiet_works() {
        init_db();
        let p = make_person("w4");
        super::save_person_quiet(&p);
        assert!(super::person("w4").is_some());
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_person_update_via_save() {
        init_db();
        let mut p = make_person("w5");
        super::save_person(&p).unwrap();
        p.name = "Updated".into();
        super::save_person(&p).unwrap();
        let loaded = super::person("w5").unwrap();
        assert_eq!(loaded.name, "Updated");
    }

    // --- Prediction CRUD ---

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_and_load_predictions() {
        init_db();
        let pred = make_prediction("wp1", "p1");
        super::save_prediction(&pred).unwrap();
        let all = super::all_predictions();
        assert!(all.iter().any(|x| x.id == "wp1"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_predictions_for_person_filters() {
        init_db();
        let p1 = make_prediction("wp2a", "person-a");
        let p2 = make_prediction("wp2b", "person-b");
        super::save_prediction(&p1).unwrap();
        super::save_prediction(&p2).unwrap();
        let filtered = super::predictions_for_person("person-a");
        assert!(filtered.iter().any(|x| x.id == "wp2a"));
        assert!(!filtered.iter().any(|x| x.id == "wp2b"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_delete_prediction_removes() {
        init_db();
        let pred = make_prediction("wp3", "p3");
        super::save_prediction(&pred).unwrap();
        super::delete_prediction("wp3").unwrap();
        assert!(!super::all_predictions().iter().any(|x| x.id == "wp3"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_prediction_quiet_works() {
        init_db();
        let pred = make_prediction("wp4", "p4");
        super::save_prediction_quiet(&pred);
        assert!(super::all_predictions().iter().any(|x| x.id == "wp4"));
    }

    // --- Relationship CRUD ---

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_and_load_relationship() {
        init_db();
        let r = make_relationship("wr1");
        super::save_relationship(&r).unwrap();
        let all = super::all_relationships();
        assert!(all.iter().any(|x| x.id == "wr1"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_delete_relationship_removes() {
        init_db();
        let r = make_relationship("wr2");
        super::save_relationship(&r).unwrap();
        super::delete_relationship("wr2").unwrap();
        assert!(!super::all_relationships().iter().any(|x| x.id == "wr2"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_relationship_quiet_works() {
        init_db();
        let r = make_relationship("wr3");
        super::save_relationship_quiet(&r);
        assert!(super::all_relationships().iter().any(|x| x.id == "wr3"));
    }

    // --- Team CRUD ---

    #[wasm_bindgen_test]
    fn wasm_dispatch_save_and_load_team() {
        init_db();
        let t = make_team("wt1");
        super::save_team(&t).unwrap();
        let loaded = super::team("wt1").unwrap();
        assert_eq!(loaded.name, "Team wt1");
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_all_teams_contains_saved() {
        init_db();
        let t = make_team("wt2");
        super::save_team(&t).unwrap();
        let all = super::all_teams();
        assert!(all.iter().any(|x| x.id == "wt2"));
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_team_nonexistent_is_none() {
        init_db();
        assert!(super::team("wt-no-such").is_none());
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_delete_team_removes() {
        init_db();
        let t = make_team("wt3");
        super::save_team(&t).unwrap();
        super::delete_team("wt3").unwrap();
        assert!(super::team("wt3").is_none());
    }

    // --- Upsert behavior (catches upsert → () and == → != mutations) ---

    #[wasm_bindgen_test]
    fn wasm_dispatch_upsert_person_does_not_duplicate() {
        init_db();
        let mut p = make_person("wu1");
        super::save_person(&p).unwrap();
        p.name = "Updated Name".into();
        super::save_person(&p).unwrap();
        let all = super::all_persons();
        assert_eq!(all.iter().filter(|x| x.id == "wu1").count(), 1);
        assert_eq!(super::person("wu1").unwrap().name, "Updated Name");
    }

    #[wasm_bindgen_test]
    fn wasm_dispatch_upsert_prediction_does_not_duplicate() {
        init_db();
        let mut pred = make_prediction("wup1", "p1");
        super::save_prediction(&pred).unwrap();
        pred.context = "updated".into();
        super::save_prediction(&pred).unwrap();
        let all = super::all_predictions();
        assert_eq!(all.iter().filter(|x| x.id == "wup1").count(), 1);
    }
}

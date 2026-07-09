use std::sync::OnceLock;

use peoplemodeler_core::models::Person;
use peoplemodeler_core::models::Prediction;
use peoplemodeler_core::models::Relationship;

use crate::undo;

static DB: OnceLock<Box<dyn StorageBackend + Send + Sync>> = OnceLock::new();

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    {
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
fn store_encrypted<T: serde::Serialize>(key: &str, val: &T) {
    use base64::Engine;
    let json = serde_json::to_string(val).expect("serialize");
    let enc = crate::crypto::encrypt(json.as_bytes());
    let b64 = base64::engine::general_purpose::STANDARD.encode(&enc);
    let _ = gloo_storage::LocalStorage::set(key, &b64);
}

#[cfg(target_arch = "wasm32")]
fn load_decrypted<T: serde::de::DeserializeOwned>(key: &str) -> Vec<T> {
    use base64::Engine;
    let b64: Option<String> = gloo_storage::LocalStorage::get(key).ok();
    let Some(b64) = b64 else { return Vec::new() };
    if b64.is_empty() {
        return Vec::new();
    }
    let Ok(enc) = base64::engine::general_purpose::STANDARD.decode(&b64) else {
        return Vec::new();
    };
    let Some(dec) = crate::crypto::decrypt(&enc) else {
        return Vec::new();
    };
    let Ok(json) = String::from_utf8(dec) else {
        return Vec::new();
    };
    serde_json::from_str(&json).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
struct WebStorage;

#[cfg(target_arch = "wasm32")]
impl StorageBackend for WebStorage {
    fn load_all_persons(&self) -> Vec<Person> {
        load_decrypted("pm_persons")
    }
    fn load_person(&self, id: &str) -> Option<Person> {
        self.load_all_persons().into_iter().find(|p| p.id == id)
    }
    fn save_person(&self, person: &Person) {
        let mut all: Vec<Person> = self.load_all_persons();
        upsert(&mut all, person);
        store_encrypted("pm_persons", &all);
    }
    fn delete_person(&self, id: &str) {
        let mut all: Vec<Person> = self.load_all_persons();
        all.retain(|p| p.id != id);
        store_encrypted("pm_persons", &all);
    }
    fn load_all_predictions(&self) -> Vec<Prediction> {
        load_decrypted("pm_predictions")
    }
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction> {
        self.load_all_predictions()
            .into_iter()
            .filter(|p| p.person_id == person_id)
            .collect()
    }
    fn save_prediction(&self, prediction: &Prediction) {
        let mut all: Vec<Prediction> = self.load_all_predictions();
        upsert(&mut all, prediction);
        store_encrypted("pm_predictions", &all);
    }
    fn delete_prediction(&self, id: &str) {
        let mut all: Vec<Prediction> = self.load_all_predictions();
        all.retain(|p| p.id != id);
        store_encrypted("pm_predictions", &all);
    }
    fn load_all_relationships(&self) -> Vec<Relationship> {
        load_decrypted("pm_relationships")
    }
    fn save_relationship(&self, relationship: &Relationship) {
        let mut all: Vec<Relationship> = self.load_all_relationships();
        upsert(&mut all, relationship);
        store_encrypted("pm_relationships", &all);
    }
    fn delete_relationship(&self, id: &str) {
        let mut all: Vec<Relationship> = self.load_all_relationships();
        all.retain(|r| r.id != id);
        store_encrypted("pm_relationships", &all);
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

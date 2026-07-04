use std::sync::OnceLock;

use peoplemodeler_core::models::Person;
use peoplemodeler_core::models::Prediction;

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
    DB.get().map(|b| b.as_ref()).expect("Storage not initialized. Call db::init() first.")
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
}

pub fn all_persons() -> Vec<Person> { db().load_all_persons() }
pub fn person(id: &str) -> Option<Person> { db().load_person(id) }
pub fn save_person(person: &Person) { db().save_person(person); }
pub fn delete_person(id: &str) { db().delete_person(id); }
pub fn all_predictions() -> Vec<Prediction> { db().load_all_predictions() }
pub fn predictions_for_person(person_id: &str) -> Vec<Prediction> { db().load_predictions_for_person(person_id) }
pub fn save_prediction(prediction: &Prediction) { db().save_prediction(prediction); }
pub fn delete_prediction(id: &str) { db().delete_prediction(id); }

#[cfg(target_arch = "wasm32")]
trait Identifiable {
    fn id(&self) -> &str;
}
#[cfg(target_arch = "wasm32")]
impl Identifiable for Person {
    fn id(&self) -> &str { &self.id }
}
#[cfg(target_arch = "wasm32")]
impl Identifiable for Prediction {
    fn id(&self) -> &str { &self.id }
}

#[cfg(target_arch = "wasm32")]
fn upsert<T: Clone + PartialEq>(vec: &mut Vec<T>, item: &T)
where T: Identifiable,
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
struct WebStorage;

#[cfg(target_arch = "wasm32")]
impl StorageBackend for WebStorage {
    fn load_all_persons(&self) -> Vec<Person> {
        gloo_storage::LocalStorage::get("pm_persons").unwrap_or_default()
    }
    fn load_person(&self, id: &str) -> Option<Person> {
        self.load_all_persons().into_iter().find(|p| p.id == id)
    }
    fn save_person(&self, person: &Person) {
        let mut all: Vec<Person> = self.load_all_persons();
        upsert(&mut all, person);
        gloo_storage::LocalStorage::set("pm_persons", &all).unwrap();
    }
    fn delete_person(&self, id: &str) {
        let mut all: Vec<Person> = self.load_all_persons();
        all.retain(|p| p.id != id);
        gloo_storage::LocalStorage::set("pm_persons", &all).unwrap();
    }
    fn load_all_predictions(&self) -> Vec<Prediction> {
        gloo_storage::LocalStorage::get("pm_predictions").unwrap_or_default()
    }
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction> {
        self.load_all_predictions().into_iter().filter(|p| p.person_id == person_id).collect()
    }
    fn save_prediction(&self, prediction: &Prediction) {
        let mut all: Vec<Prediction> = self.load_all_predictions();
        upsert(&mut all, prediction);
        gloo_storage::LocalStorage::set("pm_predictions", &all).unwrap();
    }
    fn delete_prediction(&self, id: &str) {
        let mut all: Vec<Prediction> = self.load_all_predictions();
        all.retain(|p| p.id != id);
        gloo_storage::LocalStorage::set("pm_predictions", &all).unwrap();
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
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS persons (id TEXT PRIMARY KEY, data TEXT NOT NULL);
             CREATE TABLE IF NOT EXISTS predictions (id TEXT PRIMARY KEY, person_id TEXT NOT NULL, data TEXT NOT NULL);",
        )
        .unwrap();
        Self { conn: std::sync::Mutex::new(conn) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StorageBackend for SqliteStorage {
    fn load_all_persons(&self) -> Vec<Person> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT data FROM persons").unwrap();
        stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).unwrap())
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }
    fn load_person(&self, id: &str) -> Option<Person> {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT data FROM persons WHERE id = ?1", [id], |row| {
            let data: String = row.get(0)?;
            serde_json::from_str(&data).map_err(|_| rusqlite::Error::ToSqlConversionFailure(Box::new(std::fmt::Error)))
        })
        .ok()
    }
    fn save_person(&self, person: &Person) {
        let conn = self.conn.lock().unwrap();
        let data = serde_json::to_string(person).unwrap();
        conn.execute("INSERT OR REPLACE INTO persons (id, data) VALUES (?1, ?2)", [&person.id, &data]).unwrap();
    }
    fn delete_person(&self, id: &str) {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM persons WHERE id = ?1", [id]).unwrap();
        conn.execute("DELETE FROM predictions WHERE person_id = ?1", [id]).unwrap();
    }
    fn load_all_predictions(&self) -> Vec<Prediction> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT data FROM predictions").unwrap();
        stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).unwrap())
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }
    fn load_predictions_for_person(&self, person_id: &str) -> Vec<Prediction> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT data FROM predictions WHERE person_id = ?1").unwrap();
        stmt.query_map([person_id], |row| {
            let data: String = row.get(0)?;
            Ok(serde_json::from_str(&data).unwrap())
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }
    fn save_prediction(&self, prediction: &Prediction) {
        let conn = self.conn.lock().unwrap();
        let data = serde_json::to_string(prediction).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO predictions (id, person_id, data) VALUES (?1, ?2, ?3)",
            [&prediction.id, &prediction.person_id, &data],
        ).unwrap();
    }
    fn delete_prediction(&self, id: &str) {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM predictions WHERE id = ?1", [id]).unwrap();
    }
}

use serde::{Deserialize, Serialize};

use peoplemodeler_core::models::{Person, Prediction};

use crate::db;

#[derive(Serialize, Deserialize)]
struct BackupData {
    version: u8,
    exported_at: i64,
    persons: Vec<Person>,
    predictions: Vec<Prediction>,
}

pub fn build_backup() -> String {
    let data = BackupData {
        version: 1,
        exported_at: chrono::Utc::now().timestamp_millis(),
        persons: db::all_persons(),
        predictions: db::all_predictions(),
    };
    serde_json::to_string_pretty(&data).expect("BackupData serialization failed")
}

pub fn restore_from_json(json: &str) -> Result<usize, String> {
    let data: BackupData =
        serde_json::from_str(json).map_err(|e| format!("Invalid backup: {e}"))?;
    let count = data.persons.len();
    for p in &data.persons {
        db::save_person(p);
    }
    for p in &data.predictions {
        db::save_prediction(p);
    }
    Ok(count)
}

#[cfg_attr(target_os = "android", allow(dead_code))]
pub const DRIVE_SCOPE: &str = "https://www.googleapis.com/auth/drive.appdata";
const DRIVE_API: &str = "https://www.googleapis.com/drive/v3/files";
const UPLOAD_API: &str = "https://www.googleapis.com/upload/drive/v3/files";

// Check HTTP response: if not 2xx, read body and return error with status + body
async fn check_response(resp: reqwest::Response) -> Result<reqwest::Response, String> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {status}: {body}"));
    }
    Ok(resp)
}

pub async fn drive_backup(token: &str, passphrase: Option<&str>) -> Result<String, String> {
    let backup = build_backup();
    let client = reqwest::Client::new();

    let (body_bytes, mime_type) = match passphrase.filter(|p| !p.is_empty()) {
        #[cfg(target_arch = "wasm32")]
        Some(pp) => {
            let enc = crate::crypto::encrypt_with_passphrase(backup.as_bytes(), pp);
            (enc, "application/octet-stream")
        }
        _ => (backup.into_bytes(), "application/json"),
    };

    let query =
        "name='people_modeler_backup.json' and 'appDataFolder' in parents and trashed=false";
    let search = check_response(
        client
            .get(DRIVE_API)
            .query(&[
                ("q", query),
                ("fields", "files(id)"),
                ("spaces", "appDataFolder"),
            ])
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?,
    )
    .await?;

    let search_body: serde_json::Value = search.json().await.map_err(|e| format!("json: {e}"))?;
    let file_id = search_body["files"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|f| f["id"].as_str().map(String::from));

    let fid = match file_id {
        Some(id) => id,
        None => {
            let meta = serde_json::json!({
                "name": "people_modeler_backup.json",
                "parents": ["appDataFolder"],
                "mimeType": mime_type
            });
            let resp = check_response(
                client
                    .post(DRIVE_API)
                    .header("Authorization", format!("Bearer {token}"))
                    .header("Content-Type", "application/json")
                    .body(meta.to_string())
                    .send()
                    .await
                    .map_err(|e| format!("send: {e}"))?,
            )
            .await?;
            let body: serde_json::Value = resp.json().await.map_err(|e| format!("json: {e}"))?;
            body["id"]
                .as_str()
                .map(String::from)
                .ok_or_else(|| format!("Failed to create Drive file: {}", body))?
        }
    };

    let url = format!("{UPLOAD_API}/{fid}?uploadType=media");
    check_response(
        client
            .patch(&url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", mime_type)
            .body(body_bytes)
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?,
    )
    .await?;
    Ok(fid)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
pub async fn drive_restore(token: &str, passphrase: Option<&str>) -> Result<usize, String> {
    let client = reqwest::Client::new();
    let query =
        "name='people_modeler_backup.json' and 'appDataFolder' in parents and trashed=false";

    let search = check_response(
        client
            .get(DRIVE_API)
            .query(&[
                ("q", query),
                ("fields", "files(id)"),
                ("spaces", "appDataFolder"),
            ])
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?,
    )
    .await?;

    let search_body: serde_json::Value = search.json().await.map_err(|e| format!("json: {e}"))?;
    let file_id = search_body["files"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|f| f["id"].as_str().map(String::from))
        .ok_or_else(|| "No backup found in Drive".to_string())?;

    let url = format!("{DRIVE_API}/{file_id}?alt=media");
    let resp = check_response(
        client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?,
    )
    .await?;

    let bytes = resp.bytes().await.map_err(|e| format!("bytes: {e}"))?;

    #[cfg(target_arch = "wasm32")]
    if let Some(pp) = passphrase.filter(|p| !p.is_empty()) {
        if !bytes.is_empty() && bytes[0] != b'{' {
            if let Some(dec) = crate::crypto::decrypt_with_passphrase(&bytes, pp) {
                let text = String::from_utf8(dec).map_err(|e| format!("utf8: {e}"))?;
                return restore_from_json(&text);
            }
            return Err("sync_wrong_passphrase".into());
        }
    }

    let text = String::from_utf8(bytes.to_vec()).map_err(|e| format!("utf8: {e}"))?;
    restore_from_json(&text)
}

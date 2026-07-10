use serde::{Deserialize, Serialize};

use peoplemodeler_core::models::{
    BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation, MotivationType, OceanScores,
    Person, Prediction, RepScores, Tag,
};

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

fn validate_backup(json: &str) -> Result<(), Vec<String>> {
    let val: serde_json::Value =
        serde_json::from_str(json).map_err(|e| vec![format!("Invalid JSON: {e}")])?;
    let mut errors = Vec::new();
    match &val["version"] {
        serde_json::Value::Null => errors.push("Missing required field: version".into()),
        serde_json::Value::Number(n) if !n.is_u64() => {
            errors.push("version must be a positive integer".into())
        }
        _ => {}
    }
    let persons = match &val["persons"] {
        serde_json::Value::Null => {
            errors.push("Missing required field: persons".into());
            None
        }
        serde_json::Value::Array(a) => Some(a),
        _ => {
            errors.push("persons must be an array".into());
            None
        }
    };
    if let Some(arr) = persons {
        for (i, p) in arr.iter().enumerate() {
            if p.get("id").and_then(|v| v.as_str()).is_none() {
                errors.push(format!("persons[{i}]: missing or invalid 'id'"));
            }
            if p.get("name").and_then(|v| v.as_str()).is_none() {
                errors.push(format!("persons[{i}]: missing or invalid 'name'"));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn restore_from_json(json: &str) -> Result<usize, String> {
    if let Err(errs) = validate_backup(json) {
        return Err(format!("Validation failed: {}", errs.join("; ")));
    }

    if let Ok(data) = serde_json::from_str::<BackupData>(json) {
        let count = data.persons.len();
        for p in &data.persons {
            db::save_person(p);
        }
        for p in &data.predictions {
            db::save_prediction(p);
        }
        return Ok(count);
    }

    let legacy: LegacyBackup = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let count = legacy.persons.len();
    for p in legacy.persons {
        db::save_person(&Person::from(p));
    }
    Ok(count)
}

// Legacy JS backup format: camelCase keys, flat ocean scores, no predictions
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyBackup {
    persons: Vec<LegacyPerson>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyPerson {
    id: String,
    name: String,
    role: String,
    context: String,
    avatar_emoji: String,
    tags: Vec<String>,
    notes: String,
    motivations: Vec<LegacyMotivation>,
    biases: Vec<LegacyBias>,
    behavioral_patterns: Vec<LegacyBehavioralPattern>,
    openness: u8,
    conscientiousness: u8,
    extraversion: u8,
    agreeableness: u8,
    neuroticism: u8,
    created_at: i64,
    updated_at: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyMotivation {
    r#type: MotivationType,
    intensity: u8,
    notes: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyBias {
    r#type: BiasType,
    intensity: u8,
    evidence: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyBehavioralPattern {
    trigger: BehaviorTrigger,
    predicted_behavior: String,
    confidence: u8,
}

impl From<LegacyPerson> for Person {
    fn from(lp: LegacyPerson) -> Self {
        Person {
            id: lp.id,
            name: lp.name,
            role: lp.role,
            context: lp.context,
            avatar_emoji: lp.avatar_emoji,
            tags: lp.tags.into_iter().map(|t| Tag { name: t, color: None }).collect(),
            notes: lp.notes,
            motivations: lp
                .motivations
                .into_iter()
                .map(|m| Motivation {
                    r#type: m.r#type,
                    intensity: m.intensity,
                    notes: m.notes,
                })
                .collect(),
            biases: lp
                .biases
                .into_iter()
                .map(|b| Bias {
                    r#type: b.r#type,
                    intensity: b.intensity,
                    evidence: b.evidence,
                })
                .collect(),
            rep_scores: RepScores::default(),
            behavioral_patterns: lp
                .behavioral_patterns
                .into_iter()
                .map(|bp| BehavioralPattern {
                    trigger: bp.trigger,
                    predicted_behavior: bp.predicted_behavior,
                    confidence: bp.confidence,
                })
                .collect(),
            ocean: OceanScores {
                openness: Some(lp.openness),
                conscientiousness: Some(lp.conscientiousness),
                extraversion: Some(lp.extraversion),
                agreeableness: Some(lp.agreeableness),
                neuroticism: Some(lp.neuroticism),
            },
            predictions: Vec::new(),
            confidence: 5,
            log: Vec::new(),
            created_at: lp.created_at,
            updated_at: lp.updated_at,
        }
    }
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

use serde::{Deserialize, Serialize};

use peoplemodeler_core::models::{
    BehavioralPattern, BehaviorTrigger, Bias, BiasType, Motivation, MotivationType, OceanScores,
    Person, Prediction,
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
    serde_json::to_string_pretty(&data).unwrap()
}

pub fn restore_from_json(json: &str) -> Result<usize, String> {
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
            tags: lp.tags,
            notes: lp.notes,
            motivations: lp.motivations.into_iter().map(|m| Motivation {
                r#type: m.r#type,
                intensity: m.intensity,
                notes: m.notes,
            }).collect(),
            biases: lp.biases.into_iter().map(|b| Bias {
                r#type: b.r#type,
                intensity: b.intensity,
                evidence: b.evidence,
            }).collect(),
            behavioral_patterns: lp.behavioral_patterns.into_iter().map(|bp| BehavioralPattern {
                trigger: bp.trigger,
                predicted_behavior: bp.predicted_behavior,
                confidence: bp.confidence,
            }).collect(),
            ocean: OceanScores {
                openness: lp.openness,
                conscientiousness: lp.conscientiousness,
                extraversion: lp.extraversion,
                agreeableness: lp.agreeableness,
                neuroticism: lp.neuroticism,
            },
            predictions: Vec::new(),
            confidence: 5,
            log: Vec::new(),
            created_at: lp.created_at,
            updated_at: lp.updated_at,
        }
    }
}

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

pub async fn drive_backup(token: &str) -> Result<String, String> {
    let backup = build_backup();
    let client = reqwest::Client::new();

    let query = "name='people_modeler_backup.json' and 'appDataFolder' in parents and trashed=false";
    let search = check_response(
        client
            .get(DRIVE_API)
            .query(&[("q", query), ("fields", "files(id)"), ("spaces", "appDataFolder")])
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?
    ).await?;

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
                "mimeType": "application/json"
            });
            let resp = check_response(
                client
                    .post(DRIVE_API)
                    .header("Authorization", format!("Bearer {token}"))
                    .header("Content-Type", "application/json")
                    .body(meta.to_string())
                    .send()
                    .await
                    .map_err(|e| format!("send: {e}"))?
            ).await?;
            let body: serde_json::Value = resp.json().await.map_err(|e| format!("json: {e}"))?;
            body["id"].as_str().map(String::from)
                .ok_or_else(|| format!("Failed to create Drive file: {}", body))?
        }
    };

    let url = format!("{UPLOAD_API}/{fid}?uploadType=media");
    check_response(
        client
            .patch(&url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .body(backup)
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?
    ).await?;
    Ok(fid)
}

pub async fn drive_restore(token: &str) -> Result<usize, String> {
    let client = reqwest::Client::new();
    let query = "name='people_modeler_backup.json' and 'appDataFolder' in parents and trashed=false";

    let search = check_response(
        client
            .get(DRIVE_API)
            .query(&[("q", query), ("fields", "files(id)"), ("spaces", "appDataFolder")])
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(|e| format!("send: {e}"))?
    ).await?;

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
            .map_err(|e| format!("send: {e}"))?
    ).await?;

    let text = resp.text().await.map_err(|e| format!("text: {e}"))?;
    restore_from_json(&text)
}

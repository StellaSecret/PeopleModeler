use serde::{Deserialize, Serialize};

use peoplemodeler_core::models::{Person, Prediction, Relationship};

use crate::db;

#[derive(Serialize, Deserialize)]
struct BackupData {
    version: u8,
    exported_at: i64,
    persons: Vec<Person>,
    predictions: Vec<Prediction>,
    #[serde(default)]
    relationships: Vec<Relationship>,
}

pub fn build_backup() -> String {
    let data = BackupData {
        version: 2,
        exported_at: chrono::Utc::now().timestamp_millis(),
        persons: db::all_persons(),
        predictions: db::all_predictions(),
        relationships: db::all_relationships(),
    };
    serde_json::to_string_pretty(&data).expect("BackupData serialization failed")
}

pub struct RestoreCount {
    pub persons: usize,
    pub relationships: usize,
}

pub fn restore_from_json(json: &str) -> Result<RestoreCount, String> {
    let data: BackupData =
        serde_json::from_str(json).map_err(|e| format!("Invalid backup: {e}"))?;
    for p in &data.persons {
        db::save_person(p).map_err(|e| format!("Restore failed (person {}): {e}", p.id))?;
    }
    for p in &data.predictions {
        db::save_prediction(p).map_err(|e| format!("Restore failed (prediction {}): {e}", p.id))?;
    }
    for r in &data.relationships {
        db::save_relationship(r)
            .map_err(|e| format!("Restore failed (relationship {}): {e}", r.id))?;
    }
    Ok(RestoreCount {
        persons: data.persons.len(),
        relationships: data.relationships.len(),
    })
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
pub async fn drive_restore(token: &str, passphrase: Option<&str>) -> Result<RestoreCount, String> {
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

#[allow(dead_code)]
fn mock_backup_json() -> &'static str {
    r#"{
  "version": 1,
  "exported_at": 1700000000000,
  "persons": [
    {
      "id": "mock-001",
      "name": "Alice",
      "role": "Engineer",
      "context": "test",
      "avatar_emoji": "🧑",
      "tags": [{"name": "alice-tag"}],
      "notes": "",
      "motivations": [
        {"type": "Achievement", "intensity": 8, "notes": "wins"}
      ],
      "biases": [
        {"type": "Confirmation", "intensity": 7, "evidence": ""}
      ],
      "rep_scores": {
        "hardworker_lazy": 8,
        "honest_deceitful": 6,
        "authoritative_submissive": null,
        "reliable_flaky": null,
        "humble_arrogant": null,
        "calm_reactive": null,
        "diplomatic_blunt": null,
        "generous_selfish": null
      },
      "behavioral_patterns": [
        {"trigger": "Stress", "predicted_behavior": "becomes_quiet", "confidence": 5}
      ],
      "ocean": {
        "openness": 5,
        "conscientiousness": 6,
        "extraversion": 7,
        "agreeableness": 8,
        "neuroticism": 3
      },
      "log": [],
      "predictions": [],
      "confidence": 5,
      "created_at": 0,
      "updated_at": 0
    },
    {
      "id": "mock-002",
      "name": "Bob",
      "role": "",
      "context": "",
      "avatar_emoji": "🧑",
      "tags": [
        {"name": "bob-tag-a"},
        {"name": "bob-tag-b"}
      ],
      "notes": "",
      "motivations": [],
      "biases": [],
      "rep_scores": {
        "hardworker_lazy": null,
        "authoritative_submissive": null,
        "honest_deceitful": null,
        "reliable_flaky": null,
        "humble_arrogant": null,
        "calm_reactive": null,
        "diplomatic_blunt": null,
        "generous_selfish": null
      },
      "behavioral_patterns": [],
      "ocean": {
        "openness": null,
        "conscientiousness": null,
        "extraversion": null,
        "agreeableness": null,
        "neuroticism": null
      },
      "log": [],
      "predictions": [],
      "confidence": 5,
      "created_at": 0,
      "updated_at": 0
    }
  ],
  "predictions": [
    {
      "id": "pred-mock",
      "person_id": "mock-001",
      "context": "sprint review",
      "predicted_outcome": "will ship",
      "actual_outcome": null,
      "accuracy": null,
      "created_at": 100,
      "resolved_at": null,
      "resolved": false
    }
  ]
}"#
}

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::{OceanScores, Person, RepScores, Tag};

    #[test]
    fn test_backup_serde_roundtrip() {
        let now = chrono::Utc::now().timestamp_millis();
        let original = BackupData {
            version: 1,
            exported_at: now,
            persons: vec![Person {
                id: "rt-001".into(),
                name: "Roundtrip Tester".into(),
                role: "QA".into(),
                context: "test".into(),
                avatar_emoji: "🧑".into(),
                tags: vec![
                    Tag {
                        name: "auto".into(),
                        color: None,
                    },
                    Tag {
                        name: "ci".into(),
                        color: Some("#ff0".into()),
                    },
                ],
                notes: String::new(),
                motivations: vec![],
                biases: vec![],
                rep_scores: RepScores::default(),
                behavioral_patterns: vec![],
                styles: vec![],
                ocean: OceanScores::default(),
                confidence: 5,
                log: vec![],
                created_at: now,
                updated_at: now,
            }],
            predictions: vec![],
            relationships: vec![],
        };

        let json = serde_json::to_string_pretty(&original).unwrap();
        let restored: BackupData = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.version, 1);
        assert_eq!(restored.exported_at, now);
        assert_eq!(restored.persons.len(), 1);
        assert_eq!(restored.persons[0].name, "Roundtrip Tester");
        assert_eq!(restored.persons[0].tags.len(), 2);
        assert_eq!(restored.persons[0].tags[1].color.as_deref(), Some("#ff0"));
    }

    #[test]
    fn test_backup_mock_json_parsing() {
        let raw = mock_backup_json();
        let data: BackupData = serde_json::from_str(raw).unwrap();

        assert_eq!(data.version, 1);
        assert_eq!(data.persons.len(), 2);
        assert_eq!(data.predictions.len(), 1);

        let alice = data.persons.iter().find(|p| p.name == "Alice").unwrap();
        assert_eq!(alice.tags.len(), 1);
        assert_eq!(alice.tags[0].name, "alice-tag");
        assert!(alice.tags[0].color.is_none());
        assert_eq!(alice.ocean.openness, Some(5));
        assert_eq!(alice.ocean.neuroticism, Some(3));
        assert_eq!(alice.rep_scores.hardworker_lazy, Some(8));
        assert_eq!(alice.rep_scores.honest_deceitful, Some(6));
        assert!(alice.rep_scores.authoritative_submissive.is_none());
        assert_eq!(alice.biases.len(), 1);
        assert_eq!(alice.motivations.len(), 1);
        assert_eq!(alice.behavioral_patterns.len(), 1);

        let bob = data.persons.iter().find(|p| p.name == "Bob").unwrap();
        assert_eq!(bob.tags.len(), 2);
        assert_eq!(bob.tags[0].name, "bob-tag-a");
        assert_eq!(bob.tags[1].name, "bob-tag-b");
        assert!(bob.biases.is_empty());
        assert!(bob.motivations.is_empty());
        assert!(bob.behavioral_patterns.is_empty());
        assert!(bob.ocean.openness.is_none());

        let pred = &data.predictions[0];
        assert_eq!(pred.person_id, "mock-001");
        assert!(!pred.resolved);
        assert!(pred.actual_outcome.is_none());
    }

    #[test]
    fn test_backup_empty_behavior_response() {
        let json = r#"{
            "version": 1,
            "exported_at": 0,
            "persons": [{
                "id": "empty-br",
                "name": "Empty Br",
                "role": "",
                "context": "",
                "avatar_emoji": "🧑",
                "tags": [],
                "notes": "",
                "motivations": [],
                "biases": [],
                "rep_scores": {},
                "behavioral_patterns": [
                    {"trigger": "Stress", "predicted_behavior": "", "confidence": 5}
                ],
                "ocean": {},
                "log": [],
                "predictions": [],
                "confidence": 5,
                "created_at": 0,
                "updated_at": 0
            }],
            "predictions": []
        }"#;
        let data: BackupData = serde_json::from_str(json).unwrap();
        assert_eq!(data.persons.len(), 1);
        assert_eq!(
            data.persons[0].behavioral_patterns[0].predicted_behavior,
            peoplemodeler_core::models::BehaviorResponse::SeeksSupport,
        );
    }
}

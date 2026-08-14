use crate::i18n;
use crate::insights::{self, InsightContext};
use crate::models::{OceanScores, Person, Prediction};
use crate::ocean;
use crate::predictions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyze_ocean(json: &str) -> String {
    let scores: OceanScores = serde_json::from_str(json).unwrap_or_default();
    ocean::interpret_all(&scores)
}

#[wasm_bindgen]
pub fn generate_insight(ctx: &str, person_json: &str) -> String {
    let p: Person = match serde_json::from_str(person_json) {
        Ok(p) => p,
        Err(_) => return "Invalid person data".into(),
    };
    let context = match ctx {
        "decision" => InsightContext::Decision,
        "team" => InsightContext::Team,
        "stress" => InsightContext::Stress,
        "communication" => InsightContext::Communication,
        "leadership" => InsightContext::Leadership,
        "growth" => InsightContext::Growth,
        _ => return "Contexte inconnu".into(),
    };
    insights::generate_insight(context, &p)
}

#[wasm_bindgen]
pub fn suggest_prediction(person_json: &str, context: &str) -> String {
    let p: Person = match serde_json::from_str(person_json) {
        Ok(p) => p,
        Err(_) => return "Invalid person data".into(),
    };
    crate::predictions::suggest_outcome(&p, context)
}

#[wasm_bindgen]
pub fn calc_accuracy(predictions_json: &str) -> f64 {
    let preds: Vec<Prediction> = serde_json::from_str(predictions_json).unwrap_or_default();
    crate::predictions::prediction_accuracy_score(&preds)
}

#[wasm_bindgen]
pub fn mot_label(id: &str, lang: &str) -> String {
    let lang = if lang == "en" {
        i18n::Lang::En
    } else {
        i18n::Lang::Fr
    };
    let mt = match id {
        "POWER" => crate::models::MotivationType::Power,
        "ACHIEVEMENT" => crate::models::MotivationType::Achievement,
        "AFFILIATION" => crate::models::MotivationType::Affiliation,
        "SECURITY" => crate::models::MotivationType::Security,
        "AUTONOMY" => crate::models::MotivationType::Autonomy,
        "RECOGNITION" => crate::models::MotivationType::Recognition,
        "LEARNING" => crate::models::MotivationType::Learning,
        "HELPING" => crate::models::MotivationType::Helping,
        _ => return "?".into(),
    };
    mt.i18n(lang).label.into()
}

#[wasm_bindgen]
pub fn mot_desc(id: &str, lang: &str) -> String {
    let lang = if lang == "en" {
        i18n::Lang::En
    } else {
        i18n::Lang::Fr
    };
    let mt = match id {
        "POWER" => crate::models::MotivationType::Power,
        "ACHIEVEMENT" => crate::models::MotivationType::Achievement,
        "AFFILIATION" => crate::models::MotivationType::Affiliation,
        "SECURITY" => crate::models::MotivationType::Security,
        "AUTONOMY" => crate::models::MotivationType::Autonomy,
        "RECOGNITION" => crate::models::MotivationType::Recognition,
        "LEARNING" => crate::models::MotivationType::Learning,
        "HELPING" => crate::models::MotivationType::Helping,
        _ => return "".into(),
    };
    mt.i18n(lang).desc.into()
}

#[wasm_bindgen]
pub fn bias_label(id: &str, lang: &str) -> String {
    let lang = if lang == "en" {
        i18n::Lang::En
    } else {
        i18n::Lang::Fr
    };
    let bt = match id {
        "CONFIRMATION" => crate::models::BiasType::Confirmation,
        "ANCHORING" => crate::models::BiasType::Anchoring,
        "AVAILABILITY" => crate::models::BiasType::Availability,
        "SUNK_COST" => crate::models::BiasType::SunkCost,
        "DUNNING_KRUGER" => crate::models::BiasType::DunningKruger,
        "IMPOSTOR" => crate::models::BiasType::Impostor,
        "LOSS_AVERSION" => crate::models::BiasType::LossAversion,
        "SOCIAL_PROOF" => crate::models::BiasType::SocialProof,
        "AUTHORITY" => crate::models::BiasType::Authority,
        "RECENCY" => crate::models::BiasType::Recency,
        "IN_GROUP" => crate::models::BiasType::InGroup,
        _ => return "?".into(),
    };
    bt.i18n(lang).label.into()
}

#[wasm_bindgen]
pub fn create_prediction(person_id: &str, context: &str, predicted_outcome: &str) -> String {
    let p = predictions::create_prediction(person_id, context, predicted_outcome);
    serde_json::to_string(&p).unwrap()
}

#[wasm_bindgen]
pub fn resolve_prediction(prediction_json: &str, actual_outcome: &str, accuracy: u8) -> String {
    let mut p: Prediction = match serde_json::from_str(prediction_json) {
        Ok(p) => p,
        Err(_) => return "Invalid prediction data".into(),
    };
    predictions::resolve_prediction(&mut p, actual_outcome, accuracy);
    match serde_json::to_string(&p) {
        Ok(s) => s,
        Err(_) => "Invalid prediction data".into(),
    }
}

#[wasm_bindgen]
pub fn bias_desc(id: &str, lang: &str) -> String {
    let lang = if lang == "en" {
        i18n::Lang::En
    } else {
        i18n::Lang::Fr
    };
    let bt = match id {
        "CONFIRMATION" => crate::models::BiasType::Confirmation,
        "ANCHORING" => crate::models::BiasType::Anchoring,
        "AVAILABILITY" => crate::models::BiasType::Availability,
        "SUNK_COST" => crate::models::BiasType::SunkCost,
        "DUNNING_KRUGER" => crate::models::BiasType::DunningKruger,
        "IMPOSTOR" => crate::models::BiasType::Impostor,
        "LOSS_AVERSION" => crate::models::BiasType::LossAversion,
        "SOCIAL_PROOF" => crate::models::BiasType::SocialProof,
        "AUTHORITY" => crate::models::BiasType::Authority,
        "RECENCY" => crate::models::BiasType::Recency,
        "IN_GROUP" => crate::models::BiasType::InGroup,
        _ => return "".into(),
    };
    bt.i18n(lang).desc.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn demo_person_json() -> String {
        r#"{
            "id": "wasm-test",
            "name": "Wasm Test",
            "role": "Tester",
            "context": "test",
            "avatar_emoji": "🧑",
            "tags": [],
            "notes": "",
            "motivations": [
                {"type": "Power", "intensity": 8, "notes": "driven"}
            ],
            "biases": [
                {"type": "Anchoring", "intensity": 7, "evidence": "sticky"}
            ],
            "rep_scores": {},
            "behavioral_patterns": [
                {"trigger": "Change", "predicted_behavior": "embraces_change"}
            ],
            "ocean": {
                "openness": 7,
                "conscientiousness": 6,
                "extraversion": 8,
                "agreeableness": 5,
                "neuroticism": 4
            },
            "log": [],
            "predictions": [],
            "confidence": 5,
            "created_at": 0,
            "updated_at": 0
        }"#
        .into()
    }

    // --- mot_label ---

    #[test]
    fn test_mot_label_all_types() {
        for (id, expected_fr) in &[
            ("POWER", "Pouvoir"),
            ("ACHIEVEMENT", "Accomplissement"),
            ("AFFILIATION", "Appartenance"),
            ("SECURITY", "Sécurité"),
            ("AUTONOMY", "Autonomie"),
            ("RECOGNITION", "Reconnaissance"),
            ("LEARNING", "Apprentissage"),
            ("HELPING", "Aider les autres"),
        ] {
            assert_eq!(mot_label(id, "fr"), *expected_fr, "FR label for {}", id);
            assert!(!mot_label(id, "en").is_empty(), "EN label for {}", id);
        }
    }

    #[test]
    fn test_mot_label_invalid() {
        assert_eq!(mot_label("UNKNOWN", "fr"), "?");
        assert_eq!(mot_label("", "en"), "?");
    }

    // --- mot_desc ---

    #[test]
    fn test_mot_desc_all_types() {
        for id in &[
            "POWER",
            "ACHIEVEMENT",
            "AFFILIATION",
            "SECURITY",
            "AUTONOMY",
            "RECOGNITION",
            "LEARNING",
            "HELPING",
        ] {
            let fr = mot_desc(id, "fr");
            let en = mot_desc(id, "en");
            assert!(!fr.is_empty(), "FR desc for {}", id);
            assert!(!en.is_empty(), "EN desc for {}", id);
        }
    }

    #[test]
    fn test_mot_desc_invalid() {
        assert_eq!(mot_desc("UNKNOWN", "fr"), "");
        assert_eq!(mot_desc("", "en"), "");
    }

    // --- bias_label ---

    #[test]
    fn test_bias_label_all_types() {
        for id in &[
            "CONFIRMATION",
            "ANCHORING",
            "AVAILABILITY",
            "SUNK_COST",
            "DUNNING_KRUGER",
            "LOSS_AVERSION",
            "SOCIAL_PROOF",
            "AUTHORITY",
            "RECENCY",
            "IN_GROUP",
        ] {
            assert!(!bias_label(id, "fr").is_empty(), "FR label for {}", id);
            assert!(!bias_label(id, "en").is_empty(), "EN label for {}", id);
        }
    }

    #[test]
    fn test_bias_label_invalid() {
        assert_eq!(bias_label("UNKNOWN", "fr"), "?");
    }

    // --- bias_desc ---

    #[test]
    fn test_bias_desc_all_types() {
        for id in &[
            "CONFIRMATION",
            "ANCHORING",
            "AVAILABILITY",
            "SUNK_COST",
            "DUNNING_KRUGER",
            "LOSS_AVERSION",
            "SOCIAL_PROOF",
            "AUTHORITY",
            "RECENCY",
            "IN_GROUP",
        ] {
            let fr = bias_desc(id, "fr");
            let en = bias_desc(id, "en");
            assert!(!fr.is_empty(), "FR desc for {}", id);
            assert!(!en.is_empty(), "EN desc for {}", id);
        }
    }

    #[test]
    fn test_bias_desc_invalid() {
        assert_eq!(bias_desc("UNKNOWN", "fr"), "");
    }

    // --- analyze_ocean ---

    #[test]
    fn test_analyze_ocean_valid() {
        let json = r#"{"openness":8,"conscientiousness":7,"extraversion":6,"agreeableness":5,"neuroticism":4}"#;
        let result = analyze_ocean(json);
        assert!(!result.is_empty(), "analyze_ocean returned empty");
        assert!(
            result.contains("ouvert") || result.contains("open"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_analyze_ocean_partial() {
        let json = r#"{"openness":8,"conscientiousness":null,"extraversion":null,"agreeableness":null,"neuroticism":null}"#;
        let result = analyze_ocean(json);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_analyze_ocean_bad_json_defaults() {
        // analyze_ocean uses unwrap_or_default so it doesn't panic
        let result = analyze_ocean("not valid json");
        assert!(
            !result.is_empty(),
            "bad json should return default interpretation"
        );
    }

    // --- generate_insight ---

    #[test]
    fn test_generate_insight_valid() {
        let json = demo_person_json();
        for ctx in &[
            "decision",
            "team",
            "stress",
            "communication",
            "leadership",
            "growth",
        ] {
            let result = generate_insight(ctx, &json);
            assert!(!result.is_empty(), "insight for context {} is empty", ctx);
            assert!(
                result.contains("Wasm Test"),
                "insight should contain person name"
            );
        }
    }

    #[test]
    fn test_generate_insight_unknown_context() {
        let json = demo_person_json();
        let result = generate_insight("unknown", &json);
        assert_eq!(result, "Contexte inconnu");
    }

    #[test]
    fn test_generate_insight_bad_json_graceful() {
        let result = generate_insight("decision", "not json");
        assert!(
            result.contains("Invalid"),
            "should return error msg, got: {}",
            result
        );
    }

    // --- suggest_prediction ---

    #[test]
    fn test_suggest_prediction_valid() {
        let json = demo_person_json();
        let result = suggest_prediction(&json, "meeting");
        assert!(!result.is_empty(), "prediction should not be empty");
    }

    #[test]
    fn test_suggest_prediction_bad_json_graceful() {
        let result = suggest_prediction("bad json", "context");
        assert!(
            result.contains("Invalid"),
            "should return error msg, got: {}",
            result
        );
    }

    // --- calc_accuracy ---

    #[test]
    fn test_calc_accuracy_empty() {
        let result = calc_accuracy("[]");
        assert!((result - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_calc_accuracy_valid() {
        let json = r#"[
            {"id":"p1","person_id":"x","context":"","predicted_outcome":"","actual_outcome":"ok","accuracy":8,"created_at":0,"resolved_at":1,"resolved":true},
            {"id":"p2","person_id":"x","context":"","predicted_outcome":"","actual_outcome":"ok","accuracy":6,"created_at":0,"resolved_at":1,"resolved":true},
            {"id":"p3","person_id":"x","context":"","predicted_outcome":"","actual_outcome":"ok","accuracy":7,"created_at":0,"resolved_at":1,"resolved":true}
        ]"#;
        let result = calc_accuracy(json);
        assert!(
            (result - 70.0).abs() < 0.01,
            "expected ~70.0, got {}",
            result
        );
    }

    #[test]
    fn test_calc_accuracy_bad_json_defaults() {
        let result = calc_accuracy("bad json");
        assert!((result - 0.0).abs() < 0.001);
    }

    // --- create_prediction ---

    #[test]
    fn test_create_prediction_basic() {
        let json = create_prediction("person-1", "review", "will pass");
        let pred: Prediction = serde_json::from_str(&json).unwrap();
        assert_eq!(pred.person_id, "person-1");
        assert_eq!(pred.context, "review");
        assert_eq!(pred.predicted_outcome, "will pass");
        assert!(!pred.resolved);
        assert!(pred.actual_outcome.is_none());
    }

    // --- resolve_prediction ---

    #[test]
    fn test_resolve_prediction_roundtrip() {
        let created = create_prediction("p1", "test", "outcome A");
        let resolved = resolve_prediction(&created, "outcome A actually happened", 9);
        let pred: Prediction = serde_json::from_str(&resolved).unwrap();
        assert!(pred.resolved);
        assert_eq!(
            pred.actual_outcome.as_deref(),
            Some("outcome A actually happened")
        );
        assert_eq!(pred.accuracy, Some(9));
    }

    #[test]
    fn test_resolve_prediction_bad_json_graceful() {
        let result = resolve_prediction("bad json", "ok", 5);
        assert!(
            result.contains("Invalid"),
            "should return error msg, got: {}",
            result
        );
    }
}

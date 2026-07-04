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
    let p: Person = serde_json::from_str(person_json).unwrap();
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
    let p: Person = serde_json::from_str(person_json).unwrap();
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
    let mut p: Prediction = serde_json::from_str(prediction_json).unwrap();
    predictions::resolve_prediction(&mut p, actual_outcome, accuracy);
    serde_json::to_string(&p).unwrap()
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
        "LOSS_AVERSION" => crate::models::BiasType::LossAversion,
        "SOCIAL_PROOF" => crate::models::BiasType::SocialProof,
        "AUTHORITY" => crate::models::BiasType::Authority,
        "RECENCY" => crate::models::BiasType::Recency,
        "IN_GROUP" => crate::models::BiasType::InGroup,
        _ => return "".into(),
    };
    bt.i18n(lang).desc.into()
}

use crate::models::{BiasType, MotivationType, Person, Prediction};
use uuid::Uuid;

pub fn create_prediction(person_id: &str, context: &str, predicted_outcome: &str) -> Prediction {
    Prediction {
        id: Uuid::new_v4().to_string(),
        person_id: person_id.to_string(),
        context: context.to_string(),
        predicted_outcome: predicted_outcome.to_string(),
        actual_outcome: None,
        accuracy: None,
        created_at: timestamp_now(),
        resolved_at: None,
        resolved: false,
    }
}

pub fn resolve_prediction(p: &mut Prediction, actual_outcome: &str, accuracy: u8) {
    p.actual_outcome = Some(actual_outcome.to_string());
    p.accuracy = Some(accuracy.min(10));
    p.resolved_at = Some(timestamp_now());
    p.resolved = true;
}

pub fn prediction_accuracy_score(predictions: &[Prediction]) -> f64 {
    let resolved: Vec<_> = predictions.iter().filter(|p| p.resolved).collect();
    if resolved.is_empty() {
        return 0.0;
    }
    let sum: u32 = resolved
        .iter()
        .map(|p| p.accuracy.unwrap_or(0) as u32)
        .sum();
    sum as f64 / resolved.len() as f64 * 10.0
}

pub fn suggest_outcome(person: &Person, _context: &str) -> String {
    let top_mot = person.top_motivation();
    let top_bias = person.top_bias();

    match (top_mot, top_bias) {
        (Some(m), Some(b)) => format!(
            "Profil : motivation {mot}, biais {bias}. Probabilité qu'il {action} dans ce contexte.",
            mot = m.r#type.i18n(crate::i18n::Lang::Fr).label,
            bias = b.r#type.i18n(crate::i18n::Lang::Fr).label,
            action = action_verb(m.r#type, b.r#type),
        ),
        (Some(m), None) => format!(
            "Motivé par {mot}. Ses actions seront alignées sur ce driver.",
            mot = m.r#type.i18n(crate::i18n::Lang::Fr).label,
        ),
        (None, Some(b)) => format!(
            "Attention au biais de {bias} qui pourrait influencer son jugement.",
            bias = b.r#type.i18n(crate::i18n::Lang::Fr).label,
        ),
        (None, None) => "Profil insuffisamment défini pour générer une prédiction.".into(),
    }
}

fn action_verb(mot: MotivationType, _bias: BiasType) -> &'static str {
    match mot {
        MotivationType::Power => "cherche à prendre le contrôle",
        MotivationType::Achievement => "se focalise sur l'objectif",
        MotivationType::Affiliation => "cherche le consensus",
        MotivationType::Security => "privilégie la prudence",
        MotivationType::Autonomy => "revendique son indépendance",
        MotivationType::Recognition => "cherche la validation",
        MotivationType::Learning => "explore les options",
        MotivationType::Helping => "propose son aide",
    }
}

fn timestamp_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

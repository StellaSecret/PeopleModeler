pub mod i18n;
pub mod insights;
pub mod models;
pub mod ocean;
pub mod predictions;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(test)]
mod tests {
    use crate::i18n::*;
    use crate::insights::{self, InsightContext};
    use crate::models::*;
    use crate::ocean;
    use crate::predictions;

    fn demo_person() -> Person {
        Person {
            id: "demo-001".into(),
            name: "Alexandre Dubois".into(),
            role: "Directeur Commercial".into(),
            context: "Contexte pro · Partenaire".into(),
            avatar_emoji: "🧠".into(),
            tags: vec!["Business".into(), "Décideur".into(), "Négociateur".into()],
            notes: String::new(),
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Power,
                    intensity: 9,
                    notes: "Cherche toujours à être en position de force".into(),
                },
                Motivation {
                    r#type: MotivationType::Recognition,
                    intensity: 7,
                    notes: "A besoin de validation publique".into(),
                },
                Motivation {
                    r#type: MotivationType::Achievement,
                    intensity: 8,
                    notes: "Très orienté résultats".into(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::Anchoring,
                    intensity: 8,
                    evidence: "Reste bloqué sur le premier chiffre".into(),
                },
                Bias {
                    r#type: BiasType::Confirmation,
                    intensity: 6,
                    evidence: "Ignore les données contradictoires".into(),
                },
            ],
            behavioral_patterns: vec![],
            ocean: OceanScores {
                openness: 8,
                conscientiousness: 6,
                extraversion: 9,
                agreeableness: 4,
                neuroticism: 5,
            },
            predictions: vec![Prediction {
                id: "p1".into(),
                person_id: "demo-001".into(),
                context: "Réunion budget".into(),
                predicted_outcome: "Va négocier".into(),
                actual_outcome: Some("A négocié".into()),
                accuracy: Some(7),
                created_at: 1000,
                resolved_at: Some(2000),
                resolved: true,
            }],
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let p = demo_person();
        let json = serde_json::to_string(&p).unwrap();
        let back: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(p.name, back.name);
        assert_eq!(p.motivations.len(), back.motivations.len());
        assert_eq!(p.biases.len(), back.biases.len());
    }

    #[test]
    fn test_top_motivation() {
        let p = demo_person();
        let top = p.top_motivation().unwrap();
        assert_eq!(top.r#type, MotivationType::Power);
        assert_eq!(top.intensity, 9);
    }

    #[test]
    fn test_top_bias() {
        let p = demo_person();
        let top = p.top_bias().unwrap();
        assert_eq!(top.r#type, BiasType::Anchoring);
        assert_eq!(top.intensity, 8);
    }

    #[test]
    fn test_ocean_interpretation() {
        let p = demo_person();
        let result = ocean::interpret_all(&p.ocean);
        assert!(result.contains("ouvert"));
        assert!(result.contains("modéré"));
        assert!(result.contains("extraverti"));
    }

    #[test]
    fn test_motivation_i18n() {
        let fr = MotivationType::Power.i18n(Lang::Fr);
        assert_eq!(fr.label, "Pouvoir");
        let en = MotivationType::Power.i18n(Lang::En);
        assert_eq!(en.label, "Power");
    }

    #[test]
    fn test_bias_i18n() {
        let fr = BiasType::Confirmation.i18n(Lang::Fr);
        assert!(fr.label.contains("confirmation"));
        let en = BiasType::Confirmation.i18n(Lang::En);
        assert!(en.label.contains("Confirmation"));
    }

    #[test]
    fn test_insight_generation() {
        let p = demo_person();
        let insight = insights::generate_insight(InsightContext::Decision, &p);
        assert!(insight.contains("Alexandre Dubois"));
        assert!(insight.contains("Pouvoir"));
        assert!(insight.contains("Ancrage"));
    }

    #[test]
    fn test_prediction_accuracy() {
        let p = demo_person();
        let score = predictions::prediction_accuracy_score(&p.predictions);
        assert!((score - 70.0).abs() < 0.01, "Expected ~70, got {}", score);
    }

    #[test]
    fn test_empty_prediction_accuracy() {
        let score = predictions::prediction_accuracy_score(&[]);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_suggest_outcome() {
        let p = demo_person();
        let suggestion = predictions::suggest_outcome(&p, "réunion");
        assert!(suggestion.contains("Pouvoir"));
        assert!(suggestion.contains("Ancrage"));
    }

    #[test]
    fn test_motivation_enum_all() {
        assert_eq!(MotivationType::ALL.len(), 8);
    }

    #[test]
    fn test_bias_enum_all() {
        assert_eq!(BiasType::ALL.len(), 10);
    }

    #[test]
    fn test_person_serialization_minimal() {
        let p = Person {
            id: "x".into(),
            name: "Test".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            behavioral_patterns: vec![],
            ocean: OceanScores::default(),
            predictions: vec![],
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"name\":\"Test\""));
    }
}

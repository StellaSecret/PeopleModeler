pub mod i18n;
pub mod insights;
pub mod models;
pub mod ocean;
pub mod predictions;
pub mod synergy;

#[cfg(any(target_arch = "wasm32", test))]
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
            tags: vec![
                Tag {
                    name: "Business".into(),
                    color: None,
                },
                Tag {
                    name: "Décideur".into(),
                    color: None,
                },
                Tag {
                    name: "Négociateur".into(),
                    color: None,
                },
            ],
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
            rep_scores: RepScores::default(),
            ocean: OceanScores {
                openness: Some(8),
                conscientiousness: Some(6),
                extraversion: Some(9),
                agreeableness: Some(4),
                neuroticism: Some(5),
            },
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
        // tags are Tag type
        assert_eq!(back.tags.len(), 3);
        assert_eq!(back.tags[0].name, "Business");
        assert!(back.tags[0].color.is_none());
        // ocean fields are Option<u8>
        assert_eq!(back.ocean.openness, Some(8));
        assert_eq!(back.ocean.conscientiousness, Some(6));
        assert_eq!(back.ocean.extraversion, Some(9));
        assert_eq!(back.ocean.agreeableness, Some(4));
        assert_eq!(back.ocean.neuroticism, Some(5));
        // has_any() on default RepScores returns false
        assert!(!RepScores::default().has_any());
        // demo_person uses RepScores::default() so also false
        assert!(!p.rep_scores.has_any());
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
        let score = predictions::prediction_accuracy_score(&[]);
        assert_eq!(score, 0.0);
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
    fn test_insight_context_all() {
        assert_eq!(InsightContext::ALL.len(), 6);
    }

    #[test]
    fn test_team_insight() {
        let p = demo_person();
        let insight = insights::generate_insight(InsightContext::Team, &p);
        assert!(insight.contains("Alexandre Dubois"));
        assert!(insight.contains("Dynamique"));
    }

    #[test]
    fn test_stress_insight() {
        let p = demo_person();
        let insight = insights::generate_insight(InsightContext::Stress, &p);
        assert!(insight.contains("Alexandre Dubois"));
        assert!(insight.contains("stress"));
    }

    #[test]
    fn test_communication_insight() {
        let p = demo_person();
        let insight = insights::generate_insight(InsightContext::Communication, &p);
        assert!(insight.contains("Alexandre Dubois"));
        assert!(insight.contains("communication"));
    }

    #[test]
    fn test_leadership_insight() {
        let p = demo_person();
        let insight = insights::generate_insight(InsightContext::Leadership, &p);
        assert!(insight.contains("Alexandre Dubois"));
        assert!(insight.contains("Leadership"));
    }

    #[test]
    fn test_growth_insight() {
        let p = demo_person();
        let insight = insights::generate_insight(InsightContext::Growth, &p);
        assert!(insight.contains("Alexandre Dubois"));
        assert!(insight.contains("Développement"));
    }

    #[test]
    fn test_motivation_emoji() {
        for m in &MotivationType::ALL {
            assert!(!m.emoji().is_empty(), "emoji for {:?} is empty", m);
        }
    }

    #[test]
    fn test_bias_emoji() {
        for b in &BiasType::ALL {
            assert!(!b.emoji().is_empty(), "emoji for {:?} is empty", b);
        }
    }

    #[test]
    fn test_behavior_trigger_emoji() {
        for t in &[
            BehaviorTrigger::Stress,
            BehaviorTrigger::Conflict,
            BehaviorTrigger::Success,
            BehaviorTrigger::Uncertainty,
            BehaviorTrigger::Recognition,
            BehaviorTrigger::Threatened,
            BehaviorTrigger::Change,
            BehaviorTrigger::Feedback,
        ] {
            assert!(!t.emoji().is_empty(), "emoji for {:?} is empty", t);
        }
    }

    #[test]
    fn test_behavior_trigger_serde() {
        for expected in &[
            BehaviorTrigger::Stress,
            BehaviorTrigger::Conflict,
            BehaviorTrigger::Success,
            BehaviorTrigger::Uncertainty,
            BehaviorTrigger::Recognition,
            BehaviorTrigger::Threatened,
            BehaviorTrigger::Change,
            BehaviorTrigger::Feedback,
        ] {
            let json = serde_json::to_string(expected).unwrap();
            let back: BehaviorTrigger = serde_json::from_str(&json).unwrap();
            assert_eq!(*expected, back);
        }
    }

    #[test]
    fn test_interaction_entry_serde() {
        let entry = InteractionEntry {
            id: "e1".into(),
            timestamp: 1000,
            text: "Discussed project goals".into(),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let back: InteractionEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry.id, back.id);
        assert_eq!(entry.text, back.text);
    }

    #[test]
    fn test_person_with_behavioral_patterns_serde() {
        let p = Person {
            id: "bp-test".into(),
            name: "Pattern Test".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![
                BehavioralPattern {
                    trigger: BehaviorTrigger::Change,
                    predicted_behavior: BehaviorResponse::EmbracesChange,
                    intensity: 5,
                },
                BehavioralPattern {
                    trigger: BehaviorTrigger::Feedback,
                    predicted_behavior: BehaviorResponse::AsksForDetails,
                    intensity: 5,
                },
            ],
            ocean: OceanScores::default(),
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("Change"));
        assert!(json.contains("Feedback"));
        let back: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(back.behavioral_patterns.len(), 2);
        assert_eq!(back.behavioral_patterns[0].trigger, BehaviorTrigger::Change);
        assert_eq!(
            back.behavioral_patterns[1].trigger,
            BehaviorTrigger::Feedback
        );
    }

    #[test]
    fn test_rep_scores_serde() {
        let s = RepScores {
            hardworker_lazy: Some(8),
            authoritative_submissive: Some(3),
            honest_deceitful: None,
            reliable_flaky: Some(7),
            humble_arrogant: Some(9),
            calm_reactive: Some(2),
            diplomatic_blunt: None,
            generous_selfish: Some(6),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: RepScores = serde_json::from_str(&json).unwrap();
        assert_eq!(back.hardworker_lazy, Some(8));
        assert_eq!(back.authoritative_submissive, Some(3));
        assert_eq!(back.honest_deceitful, None);
        assert_eq!(back.generous_selfish, Some(6));
    }

    #[test]
    fn test_rep_scores_default() {
        let s = RepScores::default();
        assert_eq!(s.score(RepDim::HardworkerLazy), None);
        assert_eq!(s.score(RepDim::GenerousSelfish), None);
    }

    #[test]
    fn test_rep_dim_all() {
        assert_eq!(RepDim::ALL.len(), 8);
    }

    #[test]
    fn test_rep_dim_i18n() {
        let fr = RepDim::HardworkerLazy.i18n(Lang::Fr);
        assert_eq!(fr.label_a, "Travailleur");
        assert_eq!(fr.label_b, "Paresseux");
        let en = RepDim::HardworkerLazy.i18n(Lang::En);
        assert_eq!(en.label_a, "Hardworker");
        assert_eq!(en.label_b, "Lazy");
    }

    #[test]
    fn test_rep_dim_emoji() {
        for d in &RepDim::ALL {
            assert!(!d.emoji().is_empty(), "emoji for {:?} is empty", d);
        }
    }

    #[test]
    fn test_rep_scores_in_person_serde() {
        let p = Person {
            id: "rep-test".into(),
            name: "Rep Test".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores {
                hardworker_lazy: Some(9),
                reliable_flaky: Some(7),
                ..RepScores::default()
            },
            behavioral_patterns: vec![],
            ocean: OceanScores::default(),
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(back.rep_scores.hardworker_lazy, Some(9));
        assert_eq!(back.rep_scores.reliable_flaky, Some(7));
        assert_eq!(back.rep_scores.humble_arrogant, None);
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
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            ocean: OceanScores::default(),
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"name\":\"Test\""));
    }
}

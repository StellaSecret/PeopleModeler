pub mod advice;
pub mod i18n;
pub mod insights;
pub mod model_config;
pub mod models;
pub mod ocean;
pub mod predictions;
pub mod synergy;
pub mod validation;

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
            styles: vec![],
            values: vec![],
            rep_scores: RepScores::default(),
            ocean: OceanScores {
                openness: Some(8),
                conscientiousness: Some(6),
                extraversion: Some(9),
                agreeableness: Some(4),
                neuroticism: Some(5),
            },
            resilience: Some(7),
            risk_appetite: Some(8),
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
        assert_eq!(MotivationType::ALL.len(), 10);
    }

    #[test]
    fn test_bias_enum_all() {
        assert_eq!(BiasType::ALL.len(), 12);
    }

    #[test]
    fn test_value_enum_all() {
        assert_eq!(ValueType::ALL.len(), 10);
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
            BehaviorTrigger::Injustice,
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
            BehaviorTrigger::Injustice,
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
            valence: Some(2),
            trigger: Some(BehaviorTrigger::Success),
            target_id: Some("p2".into()),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let back: InteractionEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry.id, back.id);
        assert_eq!(entry.text, back.text);
        assert_eq!(entry.valence, back.valence);
        assert_eq!(entry.trigger, back.trigger);
        assert_eq!(entry.target_id, back.target_id);
    }

    #[test]
    fn test_interaction_entry_backcompat() {
        let json = r#"{"id":"e1","timestamp":1000,"text":"old entry"}"#;
        let e: InteractionEntry = serde_json::from_str(json).unwrap();
        assert_eq!(e.valence, None);
        assert_eq!(e.trigger, None);
        assert_eq!(e.target_id, None);
        assert_eq!(e.text, "old entry");
    }

    #[test]
    fn test_interaction_entry_valence_clamped() {
        let json = r#"{"id":"e1","timestamp":1000,"text":"t","valence":99,"trigger":"Success","target_id":"p2"}"#;
        let e: InteractionEntry = serde_json::from_str(json).unwrap();
        assert_eq!(e.valence, Some(3));
        assert_eq!(e.trigger, Some(BehaviorTrigger::Success));
        assert_eq!(e.target_id.as_deref(), Some("p2"));

        let json = r#"{"id":"e2","timestamp":1000,"text":"t","valence":-99}"#;
        let e: InteractionEntry = serde_json::from_str(json).unwrap();
        assert_eq!(e.valence, Some(-3));
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
                    notes: String::new(),
                },
                BehavioralPattern {
                    trigger: BehaviorTrigger::Feedback,
                    predicted_behavior: BehaviorResponse::AsksForDetails,
                    notes: String::new(),
                },
            ],
            styles: vec![],
            values: vec![],
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
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
            fair_favoritism: None,
            trusting_suspicious: Some(7),
            assertive_passive: Some(4),
            empathetic_detached: None,
            adaptable_rigid: Some(5),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: RepScores = serde_json::from_str(&json).unwrap();
        assert_eq!(back.hardworker_lazy, Some(8));
        assert_eq!(back.authoritative_submissive, Some(3));
        assert_eq!(back.honest_deceitful, None);
        assert_eq!(back.generous_selfish, Some(6));
        assert_eq!(back.trusting_suspicious, Some(7));
        assert_eq!(back.adaptable_rigid, Some(5));
    }

    #[test]
    fn test_rep_scores_default() {
        let s = RepScores::default();
        assert_eq!(s.score(RepDim::HardworkerLazy), None);
        assert_eq!(s.score(RepDim::GenerousSelfish), None);
    }

    #[test]
    fn test_rep_dim_all() {
        assert_eq!(RepDim::ALL.len(), 13);
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
            styles: vec![],
            values: vec![],
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
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
    fn test_style_type_all_count() {
        assert_eq!(StyleType::ALL.len(), 41);
    }

    #[test]
    fn test_style_category_all_count() {
        assert_eq!(StyleCategory::ALL.len(), 8);
    }

    #[test]
    fn test_style_type_category_mapping() {
        use StyleCategory::*;
        for st in &StyleType::ALL {
            let cat = st.category();
            match st {
                StyleType::DirectCommunicator
                | StyleType::DiplomaticCommunicator
                | StyleType::ReservedCommunicator
                | StyleType::ExpressiveCommunicator => assert_eq!(cat, Communication),
                StyleType::Competing
                | StyleType::Collaborating
                | StyleType::Compromising
                | StyleType::Avoiding
                | StyleType::Accommodating => assert_eq!(cat, ConflictResolution),
                StyleType::Analytical
                | StyleType::Intuitive
                | StyleType::Participatory
                | StyleType::Autocratic
                | StyleType::ConsensusDriven => assert_eq!(cat, DecisionMaking),
                StyleType::Visionary
                | StyleType::Servant
                | StyleType::Transactional
                | StyleType::Transformational
                | StyleType::Bureaucratic => assert_eq!(cat, Leadership),
                StyleType::PastOriented
                | StyleType::PresentOriented
                | StyleType::FutureOriented => assert_eq!(cat, TimeOrientation),
                StyleType::RuleBased
                | StyleType::OutcomeBased
                | StyleType::VirtueBased
                | StyleType::Relativist => assert_eq!(cat, MoralFramework),
                StyleType::Opportunistic
                | StyleType::Intrusive
                | StyleType::Manipulative
                | StyleType::PassiveAggressive
                | StyleType::Controlling
                | StyleType::Detached
                | StyleType::Respectful
                | StyleType::Empathetic
                | StyleType::Supportive
                | StyleType::Nurturing => assert_eq!(cat, InterpersonalConduct),
                StyleType::ExtendsTrustFreely
                | StyleType::EarnsTrustGradually
                | StyleType::VerifiesTrust
                | StyleType::Guarded
                | StyleType::RepairsTrustActively => assert_eq!(cat, TrustStyle),
            }
        }
    }

    #[test]
    fn test_style_type_emoji() {
        for st in &StyleType::ALL {
            assert!(!st.emoji().is_empty(), "emoji for {:?} is empty", st);
        }
    }

    #[test]
    fn test_style_type_i18n_label() {
        for st in &StyleType::ALL {
            let fr = st.i18n_label(Lang::Fr);
            let en = st.i18n_label(Lang::En);
            assert!(!fr.is_empty(), "FR label for {:?} is empty", st);
            assert!(!en.is_empty(), "EN label for {:?} is empty", st);
        }
    }

    #[test]
    fn test_style_type_i18n_desc() {
        for st in &StyleType::ALL {
            let fr = st.i18n_desc(Lang::Fr);
            let en = st.i18n_desc(Lang::En);
            assert!(!fr.is_empty(), "FR desc for {:?} is empty", st);
            assert!(!en.is_empty(), "EN desc for {:?} is empty", st);
        }
    }

    #[test]
    fn test_style_type_category_all_categories_covered() {
        let mut seen = Vec::new();
        for st in &StyleType::ALL {
            let cat = st.category();
            if !seen.contains(&cat) {
                seen.push(cat);
            }
        }
        assert_eq!(seen.len(), 8);
        for cat in &StyleCategory::ALL {
            assert!(seen.contains(cat), "category {:?} not covered", cat);
        }
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
            styles: vec![],
            values: vec![],
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"name\":\"Test\""));
    }
}

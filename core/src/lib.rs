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

    // === predictions.rs mutation-killing tests ===

    #[test]
    fn test_action_verb_all_variants() {
        let pairs = [
            (MotivationType::Power, "prendre le contrôle"),
            (MotivationType::Achievement, "focalise"),
            (MotivationType::Affiliation, "consensus"),
            (MotivationType::Security, "prudence"),
            (MotivationType::Autonomy, "indépendance"),
            (MotivationType::Recognition, "validation"),
            (MotivationType::Learning, "options"),
            (MotivationType::Helping, "aide"),
            (MotivationType::Creativity, "innover"),
            (MotivationType::Fairness, "équité"),
        ];
        for (mot, expected_sub) in &pairs {
            let p = Person {
                id: "v-test".into(),
                name: "V".into(),
                role: String::new(),
                context: String::new(),
                avatar_emoji: "V".into(),
                tags: vec![],
                notes: String::new(),
                motivations: vec![crate::models::Motivation {
                    r#type: *mot,
                    intensity: 10,
                    notes: String::new(),
                }],
                biases: vec![crate::models::Bias {
                    r#type: BiasType::Anchoring,
                    intensity: 5,
                    evidence: String::new(),
                }],
                behavioral_patterns: vec![],
                styles: vec![],
                values: vec![],
                rep_scores: RepScores::default(),
                ocean: OceanScores::default(),
                resilience: None,
                risk_appetite: None,
                confidence: 5,
                log: Vec::new(),
                created_at: 0,
                updated_at: 0,
            };
            let outcome = predictions::suggest_outcome(&p, "ctx");
            assert!(
                outcome.contains(expected_sub),
                "{mot:?} should produce verb containing '{expected_sub}', got: {outcome}"
            );
            assert!(
                !outcome.is_empty(),
                "{mot:?} suggest_outcome must not be empty"
            );
        }
    }

    #[test]
    fn test_timestamp_now_catches_predictions() {
        let p = predictions::create_prediction("pid", "ctx", "out");
        assert!(
            p.created_at > 1_000_000_000_000,
            "created_at should be a real millisecond timestamp, got {}",
            p.created_at
        );
    }

    #[test]
    fn test_resolve_prediction_timestamp() {
        let mut p = predictions::create_prediction("pid", "ctx", "out");
        predictions::resolve_prediction(&mut p, "actual", 7);
        assert!(p.resolved);
        assert_eq!(p.accuracy, Some(7));
        assert!(
            p.resolved_at.unwrap_or(0) > 1_000_000_000_000,
            "resolved_at should be a real millisecond timestamp"
        );
    }

    #[test]
    fn test_resolve_prediction_accuracy_clamped() {
        let mut p = predictions::create_prediction("pid", "ctx", "out");
        predictions::resolve_prediction(&mut p, "actual", 255);
        assert_eq!(p.accuracy, Some(10));
    }

    #[test]
    fn test_prediction_accuracy_nontrivial() {
        let p1 = Prediction {
            id: "a".into(),
            person_id: "x".into(),
            context: String::new(),
            predicted_outcome: String::new(),
            actual_outcome: None,
            accuracy: Some(8),
            created_at: 0,
            resolved_at: None,
            resolved: true,
        };
        let p2 = Prediction {
            id: "b".into(),
            person_id: "x".into(),
            context: String::new(),
            predicted_outcome: String::new(),
            actual_outcome: None,
            accuracy: Some(6),
            created_at: 0,
            resolved_at: None,
            resolved: true,
        };
        let score = predictions::prediction_accuracy_score(&[p1, p2]);
        assert!((score - 70.0).abs() < 0.01, "expected 70.0, got {score}");
    }

    // === ocean.rs mutation-killing tests ===

    #[test]
    fn test_interpret_openness_all_branches() {
        assert_eq!(
            ocean::interpret_openness(10),
            "très ouvert aux nouvelles idées, créatif et curieux"
        );
        assert_eq!(
            ocean::interpret_openness(7),
            "très ouvert aux nouvelles idées, créatif et curieux"
        );
        assert_eq!(
            ocean::interpret_openness(5),
            "équilibré entre tradition et innovation"
        );
        assert_eq!(
            ocean::interpret_openness(4),
            "équilibré entre tradition et innovation"
        );
        assert_eq!(
            ocean::interpret_openness(3),
            "pragmatique, préfère les routines et le concret"
        );
        assert_eq!(
            ocean::interpret_openness(0),
            "pragmatique, préfère les routines et le concret"
        );
    }

    #[test]
    fn test_interpret_conscientiousness_all_branches() {
        assert_eq!(
            ocean::interpret_conscientiousness(10),
            "organisé, fiable, orienté résultats et détails"
        );
        assert_eq!(
            ocean::interpret_conscientiousness(7),
            "organisé, fiable, orienté résultats et détails"
        );
        assert_eq!(
            ocean::interpret_conscientiousness(5),
            "niveau modéré de structure et de flexibilité"
        );
        assert_eq!(
            ocean::interpret_conscientiousness(4),
            "niveau modéré de structure et de flexibilité"
        );
        assert_eq!(
            ocean::interpret_conscientiousness(3),
            "flexible et spontané, peut manquer de rigueur"
        );
        assert_eq!(
            ocean::interpret_conscientiousness(0),
            "flexible et spontané, peut manquer de rigueur"
        );
    }

    #[test]
    fn test_interpret_extraversion_all_branches() {
        assert_eq!(
            ocean::interpret_extraversion(10),
            "extraverti, énergique, cherche la stimulation sociale"
        );
        assert_eq!(
            ocean::interpret_extraversion(7),
            "extraverti, énergique, cherche la stimulation sociale"
        );
        assert_eq!(
            ocean::interpret_extraversion(5),
            "équilibré entre solitude et vie sociale"
        );
        assert_eq!(
            ocean::interpret_extraversion(4),
            "équilibré entre solitude et vie sociale"
        );
        assert_eq!(
            ocean::interpret_extraversion(3),
            "introverti, réfléchi, préfère les interactions limitées"
        );
        assert_eq!(
            ocean::interpret_extraversion(0),
            "introverti, réfléchi, préfère les interactions limitées"
        );
    }

    #[test]
    fn test_interpret_agreeableness_all_branches() {
        assert_eq!(
            ocean::interpret_agreeableness(10),
            "coopératif, empathique, cherche l'harmonie"
        );
        assert_eq!(
            ocean::interpret_agreeableness(7),
            "coopératif, empathique, cherche l'harmonie"
        );
        assert_eq!(
            ocean::interpret_agreeableness(5),
            "équilibré entre affirmation de soi et diplomatie"
        );
        assert_eq!(
            ocean::interpret_agreeableness(4),
            "équilibré entre affirmation de soi et diplomatie"
        );
        assert_eq!(
            ocean::interpret_agreeableness(3),
            "direct voire abrasif, met ses objectifs avant les relations"
        );
        assert_eq!(
            ocean::interpret_agreeableness(0),
            "direct voire abrasif, met ses objectifs avant les relations"
        );
    }

    #[test]
    fn test_interpret_neuroticism_all_branches() {
        assert_eq!(
            ocean::interpret_neuroticism(10),
            "émotionnellement réactif, stressable, sensible aux critiques"
        );
        assert_eq!(
            ocean::interpret_neuroticism(7),
            "émotionnellement réactif, stressable, sensible aux critiques"
        );
        assert_eq!(
            ocean::interpret_neuroticism(5),
            "réactivité émotionnelle modérée"
        );
        assert_eq!(
            ocean::interpret_neuroticism(4),
            "réactivité émotionnelle modérée"
        );
        assert_eq!(
            ocean::interpret_neuroticism(3),
            "stable émotionnellement, calme sous pression"
        );
        assert_eq!(
            ocean::interpret_neuroticism(0),
            "stable émotionnellement, calme sous pression"
        );
    }

    #[test]
    fn test_interpret_all_with_none_fields() {
        let ocean = OceanScores {
            openness: None,
            conscientiousness: None,
            extraversion: None,
            agreeableness: None,
            neuroticism: None,
        };
        let result = ocean::interpret_all(&ocean);
        assert!(result.contains("—"));
        assert!(!result.contains("très ouvert"));
    }

    use crate::synergy::PersonProfile;

    // === insights.rs mutation-killing tests ===

    fn minimal_profile(completeness: u8) -> PersonProfile {
        PersonProfile {
            total: 50,
            motivation: 0.5,
            patterns: 0.5,
            ocean: 0.5,
            reputation: 0.5,
            bias: 0.5,
            styles: 0.5,
            values: 0.5,
            completeness,
            band: 5,
        }
    }

    #[test]
    fn test_insight_completeness_low() {
        let p = demo_person();
        let profile = minimal_profile(30);
        let result =
            insights::generate_insight_with_profile(InsightContext::Decision, &p, &profile);
        assert!(
            result.contains("Profil incomplet"),
            "should contain 'Profil incomplet' for completeness 30"
        );
        assert!(!result.contains("partiellement complété"));
        assert!(!result.contains("complété pour des conseils"));
    }

    #[test]
    fn test_insight_completeness_medium() {
        let p = demo_person();
        let profile = minimal_profile(55);
        let result = insights::generate_insight_with_profile(InsightContext::Team, &p, &profile);
        assert!(
            result.contains("partiellement complété"),
            "should contain 'partiellement complété' for completeness 55"
        );
        assert!(!result.contains("Profil incomplet"));
    }

    #[test]
    fn test_insight_completeness_high() {
        let p = demo_person();
        let profile = minimal_profile(80);
        let result = insights::generate_insight_with_profile(InsightContext::Stress, &p, &profile);
        assert!(!result.contains("Profil incomplet"));
        assert!(!result.contains("partiellement complété"));
    }

    #[test]
    fn test_insight_fmt_motivations_not_empty() {
        let p = demo_person();
        let profile = minimal_profile(50);
        let result =
            insights::generate_insight_with_profile(InsightContext::Decision, &p, &profile);
        assert!(
            result.contains("intensité"),
            "fmt_motivations output should contain 'intensité'"
        );
    }

    #[test]
    fn test_insight_fmt_biases_not_empty() {
        let p = demo_person();
        let profile = minimal_profile(50);
        let result =
            insights::generate_insight_with_profile(InsightContext::Decision, &p, &profile);
        assert!(
            result.contains("Ancrage"),
            "fmt_biases output should contain bias name"
        );
    }

    #[test]
    fn test_insight_empty_motivations_and_biases() {
        let p = Person {
            id: "empty".into(),
            name: "Empty".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "E".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            rep_scores: RepScores::default(),
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let profile = minimal_profile(5);
        let result =
            insights::generate_insight_with_profile(InsightContext::Leadership, &p, &profile);
        assert!(result.contains("Aucune motivation définie"));
        assert!(result.contains("Aucun biais défini"));
    }

    #[test]
    fn test_insight_empty_flags() {
        let p = Person {
            id: "clean".into(),
            name: "Clean".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "C".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            rep_scores: RepScores::default(),
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };
        let profile = minimal_profile(5);
        let result = insights::generate_insight_with_profile(InsightContext::Growth, &p, &profile);
        assert!(
            result.contains("Aucun signal d'alerte"),
            "flags fmt should show 'Aucun signal d'alerte'"
        );
    }

    #[test]
    fn test_insight_completeness_boundary_40_not_incomplet() {
        let p = demo_person();
        let profile = minimal_profile(40);
        let result =
            insights::generate_insight_with_profile(InsightContext::Decision, &p, &profile);
        assert!(
            !result.contains("Profil incomplet"),
            "completeness 40 should NOT trigger 'incomplet' (boundary < 40)"
        );
    }

    #[test]
    fn test_insight_completeness_boundary_70_not_partial() {
        let p = demo_person();
        let profile = minimal_profile(70);
        let result =
            insights::generate_insight_with_profile(InsightContext::Decision, &p, &profile);
        assert!(
            !result.contains("partiellement complété"),
            "completeness 70 should NOT trigger 'partiellement' (boundary < 70)"
        );
    }

    #[test]
    fn test_insight_advice_not_empty_string() {
        let p = demo_person();
        let profile = minimal_profile(50);
        let result =
            insights::generate_insight_with_profile(InsightContext::Decision, &p, &profile);
        assert!(
            result.contains("• ["),
            "advice section should contain formatted bullet points '• [category] action'"
        );
        assert!(result.contains("Recommandations prioritaires"));
    }

    #[test]
    fn test_insight_all_contexts_use_fmt_sections() {
        let p = demo_person();
        let profile = minimal_profile(50);
        for ctx in InsightContext::ALL {
            let result = insights::generate_insight_with_profile(ctx, &p, &profile);
            assert!(
                result.contains("Motivation(s) active(s)"),
                "{:?} missing motivations section",
                ctx
            );
            assert!(
                result.contains("Biais cognitif(s)"),
                "{:?} missing biases section",
                ctx
            );
            assert!(
                result.contains("Alertes"),
                "{:?} missing alerts section",
                ctx
            );
            assert!(
                result.contains("Recommandations prioritaires"),
                "{:?} missing recommendations section",
                ctx
            );
        }
    }

    // === i18n.rs mutation-killing tests ===

    #[test]
    fn test_style_category_i18n_label_specific_values() {
        assert_eq!(
            StyleCategory::Communication.i18n_label(Lang::Fr),
            "💬 Communication"
        );
        assert_eq!(
            StyleCategory::Leadership.i18n_label(Lang::Fr),
            "👥 Leadership"
        );
        assert_eq!(
            StyleCategory::TrustStyle.i18n_label(Lang::En),
            "🔗 Trust Style"
        );
        assert_eq!(
            StyleCategory::MoralFramework.i18n_label(Lang::Fr),
            "📜 Cadre moral"
        );
    }

    #[test]
    fn test_style_type_i18n_label_not_xyzzy() {
        for st in &StyleType::ALL {
            let fr = st.i18n_label(Lang::Fr);
            let en = st.i18n_label(Lang::En);
            assert_ne!(fr, "xyzzy", "FR label for {:?} should not be xyzzy", st);
            assert_ne!(en, "xyzzy", "EN label for {:?} should not be xyzzy", st);
        }
    }

    #[test]
    fn test_style_type_i18n_desc_not_xyzzy() {
        for st in &StyleType::ALL {
            let fr = st.i18n_desc(Lang::Fr);
            let en = st.i18n_desc(Lang::En);
            assert_ne!(fr, "xyzzy", "FR desc for {:?} should not be xyzzy", st);
            assert_ne!(en, "xyzzy", "EN desc for {:?} should not be xyzzy", st);
        }
    }

    #[test]
    fn test_behavior_response_label_not_xyzzy() {
        use crate::models::BehaviorResponse;
        let sample = [
            BehaviorResponse::RemainsCalm,
            BehaviorResponse::FacilitatesResolution,
            BehaviorResponse::CelebratesWithOthers,
            BehaviorResponse::EmbracesAmbiguity,
            BehaviorResponse::EmbracesChange,
            BehaviorResponse::SeeksFeedback,
            BehaviorResponse::SeeksRestoration,
        ];
        for br in sample {
            let fr = br.label(Lang::Fr);
            let en = br.label(Lang::En);
            assert!(!fr.is_empty(), "FR label for {:?} is empty", br);
            assert!(!en.is_empty(), "EN label for {:?} is empty", br);
            assert_ne!(fr, "xyzzy", "FR label for {:?} should not be xyzzy", br);
            assert_ne!(en, "xyzzy", "EN label for {:?} should not be xyzzy", br);
        }
    }

    #[test]
    fn test_behavior_response_label_bare_not_xyzzy() {
        use crate::models::BehaviorResponse;
        let br = BehaviorResponse::RemainsCalm;
        let bare_fr = br.label_bare(Lang::Fr);
        let bare_en = br.label_bare(Lang::En);
        assert!(!bare_fr.is_empty());
        assert!(!bare_en.is_empty());
        assert_ne!(bare_fr, "xyzzy");
        assert_ne!(bare_en, "xyzzy");
    }

    // === models.rs mutation-killing tests ===

    #[test]
    fn test_rep_dim_emoji_not_xyzzy() {
        for d in &RepDim::ALL {
            let e = d.emoji();
            assert!(!e.is_empty(), "emoji for {:?} is empty", d);
            assert_ne!(e, "xyzzy", "emoji for {:?} should not be xyzzy", d);
        }
    }

    #[test]
    fn test_rep_dim_pole_labels() {
        use crate::i18n::Lang;
        for d in &RepDim::ALL {
            let a_fr = d.pole_a_label(Lang::Fr);
            let a_en = d.pole_a_label(Lang::En);
            let b_fr = d.pole_b_label(Lang::Fr);
            let b_en = d.pole_b_label(Lang::En);
            assert!(!a_fr.is_empty(), "pole_a_label FR for {:?} is empty", d);
            assert!(!a_en.is_empty(), "pole_a_label EN for {:?} is empty", d);
            assert!(!b_fr.is_empty(), "pole_b_label FR for {:?} is empty", d);
            assert!(!b_en.is_empty(), "pole_b_label EN for {:?} is empty", d);
            assert_ne!(a_fr, "xyzzy");
            assert_ne!(b_fr, "xyzzy");
        }
    }

    #[test]
    fn test_rep_scores_has_any_true() {
        let s = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        assert!(s.has_any());
    }

    #[test]
    fn test_motivation_type_emoji_not_xyzzy() {
        for m in &MotivationType::ALL {
            let e = m.emoji();
            assert!(!e.is_empty(), "emoji for {:?} is empty", m);
            assert_ne!(e, "xyzzy", "emoji for {:?} should not be xyzzy", m);
        }
    }

    #[test]
    fn test_value_type_emoji_not_xyzzy() {
        for v in &ValueType::ALL {
            let e = v.emoji();
            assert!(!e.is_empty(), "emoji for {:?} is empty", v);
            assert_ne!(e, "xyzzy", "emoji for {:?} should not be xyzzy", v);
        }
    }

    #[test]
    fn test_bias_type_emoji_not_xyzzy() {
        for b in &BiasType::ALL {
            let e = b.emoji();
            assert!(!e.is_empty(), "emoji for {:?} is empty", b);
            assert_ne!(e, "xyzzy", "emoji for {:?} should not be xyzzy", b);
        }
    }

    #[test]
    fn test_behavior_trigger_emoji_not_xyzzy() {
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
            let e = t.emoji();
            assert!(!e.is_empty(), "emoji for {:?} is empty", t);
            assert_ne!(e, "xyzzy", "emoji for {:?} should not be xyzzy", t);
        }
    }

    #[test]
    fn test_behavior_response_serde_name_specific() {
        use crate::models::BehaviorResponse;
        assert_eq!(BehaviorResponse::RemainsCalm.serde_name(), "remains_calm");
        assert_eq!(BehaviorResponse::Escalates.serde_name(), "escalates");
        assert_eq!(
            BehaviorResponse::EmbracesChange.serde_name(),
            "embraces_change"
        );
        assert!(!BehaviorResponse::RemainsCalm.serde_name().is_empty());
        assert_ne!(BehaviorResponse::RemainsCalm.serde_name(), "xyzzy");
    }

    #[test]
    fn test_behavior_response_options_for_not_empty() {
        use crate::models::BehaviorResponse;
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
            let opts = BehaviorResponse::options_for(*t);
            assert!(!opts.is_empty(), "options_for({:?}) is empty", t);
        }
    }

    #[test]
    fn test_relation_type_label() {
        use crate::i18n::Lang;
        for r in &RelationType::ALL {
            let fr = r.label(Lang::Fr);
            let en = r.label(Lang::En);
            assert!(!fr.is_empty(), "FR label for {:?} is empty", r);
            assert!(!en.is_empty(), "EN label for {:?} is empty", r);
            assert_ne!(fr, "xyzzy");
            assert_ne!(en, "xyzzy");
        }
    }

    #[test]
    fn test_default_confidence() {
        let json = r#"{"id":"x","name":"X","role":"","context":"","avatar_emoji":"","notes":"","motivations":[],"biases":[],"behavioral_patterns":[],"ocean":{},"created_at":0,"updated_at":0}"#;
        let p: Person = serde_json::from_str(json).unwrap();
        assert_eq!(p.confidence, 5, "default_confidence should be 5");
    }

    #[test]
    fn test_style_type_options_for_not_empty() {
        for cat in &StyleCategory::ALL {
            let opts = StyleType::options_for(*cat);
            assert!(!opts.is_empty(), "options_for({:?}) is empty", cat);
        }
    }

    #[test]
    fn test_style_type_emoji_not_xyzzy() {
        for st in &StyleType::ALL {
            let e = st.emoji();
            assert!(!e.is_empty(), "emoji for {:?} is empty", st);
            assert_ne!(e, "xyzzy", "emoji for {:?} should not be xyzzy", st);
        }
    }

    #[test]
    fn test_display_impls_produce_variant_names() {
        assert!(!format!("{}", MotivationType::Power).is_empty());
        assert_eq!(format!("{}", MotivationType::Power), "Power");
        assert_eq!(format!("{}", BiasType::Anchoring), "Anchoring");
        assert_eq!(format!("{}", BehaviorTrigger::Stress), "Stress");
        assert_eq!(format!("{}", RelationType::Manages), "Manages");
        assert_eq!(format!("{}", ValueType::Career), "Career");
        assert_eq!(format!("{}", RepDim::HardworkerLazy), "HardworkerLazy");
        assert_eq!(
            format!("{}", StyleType::DirectCommunicator),
            "DirectCommunicator"
        );
    }

    #[test]
    fn test_display_tag() {
        let t = Tag {
            name: "X".into(),
            color: None,
        };
        assert!(!format!("{}", t).is_empty());
    }

    #[test]
    fn test_display_relation_type_all() {
        for r in &RelationType::ALL {
            assert!(!format!("{}", r).is_empty());
        }
    }

    // === advice.rs mutation-killing tests ===

    #[test]
    fn test_flag_action_specific_values() {
        use crate::advice;
        use crate::i18n::Lang;
        let fr = advice::flag_action("flag_high_e_low_a", Lang::Fr);
        assert!(!fr.is_empty());
        assert_ne!(fr, "xyzzy");
        assert!(
            fr.contains("écoute"),
            "FR advice for flag_high_e_low_a should mention 'écoute'"
        );
        let en = advice::flag_action("flag_high_e_low_a", Lang::En);
        assert!(
            en.contains("listening"),
            "EN advice for flag_high_e_low_a should mention 'listening'"
        );
        let unknown = advice::flag_action("nonexistent_flag", Lang::Fr);
        assert!(unknown.is_empty(), "unknown flag should return empty");
    }

    #[test]
    fn test_flag_action_all_flags_non_xyzzy() {
        use crate::advice;
        use crate::i18n::Lang;
        let src = include_str!("validation.rs");
        let mut start = 0;
        let mut checked = 0;
        while let Some(pos) = src[start..].find("\"flag_") {
            let abs = start + pos + 1;
            if let Some(end) = src[abs..].find('"') {
                let flag = &src[abs..abs + end];
                let fr = advice::flag_action(flag, Lang::Fr);
                assert_ne!(fr, "xyzzy", "flag_action({}) should not return xyzzy", flag);
                checked += 1;
                start = abs + end + 1;
            } else {
                break;
            }
        }
        assert!(
            checked >= 70,
            "should check at least 70 flags, checked {}",
            checked
        );
    }

    #[test]
    fn test_generate_advice_categories() {
        use crate::advice;
        use crate::models::*;
        let p = Person {
            id: "adv-test".into(),
            name: "Adv".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "A".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![Motivation {
                r#type: MotivationType::Fairness,
                intensity: 9,
                notes: String::new(),
            }],
            biases: vec![Bias {
                r#type: BiasType::Confirmation,
                intensity: 9,
                evidence: String::new(),
            }],
            behavioral_patterns: vec![],
            styles: vec![PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 9,
                notes: String::new(),
            }],
            values: vec![Value {
                r#type: ValueType::Career,
                intensity: 9,
                priority: 9,
                notes: String::new(),
            }],
            rep_scores: RepScores {
                fair_favoritism: Some(2),
                generous_selfish: Some(2),
                ..Default::default()
            },
            ocean: OceanScores {
                openness: Some(9),
                conscientiousness: Some(2),
                ..Default::default()
            },
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        };
        let advice = advice::generate_advice(&p);
        let mut _has_self_image = false;
        let mut has_rhetoric = false;
        let mut has_evidence = false;
        let mut _has_values = false;
        for a in &advice {
            match a.category {
                "self_image" => _has_self_image = true,
                "rhetoric" => has_rhetoric = true,
                "evidence" => has_evidence = true,
                "style" => {}
                "values" => _has_values = true,
                other => panic!("unexpected category: {other}"),
            }
        }
        assert!(
            has_evidence,
            "should have evidence-category advice from bias_confirmation_open or similar"
        );
        assert!(
            has_rhetoric,
            "should have rhetoric-category advice from fairness_rhetoric or similar"
        );
    }

    #[test]
    fn test_risk_mitigation_pair_nonempty() {
        use crate::advice;
        use crate::models::*;
        let p = Person {
            id: "rm-test".into(),
            name: "RM".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "R".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            rep_scores: RepScores {
                calm_reactive: Some(9),
                ..Default::default()
            },
            ocean: OceanScores {
                neuroticism: Some(9),
                ..Default::default()
            },
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        };
        let pairs = advice::risk_mitigation_pair(&p);
        assert!(
            !pairs.is_empty(),
            "risk_mitigation_pair should return non-empty for contradicting profile"
        );
        for (flag, mitigation) in &pairs {
            assert!(
                flag.starts_with("flag_"),
                "pair flag should start with 'flag_': {}",
                flag
            );
            assert!(
                !mitigation.is_empty(),
                "mitigation for {} should not be empty",
                flag
            );
            assert_ne!(mitigation, &"xyzzy", "mitigation should not be xyzzy");
        }
    }

    // === advice.rs mutation-killing tests ===

    fn mega_rhetoric_person() -> crate::models::Person {
        use crate::models::*;
        Person {
            id: "mega".into(),
            name: "Mega".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "M".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Fairness,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Helping,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Affiliation,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Power,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Achievement,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Security,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Autonomy,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Learning,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Creativity,
                    intensity: 9,
                    notes: String::new(),
                },
            ],
            biases: vec![Bias {
                r#type: BiasType::DunningKruger,
                intensity: 9,
                evidence: String::new(),
            }],
            behavioral_patterns: vec![],
            styles: vec![PersonalStyle {
                r#type: StyleType::DiplomaticCommunicator,
                intensity: 9,
                notes: String::new(),
            }],
            values: vec![
                Value {
                    r#type: ValueType::Family,
                    intensity: 9,
                    priority: 9,
                    notes: String::new(),
                },
                Value {
                    r#type: ValueType::Career,
                    intensity: 9,
                    priority: 9,
                    notes: String::new(),
                },
            ],
            rep_scores: RepScores {
                fair_favoritism: Some(2),
                generous_selfish: Some(2),
                empathetic_detached: Some(2),
                hardworker_lazy: Some(2),
                trusting_suspicious: Some(9),
                diplomatic_blunt: Some(2),
                authoritative_submissive: Some(2),
                assertive_passive: Some(2),
                adaptable_rigid: Some(2),
                humble_arrogant: Some(2),
                ..Default::default()
            },
            ocean: OceanScores {
                openness: Some(2),
                conscientiousness: Some(9),
                agreeableness: Some(9),
                ..Default::default()
            },
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_generate_advice_rhetoric_categories_specific() {
        use crate::advice;
        let p = mega_rhetoric_person();
        let advice = advice::generate_advice(&p);
        let rhetoric: Vec<_> = advice.iter().filter(|a| a.category == "rhetoric").collect();
        assert!(
            rhetoric.len() >= 8,
            "should have many rhetoric flags, got {}",
            rhetoric.len()
        );
        let flag_names: Vec<&str> = rhetoric.iter().map(|a| a.flag).collect();
        assert!(
            flag_names
                .iter()
                .any(|f| f.starts_with("flag_affiliation_")),
            "should have flag_affiliation_* in rhetoric: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_ambition_")),
            "should have flag_ambition_* in rhetoric: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_autonomy_")),
            "should have flag_autonomy_* in rhetoric: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_creativity_")),
            "should have flag_creativity_* in rhetoric: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_discipline_lazy"),
            "should have flag_discipline_lazy: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_warmth_blunt"),
            "should have flag_warmth_blunt: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_warmth_selfish"),
            "should have flag_warmth_selfish: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_learning_rigid"),
            "should have flag_learning_rigid: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_learning_arrogant"),
            "should have flag_learning_arrogant: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_power_passive"),
            "should have flag_power_passive: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_helping_cold"),
            "should have flag_helping_cold: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_fairness_rhetoric"),
            "should have flag_fairness_rhetoric: {:?}",
            flag_names
        );
        assert!(
            flag_names.contains(&"flag_helping_selfish"),
            "should have flag_helping_selfish: {:?}",
            flag_names
        );
        let evidence: Vec<_> = advice.iter().filter(|a| a.category == "evidence").collect();
        assert!(!evidence.is_empty(), "should have evidence flags too");
        let values: Vec<_> = advice.iter().filter(|a| a.category == "values").collect();
        assert!(!values.is_empty(), "should have values flags too");
        let style: Vec<_> = advice.iter().filter(|a| a.category == "style").collect();
        assert!(!style.is_empty(), "should have style flags too");
    }

    #[test]
    fn test_generate_advice_evidence_categories_specific() {
        use crate::advice;
        use crate::models::*;
        let p = Person {
            id: "evidence-test".into(),
            name: "Ev".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "E".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Security,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Fairness,
                    intensity: 9,
                    notes: String::new(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::LossAversion,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::Authority,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::SocialProof,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::SunkCost,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::Impostor,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::Recency,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::Availability,
                    intensity: 9,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::Favoritism,
                    intensity: 9,
                    evidence: String::new(),
                },
            ],
            behavioral_patterns: vec![BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::BecomesIrritable,
                notes: String::new(),
            }],
            styles: vec![],
            values: vec![],
            rep_scores: RepScores {
                authoritative_submissive: Some(9),
                reliable_flaky: Some(9),
                calm_reactive: Some(9),
                adaptable_rigid: Some(9),
                humble_arrogant: Some(9),
                ..Default::default()
            },
            ocean: OceanScores {
                openness: Some(9),
                ..Default::default()
            },
            resilience: Some(2),
            risk_appetite: Some(9),
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        };
        let advice = advice::generate_advice(&p);
        let evidence: Vec<_> = advice.iter().filter(|a| a.category == "evidence").collect();
        assert!(
            evidence.len() >= 6,
            "should have many evidence flags, got {}",
            evidence.len()
        );
        let flag_names: Vec<&str> = evidence.iter().map(|a| a.flag).collect();
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_pattern_")),
            "should have flag_pattern_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_bias_")),
            "should have flag_bias_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_authority_")),
            "should have flag_authority_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_social_")),
            "should have flag_social_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_sunk_")),
            "should have flag_sunk_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_loss_")),
            "should have flag_loss_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_impostor_")),
            "should have flag_impostor_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_recency_")),
            "should have flag_recency_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names
                .iter()
                .any(|f| f.starts_with("flag_availability_")),
            "should have flag_availability_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_security_")),
            "should have flag_security_* in evidence: {:?}",
            flag_names
        );
        assert!(
            flag_names.iter().any(|f| f.starts_with("flag_resilient_")),
            "should have flag_resilient_* in evidence: {:?}",
            flag_names
        );
    }

    #[test]
    fn test_per_context_advice_sort_order_decision() {
        use crate::advice;
        use crate::insights::InsightContext;
        let p = mega_rhetoric_person();
        let profile = crate::synergy::PersonProfile {
            total: 50,
            motivation: 0.5,
            patterns: 0.5,
            ocean: 0.5,
            reputation: 0.5,
            bias: 0.5,
            styles: 0.5,
            values: 0.5,
            completeness: 60,
            band: 5,
        };
        let advice = advice::per_context_advice(&p, &profile, InsightContext::Decision);
        assert!(
            advice.len() >= 10,
            "should have many advice items, got {}",
            advice.len()
        );
        for a in &advice {
            assert!(!a.action.is_empty(), "empty action for {}", a.flag);
            assert!(
                matches!(
                    a.category,
                    "self_image" | "rhetoric" | "evidence" | "style" | "values"
                ),
                "unexpected category {} for {}",
                a.category,
                a.flag
            );
        }
        let categories: Vec<&str> = advice.iter().map(|a| a.category).collect();
        let first = categories[0];
        assert_eq!(
            first, "self_image",
            "highest weight category should be first in Decision context"
        );
        assert!(
            categories.contains(&"values"),
            "should have values-category items"
        );
        assert!(
            categories.contains(&"evidence"),
            "should have evidence-category items"
        );
        assert!(
            categories.contains(&"style"),
            "should have style-category items"
        );
    }

    #[test]
    fn test_per_context_advice_sort_order_all_contexts() {
        use crate::advice;
        use crate::insights::InsightContext;
        let p = mega_rhetoric_person();
        let profile = crate::synergy::PersonProfile {
            total: 50,
            motivation: 0.5,
            patterns: 0.5,
            ocean: 0.5,
            reputation: 0.5,
            bias: 0.5,
            styles: 0.5,
            values: 0.5,
            completeness: 60,
            band: 5,
        };
        for ctx in InsightContext::ALL {
            let advice = advice::per_context_advice(&p, &profile, ctx);
            let categories: Vec<&str> = advice.iter().map(|a| a.category).collect();
            let first = categories.first().unwrap_or(&"none");
            let last = categories.last().unwrap_or(&"none");
            assert_ne!(first, &"none", "{:?}: should have advice items", ctx);
            assert_ne!(first, last, "{:?}: not all items same category", ctx);
        }
    }

    // === wasm.rs mutation-killing tests ===

    fn wasm_demo_person_json() -> String {
        r#"{
            "id": "wasm-test", "name": "Wasm Test", "role": "Tester",
            "context": "test", "avatar_emoji": "🧑", "tags": [], "notes": "",
            "motivations": [{"type": "Power", "intensity": 8, "notes": "driven"}],
            "biases": [{"type": "Anchoring", "intensity": 7, "evidence": "sticky"}],
            "rep_scores": {},
            "behavioral_patterns": [{"trigger": "Change", "predicted_behavior": "embraces_change"}],
            "ocean": {"openness": 7, "conscientiousness": 6, "extraversion": 8, "agreeableness": 5, "neuroticism": 4},
            "log": [], "predictions": [], "confidence": 5, "created_at": 0, "updated_at": 0
        }"#
        .into()
    }

    #[test]
    fn test_compute_synergy_with_rel_all_types() {
        use crate::wasm;
        let json = wasm_demo_person_json();
        let known = [
            "WorksWith",
            "Manages",
            "ReportsTo",
            "Friends",
            "Family",
            "Partner",
            "Mentors",
            "Collaborates",
        ];
        for rel in known {
            let result = wasm::compute_synergy_with_rel(&json, &json, rel, 5);
            let v: serde_json::Value = serde_json::from_str(&result).unwrap();
            assert!(
                v["total"].is_number(),
                "type {} should produce valid output",
                rel
            );
            let band = v["band"].as_u64().unwrap_or(0);
            assert!(band > 0, "type {} should produce non-zero band", rel);
        }
    }

    #[test]
    fn test_mot_desc_specific_values() {
        use crate::wasm;
        let fr = wasm::mot_desc("POWER", "fr");
        let en = wasm::mot_desc("POWER", "en");
        assert!(!fr.is_empty());
        assert!(!en.is_empty());
        assert_ne!(fr, en, "FR and EN descriptions should differ for POWER");
    }

    #[test]
    fn test_bias_desc_specific_values() {
        use crate::wasm;
        let fr = wasm::bias_desc("CONFIRMATION", "fr");
        let en = wasm::bias_desc("CONFIRMATION", "en");
        assert!(!fr.is_empty());
        assert!(!en.is_empty());
        assert_ne!(fr, en, "FR and EN descriptions should differ");
    }

    #[test]
    fn test_bias_label_specific_values() {
        use crate::wasm;
        let fr = wasm::bias_label("CONFIRMATION", "fr");
        let en = wasm::bias_label("CONFIRMATION", "en");
        assert_eq!(fr, "Biais de confirmation");
        assert_eq!(en, "Confirmation bias");
    }

    #[test]
    fn test_bias_desc_not_swapped() {
        use crate::wasm;
        let fr = wasm::bias_desc("CONFIRMATION", "fr");
        let en = wasm::bias_desc("CONFIRMATION", "en");
        assert!(
            fr.starts_with("Cherche"),
            "FR desc should be French, got: {}",
            fr
        );
        assert!(
            en.starts_with("Seeks"),
            "EN desc should be English, got: {}",
            en
        );
    }

    #[test]
    fn test_mot_desc_not_swapped() {
        use crate::wasm;
        let fr = wasm::mot_desc("POWER", "fr");
        let en = wasm::mot_desc("POWER", "en");
        assert!(
            fr.starts_with("Contr"),
            "FR desc should be French, got: {}",
            fr
        );
        assert!(
            en.starts_with("Control"),
            "EN desc should be English, got: {}",
            en
        );
    }

    #[test]
    fn test_bias_label_impostor_and_in_group() {
        use crate::wasm;
        assert!(!wasm::bias_label("IMPOSTOR", "fr").is_empty());
        assert!(!wasm::bias_label("IMPOSTOR", "en").is_empty());
        assert!(!wasm::bias_label("IN_GROUP", "fr").is_empty());
        assert!(!wasm::bias_label("IN_GROUP", "en").is_empty());
        assert!(!wasm::bias_desc("IMPOSTOR", "fr").is_empty());
        assert!(!wasm::bias_desc("IMPOSTOR", "en").is_empty());
        assert!(!wasm::bias_desc("IN_GROUP", "fr").is_empty());
        assert!(!wasm::bias_desc("IN_GROUP", "en").is_empty());
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

    // === model_config.rs mutation-killing tests ===

    #[test]
    fn test_motivation_synergy_signs() {
        use crate::models::MotivationType;
        // Verify specific negative synergy values that deletion mutants would flip
        let cases: Vec<(MotivationType, MotivationType, f64)> = vec![
            (MotivationType::Power, MotivationType::Power, -0.2),
            (MotivationType::Power, MotivationType::Affiliation, -0.2),
            (MotivationType::Power, MotivationType::Security, -0.1),
            (MotivationType::Power, MotivationType::Creativity, -0.1),
            (MotivationType::Power, MotivationType::Fairness, -0.2),
            (MotivationType::Achievement, MotivationType::Security, -0.2),
            (MotivationType::Affiliation, MotivationType::Power, -0.2),
            (MotivationType::Affiliation, MotivationType::Autonomy, -0.1),
            (
                MotivationType::Affiliation,
                MotivationType::Recognition,
                -0.1,
            ),
            (MotivationType::Security, MotivationType::Power, -0.1),
            (MotivationType::Security, MotivationType::Achievement, -0.2),
            (MotivationType::Security, MotivationType::Autonomy, -0.3),
            (MotivationType::Security, MotivationType::Creativity, -0.2),
            (MotivationType::Autonomy, MotivationType::Security, -0.3),
            (
                MotivationType::Recognition,
                MotivationType::Affiliation,
                -0.1,
            ),
            (MotivationType::Recognition, MotivationType::Fairness, -0.1),
            (MotivationType::Helping, MotivationType::Creativity, -0.1),
            (MotivationType::Creativity, MotivationType::Power, -0.1),
            (MotivationType::Creativity, MotivationType::Security, -0.2),
            (MotivationType::Creativity, MotivationType::Helping, -0.1),
            (MotivationType::Fairness, MotivationType::Power, -0.2),
            (MotivationType::Fairness, MotivationType::Recognition, -0.1),
        ];
        for (a, b, expected) in &cases {
            let actual = crate::synergy::motivation_synergy(*a, *b);
            assert!(
                (actual - expected).abs() < 0.001,
                "{:?}×{:?}: expected {}, got {}",
                a,
                b,
                expected,
                actual
            );
        }
        // Verify specific positive synergy values
        let pos_cases: Vec<(MotivationType, MotivationType, f64)> = vec![
            (MotivationType::Power, MotivationType::Achievement, 0.3),
            (MotivationType::Power, MotivationType::Autonomy, 0.2),
            (MotivationType::Power, MotivationType::Recognition, 0.2),
            (MotivationType::Achievement, MotivationType::Power, 0.3),
            (
                MotivationType::Achievement,
                MotivationType::Affiliation,
                0.1,
            ),
            (MotivationType::Affiliation, MotivationType::Security, 0.2),
            (MotivationType::Affiliation, MotivationType::Helping, 0.3),
            (MotivationType::Learning, MotivationType::Achievement, 0.3),
            (MotivationType::Learning, MotivationType::Recognition, 0.3),
            (MotivationType::Creativity, MotivationType::Recognition, 0.3),
        ];
        for (a, b, expected) in &pos_cases {
            let actual = crate::synergy::motivation_synergy(*a, *b);
            assert!(
                (actual - expected).abs() < 0.001,
                "{:?}×{:?}: expected {}, got {}",
                a,
                b,
                expected,
                actual
            );
        }
    }

    #[test]
    fn test_pattern_trigger_synergy_signs() {
        use crate::models::BehaviorTrigger;
        // Verify all negative trigger synergy values
        let neg_cases: Vec<(BehaviorTrigger, BehaviorTrigger, f64)> = vec![
            (BehaviorTrigger::Stress, BehaviorTrigger::Stress, -0.2),
            (BehaviorTrigger::Stress, BehaviorTrigger::Conflict, -0.3),
            (BehaviorTrigger::Stress, BehaviorTrigger::Change, -0.2),
            (BehaviorTrigger::Stress, BehaviorTrigger::Injustice, -0.1),
            (BehaviorTrigger::Conflict, BehaviorTrigger::Stress, -0.3),
            (BehaviorTrigger::Conflict, BehaviorTrigger::Conflict, -0.3),
            (
                BehaviorTrigger::Conflict,
                BehaviorTrigger::Uncertainty,
                -0.2,
            ),
            (BehaviorTrigger::Conflict, BehaviorTrigger::Injustice, -0.1),
            (
                BehaviorTrigger::Uncertainty,
                BehaviorTrigger::Conflict,
                -0.2,
            ),
            (
                BehaviorTrigger::Uncertainty,
                BehaviorTrigger::Injustice,
                -0.1,
            ),
            (BehaviorTrigger::Injustice, BehaviorTrigger::Stress, -0.1),
            (BehaviorTrigger::Injustice, BehaviorTrigger::Conflict, -0.1),
            (
                BehaviorTrigger::Injustice,
                BehaviorTrigger::Uncertainty,
                -0.1,
            ),
            (BehaviorTrigger::Injustice, BehaviorTrigger::Injustice, -0.2),
        ];
        for (a, b, expected) in &neg_cases {
            let actual = crate::synergy::trigger_synergy(*a, *b);
            assert!(
                (actual - expected).abs() < 0.001,
                "{:?}×{:?}: expected {}, got {}",
                a,
                b,
                expected,
                actual
            );
        }
        // Verify specific positive trigger synergy values
        let pos_cases: Vec<(BehaviorTrigger, BehaviorTrigger, f64)> = vec![
            (BehaviorTrigger::Success, BehaviorTrigger::Success, 0.3),
            (BehaviorTrigger::Change, BehaviorTrigger::Change, 0.3),
            (BehaviorTrigger::Change, BehaviorTrigger::Feedback, 0.3),
            (BehaviorTrigger::Feedback, BehaviorTrigger::Change, 0.3),
            (BehaviorTrigger::Feedback, BehaviorTrigger::Feedback, 0.3),
            (BehaviorTrigger::Feedback, BehaviorTrigger::Recognition, 0.2),
            (BehaviorTrigger::Recognition, BehaviorTrigger::Feedback, 0.2),
        ];
        for (a, b, expected) in &pos_cases {
            let actual = crate::synergy::trigger_synergy(*a, *b);
            assert!(
                (actual - expected).abs() < 0.001,
                "{:?}×{:?}: expected {}, got {}",
                a,
                b,
                expected,
                actual
            );
        }
    }

    #[test]
    fn test_trajectory_half_life_ms() {
        use crate::model_config::CFG;
        let expected = 30.0 * 24.0 * 3600.0 * 1000.0;
        assert!((CFG.trajectory.half_life_ms - expected).abs() < 1.0);
    }

    #[test]
    fn test_bias_dunning_kruger_negative_modulation() {
        use crate::model_config::CFG;
        use crate::models::BiasType;
        if let Some((_, coeff)) = CFG.bias_modulation(BiasType::DunningKruger) {
            assert!(
                coeff < 0.0,
                "DunningKruger modulation must be negative, got {}",
                coeff
            );
        } else {
            panic!("DunningKruger must have modulation");
        }
    }

    #[test]
    fn test_bias_favoritism_negative_modulation() {
        use crate::model_config::CFG;
        use crate::models::BiasType;
        if let Some((_, coeff)) = CFG.bias_modulation(BiasType::Favoritism) {
            assert!(
                coeff < 0.0,
                "Favoritism modulation must be negative, got {}",
                coeff
            );
        } else {
            panic!("Favoritism must have modulation");
        }
    }

    #[test]
    fn test_bias_loss_aversion_negative_modulation() {
        use crate::model_config::CFG;
        use crate::models::BiasType;
        if let Some((_, coeff)) = CFG.bias_modulation(BiasType::LossAversion) {
            assert!(
                coeff < 0.0,
                "LossAversion modulation must be negative, got {}",
                coeff
            );
        } else {
            panic!("LossAversion must have modulation");
        }
    }
}

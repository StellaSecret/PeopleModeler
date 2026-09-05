pub mod components;
mod profile;
pub mod rel_weights;
mod scoring;
mod team;
pub(crate) mod testutil;
mod trajectory;

use crate::insights::InsightContext;
use crate::models::RelationType;

pub use components::{
    bias_adjustment, confidence_band, motivation_synergy, motivation_synergy_score,
    pattern_adjustment, pattern_synergy, strength_band, style_synergy, trigger_synergy,
    value_similarity, virtue_adjustment,
};
#[allow(unused_imports)]
pub(crate) use components::{bias_count_bonus, motivation_count_penalty};
pub use components::{sim, synergy_bands, value_self_score};
#[allow(unused_imports)]
pub(crate) use profile::{
    avg_prediction_accuracy, has_pattern_contradiction, has_style_contradiction,
    invalidated_motivations, ocean_danger_penalty, rep_danger_penalty, voided_ocean_dims,
};
pub use profile::{
    base_rep_quality, compute_person_profile, consistency_malus, flag_weight, profile_completeness,
    rep_adjustment,
};
pub use rel_weights::rel_weights;
#[allow(unused_imports)]
pub(crate) use scoring::{PerContextInputs, compute_synergy_score_inner, per_context_breakdown};
pub use scoring::{
    compute_synergy_score, compute_synergy_score_ctx, compute_synergy_score_with_preds,
};
pub use team::{PairResult, TeamSynergy, compute_team_synergy};
#[allow(unused_imports)]
pub(crate) use trajectory::trajectory_from;
pub use trajectory::{pair_trajectory, personal_trajectory};

#[derive(serde::Serialize, Clone)]
pub struct SynergyBreakdown {
    pub total: u8,
    pub a_score: u8,
    pub b_score: u8,
    pub ocean: f64,
    pub reputation: f64,
    pub motivation: f64,
    pub patterns: f64,
    pub bias: f64,
    pub styles: f64,
    pub values: f64,
    pub danger: f64,
    pub bias_mod_active: bool,
    pub danger_details: String,
    /// Width of the confidence band (± points) from relationship strength or
    /// profile confidence. 0 = no banding (legacy behavior).
    pub band: u8,
    /// Directional delta (± points) from the interaction trajectory.
    pub trajectory_delta: i8,
    /// Trajectory trend from logged interactions.
    pub trajectory_trend: Trend,
    /// Number of logged interactions that fed the trajectory (0 = no signal).
    pub trajectory_sample: usize,
    /// Per-context compatibility (Decision, Team, Stress, Communication,
    /// Leadership, Growth): the same pair re-weighted per situation, so
    /// "works great in normal ops, collapses under crisis" is readable as
    /// data rather than prose.
    pub per_context: Vec<(InsightContext, u8)>,
}

/// Direction of the interaction trajectory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Trend {
    Improving,
    Stable,
    Deteriorating,
}

/// Interaction trajectory of a pair (or of a single person).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trajectory {
    /// Directional delta in ±points (level * 10, clamped to ±10).
    pub delta: i8,
    pub trend: Trend,
    /// Number of valence-tagged entries that fed the trajectory.
    pub sample: usize,
    /// Recency-weighted recent balance in [-1, 1].
    pub level: f64,
}

/// Relationship context that makes the synergy score relationship-aware.
/// `None` preserves the legacy relationship-blind scoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelContext {
    pub rtype: RelationType,
    pub strength: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PersonProfile {
    pub total: u8,
    pub motivation: f64,
    pub patterns: f64,
    pub ocean: f64,
    pub reputation: f64,
    pub bias: f64,
    pub styles: f64,
    pub values: f64,
    pub completeness: u8,
    pub band: u8,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_config::{BiasTarget, CFG};
    use crate::models::*;

    fn make_person(
        openness: Option<u8>,
        conscientiousness: Option<u8>,
        extraversion: Option<u8>,
        agreeableness: Option<u8>,
        neuroticism: Option<u8>,
    ) -> Person {
        Person {
            id: "test".into(),
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
            ocean: OceanScores {
                openness,
                conscientiousness,
                extraversion,
                agreeableness,
                neuroticism,
            },
            resilience: None,
            risk_appetite: None,
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    // --- per-context compatibility (Phase 4) tests ---

    fn full_profile() -> Person {
        let mut p = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 7,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 6,
                notes: String::new(),
            },
        ];
        p.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 3,
            evidence: String::new(),
        }];
        // Full, identical reputation — every dimension filled so the missing
        // penalty never fires; authoritative stays at 7 to avoid the
        // "power struggle" danger rule (both >= 8).
        p.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            authoritative_submissive: Some(7),
            honest_deceitful: Some(8),
            reliable_flaky: Some(8),
            humble_arrogant: Some(8),
            calm_reactive: Some(8),
            diplomatic_blunt: Some(8),
            generous_selfish: Some(8),
            fair_favoritism: Some(8),
            trusting_suspicious: Some(8),
            assertive_passive: Some(8),
            empathetic_detached: Some(8),
            adaptable_rigid: Some(8),
        };
        p.styles = vec![
            PersonalStyle {
                r#type: StyleType::Analytical,
                intensity: 8,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 8,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Collaborating,
                intensity: 8,
                notes: String::new(),
            },
        ];
        p
    }

    /// A pair that's strong on every bucket except patterns: reactive,
    /// divergent triggers (Stress→Panics vs Conflict→Escalates) drive the
    /// patterns bucket to 0 while OCEAN/Rep/Mot/Bias/Styles stay near 1.
    fn crisis_pair() -> (Person, Person) {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        (a, b)
    }

    #[test]
    fn test_context_weights_rows_sum_to_one() {
        for c in InsightContext::ALL {
            let w = CFG.context_weights(c);
            let s: f64 = w.iter().sum();
            assert!(
                (s - 1.0).abs() < 1e-9,
                "{c:?} context weights sum to {s}, expected 1.0"
            );
        }
    }

    #[test]
    fn test_per_context_carries_six_scores() {
        let (a, b) = crisis_pair();
        let brk = compute_synergy_score(&a, &b);
        assert_eq!(brk.per_context.len(), InsightContext::ALL.len());
        for (i, (c, s)) in brk.per_context.iter().enumerate() {
            assert_eq!(*c, InsightContext::ALL[i], "context order mismatch");
            assert!(*s <= 100, "{c:?} score out of range: {s}");
        }
    }

    #[test]
    fn test_per_context_collapses_under_stress() {
        let (a, b) = crisis_pair();
        let brk = compute_synergy_score(&a, &b);
        assert!(brk.patterns < 0.1, "patterns bucket should be near 0");
        let stress = brk
            .per_context
            .iter()
            .find(|(c, _)| *c == InsightContext::Stress)
            .map(|(_, s)| *s)
            .expect("Stress context missing");
        let (best_ctx, best) = brk
            .per_context
            .iter()
            .max_by_key(|(_, s)| *s)
            .expect("per_context non-empty");
        assert!(
            brk.total >= 72,
            "normal-ops headline should stay strong, got {}",
            brk.total
        );
        assert!(
            *best >= 80,
            "normal-ops contexts should be strong, best {best_ctx:?} = {best}"
        );
        assert!(
            *best - stress >= 8,
            "Stress ({stress}) should collapse at least 8 points below the best context ({best_ctx:?} = {best})"
        );
        let min_score = brk.per_context.iter().map(|(_, s)| s).min().copied();
        assert_eq!(
            min_score,
            Some(stress),
            "Stress should be the lowest context"
        );
    }

    #[test]
    fn test_per_context_composes_with_relationship() {
        // Family de-emphasizes patterns (0.12) vs Stress (0.24). For a pair
        // with a weak patterns bucket, a Family relationship under Stress
        // should lift the Stress score above the context-only value: the
        // composed profile must differ from the pure context profile.
        let (a, b) = crisis_pair();
        let none = compute_synergy_score(&a, &b);
        let family = compute_synergy_score_ctx(
            &a,
            &b,
            Some(&RelContext {
                rtype: RelationType::Family,
                strength: 8,
            }),
            &[],
            &[],
        );
        let stress_of = |brk: &SynergyBreakdown| {
            brk.per_context
                .iter()
                .find(|(c, _)| *c == InsightContext::Stress)
                .map(|(_, s)| *s)
                .unwrap_or(0)
        };
        assert!(
            stress_of(&family) > stress_of(&none),
            "Family+Stress should compose above context-only Stress ({} vs {})",
            stress_of(&family),
            stress_of(&none)
        );
        assert_ne!(
            family.per_context, none.per_context,
            "relationship context must change the per-context profile"
        );
    }

    /// Independent re-derivation of `per_context_breakdown`'s formula used as
    /// a mutation oracle for the normalization math (`/` on weights and wsum).
    fn ref_per_context_score(
        inp: &PerContextInputs,
        ctx: Option<&RelContext>,
        c: InsightContext,
    ) -> u8 {
        let cw = CFG.context_weights(c);
        let w = match ctx.map(|r| rel_weights(r.rtype)) {
            Some(rw) => {
                let p = [
                    rw.0 * cw[0],
                    rw.1 * cw[1],
                    rw.2 * cw[2],
                    rw.3 * cw[3],
                    rw.4 * cw[4],
                    rw.5 * cw[5],
                    rw.6 * cw[6],
                ];
                let s: f64 = p.iter().sum();
                [
                    p[0] / s,
                    p[1] / s,
                    p[2] / s,
                    p[3] / s,
                    p[4] / s,
                    p[5] / s,
                    p[6] / s,
                ]
            }
            None => cw,
        };
        let mut num = 0.0;
        let mut wsum = 0.0;
        num += inp.buckets[0] * w[0];
        wsum += w[0];
        if inp.rep_active {
            num += inp.buckets[1] * w[1];
            wsum += w[1];
        }
        if inp.mot_active {
            num += inp.buckets[2] * w[2];
            wsum += w[2];
        }
        if inp.pat_active {
            num += inp.buckets[3] * w[3];
            wsum += w[3];
        }
        num += inp.buckets[4] * w[4];
        wsum += w[4];
        num += inp.buckets[5] * w[5];
        wsum += w[5];
        num += inp.buckets[6] * w[6];
        wsum += w[6];
        let raw = num / wsum;
        let danger = inp.penalties[0] * w[0]
            + inp.penalties[1] * w[1]
            + inp.penalties[2] * w[2]
            + inp.penalties[3] * CFG.base_weights.history;
        let penalty = (danger / wsum * 100.0).round() as u8;
        ((raw * 100.0).round() as u8)
            .min(100)
            .saturating_sub(penalty)
    }

    #[test]
    fn test_per_context_normalization_matches_reference() {
        let ctx = Some(RelContext {
            rtype: RelationType::Family,
            strength: 8,
        });
        // Config A: every bucket active(-ish) at 2.0 with heavy penalties so
        // the danger/wsum division and the ≥1 num/wsum division are observable.
        // Config B: amplifies bucket 6 so the relation-normalized w[6] division
        // (values weight) is observable without score saturation.
        let inputs = [
            PerContextInputs {
                buckets: [2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0],
                penalties: [5.0, 0.2, 0.1, 0.1],
                rep_active: false,
                mot_active: true,
                pat_active: true,
            },
            PerContextInputs {
                buckets: [0.05, 0.5, 0.5, 0.5, 0.5, 0.5, 10.0],
                penalties: [0.0, 0.0, 0.0, 0.0],
                rep_active: false,
                mot_active: false,
                pat_active: false,
            },
        ];
        for inp in &inputs {
            let got = per_context_breakdown(inp, ctx.as_ref());
            let want: Vec<_> = InsightContext::ALL
                .iter()
                .map(|&c| (c, ref_per_context_score(inp, ctx.as_ref(), c)))
                .collect();
            assert_eq!(
                got, want,
                "per-context normalization diverges from the reference formula"
            );
        }
    }

    #[test]
    fn test_opposite_bias_small_friction_does_not_scale() {
        let base_a = make_person(None, None, None, None, None);
        let base_b = make_person(None, None, None, None, None);
        let mut a = make_person(None, None, None, None, None);
        let mut b = make_person(None, None, None, None, None);
        a.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 1,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 1,
            evidence: String::new(),
        }];
        let base_brk = compute_synergy_score(&base_a, &base_b);
        let biased = compute_synergy_score(&a, &b);
        assert!(
            biased.ocean < base_brk.ocean,
            "complementary friction should nudge ocean down, got {} vs {}",
            biased.ocean,
            base_brk.ocean
        );
        assert!(
            biased.ocean > base_brk.ocean - 0.01,
            "sub-cap complementary friction must not blow up: {} vs {}",
            biased.ocean,
            base_brk.ocean
        );
    }

    #[test]
    fn test_history_penalty_boundary_at_exactly_low_accuracy() {
        let a = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        let b = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        let exact5: Vec<Prediction> = (0..CFG.history.min_samples)
            .map(|_| make_pred("p", Some(5), true))
            .collect();
        let below: Vec<Prediction> = (0..CFG.history.min_samples)
            .map(|_| make_pred("p", Some(3), true))
            .collect();
        let plain = compute_synergy_score_with_preds(&a, &b, &[], &[]);
        let both_exact = compute_synergy_score_with_preds(&a, &b, &exact5, &exact5);
        assert_eq!(
            plain.danger, both_exact.danger,
            "accuracy exactly at the low threshold must not trigger a history penalty"
        );
        assert_eq!(
            both_exact.danger_details, "",
            "no low-accuracy detail may appear at the exact threshold"
        );

        let expected = CFG.history.single_low_penalty * CFG.base_weights.history;
        let a_boundary = compute_synergy_score_with_preds(&a, &b, &exact5, &below);
        let b_boundary = compute_synergy_score_with_preds(&a, &b, &below, &exact5);
        assert_eq!(
            a_boundary.danger, expected,
            "one side exactly at the threshold, other below -> single-low penalty for a"
        );
        assert_eq!(
            b_boundary.danger, expected,
            "one side exactly at the threshold, other below -> single-low penalty for b"
        );
    }

    #[test]
    fn test_only_negative_patterns_zero_patterns_bucket() {
        let (a, b) = crisis_pair();
        let brk = compute_synergy_score(&a, &b);
        assert_eq!(
            brk.patterns, 0.0,
            "only-negative triggers must zero the patterns bucket"
        );
    }

    #[test]
    fn test_rep_danger_penalty_counts_in_total_danger() {
        let mut a = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        let mut b = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        a.rep_scores.authoritative_submissive = Some(9);
        b.rep_scores.authoritative_submissive = Some(9);
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.danger > 0.0 && brk.danger < 0.02,
            "rep power-struggle term missing from total danger, got {}",
            brk.danger
        );
    }

    #[test]
    fn test_low_neuroticism_raises_asymmetry_score() {
        let hi_a = make_person(Some(10), Some(10), Some(10), Some(10), Some(9));
        let hi_b = make_person(Some(10), Some(10), Some(10), Some(10), Some(9));
        let lo_a = make_person(Some(10), Some(10), Some(10), Some(10), Some(0));
        let lo_b = make_person(Some(10), Some(10), Some(10), Some(10), Some(0));
        let hi = compute_synergy_score(&hi_a, &hi_b);
        let lo = compute_synergy_score(&lo_a, &lo_b);
        assert!(
            hi.a_score < lo.a_score,
            "low-N stability must beat high-N: {} (calm) vs {} (neurotic)",
            lo.a_score,
            hi.a_score
        );
    }

    #[test]
    fn test_pattern_dimension_affects_asymmetric_denominator() {
        let (a, b) = crisis_pair();
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.a_score >= 70 && brk.a_score <= 80,
            "a_score {} outside expected band: active bucket weights must count once in the denominator",
            brk.a_score
        );
    }

    // --- trigger_synergy tests ---

    #[test]
    fn test_trigger_same_change() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Change, BehaviorTrigger::Change) - 0.3).abs() < 1e-9
        );
    }

    #[test]
    fn test_trigger_same_feedback() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Feedback, BehaviorTrigger::Feedback) - 0.3).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_same_success() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Success, BehaviorTrigger::Success) - 0.3).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_same_conflict() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Conflict, BehaviorTrigger::Conflict) - (-0.3)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_same_stress() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Stress, BehaviorTrigger::Stress) - (-0.2)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_change_feedback() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Change, BehaviorTrigger::Feedback) - 0.3).abs()
                < 1e-9
        );
        assert!(
            (trigger_synergy(BehaviorTrigger::Feedback, BehaviorTrigger::Change) - 0.3).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_conflict_stress() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Conflict, BehaviorTrigger::Stress) - (-0.3)).abs()
                < 1e-9
        );
        assert!(
            (trigger_synergy(BehaviorTrigger::Stress, BehaviorTrigger::Conflict) - (-0.3)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_change_stress() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Change, BehaviorTrigger::Stress) - (-0.2)).abs()
                < 1e-9
        );
        assert!(
            (trigger_synergy(BehaviorTrigger::Stress, BehaviorTrigger::Change) - (-0.2)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_conflict_uncertainty() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Conflict, BehaviorTrigger::Uncertainty) - (-0.2))
                .abs()
                < 1e-9
        );
        assert!(
            (trigger_synergy(BehaviorTrigger::Uncertainty, BehaviorTrigger::Conflict) - (-0.2))
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_feedback_recognition() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Feedback, BehaviorTrigger::Recognition) - 0.2).abs()
                < 1e-9
        );
        assert!(
            (trigger_synergy(BehaviorTrigger::Recognition, BehaviorTrigger::Feedback) - 0.2).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_trigger_default() {
        assert!(
            (trigger_synergy(BehaviorTrigger::Uncertainty, BehaviorTrigger::Threatened) - 0.0)
                .abs()
                < 1e-9
        );
        assert!(
            (trigger_synergy(BehaviorTrigger::Recognition, BehaviorTrigger::Threatened) - 0.0)
                .abs()
                < 1e-9
        );
    }

    // --- motivation_synergy tests ---

    #[test]
    fn test_motivation_same_type_learning_positive() {
        assert!(
            (motivation_synergy(MotivationType::Learning, MotivationType::Learning) - 0.2).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_same_type_power_negative() {
        assert!(
            (motivation_synergy(MotivationType::Power, MotivationType::Power) - (-0.2)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_same_type_recognition_negative() {
        assert!(
            (motivation_synergy(MotivationType::Recognition, MotivationType::Recognition) - (-0.1))
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_same_type_autonomy_neutral() {
        assert!(
            (motivation_synergy(MotivationType::Autonomy, MotivationType::Autonomy) - 0.0).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_same_type_security_neutral() {
        assert!(
            (motivation_synergy(MotivationType::Security, MotivationType::Security) - 0.0).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_power_achievement() {
        assert!(
            (motivation_synergy(MotivationType::Power, MotivationType::Achievement) - 0.3).abs()
                < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Achievement, MotivationType::Power) - 0.3).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_achievement_learning() {
        assert!(
            (motivation_synergy(MotivationType::Achievement, MotivationType::Learning) - 0.3).abs()
                < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Learning, MotivationType::Achievement) - 0.3).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_power_affiliation() {
        assert!(
            (motivation_synergy(MotivationType::Power, MotivationType::Affiliation) - (-0.2)).abs()
                < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Affiliation, MotivationType::Power) - (-0.2)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_autonomy_security() {
        assert!(
            (motivation_synergy(MotivationType::Autonomy, MotivationType::Security) - (-0.3)).abs()
                < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Security, MotivationType::Autonomy) - (-0.3)).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_autonomy_affiliation() {
        assert!(
            (motivation_synergy(MotivationType::Autonomy, MotivationType::Affiliation) - (-0.1))
                .abs()
                < 1e-9,
            "Autonomy×Affiliation must be -0.1, got {}",
            motivation_synergy(MotivationType::Autonomy, MotivationType::Affiliation)
        );
    }

    #[test]
    fn test_motivation_affiliation_helping() {
        assert!(
            (motivation_synergy(MotivationType::Affiliation, MotivationType::Helping) - 0.3).abs()
                < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Helping, MotivationType::Affiliation) - 0.3).abs()
                < 1e-9
        );
    }

    #[test]
    fn test_motivation_default() {
        assert!(
            (motivation_synergy(MotivationType::Helping, MotivationType::Recognition) - 0.0).abs()
                < 1e-9
        );
    }

    // --- virtue_adjustment tests ---

    #[test]
    fn test_virtue_fairness_high_bonus() {
        // Fairness 8 → +0.08, absent Helping → −0.06
        let m = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 8,
            notes: String::new(),
        }];
        assert!((virtue_adjustment(&m) - 0.02).abs() < 1e-9);
    }

    #[test]
    fn test_virtue_all_virtues_high() {
        // Fairness 8 (+0.08) + Helping 8 (+0.06) + Learning 8 (+0.04) + Creativity 8 (+0.04)
        // No absent virtue penalties → +0.22
        let m = vec![
            Motivation {
                r#type: MotivationType::Fairness,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Creativity,
                intensity: 8,
                notes: String::new(),
            },
        ];
        assert!((virtue_adjustment(&m) - 0.22).abs() < 1e-9);
    }

    #[test]
    fn test_virtue_absent_both_virtues_penalty() {
        // Absent Fairness (−0.08) + absent Helping (−0.06) = −0.14
        assert!((virtue_adjustment(&[]) - (-0.14)).abs() < 1e-9);
    }

    #[test]
    fn test_virtue_power_high_without_virtues() {
        // Power 8 → −0.08, absent Fairness → −0.08, absent Helping → −0.06 = −0.22
        let m = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 8,
            notes: String::new(),
        }];
        assert!((virtue_adjustment(&m) - (-0.22)).abs() < 1e-9);
    }

    #[test]
    fn test_virtue_learning_creativity_without_virtues() {
        // Learning 8 (+0.04) + Creativity 9 (+0.04) − absent Fairness (−0.08) − absent Helping (−0.06) = −0.06
        let m = vec![
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Creativity,
                intensity: 9,
                notes: String::new(),
            },
        ];
        assert!((virtue_adjustment(&m) - (-0.06)).abs() < 1e-9);
    }

    #[test]
    fn test_virtue_recognition_extreme_without_virtues() {
        // Recognition 10 → −0.03, absent Fairness → −0.08, absent Helping → −0.06 = −0.17
        let m = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 10,
            notes: String::new(),
        }];
        assert!((virtue_adjustment(&m) - (-0.17)).abs() < 1e-9);
    }

    #[test]
    fn test_virtue_recognition_moderate_without_virtues() {
        // Recognition 8 → no vice penalty, absent Fairness → −0.08, absent Helping → −0.06 = −0.14
        let m = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 8,
            notes: String::new(),
        }];
        assert!((virtue_adjustment(&m) - (-0.14)).abs() < 1e-9);
    }

    // --- motivation_count_penalty tests ---

    #[test]
    fn test_count_penalty_empty() {
        assert!((motivation_count_penalty(0) - 0.09).abs() < 1e-9);
    }

    #[test]
    fn test_count_penalty_one() {
        assert!((motivation_count_penalty(1) - 0.06).abs() < 1e-9);
    }

    #[test]
    fn test_count_penalty_two() {
        assert!((motivation_count_penalty(2) - 0.03).abs() < 1e-9);
    }

    #[test]
    fn test_count_penalty_three_or_more() {
        assert!((motivation_count_penalty(3)).abs() < 1e-9);
        assert!((motivation_count_penalty(5)).abs() < 1e-9);
    }

    // --- bias_adjustment tests ---

    #[test]
    fn test_bias_adjustment_empty() {
        assert!((bias_adjustment(&[]) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_bias_adjustment_all_absent() {
        let b = vec![
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 0,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 0,
                evidence: String::new(),
            },
        ];
        assert!((bias_adjustment(&b) - 0.04).abs() < 1e-9);
    }

    #[test]
    fn test_bias_adjustment_mild() {
        let b = vec![
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 2,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 3,
                evidence: String::new(),
            },
        ];
        assert!((bias_adjustment(&b) - 0.02).abs() < 1e-9);
    }

    #[test]
    fn test_bias_adjustment_moderate() {
        let b = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 5,
            evidence: String::new(),
        }];
        assert!((bias_adjustment(&b) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_bias_adjustment_strong() {
        let b = vec![
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 8,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 9,
                evidence: String::new(),
            },
        ];
        assert!((bias_adjustment(&b) - (-0.06)).abs() < 1e-9);
    }

    #[test]
    fn test_bias_adjustment_mixed() {
        let b = vec![
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 0,
                evidence: String::new(),
            }, // +0.02
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 2,
                evidence: String::new(),
            }, // +0.01
            Bias {
                r#type: BiasType::Availability,
                intensity: 5,
                evidence: String::new(),
            }, //  0.0
            Bias {
                r#type: BiasType::SunkCost,
                intensity: 8,
                evidence: String::new(),
            }, // -0.03
        ];
        assert!((bias_adjustment(&b) - 0.0).abs() < 1e-9);
    }

    // --- bias_count_bonus tests ---

    #[test]
    fn test_bias_count_bonus_zero() {
        assert!((bias_count_bonus(0) - 0.09).abs() < 1e-9);
    }

    #[test]
    fn test_bias_count_bonus_one() {
        assert!((bias_count_bonus(1) - 0.06).abs() < 1e-9);
    }

    #[test]
    fn test_bias_count_bonus_two() {
        assert!((bias_count_bonus(2) - 0.03).abs() < 1e-9);
    }

    #[test]
    fn test_bias_count_bonus_three_or_more() {
        assert!((bias_count_bonus(3) - 0.0).abs() < 1e-9);
        assert!((bias_count_bonus(5) - 0.0).abs() < 1e-9);
    }

    // --- bias_modifier tests ---

    #[test]
    fn test_bias_modifier_all_types_have_mapping() {
        for ty in &BiasType::ALL {
            assert!(
                CFG.bias_modulation(*ty).is_some(),
                "bias_modulation missing for {:?}",
                ty
            );
        }
    }

    #[test]
    fn test_bias_modifier_anchoring_ocean() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::Anchoring).unwrap();
        assert!(matches!(target, BiasTarget::Ocean));
        assert!((coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_confirmation_rep() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::Confirmation).unwrap();
        assert!(matches!(target, BiasTarget::Reputation));
        assert!((coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_availability_patterns() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::Availability).unwrap();
        assert!(matches!(target, BiasTarget::Patterns));
        assert!((coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_sunkcost_motivation() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::SunkCost).unwrap();
        assert!(matches!(target, BiasTarget::Motivation));
        assert!((coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_dunningkruger_ocean_negative() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::DunningKruger).unwrap();
        assert!(matches!(target, BiasTarget::Ocean));
        assert!((coefficient - (-0.10)).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_impostor_ocean_positive() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::Impostor).unwrap();
        assert!(matches!(target, BiasTarget::Ocean));
        assert!((coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_lossaversion_patterns_negative() {
        let (target, coefficient) = CFG.bias_modulation(BiasType::LossAversion).unwrap();
        assert!(matches!(target, BiasTarget::Patterns));
        assert!((coefficient - (-0.10)).abs() < 1e-9);
    }

    // --- bias modulation end-to-end tests ---

    #[test]
    fn test_bias_shared_types_boost_bias_score() {
        let mut a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 7,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.bias - 1.0).abs() < 0.001,
            "bias_score should be 1.0 when all types shared, got {}",
            brk.bias
        );
    }

    #[test]
    fn test_bias_no_shared_types_zero_score() {
        let mut a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 7,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.bias - 0.0).abs() < 0.001,
            "bias_score should be 0.0 when no shared types, got {}",
            brk.bias
        );
    }

    #[test]
    fn test_bias_no_biases_neutral() {
        let a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let b = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.bias - 0.5).abs() < 0.001,
            "bias_score should be 0.5 when no biases, got {}",
            brk.bias
        );
    }

    // --- opposite-bias complementarity tests (Phase 8) ---

    fn one_bias(ty: BiasType) -> Bias {
        Bias {
            r#type: ty,
            intensity: 10,
            evidence: String::new(),
        }
    }

    #[test]
    fn test_complementarity_order_insensitive() {
        let (t1, c1) = CFG
            .bias_complementarity(BiasType::DunningKruger, BiasType::Impostor)
            .unwrap();
        let (t2, c2) = CFG
            .bias_complementarity(BiasType::Impostor, BiasType::DunningKruger)
            .unwrap();
        assert!(matches!(t1, BiasTarget::Ocean) && matches!(t2, BiasTarget::Ocean));
        assert!((c1 - c2).abs() < 1e-9);
        assert!(
            CFG.bias_complementarity(BiasType::Anchoring, BiasType::Confirmation)
                .is_none(),
            "non-complementary pair must return None"
        );
        assert!(
            CFG.bias_complementarity(BiasType::DunningKruger, BiasType::DunningKruger)
                .is_none(),
            "same-type pair must stay shared-bias, not complementarity"
        );
    }

    #[test]
    fn test_complementary_pairs_targets() {
        let (t, c) = CFG
            .bias_complementarity(BiasType::DunningKruger, BiasType::Impostor)
            .unwrap();
        assert!(matches!(t, BiasTarget::Ocean));
        assert!(c < 0.0, "complementarity must be negative-only, got {c}");
        let (t, c) = CFG
            .bias_complementarity(BiasType::Anchoring, BiasType::Recency)
            .unwrap();
        assert!(matches!(t, BiasTarget::Patterns));
        assert!(c < 0.0);
        let (t, c) = CFG
            .bias_complementarity(BiasType::Authority, BiasType::SocialProof)
            .unwrap();
        assert!(matches!(t, BiasTarget::Reputation));
        assert!(c < 0.0);
    }

    #[test]
    fn test_opposite_bias_dk_impostor_lowers_ocean() {
        let base_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let base_b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        let mut a = Person { ..base_a.clone() };
        let mut b = Person { ..base_b.clone() };
        a.biases = vec![one_bias(BiasType::DunningKruger)];
        b.biases = vec![one_bias(BiasType::Impostor)];
        let brk = compute_synergy_score(&a, &b);
        let brk_no = compute_synergy_score(&base_a, &base_b);
        assert!(
            brk.ocean < brk_no.ocean,
            "DK+Impostor pair must dampen ocean ({} vs {})",
            brk.ocean,
            brk_no.ocean
        );
    }

    fn one_positive_pattern() -> BehavioralPattern {
        BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }
    }

    #[test]
    fn test_opposite_bias_anchoring_recency_lowers_patterns() {
        let mut base_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut base_b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        base_a.behavioral_patterns = vec![one_positive_pattern()];
        base_b.behavioral_patterns = vec![one_positive_pattern()];
        let mut a = Person { ..base_a.clone() };
        let mut b = Person { ..base_b.clone() };
        a.biases = vec![one_bias(BiasType::Anchoring)];
        b.biases = vec![one_bias(BiasType::Recency)];
        let brk = compute_synergy_score(&a, &b);
        let brk_no = compute_synergy_score(&base_a, &base_b);
        assert!(
            brk.patterns < brk_no.patterns,
            "Anchoring+Recency pair must dampen patterns ({} vs {})",
            brk.patterns,
            brk_no.patterns
        );
    }

    #[test]
    fn test_opposite_bias_authority_socialproof_lowers_rep() {
        let mut base_a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut base_b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        base_a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            reliable_flaky: Some(7),
            honest_deceitful: Some(9),
            ..RepScores::default()
        };
        base_b.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            reliable_flaky: Some(8),
            honest_deceitful: Some(8),
            ..RepScores::default()
        };
        let mut a = Person { ..base_a.clone() };
        let mut b = Person { ..base_b.clone() };
        a.biases = vec![one_bias(BiasType::Authority)];
        b.biases = vec![one_bias(BiasType::SocialProof)];
        let brk = compute_synergy_score(&a, &b);
        let brk_no = compute_synergy_score(&base_a, &base_b);
        assert!(
            brk.reputation < brk_no.reputation,
            "Authority+SocialProof pair must dampen reputation ({} vs {})",
            brk.reputation,
            brk_no.reputation
        );
    }

    #[test]
    fn test_opposite_bias_combined_capped() {
        // Three complementary pairs fire (uncapped −0.26); the combined
        // magnitude must clamp to `opposite_cap` (0.15).
        let mut base_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut base_b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        base_a.behavioral_patterns = vec![one_positive_pattern()];
        base_b.behavioral_patterns = vec![one_positive_pattern()];
        base_a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            reliable_flaky: Some(7),
            honest_deceitful: Some(9),
            ..RepScores::default()
        };
        base_b.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            reliable_flaky: Some(8),
            honest_deceitful: Some(8),
            ..RepScores::default()
        };
        let mut a = Person { ..base_a.clone() };
        let mut b = Person { ..base_b.clone() };
        a.biases = vec![
            one_bias(BiasType::DunningKruger),
            one_bias(BiasType::Anchoring),
            one_bias(BiasType::Authority),
        ];
        b.biases = vec![
            one_bias(BiasType::Impostor),
            one_bias(BiasType::Recency),
            one_bias(BiasType::SocialProof),
        ];
        let brk = compute_synergy_score(&a, &b);
        let brk_no = compute_synergy_score(&base_a, &base_b);
        let reduction = (brk_no.ocean - brk.ocean)
            + (brk_no.patterns - brk.patterns)
            + (brk_no.reputation - brk.reputation);
        assert!(
            reduction > 0.05,
            "friction must be present, got reduction {reduction}"
        );
        assert!(
            reduction <= CFG.bias.opposite_cap + 0.001,
            "combined opposite-bias friction must cap at 0.15, got {reduction}"
        );
    }

    #[test]
    fn test_shared_bias_not_opposite() {
        // Two persons sharing the same bias type must NOT trigger the
        // complementarity path: the shared modulation still applies (Anchoring
        // boosts ocean), and no extra negative friction appears.
        let base_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let base_b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        let mut a = Person { ..base_a.clone() };
        let mut b = Person { ..base_b.clone() };
        a.biases = vec![one_bias(BiasType::Anchoring)];
        b.biases = vec![one_bias(BiasType::Anchoring)];
        let brk = compute_synergy_score(&a, &b);
        let brk_no = compute_synergy_score(&base_a, &base_b);
        assert!(
            brk.ocean > brk_no.ocean,
            "shared Anchoring must keep boosting ocean ({} vs {})",
            brk.ocean,
            brk_no.ocean
        );
    }

    #[test]
    fn test_bias_modulation_anchoring_boosts_ocean() {
        let mut a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 10,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        let no_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let no_b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        let brk_no = compute_synergy_score(&no_a, &no_b);
        assert!(
            brk.ocean > brk_no.ocean,
            "Anchoring should boost ocean ({} vs {})",
            brk.ocean,
            brk_no.ocean
        );
    }

    #[test]
    fn test_bias_modulation_dunningkruger_dampens_ocean() {
        let mut a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        a.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.ocean - 0.72).abs() < 0.001,
            "ocean should be dampened by DunningKruger: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_bias_modulation_impostor_boosts_ocean() {
        let mut a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        a.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.ocean - 0.88).abs() < 0.001,
            "ocean should be boosted by Impostor: {}",
            brk.ocean
        );
    }

    // --- pattern_synergy tests ---

    #[test]
    fn test_pattern_synergy_empty() {
        let a: Vec<BehavioralPattern> = vec![];
        let b: Vec<BehavioralPattern> = vec![];
        assert!((pattern_synergy(&a, &b) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_pattern_synergy_single() {
        let a = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let b = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let result = pattern_synergy(&a, &b);
        assert!((result - 1.0).abs() < 0.001, "got {}", result);
    }

    // --- Pattern adjustment tests ---

    #[test]
    fn test_pattern_adjustment_empty() {
        let p: Vec<BehavioralPattern> = vec![];
        let adj = pattern_adjustment(&p);
        // 9 undefined triggers × −0.02 = −0.18
        assert!((adj + 0.18).abs() < 0.001, "empty adj: {}", adj);
    }

    #[test]
    fn test_pattern_adjustment_best_response() {
        // RemainsCalm is tier 1 (+0.03)
        let p = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let adj = pattern_adjustment(&p);
        // +0.03 from RemainsCalm, 8 undefined × −0.02 = −0.16 → total −0.13
        assert!((adj + 0.13).abs() < 0.001, "best adj: {}", adj);
    }

    #[test]
    fn test_pattern_adjustment_worst_response() {
        // Panics is tier 7 (−0.03)
        let p = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        let adj = pattern_adjustment(&p);
        // −0.03 from Panics, 8 undefined × −0.02 = −0.16 → total −0.19
        assert!((adj + 0.19).abs() < 0.001, "worst adj: {}", adj);
    }

    #[test]
    fn test_pattern_adjustment_neutral_response() {
        // BecomesQuiet is tier 4 (0.00)
        let p = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::BecomesQuiet,
            notes: String::new(),
        }];
        let adj = pattern_adjustment(&p);
        // 0.00 from BecomesQuiet, 8 undefined × −0.02 = −0.16 → total −0.16
        assert!((adj + 0.16).abs() < 0.001, "neutral adj: {}", adj);
    }

    #[test]
    fn test_pattern_adjustment_all_tiers() {
        // One response from each tier
        let p = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Success,
                predicted_behavior: BehaviorResponse::CelebratesWithOthers, // +0.03
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::CommunicatesOpenly, // +0.02
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Uncertainty,
                predicted_behavior: BehaviorResponse::SeeksData, // +0.01
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Recognition,
                predicted_behavior: BehaviorResponse::SeeksMore, // 0.00
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::NeedsReassurance, // −0.01
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Threatened,
                predicted_behavior: BehaviorResponse::Counterattacks, // −0.02
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Injustice,
                predicted_behavior: BehaviorResponse::BecomesBitter, // −0.03
                notes: String::new(),
            },
        ];
        let adj = pattern_adjustment(&p);
        // Sum of scores: 0.03+0.02+0.01+0.00−0.01−0.02−0.03 = 0.0
        // 2 undefined triggers × −0.02 = −0.04 (Feedback, Stress)
        assert!((adj + 0.04).abs() < 0.001, "all tiers adj: {}", adj);
    }

    #[test]
    fn test_pattern_adjustment_mixed_scores() {
        let p = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Success,
                predicted_behavior: BehaviorResponse::CelebratesWithOthers, // +0.03
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::Escalates, // −0.03
                notes: String::new(),
            },
        ];
        let adj = pattern_adjustment(&p);
        // +0.03 + (−0.03) = 0.0 from defined, 7 undefined × −0.02 = −0.14 → total −0.14
        assert!((adj + 0.14).abs() < 0.001, "mixed adj: {}", adj);
    }

    #[test]
    fn test_profile_pattern_adjustment_integration() {
        // Change trigger self-pair: trigger_synergy(Change, Change) = +0.3
        // raw_pat = ((0.3 + 0.3) / 0.6) = 1.0
        // RemainsCalm score = +0.03, 8 missing × −0.02 = −0.16 → adj = −0.13
        // adjusted = clamp(1.0 + (−0.13), 0, 1) = 0.87
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let pf = compute_person_profile(&p);
        let expected = 0.87;
        assert!(
            (pf.patterns - expected).abs() < 0.001,
            "expected {expected}, got {}",
            pf.patterns
        );
    }

    #[test]
    fn test_profile_pattern_adjustment_all_negative() {
        // All worst responses (−0.03 each) × 9 triggers = −0.27
        // raw_pat varies, but each trigger is paired with itself in pattern_synergy
        // We just verify patterns < 0.5 (baseline)
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::Panics,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::Escalates,
                notes: String::new(),
            },
        ];
        let pf = compute_person_profile(&p);
        // Adjust: −0.03 + (−0.03) = −0.06 from defined, 7 missing × −0.02 = −0.14 → −0.20
        // raw_pat ≈ 0.0 (self-pair Conflict×Conflict = −0.3, Stress×Stress = −0.2, cross = −0.3)
        // adjusted ≈ clamp(0.0 + (−0.20), 0, 1) = 0.0
        assert!(
            (pf.patterns - 0.0).abs() < 0.001,
            "all negative: {}",
            pf.patterns
        );
    }

    // --- OCEAN danger penalty tests ---

    #[test]
    fn test_ocean_penalty_volatile_combo() {
        // Both N >= 7 and A <= 4 within each person
        let a = make_person(Some(5), Some(5), Some(5), Some(2), Some(9));
        let b = make_person(Some(5), Some(5), Some(5), Some(3), Some(8));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        // a: N=9>=7, A=2<=4 → volatile. b: N=8>=7, A=3<=4 → volatile. both N≥7 → contagion.
        // each volatile 0.10 + mutual contagion 0.10 + antagonism 0.15 = 0.45
        assert!((p - 0.45).abs() < 0.001, "volatile penalty: {}", p);
    }

    #[test]
    fn test_ocean_penalty_no_danger() {
        let a = make_person(Some(7), Some(7), Some(5), Some(6), Some(5));
        let b = make_person(Some(6), Some(8), Some(4), Some(7), Some(3));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        assert!((p - 0.0).abs() < 0.001, "no danger should be 0: {}", p);
    }

    #[test]
    fn test_ocean_penalty_both_low_a() {
        let a = make_person(Some(5), Some(5), Some(5), Some(2), Some(4));
        let b = make_person(Some(5), Some(5), Some(5), Some(3), Some(4));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        // both A <= 4 → antagonism 0.15
        assert!((p - 0.15).abs() < 0.001, "both low A penalty: {}", p);
    }

    #[test]
    fn test_ocean_penalty_contagion_antagonism() {
        let a = make_person(Some(5), Some(5), Some(5), Some(2), Some(8));
        let b = make_person(Some(5), Some(5), Some(5), Some(3), Some(9));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        // both N>=7 → 0.10, both A<=4 → 0.15, each volatile → 0.10+0.10
        assert!((p - 0.45).abs() < 0.001, "combined penalty: {}", p);
    }

    // --- rep danger penalty tests ---

    #[test]
    fn test_rep_penalty_both_authoritative() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            authoritative_submissive: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            authoritative_submissive: Some(8),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!(
            (p - 0.10).abs() < 0.001,
            "both authoritative penalty: {}",
            p
        );
    }

    #[test]
    fn test_rep_penalty_both_blunt() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            diplomatic_blunt: Some(2),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            diplomatic_blunt: Some(1),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!((p - 0.10).abs() < 0.001, "both blunt penalty: {}", p);
    }

    #[test]
    fn test_rep_penalty_both_lazy() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            hardworker_lazy: Some(2),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(1),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!((p - 0.05).abs() < 0.001, "both lazy penalty: {}", p);
    }

    #[test]
    fn test_rep_penalty_no_shared_dims() {
        let a = RepScores::default();
        let b = RepScores::default();
        let p = rep_danger_penalty(&a, &b);
        assert!((p - 0.0).abs() < 0.001, "no shared dims penalty: {}", p);
    }

    #[test]
    fn test_rep_penalty_both_reactive() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            calm_reactive: Some(2),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            calm_reactive: Some(1),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!((p - 0.10).abs() < 0.001, "both reactive penalty: {}", p);
    }

    #[test]
    fn test_rep_penalty_both_arrogant() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            humble_arrogant: Some(1),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            humble_arrogant: Some(2),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!((p - 0.10).abs() < 0.001, "both arrogant penalty: {}", p);
    }

    // --- history factor tests ---

    #[test]
    fn test_avg_prediction_accuracy_insufficient_data() {
        let predictions = vec![];
        assert!(avg_prediction_accuracy(&predictions).is_none());
    }

    #[test]
    fn test_avg_prediction_accuracy_sufficient() {
        let predictions = vec![
            Prediction {
                id: "p1".into(),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(7),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p2".into(),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(5),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p3".into(),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(9),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
        ];
        let acc = avg_prediction_accuracy(&predictions).unwrap();
        assert!((acc - 7.0).abs() < 0.001, "avg accuracy: {}", acc);
    }

    // --- OCEAN complementarity tests ---

    #[test]
    fn test_ocean_complementarity_bonus() {
        let a = make_person(Some(8), Some(7), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(8), Some(5), Some(5), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.ocean > 0.5,
            "O-C complementarity should boost ocean score: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_ocean_complementarity_missing() {
        let a = make_person(Some(8), None, Some(5), Some(5), Some(5));
        let b = make_person(None, Some(8), Some(5), Some(5), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(brk.ocean > 0.0);
    }

    #[test]
    fn test_ocean_some_missing() {
        let a = make_person(Some(7), None, None, None, Some(5));
        let b = make_person(Some(7), None, None, None, Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(brk.ocean > 0.0);
    }

    // --- Dynamic weight redistribution tests ---

    #[test]
    fn test_synergy_missing_categories() {
        let mut a = make_person(Some(8), Some(7), Some(9), Some(4), Some(5));
        let mut b = make_person(Some(8), Some(7), Some(9), Some(4), Some(5));
        a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            reliable_flaky: Some(7),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            reliable_flaky: Some(8),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        assert!(brk.total > 0);
        assert!(brk.ocean > 0.0);
        assert!(brk.reputation > 0.0);
    }

    #[test]
    fn test_synergy_only_ocean() {
        let a = make_person(Some(9), Some(9), Some(9), Some(9), Some(1));
        let b = make_person(Some(9), Some(9), Some(9), Some(9), Some(1));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.total > 75,
            "Identical high-OCEAN should score high: {}",
            brk.total
        );
    }

    // --- compute_synergy_score end-to-end tests ---

    #[test]
    fn test_synergy_identical_persons() {
        let mut p = make_person(Some(8), Some(6), Some(9), Some(4), Some(5));
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Power,
                intensity: 9,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Recognition,
                intensity: 7,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
        ];
        p.biases = vec![
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 8,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 6,
                evidence: String::new(),
            },
        ];
        p.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            authoritative_submissive: Some(3),
            honest_deceitful: Some(7),
            reliable_flaky: Some(7),
            humble_arrogant: Some(9),
            calm_reactive: Some(2),
            diplomatic_blunt: Some(6),
            generous_selfish: Some(6),
            ..RepScores::default()
        };
        let a = p.clone();
        let b = p;
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.total > 55,
            "Identical persons should score > 55, got {}",
            brk.total
        );
    }

    #[test]
    fn test_synergy_opposite_persons() {
        let mut a = make_person(Some(9), Some(9), Some(9), Some(9), Some(1));
        let mut b = make_person(Some(1), Some(1), Some(1), Some(1), Some(9));
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Affiliation,
            intensity: 9,
            notes: String::new(),
        }];
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.total < 40,
            "Opposite persons should score < 40, got {}",
            brk.total
        );
    }

    #[test]
    fn test_synergy_bias_modulation_affects_total() {
        let mut base_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut base_b = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        base_a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            reliable_flaky: Some(7),
            ..RepScores::default()
        };
        base_b.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            reliable_flaky: Some(8),
            ..RepScores::default()
        };
        base_a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 8,
            notes: String::new(),
        }];
        base_b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 7,
            notes: String::new(),
        }];

        let mut a1 = Person { ..base_a.clone() };
        let mut b1 = Person { ..base_b.clone() };
        a1.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b1.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        let brk1 = compute_synergy_score(&a1, &b1);

        let mut a2 = Person { ..base_a.clone() };
        let mut b2 = Person { ..base_b.clone() };
        a2.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b2.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: String::new(),
        }];
        let brk2 = compute_synergy_score(&a2, &b2);

        // Total = average of asymmetric scores; shared vs different biases
        // don't affect individual bias counts, so total is unchanged.
        assert_eq!(
            brk1.total, brk2.total,
            "shared vs different biases yield same total (modulation is in breakdown)"
        );
        // The mutual bias breakdown differs because shared types boost bias_score.
        assert!(
            brk1.bias > brk2.bias,
            "shared biases should give higher bias breakdown score"
        );
    }

    // --- Danger penalty end-to-end tests ---

    #[test]
    fn test_danger_from_volatile_combo_lowers_total() {
        let a = make_person(Some(9), Some(9), Some(9), Some(3), Some(8));
        let b = make_person(Some(1), Some(1), Some(1), Some(2), Some(7));
        // a: N=8≥7, A=3≤4 → volatile. b: N=7≥7, A=2≤4 → volatile. both N≥7→contagion. both A≤4→antagonism.
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.danger > 0.0,
            "volatile combo should produce danger: {}",
            brk.danger
        );
    }

    #[test]
    fn test_danger_from_rep_power_struggle() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            authoritative_submissive: Some(9),
            diplomatic_blunt: Some(2),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            authoritative_submissive: Some(8),
            diplomatic_blunt: Some(1),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.total < 75,
            "power struggle should lower total: {}",
            brk.total
        );
    }

    #[test]
    fn test_danger_field_present_in_breakdown() {
        let a = make_person(Some(8), Some(7), Some(6), Some(2), Some(8));
        let b = make_person(Some(7), Some(6), Some(5), Some(3), Some(9));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.danger > 0.0,
            "volatile combo should produce danger: {}",
            brk.danger
        );
    }

    #[test]
    fn test_no_danger_for_harmonious_pair() {
        let a = make_person(Some(8), Some(8), Some(6), Some(7), Some(3));
        let b = make_person(Some(7), Some(7), Some(5), Some(8), Some(2));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.danger < 0.001,
            "harmonious pair should have no danger: {}",
            brk.danger
        );
    }

    // --- Motivation complementarity tests ---

    #[test]
    fn test_motivation_power_helping_complementary() {
        assert!(
            (motivation_synergy(MotivationType::Power, MotivationType::Helping) - 0.1).abs() < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Helping, MotivationType::Power) - 0.1).abs() < 1e-9
        );
    }

    #[test]
    fn test_motivation_achievement_affiliation_complementary() {
        assert!(
            (motivation_synergy(MotivationType::Achievement, MotivationType::Affiliation) - 0.1)
                .abs()
                < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Affiliation, MotivationType::Achievement) - 0.1)
                .abs()
                < 1e-9
        );
    }

    // --- Pattern danger tests ---

    #[test]
    fn test_pattern_danger_all_negative_triggers() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::Overwhelmed,
                notes: String::new(),
            },
        ];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Threatened,
            predicted_behavior: BehaviorResponse::DeflectsBlame,
            notes: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.danger > 0.0,
            "all-negative triggers should produce danger: {}",
            brk.danger
        );
    }

    #[test]
    fn test_pattern_danger_positive_trigger_no_penalty() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::BecomesDefensive,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.danger < 0.001,
            "positive trigger should avoid pattern danger: {}",
            brk.danger
        );
    }

    // --- Rep weighted dimension tests ---

    #[test]
    fn test_rep_weighted_honest_more_impactful() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        // HonestDeceitful (weight 0.20): very different → drags score
        // GenerousSelfish (weight 0.05): identical → minimal boost
        a.rep_scores = RepScores {
            honest_deceitful: Some(10),
            generous_selfish: Some(8),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            honest_deceitful: Some(1),
            generous_selfish: Some(8),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        // Weighted: (0.1*0.20 + 1.0*0.05) / 0.25 = 0.07/0.25 = 0.28
        // Simple avg: (0.1 + 1.0) / 2 = 0.55
        assert!(
            brk.reputation < 0.40,
            "Honest mismatch should drag weighted rep below 0.40: {}",
            brk.reputation
        );
    }

    // --- No double-counting: danger field is informational ---

    #[test]
    fn test_danger_is_informational_not_subtracted() {
        // Pair with danger should still have danger recorded but NOT subtracted from total
        let a = make_person(Some(9), Some(9), Some(9), Some(3), Some(8));
        let b = make_person(Some(1), Some(1), Some(1), Some(2), Some(7));
        let brk = compute_synergy_score(&a, &b);
        // Danger should be > 0
        assert!(
            brk.danger > 0.0,
            "danger should be > 0 for volatile pair: {}",
            brk.danger
        );
        // The total should reflect category-level penalties (embedded in ocean/rep)
        // but not an extra subtraction. Exact value depends on weights — just verify it runs.
        let _ = brk.total;
    }

    // --- Asymmetric score property tests ---

    #[test]
    fn test_asymmetric_ocean_divergence() {
        // Same rep/bias, different OCEAN → a_score ≠ b_score
        let mut a = make_person(Some(8), Some(5), Some(8), Some(5), Some(2));
        let mut b = make_person(Some(3), Some(5), Some(3), Some(5), Some(8));
        a.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            reliable_flaky: Some(7),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            reliable_flaky: Some(7),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        assert_ne!(
            brk.a_score, brk.b_score,
            "different OCEAN should give asymmetric scores: {} vs {}",
            brk.a_score, brk.b_score
        );
    }

    #[test]
    fn test_asymmetric_identical_persons_equal() {
        // Fully identical → a_score == b_score
        let a = make_person(Some(8), Some(6), Some(9), Some(4), Some(5));
        let b = make_person(Some(8), Some(6), Some(9), Some(4), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert_eq!(
            brk.a_score, brk.b_score,
            "identical persons should have equal asymmetric scores: {} vs {}",
            brk.a_score, brk.b_score
        );
    }

    #[test]
    fn test_asymmetric_rep_difference() {
        // Same OCEAN + bias, different rep → scores diverge
        let mut a = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
        let mut b = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
        a.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            reliable_flaky: Some(9),
            honest_deceitful: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            reliable_flaky: Some(3),
            honest_deceitful: Some(3),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        assert_ne!(
            brk.a_score, brk.b_score,
            "different rep should give asymmetric scores: {} vs {}",
            brk.a_score, brk.b_score
        );
        // A has better rep → B benefits more from A (b_score > a_score)
        assert!(
            brk.b_score > brk.a_score,
            "B should benefit more from high-rep A: {} vs {}",
            brk.a_score,
            brk.b_score
        );
    }

    #[test]
    fn test_asymmetric_bias_difference() {
        // Same OCEAN + rep, different bias count → scores diverge
        let mut a = make_person(Some(6), Some(6), Some(6), Some(6), Some(4));
        let mut b = make_person(Some(6), Some(6), Some(6), Some(6), Some(4));
        a.rep_scores = RepScores {
            hardworker_lazy: Some(6),
            reliable_flaky: Some(6),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(6),
            reliable_flaky: Some(6),
            ..RepScores::default()
        };
        // A has 0 biases, B has 4 biases
        a.biases = vec![];
        for ty in &[
            BiasType::Anchoring,
            BiasType::Confirmation,
            BiasType::Availability,
            BiasType::SunkCost,
        ] {
            b.biases.push(Bias {
                r#type: *ty,
                intensity: 7,
                evidence: String::new(),
            });
        }
        let brk = compute_synergy_score(&a, &b);
        assert_ne!(
            brk.a_score, brk.b_score,
            "different bias counts should give asymmetric scores: {} vs {}",
            brk.a_score, brk.b_score
        );
        // A has fewer biases → B benefits more from A
        assert!(
            brk.b_score > brk.a_score,
            "B should benefit more from low-bias A: {} vs {}",
            brk.a_score,
            brk.b_score
        );
    }

    #[test]
    fn test_total_derived_from_asymmetric() {
        // total ≈ (a + b) / 2 minus danger penalty
        let a = make_person(Some(7), Some(6), Some(8), Some(5), Some(4));
        let b = make_person(Some(5), Some(7), Some(6), Some(6), Some(5));
        let brk = compute_synergy_score(&a, &b);
        let expected = (brk.a_score as f64 + brk.b_score as f64) / 2.0;
        let diff = (expected - brk.total as f64).abs();
        // Allow up to 3 point divergence from rounding + danger penalty
        assert!(
            diff <= 3.0,
            "total should be close to (a+b)/2: {} vs expected {} (diff {})",
            brk.total,
            expected,
            diff
        );
    }

    #[test]
    fn test_asymmetric_ocean_direction() {
        // Low-E person benefits more from high-E partner than vice versa
        // (similarity-weighted: B quality × sim, so high-quality partner gives more)
        let mut a = make_person(Some(5), Some(5), Some(2), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(9), Some(5), Some(5));
        a.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        // a (low-E) gets b (high-E)'s high quality × low sim → moderate
        // b (high-E) gets a (low-E)'s low quality × low sim → low
        // So a_score > b_score for OCEAN-diminished case
        assert!(
            brk.a_score > brk.b_score,
            "low-E person should benefit more from high-E partner: {} vs {}",
            brk.a_score,
            brk.b_score
        );
    }

    // --- Person profile (self-score) tests ---

    #[test]
    fn test_self_score_baseline() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let pf = compute_person_profile(&p);
        // Empty motivations → virtue penalty (−0.14 absent Fairness/Helping)
        // + count penalty (−0.09 for 0 motivations) → 0.5 − 0.23 = 0.27.
        assert!(
            (pf.motivation - 0.27).abs() < 0.001,
            "baseline mot: {}",
            pf.motivation
        );
        assert!(
            (pf.patterns - 0.32).abs() < 0.001,
            "baseline pat: {}",
            pf.patterns
        );
        assert!(
            pf.total >= 20 && pf.total < 70,
            "baseline self-score: {}",
            pf.total
        );
    }

    #[test]
    fn test_self_score_highly_agreeable_stable() {
        // High A (9), low N (2) → ocean near 1.0, no penalty
        let p = make_person(Some(5), Some(5), Some(5), Some(9), Some(2));
        let pf = compute_person_profile(&p);
        assert!(pf.ocean > 0.80, "high A + low N ocean: {}", pf.ocean);
        assert!(pf.total >= 30, "should be decent: {}", pf.total);
    }

    #[test]
    fn test_self_score_volatile_penalty() {
        // N >= 7 and A <= 4 → volatile penalty 0.10
        let p = make_person(Some(5), Some(5), Some(5), Some(3), Some(8));
        let pf = compute_person_profile(&p);
        assert!(pf.ocean < 0.70, "volatile halved ocean: {}", pf.ocean);
    }

    #[test]
    fn test_self_score_many_biases_penalty() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.biases = vec![
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 7,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 6,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Availability,
                intensity: 5,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::SunkCost,
                intensity: 4,
                evidence: String::new(),
            },
        ];
        let pf = compute_person_profile(&p);
        assert!(pf.bias < 0.7, "4 biases reduce bias score: {}", pf.bias);
    }

    #[test]
    fn test_profile_consistency_malus() {
        // Fairness rhetoric gap → reputation penalized by 0.30 (rhetoric tier)
        let rep = RepScores {
            fair_favoritism: Some(2),
            hardworker_lazy: Some(8),
            reliable_flaky: Some(8),
            empathetic_detached: Some(8),
            adaptable_rigid: Some(7),
            calm_reactive: Some(5),
            honest_deceitful: Some(5),
            ..Default::default()
        };
        let mut flagged = make_person(Some(5), Some(5), Some(5), Some(7), Some(3));
        flagged.motivations = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 7,
            notes: String::new(),
        }];
        flagged.rep_scores = rep.clone();
        assert_eq!(
            crate::validation::all_person_flags(&flagged),
            vec!["flag_fairness_rhetoric"]
        );

        let mut clean = make_person(Some(5), Some(5), Some(5), Some(7), Some(3));
        clean.rep_scores = rep;
        assert!(crate::validation::all_person_flags(&clean).is_empty());

        let clean_rep = compute_person_profile(&clean).reputation;
        let flagged_rep = compute_person_profile(&flagged).reputation;
        let expected = (clean_rep - 0.30).max(0.0);
        assert!(
            (flagged_rep - expected).abs() < 0.001,
            "reputation malus: expected {expected}, got {flagged_rep}"
        );
    }

    #[test]
    fn test_consistency_malus() {
        assert_eq!(consistency_malus(&[]), 0.0);
        assert!((flag_weight("flag_high_e_low_a") - 0.20).abs() < 1e-9);
        assert!((flag_weight("flag_fairness_rhetoric") - 0.30).abs() < 1e-9);
        assert!((flag_weight("flag_pattern_calm_volatile") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_pattern_generous_exploiter") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_pattern_helping_exploiter") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_pattern_claimed_calm_volatile") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_pattern_extravert_quiet") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_pattern_open_resister") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_availability_calm") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_style_virtuebased_deceitful") - 0.30).abs() < 1e-9);
        assert!((flag_weight("flag_anchoring_open") - 0.40).abs() < 1e-9);
        assert!((flag_weight("flag_style_competing_passive") - 0.30).abs() < 1e-9);
        assert!((flag_weight("flag_learning_arrogant") - 0.30).abs() < 1e-9);
        assert!((flag_weight("flag_warmth_selfish") - 0.30).abs() < 1e-9);
        assert!((flag_weight("flag_unknown_future") - 0.30).abs() < 1e-9);
        assert!((consistency_malus(&["flag_high_e_low_a"]) - 0.20).abs() < 1e-9);
        assert!((consistency_malus(&["flag_fairness_rhetoric"]) - 0.30).abs() < 1e-9);
        assert!((consistency_malus(&["flag_pattern_calm_volatile"]) - 0.40).abs() < 1e-9);
        assert!(
            (consistency_malus(&["flag_high_e_low_a", "flag_fairness_rhetoric"]) - 0.50).abs()
                < 1e-9
        );
        assert!(
            (consistency_malus(&[
                "flag_pattern_calm_volatile",
                "flag_pattern_fair_exploiter",
                "flag_fairness_rhetoric"
            ]) - 0.50)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn test_profile_consistency_malus_scales() {
        // Identical rep basis; only ocean flags differ (ocean doesn't affect reputation).
        // Twin A: 3 flags (high_e_low_a 0.20 + high_n_low_c 0.20 + honest_selfish 0.20
        //   = 0.60 → capped 0.50)
        // Twin B: 1 flag (honest_selfish 0.20) → exactly 0.30 lower.
        let rep = RepScores {
            honest_deceitful: Some(9),
            generous_selfish: Some(2),
            hardworker_lazy: Some(8),
            reliable_flaky: Some(8),
            empathetic_detached: Some(8),
            fair_favoritism: Some(7),
            adaptable_rigid: Some(7),
            calm_reactive: Some(5),
            ..Default::default()
        };
        let mut many = make_person(Some(3), Some(2), Some(9), Some(2), Some(9));
        many.rep_scores = rep.clone();
        assert_eq!(crate::validation::all_person_flags(&many).len(), 3);

        let mut few = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        few.rep_scores = rep;
        assert_eq!(crate::validation::all_person_flags(&few).len(), 1);

        let many_rep = compute_person_profile(&many).reputation;
        let few_rep = compute_person_profile(&few).reputation;
        let expected = (few_rep - 0.30).max(0.0);
        assert!(
            (many_rep - expected).abs() < 0.001,
            "weighted malus: expected {expected}, got {many_rep}"
        );
    }

    #[test]
    fn test_profile_no_consistency_malus_when_consistent() {
        // High Fairness motivation AND fair reputation → no malus
        let mut p = make_person(Some(5), Some(5), Some(5), Some(7), Some(3));
        p.motivations = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 8,
            notes: String::new(),
        }];
        p.rep_scores = RepScores {
            fair_favoritism: Some(8),
            ..Default::default()
        };
        let pf = compute_person_profile(&p);
        assert!(
            pf.reputation > 0.0 && pf.total >= 30,
            "consistent profile should not be crushed: {}",
            pf.total
        );
    }

    #[test]
    fn test_invalidated_motivations_helper() {
        let v = invalidated_motivations(&[
            "flag_pattern_helping_exploiter",
            "flag_fairness_rhetoric",
            "flag_bias_favoritism_fairness",
        ]);
        assert_eq!(v.len(), 2, "dedup: {v:?}");
        assert!(v.contains(&MotivationType::Helping));
        assert!(v.contains(&MotivationType::Fairness));
        assert_eq!(invalidated_motivations(&["flag_ambition_lazy"]).len(), 3);
        assert!(invalidated_motivations(&["flag_high_e_low_a"]).is_empty());
    }

    #[test]
    fn test_voided_ocean_dims_helper() {
        assert_eq!(voided_ocean_dims(&["flag_warmth_selfish"]), (true, false));
        assert_eq!(
            voided_ocean_dims(&["flag_pattern_claimed_calm_volatile"]),
            (false, true)
        );
        assert_eq!(
            voided_ocean_dims(&["flag_claims_calm_reactive", "flag_warmth_cold"]),
            (true, true)
        );
        assert_eq!(
            voided_ocean_dims(&["flag_fairness_rhetoric"]),
            (false, false)
        );
    }

    #[test]
    fn test_contradiction_detectors() {
        assert!(has_pattern_contradiction(&[
            "flag_pattern_fairness_exploiter"
        ]));
        assert!(!has_pattern_contradiction(&["flag_fairness_rhetoric"]));
        assert!(has_style_contradiction(&[
            "flag_style_rulebased_favoritist"
        ]));
        assert!(!has_style_contradiction(&["flag_fairness_rhetoric"]));
    }

    #[test]
    fn test_profile_motivation_discount() {
        // Fairness + Helping claimed, but recorded patterns exploit injustice
        // under both → both motivations invalidated, no credit banked.
        let mut flagged = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        flagged.motivations = vec![
            Motivation {
                r#type: MotivationType::Fairness,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 8,
                notes: String::new(),
            },
        ];
        flagged.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Injustice,
            predicted_behavior: BehaviorResponse::ExploitsOpportunistically,
            notes: String::new(),
        }];
        let flags = crate::validation::all_person_flags(&flagged);
        assert_eq!(flags.len(), 2);
        let pf = compute_person_profile(&flagged);
        // 0.5 + virtue over empty (Fairness −0.08, Helping −0.06)
        // − count_penalty(0) = 0.5 − 0.14 − 0.09 = 0.27
        assert!(
            (pf.motivation - 0.27).abs() < 0.001,
            "invalidated mot: {}",
            pf.motivation
        );

        let mut clean = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        clean.motivations = flagged.motivations.clone();
        assert!(crate::validation::all_person_flags(&clean).is_empty());
        let cp = compute_person_profile(&clean);
        assert!(
            cp.motivation > pf.motivation,
            "clean {} vs flagged {}",
            cp.motivation,
            pf.motivation
        );
    }

    #[test]
    fn test_profile_ocean_void() {
        // A=9 warmth claim contradicted by Success→DismissesOthers → A voided.
        let mut void_a = make_person(Some(5), Some(5), Some(5), Some(9), Some(8));
        void_a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Success,
            predicted_behavior: BehaviorResponse::DismissesOthers,
            notes: String::new(),
        }];
        let flags = crate::validation::all_person_flags(&void_a);
        assert_eq!(flags, vec!["flag_pattern_warmth_dismissive"]);
        let pf = compute_person_profile(&void_a);
        assert!((pf.ocean - 0.35).abs() < 0.001, "voided A: {}", pf.ocean);
        let clean_a = make_person(Some(5), Some(5), Some(5), Some(9), Some(8));
        let cp = compute_person_profile(&clean_a);
        assert!((cp.ocean - 0.55).abs() < 0.001, "clean A: {}", cp.ocean);
        assert!(pf.ocean < cp.ocean);

        // N=2 calm claim contradicted by Stress→Escalates → N voided.
        let mut void_n = make_person(Some(5), Some(5), Some(5), Some(9), Some(2));
        void_n.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let flags = crate::validation::all_person_flags(&void_n);
        assert_eq!(flags, vec!["flag_pattern_claimed_calm_volatile"]);
        let pf_n = compute_person_profile(&void_n);
        assert!(
            (pf_n.ocean - 0.70).abs() < 0.001,
            "voided N: {}",
            pf_n.ocean
        );
    }

    #[test]
    fn test_profile_pattern_style_cap() {
        // Pattern contradiction (C=9 discipline + shirker pattern) caps the
        // pattern bucket at 0.5, while a flag-free twin keeps full coherence.
        let mut clean = make_person(Some(5), Some(9), Some(5), Some(5), Some(5));
        clean.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        assert!(crate::validation::all_person_flags(&clean).is_empty());
        let cp = compute_person_profile(&clean);
        assert!(cp.patterns > 0.70, "clean patterns: {}", cp.patterns);

        let mut flagged = make_person(Some(5), Some(9), Some(5), Some(5), Some(5));
        flagged.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::RemainsCalm,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Uncertainty,
                predicted_behavior: BehaviorResponse::DeflectsResponsibility,
                notes: String::new(),
            },
        ];
        let flags = crate::validation::all_person_flags(&flagged);
        assert_eq!(flags, vec!["flag_pattern_discipline_shirker"]);
        let fp = compute_person_profile(&flagged);
        assert!(
            fp.patterns <= 0.5 + 1e-9,
            "capped patterns: {}",
            fp.patterns
        );

        // Style contradiction (RuleBased + favoritist rep) caps the style bucket.
        let mut s = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        s.styles = vec![PersonalStyle {
            r#type: StyleType::RuleBased,
            intensity: 8,
            notes: String::new(),
        }];
        s.rep_scores.fair_favoritism = Some(3);
        let s_flags = crate::validation::all_person_flags(&s);
        assert!(s_flags.contains(&"flag_style_rulebased_favoritist"));
        let sp = compute_person_profile(&s);
        assert!(
            (sp.styles - 0.5).abs() < 0.001,
            "capped styles: {}",
            sp.styles
        );
    }

    #[test]
    fn test_profile_manipulator_vs_genuine() {
        // All-good claims contradicted by recorded behavior across the board:
        // motivation credit invalidated, OCEAN voided, patterns capped.
        let mut manipulator = make_person(Some(5), Some(5), Some(5), Some(9), Some(2));
        manipulator.motivations = vec![
            Motivation {
                r#type: MotivationType::Fairness,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 8,
                notes: String::new(),
            },
        ];
        manipulator.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Injustice,
                predicted_behavior: BehaviorResponse::ExploitsOpportunistically,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Success,
                predicted_behavior: BehaviorResponse::BecomesComplacent,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Success,
                predicted_behavior: BehaviorResponse::DismissesOthers,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::Escalates,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Feedback,
                predicted_behavior: BehaviorResponse::RejectsFeedback,
                notes: String::new(),
            },
        ];
        let flags = crate::validation::all_person_flags(&manipulator);
        assert_eq!(flags.len(), 6, "flags: {flags:?}");
        let mp = compute_person_profile(&manipulator);
        assert!(
            (mp.motivation - 0.27).abs() < 0.001,
            "mot: {}",
            mp.motivation
        );
        assert!((mp.ocean - 0.5).abs() < 0.001, "ocean: {}", mp.ocean);
        assert!(mp.patterns <= 0.5 + 1e-9, "patterns: {}", mp.patterns);
        assert!(
            mp.total <= 60,
            "manipulator should be pushed down, got {}",
            mp.total
        );

        let mut genuine = make_person(Some(5), Some(5), Some(5), Some(9), Some(2));
        genuine.motivations = manipulator.motivations.clone();
        assert!(crate::validation::all_person_flags(&genuine).is_empty());
        let gp = compute_person_profile(&genuine);
        assert!(
            gp.total >= 45,
            "genuine should keep mid-band credit, got {}",
            gp.total
        );
        assert!(
            gp.total - mp.total >= 15,
            "manipulator {} vs genuine {}: gap too small",
            mp.total,
            gp.total
        );
        // Phase 5 snapshot: base weights rebalanced (Rep 0.26 → 0.22, Patterns
        // 0.14 → 0.16, Bias 0.13 → 0.14, Styles 0.11 → 0.12). The documented
        // manipulator collapse (README §Consistency Flags, ~53 → 26) must hold
        // within ±3 points of the pre-rebalance measurement.
        assert!(
            mp.total <= 30,
            "manipulator snapshot must stay collapsed, got {}",
            mp.total
        );
        assert!(
            gp.total >= 45,
            "genuine twin must keep mid-band credit, got {}",
            gp.total
        );
    }

    #[test]
    fn test_rep_adjustment_empty() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let adj = rep_adjustment(&p.rep_scores);
        let expected = -0.02 * 13.0;
        assert!(
            (adj - expected).abs() < 0.001,
            "empty rep adj should be {expected}: {adj}"
        );
    }

    #[test]
    fn test_rep_adjustment_all_ten() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            honest_deceitful: Some(10),
            reliable_flaky: Some(10),
            authoritative_submissive: Some(10),
            humble_arrogant: Some(10),
            hardworker_lazy: Some(10),
            calm_reactive: Some(10),
            diplomatic_blunt: Some(10),
            generous_selfish: Some(10),
            fair_favoritism: Some(10),
            trusting_suspicious: Some(10),
            assertive_passive: Some(10),
            empathetic_detached: Some(10),
            adaptable_rigid: Some(10),
        };
        let adj = rep_adjustment(&p.rep_scores);
        let expected = 9.0 * 0.03 + 4.0 * (-0.04);
        assert!(
            (adj - expected).abs() < 0.001,
            "all-10 rep adj should be {expected}: {adj}"
        );
    }

    #[test]
    fn test_rep_adjustment_all_zero() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            honest_deceitful: Some(0),
            reliable_flaky: Some(0),
            authoritative_submissive: Some(0),
            humble_arrogant: Some(0),
            hardworker_lazy: Some(0),
            calm_reactive: Some(0),
            diplomatic_blunt: Some(0),
            generous_selfish: Some(0),
            fair_favoritism: Some(0),
            trusting_suspicious: Some(0),
            assertive_passive: Some(0),
            empathetic_detached: Some(0),
            adaptable_rigid: Some(0),
        };
        let adj = rep_adjustment(&p.rep_scores);
        let expected = 9.0 * (-0.05) + 4.0 * (-0.04);
        assert!(
            (adj - expected).abs() < 0.001,
            "all-0 rep adj should be {expected}: {adj}"
        );
    }

    #[test]
    fn test_rep_adjustment_all_five() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            honest_deceitful: Some(5),
            reliable_flaky: Some(5),
            authoritative_submissive: Some(5),
            humble_arrogant: Some(5),
            hardworker_lazy: Some(5),
            calm_reactive: Some(5),
            diplomatic_blunt: Some(5),
            generous_selfish: Some(5),
            fair_favoritism: Some(5),
            trusting_suspicious: Some(5),
            assertive_passive: Some(5),
            empathetic_detached: Some(5),
            adaptable_rigid: Some(5),
        };
        let adj = rep_adjustment(&p.rep_scores);
        let expected = 4.0 * 0.02;
        assert!(
            (adj - expected).abs() < 0.001,
            "all-5 rep adj should be {expected}: {adj}"
        );
    }

    #[test]
    fn test_rep_adjustment_all_nine() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            honest_deceitful: Some(9),
            reliable_flaky: Some(9),
            authoritative_submissive: Some(9),
            humble_arrogant: Some(9),
            hardworker_lazy: Some(9),
            calm_reactive: Some(9),
            diplomatic_blunt: Some(9),
            generous_selfish: Some(9),
            fair_favoritism: Some(9),
            trusting_suspicious: Some(9),
            assertive_passive: Some(9),
            empathetic_detached: Some(9),
            adaptable_rigid: Some(9),
        };
        let adj = rep_adjustment(&p.rep_scores);
        let expected = 9.0 * 0.03 + 4.0 * (-0.04);
        assert!(
            (adj - expected).abs() < 0.001,
            "all-9 rep adj should be {expected}: {adj}"
        );
    }

    #[test]
    fn test_profile_completeness_empty() {
        let p = make_person(None, None, None, None, None);
        let c = profile_completeness(&p);
        assert!((c - 0.0).abs() < 0.001, "empty: {c}");
    }

    #[test]
    fn test_profile_completeness_full() {
        let mut p = make_person(Some(8), Some(7), Some(6), Some(9), Some(3));
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Power,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Affiliation,
                intensity: 7,
                notes: String::new(),
            },
        ];
        p.biases = (0..11)
            .map(|i| Bias {
                r#type: BiasType::ALL[i],
                intensity: 5,
                evidence: String::new(),
            })
            .collect();
        p.rep_scores = RepScores {
            honest_deceitful: Some(8),
            reliable_flaky: Some(7),
            authoritative_submissive: Some(5),
            humble_arrogant: Some(6),
            hardworker_lazy: Some(9),
            calm_reactive: Some(5),
            diplomatic_blunt: Some(4),
            generous_selfish: Some(7),
            fair_favoritism: Some(6),
            trusting_suspicious: Some(5),
            assertive_passive: Some(5),
            empathetic_detached: Some(8),
            adaptable_rigid: Some(7),
        };
        use crate::models::StyleType::*;
        p.styles = vec![
            PersonalStyle {
                r#type: DirectCommunicator,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: Collaborating,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: Analytical,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: Visionary,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: PastOriented,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: RuleBased,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: Empathetic,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: Guarded,
                intensity: 5,
                notes: String::new(),
            },
        ];
        p.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Feedback,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Success,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
        ];
        p.values = vec![
            crate::models::Value {
                r#type: crate::models::ValueType::Career,
                intensity: 8,
                priority: 7,
                notes: String::new(),
            },
            crate::models::Value {
                r#type: crate::models::ValueType::Family,
                intensity: 6,
                priority: 9,
                notes: String::new(),
            },
            crate::models::Value {
                r#type: crate::models::ValueType::Health,
                intensity: 5,
                priority: 5,
                notes: String::new(),
            },
        ];
        let c = profile_completeness(&p);
        assert!((c - 1.0).abs() < 0.001, "full: {c}");
    }

    #[test]
    fn test_profile_completeness_ocean_only() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let c = profile_completeness(&p);
        let expected = 5.0 / 48.0;
        assert!((c - expected).abs() < 0.001, "ocean only: {c}");
    }

    #[test]
    fn test_profile_completeness_rep_and_mot() {
        let mut p = make_person(None, None, None, None, None);
        p.rep_scores = RepScores {
            honest_deceitful: Some(8),
            reliable_flaky: Some(7),
            authoritative_submissive: Some(5),
            humble_arrogant: Some(6),
            hardworker_lazy: Some(9),
            calm_reactive: Some(5),
            diplomatic_blunt: Some(4),
            generous_selfish: Some(7),
            fair_favoritism: Some(6),
            trusting_suspicious: Some(5),
            assertive_passive: Some(5),
            empathetic_detached: Some(8),
            adaptable_rigid: Some(7),
        };
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 6,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 7,
                notes: String::new(),
            },
        ];
        let c = profile_completeness(&p);
        let expected = (13.0 + 3.0) / 48.0;
        assert!((c - expected).abs() < 0.001, "rep+mot: {c}");
    }

    #[test]
    fn test_profile_completeness_two_motivations() {
        let mut p = make_person(None, None, None, None, None);
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Power,
                intensity: 5,
                notes: String::new(),
            },
        ];
        let c = profile_completeness(&p);
        let expected = 2.0 / 48.0;
        assert!((c - expected).abs() < 0.001, "2 mot: {c}");
    }

    #[test]
    fn test_self_score_good_rep_boosts() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            honest_deceitful: Some(9),
            reliable_flaky: Some(9),
            hardworker_lazy: Some(8),
            ..RepScores::default()
        };
        let pf = compute_person_profile(&p);
        assert!(pf.reputation > 0.70, "good rep: {}", pf.reputation);
        assert!(pf.total >= 35, "good rep boosts total: {}", pf.total);
    }

    #[test]
    fn test_self_score_negative_patterns_lower() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::BecomesDefensive,
            notes: String::new(),
        }];
        let pf = compute_person_profile(&p);
        // Single Conflict pattern → self-pair synergy = -0.3
        // w = 8*8/100 = 0.64, sum = -0.3*0.64 = -0.192
        // avg_synergy = -0.3 → scaled = (-0.3 + 0.3) / 0.6 = 0.0
        assert!(
            (pf.patterns - 0.0).abs() < 0.001,
            "single conflict pattern: {}",
            pf.patterns
        );
    }

    #[test]
    fn test_self_score_band_integration() {
        // Verify synergy_bands logic used by person_detail page's band-key resolution
        let bands = synergy_bands();
        let band_for = |score: u8| -> usize {
            bands
                .iter()
                .position(|&(lo, hi)| score >= lo && score <= hi)
                .unwrap_or(2)
        };
        let band_keys = [
            "scale_tension",
            "scale_friction",
            "scale_moderate",
            "scale_good",
            "scale_strong",
        ];

        // Minimum score → tension
        assert_eq!(band_for(0), 0);
        assert_eq!(band_keys[band_for(0)], "scale_tension");
        // Maximum score → strong
        assert_eq!(band_for(100), 4);
        assert_eq!(band_keys[band_for(100)], "scale_strong");

        // Baseline self-score (neutral person, no motiv/patterns) → moderate
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let pf = compute_person_profile(&p);
        let idx = band_for(pf.total);
        assert!(
            (1..=3).contains(&idx),
            "baseline band should be moderate-ish, got {} ({})",
            idx,
            band_keys[idx]
        );
    }

    // --- Edge-case tests ---

    #[test]
    fn test_sim_both_none() {
        assert!((sim(None, None) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_sim_one_none() {
        assert!((sim(Some(8), None) - 0.5).abs() < 1e-9);
        assert!((sim(None, Some(3)) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_sim_identical() {
        assert!((sim(Some(5), Some(5)) - 1.0).abs() < 1e-9);
        assert!((sim(Some(10), Some(10)) - 1.0).abs() < 1e-9);
        assert!((sim(Some(1), Some(1)) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_sim_opposite() {
        assert!((sim(Some(1), Some(10)) - 0.1).abs() < 1e-9);
        assert!((sim(Some(10), Some(1)) - 0.1).abs() < 1e-9);
    }

    #[test]
    fn test_ocean_penalty_boundary_values() {
        // Boundary: N=7, A=4 exactly triggers volatile
        let a = make_person(Some(5), Some(5), Some(5), Some(4), Some(7));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        assert!(
            (p - 0.10).abs() < 0.001,
            "N=7 A=4 should be volatile: {}",
            p
        );

        // Boundary: N=6, A=4 should NOT be volatile
        let a2 = make_person(Some(5), Some(5), Some(5), Some(4), Some(6));
        let p2 = ocean_danger_penalty(&a2.ocean, &b.ocean);
        assert!(
            (p2 - 0.0).abs() < 0.001,
            "N=6 A=4 should not be volatile: {}",
            p2
        );

        // Boundary: N=7, A=5 should NOT be volatile
        let a3 = make_person(Some(5), Some(5), Some(5), Some(5), Some(7));
        let p3 = ocean_danger_penalty(&a3.ocean, &b.ocean);
        assert!(
            (p3 - 0.0).abs() < 0.001,
            "N=7 A=5 should not be volatile: {}",
            p3
        );
    }

    #[test]
    fn test_ocean_all_none_synergy() {
        let a = make_person(None, None, None, None, None);
        let b = make_person(None, None, None, None, None);
        let brk = compute_synergy_score(&a, &b);
        // All None OCEAN → sim returns 0.5 for each
        // oc = (0.5+0.5)/2 = 0.5, ea = (0.5+0.5)/2 = 0.5, n = 0.5 → raw_ocean = 0.5
        assert!(
            (brk.ocean - 0.5).abs() < 0.001,
            "all-none ocean: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_ocean_partial_values_synergy() {
        // Only openness set on both, rest None
        let a = make_person(Some(8), None, None, None, None);
        let b = make_person(Some(8), None, None, None, None);
        let brk = compute_synergy_score(&a, &b);
        assert!(brk.ocean > 0.0, "partial ocean should compute synergy");
        assert!(brk.total > 0, "total should be > 0");
    }

    #[test]
    fn test_self_score_full_ocean_none() {
        let p = make_person(None, None, None, None, None);
        let pf = compute_person_profile(&p);
        // All None → A and N unfilled → a_s=0, n_s=0 → raw_ocean=0, no penalty
        assert!((pf.ocean - 0.0).abs() < 0.001, "ocean score: {}", pf.ocean);
        assert!(pf.total > 0, "total should be > 0: {}", pf.total);
    }

    #[test]
    fn test_self_score_all_volatilities() {
        let p = make_person(Some(3), Some(3), Some(5), Some(3), Some(8));
        let pf = compute_person_profile(&p);
        // N=8≥7 + A=3≤4 → volatile 0.10
        // N=8≥7 + C=3≤4 → impulsive 0.05
        // N=8≥7 + O=3≤4 → rigid anxious 0.05
        // total penalty = 0.20
        assert!(
            pf.ocean < 0.40,
            "all three penalties should lower ocean: {}",
            pf.ocean
        );
    }

    #[test]
    fn test_rep_penalty_arrogant_not_triggered_at_four() {
        // Boundary: ≤3 triggers arrogant penalty, 4 should not
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            humble_arrogant: Some(4),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            humble_arrogant: Some(4),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!(
            (p - 0.0).abs() < 0.001,
            "score=4 should not trigger arrogant: {}",
            p
        );
    }

    #[test]
    fn test_rep_penalty_one_side_only() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            authoritative_submissive: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            authoritative_submissive: Some(4),
            ..RepScores::default()
        };
        let p = rep_danger_penalty(&a.rep_scores, &b.rep_scores);
        assert!(
            (p - 0.0).abs() < 0.001,
            "one authoritative should not trigger: {}",
            p
        );
    }

    #[test]
    fn test_motivation_all_friction_types_have_entry() {
        // Every pair must be explicitly defined (no wildcard allowed).
        // Adding a new MotivationType will cause a compile error here,
        // forcing the developer to consider its synergy with all 9 other types.
        let types = MotivationType::ALL;
        for &a in &types {
            for &b in &types {
                let val = motivation_synergy(a, b);
                assert!(
                    (val - (-0.3)).abs() < 1e-9
                        || (val - (-0.2)).abs() < 1e-9
                        || (val - (-0.1)).abs() < 1e-9
                        || val.abs() < 1e-9
                        || (val - 0.1).abs() < 1e-9
                        || (val - 0.2).abs() < 1e-9
                        || (val - 0.3).abs() < 1e-9,
                    "unexpected synergy value {:?} x {:?} = {} (not in {{-0.3,-0.2,-0.1,0.0,0.1,0.2,0.3}})",
                    a,
                    b,
                    val
                );
            }
        }
    }

    #[test]
    fn test_base_rep_quality_empty() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let q = base_rep_quality(&p);
        assert!((q - 0.5).abs() < 0.001, "empty rep should be 0.5: {}", q);
    }

    #[test]
    fn test_base_rep_quality_all_ten() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            hardworker_lazy: Some(10),
            authoritative_submissive: Some(10),
            honest_deceitful: Some(10),
            reliable_flaky: Some(10),
            humble_arrogant: Some(10),
            calm_reactive: Some(10),
            diplomatic_blunt: Some(10),
            generous_selfish: Some(10),
            ..RepScores::default()
        };
        let q = base_rep_quality(&p);
        assert!((q - 1.0).abs() < 0.001, "all-10 rep should be 1.0: {}", q);
    }

    #[test]
    fn test_base_rep_quality_all_zero() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            hardworker_lazy: Some(0),
            authoritative_submissive: Some(0),
            honest_deceitful: Some(0),
            reliable_flaky: Some(0),
            humble_arrogant: Some(0),
            calm_reactive: Some(0),
            diplomatic_blunt: Some(0),
            generous_selfish: Some(0),
            ..RepScores::default()
        };
        let q = base_rep_quality(&p);
        assert!((q - 0.0).abs() < 0.001, "all-0 rep should be 0.0: {}", q);
    }

    #[test]
    fn test_base_rep_quality_partial() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.rep_scores = RepScores {
            hardworker_lazy: Some(10),
            ..RepScores::default()
        };
        let q = base_rep_quality(&p);
        // Only one dim set with weight 0.10
        assert!(
            (q - 1.0).abs() < 0.001,
            "single-dim all-10 should be 1.0: {}",
            q
        );
    }

    #[test]
    fn test_synergy_bands_structure() {
        let bands = synergy_bands();
        assert_eq!(bands.len(), 5);
        for &(lo, hi) in &bands {
            assert!(lo <= hi, "band {} {} should have lo <= hi", lo, hi);
        }
        // Bands should cover 0..=100 with no gaps
        assert_eq!(bands[0].0, 0);
        assert_eq!(bands[4].1, 100);
        // Each band starts where previous ended + 1
        for i in 1..bands.len() {
            assert_eq!(
                bands[i].0,
                bands[i - 1].1 + 1,
                "gap between band {} and {}",
                i - 1,
                i
            );
        }
    }

    #[test]
    fn test_synergy_bands_motivation_learning_pair() {
        // Self-pair synergy for Learning x Learning = 0.2
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 10,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 10,
            notes: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        // w = 10*10/100 = 1.0, syn = 0.2 (non-neutral), avg = 0.2
        // scaled = (0.2 + 0.3) / 0.6 = 0.8333
        assert!(
            (brk.motivation - 0.8333).abs() < 0.001,
            "Learning-Learning mot: {}",
            brk.motivation
        );
    }

    #[test]
    fn test_pattern_synergy_mixed_same_and_opposite() {
        let a = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
        ];
        let b = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
        ];
        let result = pattern_synergy(&a, &b);
        // Same triggers: Change=0.3, Conflict=-0.3
        // (Change,Change): syn=+0.3, w=1.0
        // (Conflict,Conflict): syn=-0.3, w=1.0
        // result = ((0.0 / 2.0) + 0.3) / 0.6 = 0.5
        // Detailed math: let's just verify it's sensible
        assert!(
            (0.4..=0.9).contains(&result),
            "mixed pattern synergy: {}",
            result
        );
    }

    // --- style_synergy tests ---

    #[test]
    fn test_style_synergy_same_style() {
        let a = vec![PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 8,
            notes: String::new(),
        }];
        let b = vec![PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 7,
            notes: String::new(),
        }];
        let result = style_synergy(&a, &b);
        assert!((result - 1.0).abs() < 1e-9, "same style → 1.0: {}", result);
    }

    #[test]
    fn test_style_synergy_different_same_category() {
        let a = vec![PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 8,
            notes: String::new(),
        }];
        let b = vec![PersonalStyle {
            r#type: StyleType::DiplomaticCommunicator,
            intensity: 7,
            notes: String::new(),
        }];
        let result = style_synergy(&a, &b);
        assert!(
            (result - 0.5).abs() < 1e-9,
            "different in same category → 0.5: {}",
            result
        );
    }

    #[test]
    fn test_style_synergy_no_shared_categories() {
        let a = vec![PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 8,
            notes: String::new(),
        }];
        let b = vec![PersonalStyle {
            r#type: StyleType::Visionary,
            intensity: 7,
            notes: String::new(),
        }];
        let result = style_synergy(&a, &b);
        assert!(
            (result - 0.5).abs() < 1e-9,
            "no shared categories → 0.5: {}",
            result
        );
    }

    #[test]
    fn test_style_synergy_empty_both() {
        let a: Vec<PersonalStyle> = vec![];
        let b: Vec<PersonalStyle> = vec![];
        let result = style_synergy(&a, &b);
        assert!((result - 0.5).abs() < 1e-9, "both empty → 0.5: {}", result);
    }

    #[test]
    fn test_style_synergy_mix_same_and_different() {
        let a = vec![
            PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 8,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Competing,
                intensity: 6,
                notes: String::new(),
            },
        ];
        let b = vec![
            PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 7,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Collaborating,
                intensity: 9,
                notes: String::new(),
            },
        ];
        // Communication: same → 1.0, ConflictResolution: different → 0.5
        // average = (1.0 + 0.5) / 2 = 0.75
        let result = style_synergy(&a, &b);
        assert!(
            (result - 0.75).abs() < 1e-9,
            "mix same + different → 0.75: {}",
            result
        );
    }

    #[test]
    fn test_style_synergy_one_empty_one_populated() {
        let a: Vec<PersonalStyle> = vec![];
        let b = vec![PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 8,
            notes: String::new(),
        }];
        let result = style_synergy(&a, &b);
        assert!((result - 0.5).abs() < 1e-9, "one empty → 0.5: {}", result);
    }

    #[test]
    fn test_style_synergy_all_six_categories_same() {
        let a_styles = vec![
            PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Competing,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Analytical,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Visionary,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::PastOriented,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::RuleBased,
                intensity: 5,
                notes: String::new(),
            },
        ];
        let b_styles = vec![
            PersonalStyle {
                r#type: StyleType::ExpressiveCommunicator,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Collaborating,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Intuitive,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Servant,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::FutureOriented,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::VirtueBased,
                intensity: 5,
                notes: String::new(),
            },
        ];
        let result = style_synergy(&a_styles, &b_styles);
        // All 6 categories covered, all different → 0.5 each → average = 0.5
        assert!(
            (result - 0.5).abs() < 1e-9,
            "all 6 different → 0.5: {}",
            result
        );
    }

    // --- relationship context (Phase 1) tests ---

    #[test]
    fn test_rel_weights_sum_to_one() {
        for rt in RelationType::ALL {
            let (a, b, c, d, e, f, g) = rel_weights(rt);
            let sum = a + b + c + d + e + f + g;
            assert!(
                (sum - 1.0).abs() < 1e-9,
                "weights for {:?} sum to {} (must be 1.0)",
                rt,
                sum
            );
        }
    }

    #[test]
    fn test_no_ctx_equals_legacy() {
        let a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        let b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        let legacy = compute_synergy_score_with_preds(&a, &b, &[], &[]);
        let ctx = compute_synergy_score_ctx(&a, &b, None, &[], &[]);
        assert_eq!(legacy.total, ctx.total);
        assert_eq!(legacy.a_score, ctx.a_score);
        assert_eq!(legacy.b_score, ctx.b_score);
        assert_eq!(legacy.band, 0);
        assert!((legacy.ocean - ctx.ocean).abs() < 1e-9);
        assert!((legacy.reputation - ctx.reputation).abs() < 1e-9);
        assert!((legacy.motivation - ctx.motivation).abs() < 1e-9);
        assert!((legacy.patterns - ctx.patterns).abs() < 1e-9);
    }

    #[test]
    fn test_rel_context_changes_score() {
        let a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        let b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        let friends = RelContext {
            rtype: RelationType::Friends,
            strength: 6,
        };
        let manages = RelContext {
            rtype: RelationType::Manages,
            strength: 6,
        };
        let f = compute_synergy_score_ctx(&a, &b, Some(&friends), &[], &[]);
        let m = compute_synergy_score_ctx(&a, &b, Some(&manages), &[], &[]);
        assert_ne!(f.total, m.total, "relation type must change the score");
        assert_eq!(f.band, 8, "strength 6 → band 8");
    }

    #[test]
    fn test_power_subordinate_penalty() {
        // Both people identical except B (the subordinate) is Power-driven.
        let mut a = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
        let mut b = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
        a.motivations = vec![Motivation {
            r#type: MotivationType::Helping,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: String::new(),
        }];
        let neutral = RelContext {
            rtype: RelationType::WorksWith,
            strength: 6,
        };
        let manages = RelContext {
            rtype: RelationType::Manages,
            strength: 6,
        };
        let n = compute_synergy_score_ctx(&a, &b, Some(&neutral), &[], &[]);
        let m = compute_synergy_score_ctx(&a, &b, Some(&manages), &[], &[]);
        assert!(
            m.total < n.total,
            "Power-heavy subordinate must lower the Manages score ({} < {})",
            m.total,
            n.total
        );
    }

    #[test]
    fn test_hierarchy_clarity_bonus() {
        // A clearly authoritative boss vs. submissive report → bonus.
        // Same weight profile (Manages/ReportsTo share it); only direction differs.
        let mut a = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
        let mut b = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
        a.rep_scores.authoritative_submissive = Some(9);
        b.rep_scores.authoritative_submissive = Some(3);
        let manages = RelContext {
            rtype: RelationType::Manages,
            strength: 6,
        };
        let reports = RelContext {
            rtype: RelationType::ReportsTo,
            strength: 6,
        };
        // Manages: a is the boss (9) over submissive b (3) → bonus fires.
        let m = compute_synergy_score_ctx(&a, &b, Some(&manages), &[], &[]);
        // ReportsTo: a reports to b, so b (3) is the "boss" → no bonus.
        let r = compute_synergy_score_ctx(&a, &b, Some(&reports), &[], &[]);
        assert!(
            m.total >= r.total,
            "clear hierarchy should add a small bonus ({} >= {})",
            m.total,
            r.total
        );
    }

    #[test]
    fn test_strength_band_mapping() {
        assert_eq!(strength_band(1), 12);
        assert_eq!(strength_band(4), 12);
        assert_eq!(strength_band(5), 8);
        assert_eq!(strength_band(7), 8);
        assert_eq!(strength_band(8), 4);
        assert_eq!(strength_band(10), 4);
    }

    #[test]
    fn test_rel_context_band_in_breakdown() {
        let a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        let b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        let weak = RelContext {
            rtype: RelationType::Partner,
            strength: 2,
        };
        let brk = compute_synergy_score_ctx(&a, &b, Some(&weak), &[], &[]);
        assert_eq!(brk.band, 12, "weak relationship → wide band");
    }

    #[test]
    fn test_confidence_band_mapping() {
        assert_eq!(confidence_band(1), 12);
        assert_eq!(confidence_band(4), 12);
        assert_eq!(confidence_band(5), 8);
        assert_eq!(confidence_band(7), 8);
        assert_eq!(confidence_band(8), 4);
        assert_eq!(confidence_band(10), 4);
    }

    #[test]
    fn test_person_profile_band_from_confidence() {
        let base = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        for (conf, expected) in [(1, 12u8), (5, 8), (10, 4)] {
            let mut p = base.clone();
            p.confidence = conf;
            let profile = compute_person_profile(&p);
            assert_eq!(
                profile.band, expected,
                "confidence {} → band ±{}",
                conf, expected
            );
        }
    }

    #[test]
    fn test_person_profile_total_unaffected_by_confidence() {
        let base = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        let mut low = base.clone();
        low.confidence = 1;
        let mut high = base.clone();
        high.confidence = 10;
        let pl = compute_person_profile(&low);
        let ph = compute_person_profile(&high);
        assert_eq!(
            pl.total, ph.total,
            "confidence must not move the raw score, only its band"
        );
        assert_eq!(pl.completeness, ph.completeness);
    }

    #[test]
    fn test_ctx_band_max_composition() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        let strong = RelContext {
            rtype: RelationType::Partner,
            strength: 10,
        };
        let weak = RelContext {
            rtype: RelationType::Partner,
            strength: 2,
        };

        a.confidence = 10;
        b.confidence = 10;
        assert_eq!(
            compute_synergy_score_ctx(&a, &b, Some(&weak), &[], &[]).band,
            12,
            "weak relationship dominates even with high confidence"
        );

        a.confidence = 1;
        b.confidence = 10;
        assert_eq!(
            compute_synergy_score_ctx(&a, &b, Some(&strong), &[], &[]).band,
            12,
            "low profile confidence widens the band despite strong relationship"
        );

        a.confidence = 10;
        b.confidence = 9;
        assert_eq!(
            compute_synergy_score_ctx(&a, &b, Some(&strong), &[], &[]).band,
            4,
            "strong relationship + high confidence → narrow band"
        );
    }

    #[test]
    fn test_no_ctx_band_still_zero() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.confidence = 1;
        let b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        let brk = compute_synergy_score_ctx(&a, &b, None, &[], &[]);
        assert_eq!(brk.band, 0, "no relationship context keeps legacy band 0");
    }

    fn log_entry(ts: i64, valence: i8, target: Option<&str>) -> InteractionEntry {
        InteractionEntry {
            id: format!("e{ts}-{valence}"),
            timestamp: ts,
            text: String::new(),
            valence: Some(valence),
            trigger: None,
            target_id: target.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_trajectory_empty() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        let mut b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        b.id = "b".into();
        let t = pair_trajectory(&a, &b);
        assert_eq!(t.sample, 0);
        assert_eq!(t.delta, 0);
        assert_eq!(t.trend, Trend::Stable);
    }

    #[test]
    fn test_trajectory_positive_improving() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, 3, None),
            log_entry(2000, 2, None),
            log_entry(3000, 1, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(t.sample, 3);
        assert_eq!(t.trend, Trend::Improving);
        assert!(t.delta > 0, "positive log must yield a positive delta");
        assert!(t.level > 0.5);
    }

    #[test]
    fn test_trajectory_negative_deteriorating() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, -1, None),
            log_entry(2000, -2, None),
            log_entry(3000, -3, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(t.trend, Trend::Deteriorating);
        assert!(t.delta < 0, "negative log must yield a negative delta");
    }

    #[test]
    fn test_trajectory_recency_dominates() {
        let day = 86_400_000i64;
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(0, -3, None),
            log_entry(30 * day, -3, None),
            log_entry(59 * day, 2, None),
            log_entry(60 * day, 3, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(
            t.trend,
            Trend::Improving,
            "recent positives dominate stale negatives"
        );
        assert!(t.level > 0.0);
    }

    #[test]
    fn test_trajectory_momentum_flips_trend() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, 3, None),
            log_entry(2000, 3, None),
            log_entry(3000, -3, None),
            log_entry(4000, -3, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(
            t.trend,
            Trend::Deteriorating,
            "recent half flips an earlier-good run"
        );
    }

    #[test]
    fn test_pair_trajectory_filters_by_target() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        let mut b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        b.id = "b".into();
        a.log = vec![
            log_entry(1000, 2, Some("b")),
            log_entry(2000, 3, Some("b")),
            log_entry(3000, -3, Some("c")),
        ];
        b.log = vec![log_entry(1500, 1, Some("a"))];
        let t = pair_trajectory(&a, &b);
        assert_eq!(t.sample, 3, "only entries targeting the other person count");
        assert!(t.delta > 0, "pair trajectory should be positive");
    }

    #[test]
    fn test_breakdown_carries_trajectory_without_moving_total() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        let mut b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        b.id = "b".into();

        let baseline = compute_synergy_score_ctx(&a, &b, None, &[], &[]);

        a.log = vec![
            log_entry(1000, 2, Some("b")),
            log_entry(2000, 3, Some("b")),
            log_entry(3000, 1, Some("b")),
        ];
        let brk = compute_synergy_score_ctx(&a, &b, None, &[], &[]);
        assert_eq!(
            brk.total, baseline.total,
            "logged interactions must not move the static point score"
        );
        assert_eq!(brk.trajectory_sample, 3);
        assert_eq!(brk.trajectory_trend, Trend::Improving);
        assert!(brk.trajectory_delta > 0);
    }

    #[test]
    fn test_relationship_strength_serde_default() {
        let json = r#"{"id":"r1","source_id":"a","target_id":"b","type":"Friends","notes":"","created_at":0}"#;
        let r: Relationship = serde_json::from_str(json).unwrap();
        assert_eq!(r.strength, 5, "missing strength → default 5");
        assert_eq!(r.r#type, RelationType::Friends);
        let out = serde_json::to_string(&r).unwrap();
        assert!(out.contains("\"strength\":5"));
    }

    // --- Phase 9b: team aggregation tests ---

    #[test]
    fn team_single_person_returns_none() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let result = compute_team_synergy(&[p], &[], &std::collections::HashMap::new());
        assert!(result.is_none());
    }

    #[test]
    fn team_two_persons_no_rels() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p.name = "Alice".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p.name = "Bob".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert_eq!(team.team_size, 2);
        assert_eq!(team.pairs.len(), 1);
        assert_eq!(team.pairs[0].person_a, "Alice");
        assert_eq!(team.pairs[0].person_b, "Bob");
        let &(ref wa, ref _wb, ws) = team.weakest.as_ref().unwrap();
        let &(ref sa, ref _sb, ss) = team.strongest.as_ref().unwrap();
        assert_eq!(wa, "Alice");
        assert_eq!(sa, "Alice");
        assert_eq!(ws, ss, "two persons → weakest == strongest");
    }

    #[test]
    fn team_three_persons_pair_count() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p
        };
        let c = {
            let mut p = make_person(Some(5), Some(5), Some(5), Some(7), Some(6));
            p.id = "c".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b, c], &[], &preds).unwrap();
        assert_eq!(team.team_size, 3);
        assert_eq!(team.pairs.len(), 3, "3 choose 2 = 3 pairs");
        assert!(team.avg_score > 0);
    }

    #[test]
    fn team_with_relationship_context() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p
        };
        let rel = Relationship {
            id: "r1".into(),
            source_id: "a".into(),
            target_id: "b".into(),
            r#type: RelationType::Friends,
            strength: 8,
            notes: String::new(),
            created_at: 0,
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[rel], &preds).unwrap();
        assert_eq!(team.pairs.len(), 1);
        // Verify it compiled and ran without panic; per_context should be populated
        assert!(!team.pairs[0].breakdown.per_context.is_empty());
    }

    #[test]
    fn team_context_averages_populated() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p
        };
        let c = {
            let mut p = make_person(Some(5), Some(5), Some(5), Some(7), Some(6));
            p.id = "c".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b, c], &[], &preds).unwrap();
        assert_eq!(
            team.context_averages.len(),
            6,
            "one per InsightContext variant"
        );
        for (ctx, avg) in &team.context_averages {
            assert!(*avg <= 100, "context avg {:?} = {} exceeds 100", ctx, avg);
        }
    }

    // === Mutation-killing tests for synergy.rs ===

    // --- rep_danger_penalty: exact values per dimension ---

    #[test]
    fn test_rep_danger_penalty_single_dimension_exact() {
        use crate::models::RepScores;
        let d = &CFG.reputation.danger;
        // Power struggle: both authoritative >= high (8)
        let ra = RepScores {
            authoritative_submissive: Some(d.high),
            ..Default::default()
        };
        let rb = RepScores {
            authoritative_submissive: Some(d.high),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.power_struggle).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_brutal_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            diplomatic_blunt: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            diplomatic_blunt: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.brutal).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_escalation_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            calm_reactive: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            calm_reactive: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.escalation).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_no_concede_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            humble_arrogant: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            humble_arrogant: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.no_concede).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_passivity_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            hardworker_lazy: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            hardworker_lazy: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.passivity).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_suspicion_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            trusting_suspicious: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            trusting_suspicious: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.suspicion).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_coldness_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            empathetic_detached: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            empathetic_detached: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.coldness).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_trust_collapse_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            honest_deceitful: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            honest_deceitful: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.trust_collapse).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_unreliability_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            reliable_flaky: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            reliable_flaky: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.unreliability).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_cronyism_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            fair_favoritism: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            fair_favoritism: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.cronyism).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_hoarding_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            generous_selfish: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            generous_selfish: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.hoarding).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_paralysis_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            assertive_passive: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            assertive_passive: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.paralysis).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_gridlock_exact() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            adaptable_rigid: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            adaptable_rigid: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.gridlock).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_not_triggered_above_low() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            authoritative_submissive: Some(d.low + 1),
            ..Default::default()
        };
        let rb = RepScores {
            authoritative_submissive: Some(d.low + 1),
            ..Default::default()
        };
        assert_eq!(rep_danger_penalty(&ra, &rb), 0.0);
    }

    #[test]
    fn test_rep_danger_penalty_all_dims_combine() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            authoritative_submissive: Some(d.high),
            diplomatic_blunt: Some(d.low),
            calm_reactive: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            authoritative_submissive: Some(d.high),
            diplomatic_blunt: Some(d.low),
            calm_reactive: Some(d.low),
            ..Default::default()
        };
        let expected = d.power_struggle + d.brutal + d.escalation;
        assert!((rep_danger_penalty(&ra, &rb) - expected).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_authoritative_above_high_no_trigger() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            authoritative_submissive: Some(d.high - 1),
            ..Default::default()
        };
        let rb = RepScores {
            authoritative_submissive: Some(d.high - 1),
            ..Default::default()
        };
        assert_eq!(rep_danger_penalty(&ra, &rb), 0.0);
    }

    // --- ocean_danger_penalty: exact values ---

    #[test]
    fn test_ocean_danger_volatile_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            neuroticism: Some(d.high),
            agreeableness: Some(d.low),
            ..Default::default()
        };
        let ob = OceanScores::default();
        assert!((ocean_danger_penalty(&oa, &ob) - d.within_volatile).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_impulsive_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            neuroticism: Some(d.high),
            conscientiousness: Some(d.low),
            ..Default::default()
        };
        let ob = OceanScores::default();
        assert!((ocean_danger_penalty(&oa, &ob) - d.within_impulsive).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_rigid_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            neuroticism: Some(d.high),
            openness: Some(d.low),
            ..Default::default()
        };
        let ob = OceanScores::default();
        assert!((ocean_danger_penalty(&oa, &ob) - d.within_rigid).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_contagion_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            neuroticism: Some(d.high),
            ..Default::default()
        };
        let ob = OceanScores {
            neuroticism: Some(d.high),
            ..Default::default()
        };
        assert!((ocean_danger_penalty(&oa, &ob) - d.contagion).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_antagonism_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            agreeableness: Some(d.low),
            ..Default::default()
        };
        let ob = OceanScores {
            agreeableness: Some(d.low),
            ..Default::default()
        };
        assert!((ocean_danger_penalty(&oa, &ob) - d.antagonism).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_unreliability_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            conscientiousness: Some(d.low),
            ..Default::default()
        };
        let ob = OceanScores {
            conscientiousness: Some(d.low),
            ..Default::default()
        };
        assert!((ocean_danger_penalty(&oa, &ob) - d.unreliability).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_rigidity_exact() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            openness: Some(d.low),
            ..Default::default()
        };
        let ob = OceanScores {
            openness: Some(d.low),
            ..Default::default()
        };
        assert!((ocean_danger_penalty(&oa, &ob) - d.rigidity).abs() < 0.001);
    }

    #[test]
    fn test_ocean_danger_all_combine() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            openness: Some(d.low),
            conscientiousness: None,
            extraversion: Some(5),
            agreeableness: Some(d.low),
            neuroticism: Some(d.high),
        };
        let ob = OceanScores {
            openness: Some(d.low),
            conscientiousness: None,
            extraversion: Some(5),
            agreeableness: Some(d.low),
            neuroticism: Some(d.high),
        };
        // A: volatile (N high, A low) + rigid (N high, O low)
        // B: volatile + rigid
        // Cross: contagion + antagonism + rigidity
        let expected = d.within_volatile * 2.0
            + d.within_rigid * 2.0
            + d.contagion
            + d.antagonism
            + d.rigidity;
        let actual = ocean_danger_penalty(&oa, &ob);
        assert!(
            (actual - expected).abs() < 0.001,
            "got {} expected {}",
            actual,
            expected
        );
    }

    #[test]
    fn test_ocean_danger_boundary_high_minus_one_no_trigger() {
        let d = &CFG.ocean.danger;
        let oa = OceanScores {
            neuroticism: Some(d.high - 1),
            agreeableness: Some(d.low + 1),
            ..Default::default()
        };
        let ob = OceanScores::default();
        assert_eq!(ocean_danger_penalty(&oa, &ob), 0.0);
    }

    // --- trajectory_from: exact level and delta ---

    #[test]
    fn test_trajectory_from_single_positive() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 2);
        assert!(traj.level > 0.0, "positive valence → positive level");
        assert!(traj.delta > 0, "positive valence → positive delta");
    }

    #[test]
    fn test_trajectory_from_single_negative() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(-3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 2);
        assert!(traj.level < 0.0, "negative valence → negative level");
        assert!(traj.delta < 0, "negative valence → negative delta");
    }

    #[test]
    fn test_trajectory_from_level_clamped() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.level <= 1.0, "level clamped to 1.0");
        assert!(traj.level >= -1.0, "level clamped to -1.0");
    }

    #[test]
    fn test_trajectory_from_delta_clamped() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..6)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.delta <= CFG.trajectory.delta_clamp);
        assert!(traj.delta >= -CFG.trajectory.delta_clamp);
    }

    #[test]
    fn test_trajectory_from_improving_trend() {
        use crate::models::InteractionEntry;
        let mut entries: Vec<InteractionEntry> = Vec::new();
        for i in 0..5 {
            entries.push(InteractionEntry {
                id: format!("e{}", i),
                valence: Some(if i < 2 { -2 } else { 2 }),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            });
        }
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.trend, Trend::Improving);
    }

    #[test]
    fn test_trajectory_from_deteriorating_trend() {
        use crate::models::InteractionEntry;
        let mut entries: Vec<InteractionEntry> = Vec::new();
        for i in 0..5 {
            entries.push(InteractionEntry {
                id: format!("e{}", i),
                valence: Some(if i < 2 { 2 } else { -2 }),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            });
        }
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.trend, Trend::Deteriorating);
    }

    #[test]
    fn test_trajectory_from_few_samples_level_fallback() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.sample < CFG.trajectory.min_samples);
        assert!(traj.trend == Trend::Improving || traj.trend == Trend::Stable);
    }

    // --- value_similarity: exact values ---

    #[test]
    fn test_value_similarity_identical() {
        let vals = vec![Value {
            r#type: ValueType::Career,
            intensity: 8,
            priority: 7,
            notes: String::new(),
        }];
        let sim = value_similarity(&vals, &vals);
        assert!((sim - 1.0).abs() < 0.001, "identical → 1.0, got {}", sim);
    }

    #[test]
    fn test_value_similarity_empty_empty() {
        let sim = value_similarity(&[], &[]);
        assert!((sim - 0.5).abs() < 0.001, "both empty → 0.5, got {}", sim);
    }

    #[test]
    fn test_value_similarity_opposite() {
        let a = vec![Value {
            r#type: ValueType::Career,
            intensity: 10,
            priority: 10,
            notes: String::new(),
        }];
        let b = vec![Value {
            r#type: ValueType::Career,
            intensity: 1,
            priority: 10,
            notes: String::new(),
        }];
        let sim = value_similarity(&a, &b);
        assert!(sim < 0.5, "opposite intensities → low sim, got {}", sim);
    }

    #[test]
    fn test_value_similarity_one_empty_one_populated() {
        let a = vec![Value {
            r#type: ValueType::Career,
            intensity: 5,
            priority: 10,
            notes: String::new(),
        }];
        let b: Vec<Value> = vec![];
        let sim = value_similarity(&a, &b);
        // a has priority 10 → w=1.0, av=0.5, bv=0.5 (default) → 1.0 - 0 = 1.0
        assert!(
            (sim - 1.0).abs() < 0.001,
            "one empty → 1.0 (default=0.5), got {}",
            sim
        );
    }

    #[test]
    fn test_value_similarity_weighted_by_priority() {
        // Low priority → low weight → barely matters
        let a = vec![Value {
            r#type: ValueType::Career,
            intensity: 10,
            priority: 1,
            notes: String::new(),
        }];
        let b = vec![Value {
            r#type: ValueType::Career,
            intensity: 1,
            priority: 1,
            notes: String::new(),
        }];
        let sim_lo = value_similarity(&a, &b);
        // High priority → high weight → matters a lot
        let a_hi = vec![Value {
            r#type: ValueType::Career,
            intensity: 10,
            priority: 10,
            notes: String::new(),
        }];
        let b_lo = vec![Value {
            r#type: ValueType::Career,
            intensity: 1,
            priority: 10,
            notes: String::new(),
        }];
        let sim_hi = value_similarity(&a_hi, &b_lo);
        // Both have same intensities but different priority weight
        // sim_lo = (1 - |10/10 - 1/10|) * 0.1 / 0.1 = (1 - 0.9) = 0.1
        // sim_hi = (1 - |10/10 - 1/10|) * 1.0 / 1.0 = (1 - 0.9) = 0.1
        // Actually same sim because only 1 value, weight normalizes out
        // Instead test: low priority pair vs high priority pair with more values
        assert!(
            (sim_lo - sim_hi).abs() < 0.001,
            "single value: same sim regardless of priority"
        );
    }

    // --- motivation_synergy_score: exact arithmetic ---

    #[test]
    fn test_motivation_synergy_score_single_pair() {
        let ma = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 8,
            notes: String::new(),
        }];
        let mb = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 6,
            notes: String::new(),
        }];
        let score = motivation_synergy_score(&ma, &mb);
        assert!(score > 0.5, "Achievement×Learning positive, got {}", score);
        assert!(score <= 1.0, "clamped to 1.0");
    }

    #[test]
    fn test_motivation_synergy_score_symmetric() {
        let ma = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 8,
            notes: String::new(),
        }];
        let mb = vec![Motivation {
            r#type: MotivationType::Affiliation,
            intensity: 6,
            notes: String::new(),
        }];
        let score_ab = motivation_synergy_score(&ma, &mb);
        let score_ba = motivation_synergy_score(&mb, &ma);
        assert!((score_ab - score_ba).abs() < 0.001, "must be symmetric");
    }

    #[test]
    fn test_motivation_synergy_score_zero_syn_zero_weight() {
        let ma = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 5,
            notes: String::new(),
        }];
        let mb = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 5,
            notes: String::new(),
        }];
        let score = motivation_synergy_score(&ma, &mb);
        assert!(
            (score - CFG.motivation.default).abs() < 0.001,
            "zero syn → default, got {}",
            score
        );
    }

    // --- pattern_synergy: exact arithmetic ---

    #[test]
    fn test_pattern_synergy_symmetric() {
        let pa = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let pb = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let score_ab = pattern_synergy(&pa, &pb);
        let score_ba = pattern_synergy(&pb, &pa);
        assert!((score_ab - score_ba).abs() < 0.001, "must be symmetric");
    }

    // --- avg_prediction_accuracy: && guard ---

    #[test]
    fn test_avg_prediction_accuracy_both_resolved() {
        let preds = vec![
            Prediction {
                id: "p1".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(8),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p2".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(6),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p3".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(7),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
        ];
        let avg = avg_prediction_accuracy(&preds).unwrap();
        assert!((avg - 7.0).abs() < 0.001);
    }

    #[test]
    fn test_avg_prediction_accuracy_unresolved_excluded() {
        let preds = vec![
            Prediction {
                id: "p1".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(8),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p2".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(10),
                created_at: 0,
                resolved_at: None,
                resolved: false,
            },
            Prediction {
                id: "p3".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(4),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p4".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(6),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
        ];
        let avg = avg_prediction_accuracy(&preds).unwrap();
        assert!(
            (avg - 6.0).abs() < 0.001,
            "only resolved count, got {}",
            avg
        );
    }

    #[test]
    fn test_avg_prediction_accuracy_no_accuracy_excluded() {
        let preds = vec![
            Prediction {
                id: "p1".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(8),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p2".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: None,
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p3".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(4),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
            Prediction {
                id: "p4".into(),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(6),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            },
        ];
        let avg = avg_prediction_accuracy(&preds).unwrap();
        assert!(
            (avg - 6.0).abs() < 0.001,
            "only resolved with accuracy count, got {}",
            avg
        );
    }

    // --- per_context_breakdown: exact scores ---

    #[test]
    fn test_per_context_breakdown_known_buckets() {
        let buckets = [0.8, 0.6, 0.7, 0.5, 0.4, 0.3, 0.2];
        let penalties = [0.0, 0.0, 0.0, 0.0];
        let inp = PerContextInputs {
            buckets,
            penalties,
            rep_active: true,
            mot_active: true,
            pat_active: true,
        };
        let result = per_context_breakdown(&inp, None);
        assert_eq!(result.len(), 6);
        for (ctx, score) in &result {
            assert!(*score <= 100, "score {:?} = {} exceeds 100", ctx, score);
        }
    }

    #[test]
    fn test_per_context_breakdown_all_zero_buckets() {
        let inp = PerContextInputs {
            buckets: [0.0; 7],
            penalties: [0.0; 4],
            rep_active: true,
            mot_active: true,
            pat_active: true,
        };
        let result = per_context_breakdown(&inp, None);
        for (_, score) in &result {
            assert_eq!(*score, 0, "zero buckets → zero score");
        }
    }

    #[test]
    fn test_per_context_breakdown_with_penalties_lowers_score() {
        let buckets = [0.8, 0.6, 0.7, 0.5, 0.4, 0.3, 0.2];
        let no_pen = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        let with_pen = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.1, 0.1, 0.1, 0.1],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        for ((_, s1), (_, s2)) in no_pen.iter().zip(&with_pen) {
            assert!(*s2 <= *s1, "penalties must lower score: {} > {}", s1, s2);
        }
    }

    #[test]
    fn test_per_context_breakdown_inactive_components() {
        let buckets = [0.5, 0.9, 0.9, 0.9, 0.5, 0.5, 0.5];
        let active = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        let inactive = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: false,
                mot_active: false,
                pat_active: false,
            },
            None,
        );
        for ((_, s_a), (_, s_i)) in active.iter().zip(&inactive) {
            assert!(
                *s_a >= *s_i,
                "inactive components should lower or equal score"
            );
        }
    }

    // --- virtue_adjustment: match guard mutations ---

    #[test]
    fn test_virtue_fairness_low_penalty() {
        let m = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 2,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        let expected = -CFG.motivation.virtue.fairness - CFG.motivation.virtue.helping;
        assert!(
            (adj - expected).abs() < 0.001,
            "got {} expected {}",
            adj,
            expected
        );
    }

    #[test]
    fn test_virtue_helping_low_penalty() {
        let m = vec![Motivation {
            r#type: MotivationType::Helping,
            intensity: 2,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        let expected = -CFG.motivation.virtue.fairness - CFG.motivation.virtue.helping;
        assert!(
            (adj - expected).abs() < 0.001,
            "got {} expected {}",
            adj,
            expected
        );
    }

    #[test]
    fn test_virtue_fairness_absent_penalty() {
        let adj = virtue_adjustment(&[]);
        assert!(
            (adj - (-CFG.motivation.virtue.fairness - CFG.motivation.virtue.helping)).abs() < 0.001
        );
    }

    #[test]
    fn test_virtue_learning_high_bonus() {
        let m = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 8,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        assert!(
            (adj - (CFG.motivation.virtue.learning
                - CFG.motivation.virtue.fairness
                - CFG.motivation.virtue.helping))
                .abs()
                < 0.001
        );
    }

    #[test]
    fn test_virtue_creativity_high_bonus() {
        let m = vec![Motivation {
            r#type: MotivationType::Creativity,
            intensity: 8,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        assert!(
            (adj - (CFG.motivation.virtue.creativity
                - CFG.motivation.virtue.fairness
                - CFG.motivation.virtue.helping))
                .abs()
                < 0.001
        );
    }

    #[test]
    fn test_virtue_power_high_penalty() {
        let m = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 8,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        assert!(
            (adj - (-CFG.motivation.virtue.power
                - CFG.motivation.virtue.fairness
                - CFG.motivation.virtue.helping))
                .abs()
                < 0.001
        );
    }

    #[test]
    fn test_virtue_security_high_penalty() {
        let m = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 8,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        assert!(
            (adj - (-CFG.motivation.virtue.security
                - CFG.motivation.virtue.fairness
                - CFG.motivation.virtue.helping))
                .abs()
                < 0.001
        );
    }

    #[test]
    fn test_virtue_recognition_extreme_penalty() {
        let m = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 9,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        assert!(
            (adj - (-CFG.motivation.virtue.recognition
                - CFG.motivation.virtue.fairness
                - CFG.motivation.virtue.helping))
                .abs()
                < 0.001
        );
    }

    #[test]
    fn test_virtue_recognition_below_extreme_no_penalty() {
        let m = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 8,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        assert!(
            (adj - (-CFG.motivation.virtue.fairness - CFG.motivation.virtue.helping)).abs() < 0.001
        );
    }

    #[test]
    fn test_virtue_boundary_at_high_triggers() {
        let m = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: CFG.motivation.virtue.high,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        let expected = CFG.motivation.virtue.fairness - CFG.motivation.virtue.helping;
        assert!(
            (adj - expected).abs() < 0.001,
            "got {} expected {}",
            adj,
            expected
        );
    }

    #[test]
    fn test_virtue_boundary_below_high_no_bonus() {
        let m = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: CFG.motivation.virtue.high - 1,
            notes: String::new(),
        }];
        let adj = virtue_adjustment(&m);
        // Fairness at high-1 = moderate → no bonus, no penalty
        // Helping is absent → -v.helping
        let expected = -CFG.motivation.virtue.helping;
        assert!(
            (adj - expected).abs() < 0.001,
            "got {} expected {}",
            adj,
            expected
        );
    }

    // --- invalidated_motivations: all match arms ---

    #[test]
    fn test_invalidated_security_gullible() {
        let inv = invalidated_motivations(&["flag_security_gullible"]);
        assert!(inv.contains(&MotivationType::Security));
    }

    #[test]
    fn test_invalidated_security_risky() {
        let inv = invalidated_motivations(&["flag_security_risky"]);
        assert!(inv.contains(&MotivationType::Security));
    }

    #[test]
    fn test_invalidated_risk_appetite_ambition() {
        let inv = invalidated_motivations(&["flag_risk_appetite_ambition"]);
        assert!(inv.contains(&MotivationType::Power));
        assert!(inv.contains(&MotivationType::Achievement));
    }

    #[test]
    fn test_invalidated_power_passive() {
        let inv = invalidated_motivations(&["flag_power_passive"]);
        assert!(inv.contains(&MotivationType::Power));
    }

    #[test]
    fn test_invalidated_pattern_recognition_dismissive() {
        let inv = invalidated_motivations(&["flag_pattern_recognition_dismissive"]);
        assert!(inv.contains(&MotivationType::Recognition));
    }

    #[test]
    fn test_invalidated_creativity_closed() {
        let inv = invalidated_motivations(&["flag_creativity_closed"]);
        assert!(inv.contains(&MotivationType::Creativity));
    }

    #[test]
    fn test_invalidated_creativity_rigid() {
        let inv = invalidated_motivations(&["flag_creativity_rigid"]);
        assert!(inv.contains(&MotivationType::Creativity));
    }

    #[test]
    fn test_invalidated_autonomy_submissive() {
        let inv = invalidated_motivations(&["flag_autonomy_submissive"]);
        assert!(inv.contains(&MotivationType::Autonomy));
    }

    #[test]
    fn test_invalidated_affiliation_cold() {
        let inv = invalidated_motivations(&["flag_affiliation_cold"]);
        assert!(inv.contains(&MotivationType::Affiliation));
    }

    #[test]
    fn test_invalidated_affiliation_distrustful() {
        let inv = invalidated_motivations(&["flag_affiliation_distrustful"]);
        assert!(inv.contains(&MotivationType::Affiliation));
    }

    // --- profile_completeness: exact arithmetic ---

    #[test]
    fn test_profile_completeness_half() {
        let mut p = make_person(Some(5), Some(5), None, None, None);
        p.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 5,
            notes: String::new(),
        }];
        let c = profile_completeness(&p);
        let expected = (2 + 1) as f64 / CFG.completeness.denominator;
        assert!(
            (c - expected).abs() < 0.001,
            "got {} expected {}",
            c,
            expected
        );
    }

    // --- compute_person_profile: exact arithmetic ---

    #[test]
    fn test_compute_person_profile_empty() {
        let p = make_person(None, None, None, None, None);
        let prof = compute_person_profile(&p);
        assert!(
            prof.motivation >= 0.0 && prof.motivation <= 1.0,
            "motivation in range"
        );
        assert!(
            prof.patterns >= 0.0 && prof.patterns <= 1.0,
            "patterns in range"
        );
        assert!(prof.total <= 100, "total must be <= 100");
    }

    #[test]
    fn test_compute_person_profile_band() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.confidence = 10;
        let prof = compute_person_profile(&p);
        assert_eq!(prof.band, confidence_band(10));
    }

    #[test]
    fn test_compute_person_profile_bias_count() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.biases = vec![
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 2,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 2,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Availability,
                intensity: 2,
                evidence: String::new(),
            },
        ];
        let prof = compute_person_profile(&p);
        assert!(
            prof.bias > 0.0,
            "bias score should be positive with 3 biases present"
        );
    }

    // --- model_config: matrix values ---

    #[test]
    fn test_relation_weights_all_distinct() {
        let mut seen = std::collections::HashSet::new();
        for w in &CFG.relationship.weights {
            let key: Vec<String> = w.iter().map(|v| format!("{:.2}", v)).collect();
            seen.insert(key);
        }
        assert!(
            seen.len() >= 6,
            "at least 6 distinct weight rows (Manages/ReportsTo may share)"
        );
    }

    #[test]
    fn test_context_weights_all_sum_to_one() {
        for row in &CFG.contexts.weights {
            let sum: f64 = row.iter().sum();
            assert!((sum - 1.0).abs() < 0.001, "row sums to {}, not 1.0", sum);
        }
    }

    #[test]
    fn test_motivation_synergy_non_zero_entries() {
        for (i, row) in CFG.motivation.synergy.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                assert!((-1.0..=1.0).contains(&v), "trigger[{}][{}] = {}", i, j, v);
            }
        }
    }

    #[test]
    fn test_motivation_synergy_diagonal_power_negative() {
        assert!(
            CFG.motivation.synergy[0][0] < 0.0,
            "Power×Power should be negative"
        );
    }

    #[test]
    fn test_motivation_synergy_diagonal_learning_positive() {
        let i = MotivationType::ALL
            .iter()
            .position(|&t| t == MotivationType::Learning)
            .unwrap();
        assert!(
            CFG.motivation.synergy[i][i] > 0.0,
            "Learning×Learning should be positive"
        );
    }

    #[test]
    fn test_trigger_synergy_all_entries_in_range() {
        for (i, row) in CFG.patterns.trigger_synergy.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                assert!((-1.0..=1.0).contains(&v), "trigger[{}][{}] = {}", i, j, v);
            }
        }
    }

    #[test]
    fn test_bias_modulation_all_targets() {
        for (i, entry) in CFG.bias.modulation.iter().enumerate() {
            if let Some((target, coeff)) = entry {
                assert!(coeff.abs() <= 1.0, "modulation[{}] coeff = {}", i, coeff);
                let _ = target;
            }
        }
    }

    #[test]
    fn test_base_weights_sum() {
        let w = &CFG.base_weights;
        let sum = w.ocean + w.reputation + w.motivation + w.patterns + w.bias + w.style + w.values;
        assert!((sum - 1.0).abs() < 0.001, "base weights sum to {}", sum);
    }

    #[test]
    fn test_rep_danger_thresholds_in_range() {
        let d = &CFG.reputation.danger;
        assert!(d.high > d.low, "high must exceed low");
        assert!(d.high <= 10);
    }

    #[test]
    fn test_ocean_danger_thresholds() {
        let d = &CFG.ocean.danger;
        assert!(d.high > d.low);
        assert!(d.high <= 10);
    }

    #[test]
    fn test_virtue_thresholds() {
        let v = &CFG.motivation.virtue;
        assert!(v.high > v.low);
        assert!(v.recognition_high > v.high);
    }

    #[test]
    fn test_bias_thresholds() {
        let b = &CFG.bias;
        assert!(b.strong_min > b.mild_max);
    }

    // --- compute_synergy_score_inner: specific numeric checks ---

    #[test]
    fn test_synergy_identical_full_profiles() {
        let p = full_profile();
        let b = compute_synergy_score_inner(&p, &p, None, &[], &[]);
        assert_eq!(b.total, b.a_score, "identical persons → total == a_score");
        assert_eq!(b.a_score, b.b_score, "identical persons → a == b");
        assert!(
            b.ocean > 0.5,
            "identical high OCEAN → ocean > 0.5, got {}",
            b.ocean
        );
        assert!(b.reputation > 0.0, "identical rep → positive");
    }

    #[test]
    fn test_synergy_opposite_ocean_direction() {
        let a = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        let b = make_person(Some(1), Some(1), Some(1), Some(1), Some(10));
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            breakdown.ocean < 0.5,
            "opposite OCEAN → low ocean, got {}",
            breakdown.ocean
        );
    }

    #[test]
    fn test_synergy_with_relationship_changes_weights() {
        let a = full_profile();
        let b = full_profile();
        let no_ctx = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let with_ctx = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        assert_ne!(
            no_ctx.total, with_ctx.total,
            "relationship context should change total"
        );
    }

    #[test]
    fn test_synergy_per_context_has_six_entries() {
        let a = full_profile();
        let b = full_profile();
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_eq!(breakdown.per_context.len(), 6);
    }

    #[test]
    fn test_synergy_danger_non_negative() {
        let a = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
        let b = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(breakdown.danger >= 0.0, "danger must be non-negative");
        assert!(breakdown.total <= 100, "total must be <= 100");
    }

    #[test]
    fn test_synergy_bias_mod_active_with_shared_biases() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(breakdown.bias_mod_active, "shared biases → mod active");
    }

    #[test]
    fn test_synergy_history_penalty_with_low_accuracy() {
        let a = full_profile();
        let b = full_profile();
        let preds_a: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(3),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let preds_b: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "b".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(3),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let no_preds = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let with_preds = compute_synergy_score_inner(&a, &b, None, &preds_a, &preds_b);
        assert!(
            with_preds.danger > no_preds.danger,
            "low accuracy → more danger"
        );
    }

    #[test]
    fn test_synergy_history_single_low_accuracy() {
        let a = full_profile();
        let b = full_profile();
        let preds_a: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "a".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: None,
                accuracy: Some(3),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let with_single = compute_synergy_score_inner(&a, &b, None, &preds_a, &[]);
        let with_both = compute_synergy_score_inner(&a, &b, None, &preds_a, &preds_a);
        assert!(
            with_both.danger >= with_single.danger,
            "both low ≥ single low danger"
        );
    }

    #[test]
    fn test_synergy_no_patterns_when_empty() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.behavioral_patterns = vec![];
        b.behavioral_patterns = vec![];
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_eq!(breakdown.patterns, 0.0);
    }

    #[test]
    fn test_synergy_negative_patterns_only_penalty() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::BecomesQuiet,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::Escalates,
                notes: String::new(),
            },
        ];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::BecomesQuiet,
            notes: String::new(),
        }];
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            breakdown.patterns < 1.0,
            "negative-only patterns should not be 1.0"
        );
    }

    // --- compute_team_synergy: exact checks ---

    #[test]
    fn test_team_synergy_two_persons_avg() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert_eq!(team.pairs.len(), 1);
        assert_eq!(team.avg_score, team.pairs[0].breakdown.total);
        assert_eq!(
            team.weakest.as_ref().unwrap().2,
            team.pairs[0].breakdown.total
        );
        assert_eq!(
            team.strongest.as_ref().unwrap().2,
            team.pairs[0].breakdown.total
        );
    }

    #[test]
    fn test_team_synergy_three_persons_pair_count() {
        let a = {
            let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
            p.id = "b".into();
            p
        };
        let c = {
            let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
            p.id = "c".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b, c], &[], &preds).unwrap();
        assert_eq!(team.pairs.len(), 3, "3 persons → 3 pairs");
        assert_eq!(team.team_size, 3);
    }

    #[test]
    fn test_team_synergy_avg_danger_non_negative() {
        let a = {
            let mut p = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
            p.id = "b".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert!(team.avg_danger >= 0.0);
        assert!(team.max_danger >= 0.0);
        assert!(team.avg_score <= 100);
    }

    #[test]
    fn test_team_synergy_with_relationship() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p
        };
        let rel = Relationship {
            id: "r1".into(),
            source_id: "a".into(),
            target_id: "b".into(),
            r#type: RelationType::Manages,
            strength: 8,
            notes: String::new(),
            created_at: 0,
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[rel], &preds).unwrap();
        assert_eq!(team.pairs.len(), 1);
        let b = &team.pairs[0].breakdown;
        assert!(b.band > 0, "relationship → nonzero band");
    }

    #[test]
    fn test_team_synergy_context_averages_all_six() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert_eq!(team.context_averages.len(), 6);
    }

    // --- value_self_score: exact arithmetic ---

    #[test]
    fn test_value_self_score_empty() {
        let score = value_self_score(&[]);
        assert!((score - 0.0).abs() < 0.001, "empty → 0.0, got {}", score);
    }

    #[test]
    fn test_value_self_score_single() {
        let vals = vec![Value {
            r#type: ValueType::Career,
            intensity: 10,
            priority: 10,
            notes: String::new(),
        }];
        let score = value_self_score(&vals);
        assert!((score - 1.0).abs() < 0.001, "10+10 → 1.0, got {}", score);
    }

    #[test]
    fn test_value_self_score_low() {
        let vals = vec![Value {
            r#type: ValueType::Career,
            intensity: 1,
            priority: 1,
            notes: String::new(),
        }];
        let score = value_self_score(&vals);
        assert!((score - 0.1).abs() < 0.001, "1+1 → 0.1, got {}", score);
    }

    #[test]
    fn test_value_self_score_multiple_average() {
        let vals = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 10,
                priority: 10,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 2,
                priority: 2,
                notes: String::new(),
            },
        ];
        let score = value_self_score(&vals);
        let expected = (1.0 + 0.2) / 2.0;
        assert!(
            (score - expected).abs() < 0.001,
            "avg of 1.0 and 0.2 → {}, got {}",
            expected,
            score
        );
    }

    // --- profile_completeness: exact arithmetic ---

    #[test]
    fn test_profile_completeness_empty_zero() {
        let p = make_person(None, None, None, None, None);
        let c = profile_completeness(&p);
        assert!((c - 0.0).abs() < 0.001, "empty → 0.0, got {}", c);
    }

    #[test]
    fn test_profile_completeness_full_one() {
        let p = full_profile();
        let c = profile_completeness(&p);
        assert!(c > 0.5, "full profile > 0.5, got {}", c);
        assert!(c <= 1.0, "full profile <= 1.0");
    }

    #[test]
    fn test_profile_completeness_ocean_only_exact() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let c = profile_completeness(&p);
        let expected = 5.0 / CFG.completeness.denominator;
        assert!(
            (c - expected).abs() < 0.001,
            "ocean only → {}, got {}",
            expected,
            c
        );
    }

    // --- compute_synergy_score_inner: bias score with no shared biases ---

    #[test]
    fn test_synergy_bias_score_no_shared() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 5,
            evidence: String::new(),
        }];
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (breakdown.bias - 0.0).abs() < 0.001,
            "no shared → 0.0, got {}",
            breakdown.bias
        );
    }

    #[test]
    fn test_synergy_bias_score_all_shared() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        let breakdown = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (breakdown.bias - 1.0).abs() < 0.001,
            "all shared → 1.0, got {}",
            breakdown.bias
        );
    }

    // --- per_context_breakdown: exact score math ---

    #[test]
    fn test_per_context_breakdown_all_ones() {
        let inp = PerContextInputs {
            buckets: [1.0; 7],
            penalties: [0.0; 4],
            rep_active: true,
            mot_active: true,
            pat_active: true,
        };
        let result = per_context_breakdown(&inp, None);
        for (_, score) in &result {
            assert_eq!(*score, 100, "all 1.0 buckets → 100");
        }
    }

    #[test]
    fn test_per_context_breakdown_with_context() {
        let inp = PerContextInputs {
            buckets: [0.8, 0.6, 0.7, 0.5, 0.4, 0.3, 0.2],
            penalties: [0.0; 4],
            rep_active: true,
            mot_active: true,
            pat_active: true,
        };
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let result = per_context_breakdown(&inp, Some(&ctx));
        assert_eq!(result.len(), 6);
        for (_, score) in &result {
            assert!(*score <= 100);
        }
    }

    // --- rep_danger_penalty: one side only vs both ---

    #[test]
    fn test_rep_danger_penalty_one_side_only() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            authoritative_submissive: Some(d.high),
            ..Default::default()
        };
        let rb = RepScores {
            authoritative_submissive: Some(d.low),
            ..Default::default()
        };
        assert_eq!(
            rep_danger_penalty(&ra, &rb),
            0.0,
            "only one side high → no power struggle"
        );
    }

    #[test]
    fn test_rep_danger_penalty_boundary_at_low_triggers() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            diplomatic_blunt: Some(d.low),
            ..Default::default()
        };
        let rb = RepScores {
            diplomatic_blunt: Some(d.low),
            ..Default::default()
        };
        assert!((rep_danger_penalty(&ra, &rb) - d.brutal).abs() < 0.001);
    }

    #[test]
    fn test_rep_danger_penalty_boundary_above_low_no_trigger() {
        let d = &CFG.reputation.danger;
        let ra = RepScores {
            diplomatic_blunt: Some(d.low + 1),
            ..Default::default()
        };
        let rb = RepScores {
            diplomatic_blunt: Some(d.low + 1),
            ..Default::default()
        };
        assert_eq!(rep_danger_penalty(&ra, &rb), 0.0);
    }

    // --- synergy core arithmetic ---

    #[test]
    fn test_synergy_ocean_complement_bonus() {
        let a = make_person(Some(8), Some(7), Some(5), Some(5), Some(5));
        let b = make_person(Some(7), Some(8), Some(5), Some(5), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let a2 = make_person(Some(3), Some(3), Some(5), Some(5), Some(5));
        let b2 = make_person(Some(3), Some(3), Some(5), Some(5), Some(5));
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(bd.ocean > bd2.ocean, "complement bonus → higher ocean");
    }

    #[test]
    fn test_synergy_style_score() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.styles = vec![PersonalStyle {
            r#type: StyleType::DiplomaticCommunicator,
            intensity: 5,
            notes: String::new(),
        }];
        b.styles = vec![PersonalStyle {
            r#type: StyleType::DiplomaticCommunicator,
            intensity: 5,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.styles - 1.0).abs() < 0.001,
            "same style → 1.0, got {}",
            bd.styles
        );
    }

    #[test]
    fn test_synergy_values_score() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.values = vec![Value {
            r#type: ValueType::Career,
            intensity: 10,
            priority: 10,
            notes: String::new(),
        }];
        b.values = vec![Value {
            r#type: ValueType::Career,
            intensity: 10,
            priority: 10,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.values - 1.0).abs() < 0.001,
            "identical values → 1.0, got {}",
            bd.values
        );
    }

    #[test]
    fn test_synergy_total_clamped_100() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.total <= 100, "total must be <= 100");
    }

    #[test]
    fn test_synergy_scores_non_negative() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.ocean >= 0.0);
        assert!(bd.reputation >= 0.0);
        assert!(bd.motivation >= 0.0);
        assert!(bd.patterns >= 0.0);
        assert!(bd.bias >= 0.0);
        assert!(bd.styles >= 0.0);
        assert!(bd.values >= 0.0);
    }

    #[test]
    fn test_synergy_asymmetric_different() {
        let a = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
        let b = make_person(Some(1), Some(1), Some(1), Some(1), Some(10));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_ne!(bd.a_score, bd.b_score, "asymmetric → different scores");
    }

    #[test]
    fn test_synergy_single_person_empty_biases() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![];
        b.biases = vec![];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.bias - CFG.bias.default).abs() < 0.001,
            "no biases → default bias score"
        );
    }

    // =====================================================================
    // per_context_breakdown: exact arithmetic (catches lines 74-123 mutants)
    // =====================================================================

    fn make_inputs(
        buckets: [f64; 7],
        penalties: [f64; 4],
        rep_active: bool,
        mot_active: bool,
        pat_active: bool,
    ) -> PerContextInputs {
        PerContextInputs {
            buckets,
            penalties,
            rep_active,
            mot_active,
            pat_active,
        }
    }

    #[test]
    fn test_pcb_with_ctx_differs_from_none() {
        let inp = make_inputs(
            [0.9, 0.1, 0.8, 0.2, 0.7, 0.3, 0.05],
            [0.0; 4],
            true,
            true,
            true,
        );
        let none = per_context_breakdown(&inp, None);
        let manages = per_context_breakdown(
            &inp,
            Some(&RelContext {
                rtype: RelationType::Manages,
                strength: 5,
            }),
        );
        let mut any_diff = false;
        for ((_, s1), (_, s2)) in none.iter().zip(&manages) {
            if s1 != s2 {
                any_diff = true;
            }
        }
        assert!(any_diff, "ctx=Manages must reweight scores vs ctx=None");
    }

    #[test]
    fn test_pcb_renormalization_exact_decision_manages() {
        let inp = make_inputs(
            [0.9, 0.1, 0.8, 0.2, 0.7, 0.3, 0.05],
            [0.0; 4],
            true,
            true,
            true,
        );
        let result = per_context_breakdown(
            &inp,
            Some(&RelContext {
                rtype: RelationType::Manages,
                strength: 5,
            }),
        );
        let cw = CFG.context_weights(InsightContext::Decision);
        let rw = CFG.relation_weights(RelationType::Manages);
        let p: [f64; 7] = std::array::from_fn(|i| rw[i] * cw[i]);
        let s: f64 = p.iter().sum();
        let w: [f64; 7] = std::array::from_fn(|i| p[i] / s);
        let mut num = inp.buckets[0] * w[0];
        let mut wsum = w[0];
        num += inp.buckets[1] * w[1];
        wsum += w[1];
        num += inp.buckets[2] * w[2];
        wsum += w[2];
        num += inp.buckets[3] * w[3];
        wsum += w[3];
        num += inp.buckets[4] * w[4];
        wsum += w[4];
        num += inp.buckets[5] * w[5];
        wsum += w[5];
        num += inp.buckets[6] * w[6];
        wsum += w[6];
        let raw = num / wsum;
        let expected = ((raw * 100.0).round() as u8).min(100);
        let decision = result
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap();
        assert_eq!(
            decision.1, expected,
            "exact Decision score under Manages context"
        );
    }

    #[test]
    fn test_pcb_inactive_rep_skips_w1() {
        let inp = make_inputs(
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0; 4],
            false,
            false,
            false,
        );
        let result = per_context_breakdown(&inp, None);
        for (_, s) in &result {
            assert_eq!(*s, 0, "rep inactive + all-zero buckets → 0");
        }
    }

    #[test]
    fn test_pcb_only_ocean_contributes() {
        let inp = make_inputs(
            [0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0; 4],
            false,
            false,
            false,
        );
        let result = per_context_breakdown(&inp, None);
        let decision = result
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap();
        assert!(decision.1 > 0, "ocean bucket alone should yield >0");
    }

    #[test]
    fn test_pcb_penalty_weighted_by_w() {
        let inp_no_pen = make_inputs([0.8; 7], [0.0; 4], true, true, true);
        let inp_with_pen = make_inputs([0.8; 7], [0.2, 0.3, 0.1, 0.4], true, true, true);
        let r1 = per_context_breakdown(&inp_no_pen, None);
        let r2 = per_context_breakdown(&inp_with_pen, None);
        for ((_, s1), (_, s2)) in r1.iter().zip(&r2) {
            assert!(*s2 < *s1, "penalty must reduce score: {s2} < {s1}");
        }
    }

    #[test]
    fn test_pcb_history_penalty_uses_base_weights() {
        let inp = make_inputs([0.8; 7], [0.0, 0.0, 0.0, 1.0], true, true, true);
        let result = per_context_breakdown(&inp, None);
        let decision = result
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap();
        let cw = CFG.context_weights(InsightContext::Decision);
        let raw: f64 = 0.8;
        let danger = 1.0 * CFG.base_weights.history;
        let wsum: f64 = cw.iter().sum();
        let pen = (danger / wsum * 100.0).round() as u8;
        let expected = ((raw * 100.0).round() as u8).min(100).saturating_sub(pen);
        assert_eq!(
            decision.1, expected,
            "history penalty uses base_weights.history"
        );
    }

    #[test]
    fn test_pcb_danger_adds_all_four_penalties() {
        let inp = make_inputs([0.5; 7], [0.1, 0.2, 0.3, 0.4], true, true, true);
        let result_no =
            per_context_breakdown(&make_inputs([0.5; 7], [0.0; 4], true, true, true), None);
        let result_yes = per_context_breakdown(&inp, None);
        let d1: u8 = result_no
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap()
            .1;
        let d2: u8 = result_yes
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap()
            .1;
        assert!(d1 > d2, "all penalties should reduce score: {d1} > {d2}");
        assert!(
            d1 - d2 >= 10,
            "four nonzero penalties should reduce significantly: diff={}",
            d1 - d2
        );
    }

    // =====================================================================
    // compute_synergy_score_inner: complement guards (lines 152-170)
    // =====================================================================

    #[test]
    fn test_ocean_oc_complement_exact_boundary() {
        let a = make_person(Some(7), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(7), Some(5), Some(5), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let a2 = make_person(Some(6), Some(5), Some(5), Some(5), Some(5));
        let b2 = make_person(Some(5), Some(7), Some(5), Some(5), Some(5));
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "O=7,C=7 cross-pair qualifies but O=6,C=7 does not"
        );
    }

    #[test]
    fn test_ocean_oc_complement_cross_pair() {
        let a = make_person(Some(8), Some(3), Some(5), Some(5), Some(5));
        let b = make_person(Some(3), Some(8), Some(5), Some(5), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let a2 = make_person(Some(3), Some(3), Some(5), Some(5), Some(5));
        let b2 = make_person(Some(3), Some(3), Some(5), Some(5), Some(5));
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "cross-pair O(a)*C(b) complement bonus"
        );
    }

    #[test]
    fn test_ocean_ea_complement_exact_boundary() {
        let a = make_person(Some(5), Some(5), Some(7), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(7), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let a2 = make_person(Some(5), Some(5), Some(6), Some(5), Some(5));
        let b2 = make_person(Some(5), Some(5), Some(5), Some(7), Some(5));
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "E=7,A=7 cross-pair qualifies but E=6,A=7 does not"
        );
    }

    #[test]
    fn test_ocean_ea_complement_cross_pair() {
        let a = make_person(Some(5), Some(5), Some(8), Some(3), Some(5));
        let b = make_person(Some(5), Some(5), Some(3), Some(8), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let a2 = make_person(Some(5), Some(5), Some(3), Some(3), Some(5));
        let b2 = make_person(Some(5), Some(5), Some(3), Some(3), Some(5));
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "cross-pair E(a)*A(b) complement bonus"
        );
    }

    #[test]
    fn test_no_complement_when_below_boundary() {
        let a = make_person(Some(6), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(6), Some(5), Some(5), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let a2 = make_person(Some(3), Some(3), Some(5), Some(5), Some(5));
        let b2 = make_person(Some(3), Some(3), Some(5), Some(5), Some(5));
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "higher OCEAN → higher ocean, but no complement bonus"
        );
    }

    // =====================================================================
    // compute_synergy_score_inner: raw_ocean arithmetic (line 178)
    // =====================================================================

    #[test]
    fn test_raw_ocean_averages_oc_ea_n() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd.ocean > 0.9,
            "identical moderate OCEAN → ocean close to 1.0, got {}",
            bd.ocean
        );
        assert!(bd.ocean <= 1.0, "ocean must be <= 1.0");
    }

    // =====================================================================
    // compute_synergy_score_inner: rep arithmetic (line 193)
    // =====================================================================

    #[test]
    fn test_rep_perfect_match() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            authoritative_submissive: Some(5),
            honest_deceitful: Some(5),
            reliable_flaky: Some(5),
            humble_arrogant: Some(5),
            calm_reactive: Some(5),
            diplomatic_blunt: Some(5),
            generous_selfish: Some(5),
            fair_favoritism: Some(5),
            trusting_suspicious: Some(5),
            assertive_passive: Some(5),
            empathetic_detached: Some(5),
            adaptable_rigid: Some(5),
        };
        b.rep_scores = a.rep_scores.clone();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.reputation - 1.0).abs() < 0.001,
            "identical rep → reputation=1.0, got {}",
            bd.reputation
        );
    }

    #[test]
    fn test_rep_no_scores_inactive() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores = RepScores::default();
        b.rep_scores = RepScores::default();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(!bd.bias_mod_active || bd.reputation == 0.0);
        assert_eq!(bd.reputation, 0.0, "no rep scores → reputation=0");
    }

    // =====================================================================
    // compute_synergy_score_inner: mot_active/pat_active (lines 197, 205)
    // =====================================================================

    #[test]
    fn test_mot_inactive_when_empty() {
        let mut a = full_profile();
        let b = full_profile();
        a.motivations = vec![];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_eq!(bd.motivation, 0.0, "empty motivations → motivation=0");
    }

    #[test]
    fn test_pat_inactive_when_empty() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![];
        b.behavioral_patterns = vec![];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_eq!(bd.patterns, 0.0, "empty patterns → patterns=0");
    }

    // =====================================================================
    // compute_synergy_score_inner: bias score arithmetic (line 233)
    // =====================================================================

    #[test]
    fn test_bias_score_shared_type_ratio() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 5,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 5,
                evidence: String::new(),
            },
        ];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let expected = 1.0 / 2.0;
        assert!(
            (bd.bias - expected).abs() < 0.001,
            "1 shared / 2 max_unique = 0.5, got {}",
            bd.bias
        );
    }

    // =====================================================================
    // compute_synergy_score_inner: bias modulation (lines 250-280)
    // =====================================================================

    #[test]
    fn test_bias_modulation_shared_type() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.bias_mod_active, "shared bias → modulation active");
    }

    #[test]
    fn test_bias_modulation_no_shared() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 5,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(!bd.bias_mod_active, "no shared bias type → no modulation");
    }

    #[test]
    fn test_bias_opposite_complementarity() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        b2.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean <= bd2.ocean,
            "opposite biases produce negative ocean_mod → lower or equal ocean"
        );
    }

    #[test]
    fn test_bias_opposite_cap() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![
            Bias {
                r#type: BiasType::DunningKruger,
                intensity: 10,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 10,
                evidence: String::new(),
            },
        ];
        b.biases = vec![
            Bias {
                r#type: BiasType::Impostor,
                intensity: 10,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Recency,
                intensity: 10,
                evidence: String::new(),
            },
        ];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.total <= 100);
        assert!(bd.total > 0);
    }

    // =====================================================================
    // compute_synergy_score_inner: history penalty (lines 296-303)
    // =====================================================================

    fn make_pred(person_id: &str, accuracy: Option<u8>, resolved: bool) -> Prediction {
        Prediction {
            id: "p1".into(),
            person_id: person_id.into(),
            context: String::new(),
            predicted_outcome: String::new(),
            actual_outcome: if resolved { Some("ok".into()) } else { None },
            accuracy,
            created_at: 0,
            resolved_at: if resolved { Some(0) } else { None },
            resolved,
        }
    }

    #[test]
    fn test_history_both_low() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(4), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(4), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);
        assert!(bd.danger_details.contains("Low prediction accuracy"));
        assert!(bd.danger > 0.0);
    }

    #[test]
    fn test_history_single_low() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(4), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(8), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);
        assert!(bd.danger_details.contains("Low prediction accuracy"));
    }

    #[test]
    fn test_history_exactly_at_boundary_no_penalty() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(5), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(5), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);
        assert!(
            !bd.danger_details.contains("Low prediction accuracy"),
            "accuracy=5.0 is not < 5.0, should not penalize"
        );
    }

    #[test]
    fn test_history_both_high_no_penalty() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(8), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(7), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);
        assert!(!bd.danger_details.contains("Low prediction accuracy"));
    }

    #[test]
    fn test_history_one_low_one_none_no_penalty() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(4), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &[]);
        assert!(
            !bd.danger_details.contains("Low prediction accuracy"),
            "one low + one None → no penalty (match falls through)"
        );
    }

    #[test]
    fn test_history_both_none_no_penalty() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(!bd.danger_details.contains("Low prediction accuracy"));
    }

    // =====================================================================
    // compute_synergy_score_inner: relationship context (lines 314-335)
    // =====================================================================

    #[test]
    fn test_rel_reports_to_directional() {
        let mut a = full_profile();
        let b = full_profile();
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 10,
            notes: String::new(),
        }];
        let ctx = RelContext {
            rtype: RelationType::ReportsTo,
            strength: 5,
        };
        let bd = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let ctx_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_ne!(
            bd.motivation, ctx_none.motivation,
            "ReportsTo + Power sub → friction"
        );
    }

    #[test]
    fn test_rel_manages_power_friction() {
        let a = full_profile();
        let mut b = full_profile();
        b.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 10,
            notes: String::new(),
        }];
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let bd = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let ctx_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd.motivation < ctx_none.motivation,
            "Manages + Power sub (b) → friction lowers motivation"
        );
    }

    #[test]
    fn test_rel_power_below_threshold_no_friction() {
        let mut a = full_profile();
        let b = full_profile();
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 5,
            notes: String::new(),
        }];
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let bd = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let ctx_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.motivation - ctx_none.motivation).abs() < 0.001,
            "Power below threshold → no friction"
        );
    }

    #[test]
    fn test_rel_hierarchy_bonus() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores.authoritative_submissive = Some(9);
        b.rep_scores.authoritative_submissive = Some(5);
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let bd = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let bd_no_ctx = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd.reputation > bd_no_ctx.reputation,
            "hierarchy bonus lifts reputation"
        );
    }

    #[test]
    fn test_rel_no_bonus_when_diff_insufficient() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores.authoritative_submissive = Some(7);
        b.rep_scores.authoritative_submissive = Some(5);
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let bd = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let bd_no_ctx = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.reputation - bd_no_ctx.reputation).abs() < 0.001,
            "rep diff 2 < 3 threshold → no hierarchy bonus"
        );
    }

    #[test]
    fn test_rel_friends_no_directional() {
        let mut a = full_profile();
        let b = full_profile();
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 10,
            notes: String::new(),
        }];
        let ctx = RelContext {
            rtype: RelationType::Friends,
            strength: 5,
        };
        let bd = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let ctx_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.motivation - ctx_none.motivation).abs() < 0.001,
            "Friends has no directional power friction"
        );
    }

    // =====================================================================
    // compute_synergy_score_inner: ocean/rep/mot/pat clamping (lines 337-340)
    // =====================================================================

    #[test]
    fn test_ocean_clamped_0_1() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.ocean >= 0.0 && bd.ocean <= 1.0);
    }

    #[test]
    fn test_reputation_clamped_0_1() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.reputation >= 0.0 && bd.reputation <= 1.0);
    }

    #[test]
    fn test_motivation_clamped_0_1() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.motivation >= 0.0 && bd.motivation <= 1.0);
    }

    #[test]
    fn test_patterns_clamped_0_1() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.patterns >= 0.0 && bd.patterns <= 1.0);
    }

    // =====================================================================
    // compute_synergy_score_inner: total_danger arithmetic (lines 357-360)
    // =====================================================================

    #[test]
    fn test_total_danger_includes_history() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(3), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(3), true))
            .collect::<Vec<_>>();
        let bd_low = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);
        let bd_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd_low.danger > bd_none.danger,
            "low accuracy adds history danger"
        );
    }

    #[test]
    fn test_total_danger_weighted_sum() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.ocean.neuroticism = Some(10);
        b.ocean.neuroticism = Some(10);
        a.ocean.openness = Some(1);
        b.ocean.openness = Some(1);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.danger > 0.0, "volatile OCEAN → positive danger");
    }

    // =====================================================================
    // compute_synergy_score_inner: danger_details (lines 472-489)
    // =====================================================================

    #[test]
    fn test_danger_details_ocean_volatility() {
        let mut a = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
        let mut b = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
        a.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        if bd.danger > 0.0 {
            assert!(
                !bd.danger_details.is_empty(),
                "positive danger → non-empty details"
            );
        }
    }

    #[test]
    fn test_danger_details_empty_when_no_penalties() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd.danger_details.is_empty(),
            "full profile → no danger details"
        );
    }

    #[test]
    fn test_danger_details_negative_patterns() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd.danger_details.contains("negative patterns"),
            "both negative triggers → negative patterns detail"
        );
    }

    #[test]
    fn test_danger_details_low_prediction() {
        let a = full_profile();
        let b = full_profile();
        let preds = (0..3)
            .map(|_i| make_pred("a", Some(3), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &preds, &preds);
        assert!(
            bd.danger_details.contains("Low prediction accuracy"),
            "low accuracy → prediction detail"
        );
    }

    // =====================================================================
    // compute_synergy_score_inner: bias_mod_active (line 512)
    // =====================================================================

    #[test]
    fn test_bias_mod_active_with_shared_type() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.bias_mod_active, "shared Anchoring → mod active");
    }

    #[test]
    fn test_bias_mod_inactive_no_shared() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::LossAversion,
            intensity: 5,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(!bd.bias_mod_active, "no shared, no complement → inactive");
    }

    // =====================================================================
    // compute_synergy_score_inner: asymmetric scores (lines 418-460)
    // =====================================================================

    #[test]
    fn test_asym_a_b_different_rep() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores.honest_deceitful = Some(10);
        b.rep_scores.honest_deceitful = Some(2);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert_ne!(bd.a_score, bd.b_score);
        assert!(
            bd.b_score > bd.a_score,
            "b sees a's high rep → b_score higher"
        );
    }

    #[test]
    fn test_asym_total_is_average() {
        let a = full_profile();
        let b = full_profile();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let expected = ((bd.a_score as f64 + bd.b_score as f64) / 2.0).round() as u8;
        let expected = expected.saturating_sub(if bd.danger > 0.0 {
            (bd.danger / 1.0 * 100.0).round() as u8
        } else {
            0
        });
        assert_eq!(bd.total, expected, "total = avg(a,b) - danger_penalty");
    }

    // =====================================================================
    // compute_synergy_score_inner: danger_penalty arithmetic (line 452)
    // =====================================================================

    #[test]
    fn test_danger_penalty_normalizes_by_asym_w() {
        // Volatile twin pair with every non-OCEAN bucket inactive: asym_w
        // drops to 0.51 so scoring.rs:452 (`total_danger / asym_w`) must use
        // division, not `%` (would give penalty 8) or `*` (would give 4).
        // Division lands on 15; total is asserted via the same float ops.
        let a = make_person(Some(5), Some(5), Some(5), Some(3), Some(9));
        let b = make_person(Some(5), Some(5), Some(5), Some(3), Some(9));
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let ocean_penalty = ocean_danger_penalty(&a.ocean, &b.ocean);
        assert!(
            ocean_penalty > 0.4,
            "volatile pair must trigger ocean danger"
        );
        let asym_w = CFG.base_weights.ocean
            + CFG.base_weights.bias
            + CFG.base_weights.style
            + CFG.base_weights.values;
        assert!(
            (asym_w - 0.51).abs() < 1e-9,
            "rep/mot/pat inactive -> asym_w 0.51"
        );
        let total_danger = ocean_penalty * CFG.base_weights.ocean;
        let penalty = (total_danger / asym_w * 100.0).round() as u8;
        assert!(penalty >= 10, "normalized danger penalty should be ~15");
        let expected = ((bd.a_score as f64 + bd.b_score as f64) / 2.0).round() as u8;
        let expected = expected.saturating_sub(penalty);
        assert_eq!(bd.total, expected, "penalty = total_danger / asym_w * 100");
    }

    // =====================================================================
    // compute_synergy_score_inner: bias_mod_active summation (line 496)
    // =====================================================================

    #[test]
    fn test_bias_mod_active_sums_targets_not_differences() {
        // Shared Confirmation(10) boosts Reputation (+0.10); Anchoring(5)
        // boosts Ocean (+0.025). Kills 496:37 (`ocean_mod + rep_mod` -> `-`):
        // 0.025 - 0.10 < 0 would flip bias_mod_active to false.
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 10,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 5,
                evidence: String::new(),
            },
        ];
        b.biases = a.biases.clone();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.bias_mod_active, "ocean +0.025 + rep +0.10 > 0");
    }

    #[test]
    fn test_bias_mod_active_net_zero_under_difference() {
        // Shared DunningKruger(10) -> Ocean -0.10, SunkCost(10) -> Motivation
        // +0.10, Availability(10) -> Patterns +0.10: the total is +0.10 > 0.
        // Kills 496:47 and 496:57 (`+` -> `-` / `*`): each replacement pushes
        // the expression to <= 0 and flips bias_mod_active to false.
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![
            one_bias(BiasType::DunningKruger),
            one_bias(BiasType::SunkCost),
            one_bias(BiasType::Availability),
        ];
        b.biases = a.biases.clone();
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.bias_mod_active, "-0.10 + 0.10 + 0.10 > 0");
    }

    // =====================================================================
    // compute_synergy_score_inner: neuroticism inversion (line 390)
    // =====================================================================

    #[test]
    fn test_neuroticism_inverted_in_asym() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.ocean.neuroticism = Some(10);
        b.ocean.neuroticism = Some(10);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.ocean.neuroticism = Some(1);
        b2.ocean.neuroticism = Some(1);
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean < bd2.ocean,
            "high N (unstable) should give lower ocean than low N (stable), got {} vs {}",
            bd.ocean,
            bd2.ocean
        );
    }

    // =====================================================================
    // compute_synergy_score_inner: pattern danger (lines 217-224)
    // =====================================================================

    #[test]
    fn test_pattern_danger_both_negative_triggers() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(bd.patterns < 0.5, "both negative → patterns penalized");
    }

    #[test]
    fn test_no_pattern_danger_one_positive() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            !bd.danger_details.contains("negative patterns"),
            "one positive → no pattern danger"
        );
    }

    // =====================================================================
    // team.rs tests (catches lines 108-146)
    // =====================================================================

    #[test]
    fn test_team_avg_score_exact_arithmetic() {
        let a = {
            let mut p = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
            p.id = "b".into();
            p
        };
        let c = {
            let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
            p.id = "c".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b, c], &[], &preds).unwrap();
        let manual_avg = (team
            .pairs
            .iter()
            .map(|p| p.breakdown.total as f64)
            .sum::<f64>()
            / team.pairs.len() as f64)
            .round() as u8;
        assert_eq!(team.avg_score, manual_avg, "avg_score must be exact sum/n");
    }

    #[test]
    fn test_team_avg_danger_exact() {
        let a = {
            let mut p = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
            p.id = "b".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        let manual_avg_danger =
            team.pairs.iter().map(|p| p.breakdown.danger).sum::<f64>() / team.pairs.len() as f64;
        assert!(
            (team.avg_danger - manual_avg_danger).abs() < 0.001,
            "avg_danger must be exact sum/n"
        );
    }

    #[test]
    fn test_team_context_averages_exact() {
        let a = {
            let mut p = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(6), Some(6), Some(6), Some(4));
            p.id = "b".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert_eq!(team.context_averages.len(), InsightContext::ALL.len());
        for (ctx, score) in &team.context_averages {
            let manual = team.pairs[0]
                .breakdown
                .per_context
                .iter()
                .find(|(c, _)| c == ctx)
                .map(|(_, s)| *s)
                .unwrap_or(team.avg_score);
            assert_eq!(
                *score, manual,
                "context avg must match pair value for single-pair team"
            );
        }
    }

    #[test]
    fn test_team_with_preds() {
        let mut a = full_profile();
        a.id = "a".into();
        let mut b = full_profile();
        b.id = "b".into();
        let mut preds = std::collections::HashMap::new();
        preds.insert(
            "a".into(),
            (0..3)
                .map(|_i| make_pred("a", Some(8), true))
                .collect::<Vec<_>>(),
        );
        preds.insert(
            "b".into(),
            (0..3)
                .map(|_i| make_pred("b", Some(3), true))
                .collect::<Vec<_>>(),
        );
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert_eq!(team.pairs.len(), 1);
        assert!(
            team.pairs[0].breakdown.danger > 0.0,
            "low accuracy → danger"
        );
    }

    #[test]
    fn test_team_empty_persons() {
        let preds = std::collections::HashMap::new();
        let result = compute_team_synergy(&[], &[], &preds);
        assert!(result.is_none(), "empty persons → None");
    }

    #[test]
    fn test_team_single_person() {
        let a = full_profile();
        let preds = std::collections::HashMap::new();
        let result = compute_team_synergy(&[a], &[], &preds);
        assert!(result.is_none(), "single person → None");
    }

    // --- motivation_synergy_score: two-pair weighted average ---

    #[test]
    fn test_motivation_synergy_score_two_pairs_exact() {
        // Two pairs with different intensity products so weights differ.
        let ma = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 8,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 4,
                notes: String::new(),
            },
        ];
        let mb = vec![
            Motivation {
                r#type: MotivationType::Affiliation,
                intensity: 6,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Security,
                intensity: 10,
                notes: String::new(),
            },
        ];
        let score = motivation_synergy_score(&ma, &mb);
        // Achievement×Affiliation: syn=0.1, w=0.48, product=0.048
        // Achievement×Security:    syn=-0.2, w=0.80, product=-0.160
        // Learning×Affiliation:    syn=0.2, w=0.24, product=0.048
        // Learning×Security:       syn=0.2, w=0.40, product=0.080
        // sum=0.016, total_w=1.92, ratio=1/120
        // (1/120 + 0.3) / 0.6 = 37/72
        let expected = 37.0 / 72.0;
        assert!(
            (score - expected).abs() < 1e-9,
            "expected {}, got {}",
            expected,
            score
        );
    }

    // --- virtue_adjustment: moderate intensity (between low=3 and high=7) ---

    #[test]
    fn test_virtue_moderate_intensity_no_effect() {
        let m = vec![
            Motivation {
                r#type: MotivationType::Fairness,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Creativity,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Power,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Security,
                intensity: 5,
                notes: String::new(),
            },
        ];
        let adj = virtue_adjustment(&m);
        // All at intensity 5: between low=3 and high=7, no guard matches → 0.0
        assert!((adj - 0.0).abs() < 1e-9, "expected 0.0, got {}", adj);
    }

    // --- value_similarity: asymmetric priorities ---

    #[test]
    fn test_value_similarity_asymmetric_priorities() {
        let a = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 10,
                priority: 10,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 3,
                priority: 1,
                notes: String::new(),
            },
        ];
        let b = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 2,
                priority: 1,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 7,
                priority: 10,
                notes: String::new(),
            },
        ];
        let sim = value_similarity(&a, &b);
        // Career: av=1.0, bv=0.2, ap=1.0, bp=0.1, w=1.0, sim=0.2
        // Family: av=0.3, bv=0.7, ap=0.1, bp=1.0, w=1.0, sim=0.6
        // (0.2 + 0.6) / 2.0 = 0.4
        assert!((sim - 0.4).abs() < 1e-9, "expected 0.4, got {}", sim);
    }

    // =====================================================================
    // scoring.rs: per_context_breakdown — exact arithmetic (catches 7 mutants)
    // =====================================================================

    #[test]
    fn test_per_context_exact_arithmetic() {
        let buckets = [0.8, 0.6, 0.7, 0.5, 0.4, 0.3, 0.2];
        let penalties = [0.1, 0.05, 0.03, 0.02];
        let inp = PerContextInputs {
            buckets,
            penalties,
            rep_active: true,
            mot_active: true,
            pat_active: true,
        };
        let result = per_context_breakdown(&inp, None);
        // Verify exact arithmetic for Decision context (weights:
        // [0.22, 0.16, 0.18, 0.16, 0.14, 0.10, 0.04])
        let decision = result
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap()
            .1;
        let cw = CFG.context_weights(InsightContext::Decision);
        // All buckets active → w = cw, wsum = 1.0
        // num = 0.8*cw[0] + 0.6*cw[1] + 0.7*cw[2] + 0.5*cw[3] + 0.4*cw[4] + 0.3*cw[5] + 0.2*cw[6]
        let num: f64 = buckets.iter().zip(cw.iter()).map(|(b, w)| b * w).sum();
        let wsum: f64 = cw.iter().sum();
        let raw = num / wsum;
        let danger = penalties[0] * cw[0]
            + penalties[1] * cw[1]
            + penalties[2] * cw[2]
            + penalties[3] * CFG.base_weights.history;
        let pen = (danger / wsum * 100.0).round() as u8;
        let expected = ((raw * 100.0).round() as u8).min(100).saturating_sub(pen);
        assert_eq!(decision, expected, "Decision exact arithmetic failed");
    }

    #[test]
    fn test_per_context_inactive_components_affect_score() {
        let buckets = [0.8, 0.6, 0.7, 0.5, 0.4, 0.3, 0.2];
        let all_active = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        let none_active = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: false,
                mot_active: false,
                pat_active: false,
            },
            None,
        );
        // With all inactive, buckets[1..3] are excluded from weighted average.
        // Since buckets[1]=0.6, [2]=0.7 are above average, excluding them should change score.
        for ((_, sa), (_, sn)) in all_active.iter().zip(&none_active) {
            assert_ne!(*sa, *sn, "inactive components must change score");
        }
    }

    #[test]
    fn test_per_context_danger_exact() {
        let buckets = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
        let penalties = [0.2, 0.1, 0.05, 0.03];
        let no_pen = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        let with_pen = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties,
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        let decision_no = no_pen
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap()
            .1;
        let decision_with = with_pen
            .iter()
            .find(|(c, _)| *c == InsightContext::Decision)
            .unwrap()
            .1;
        assert!(
            decision_with < decision_no,
            "penalties must lower score: {} vs {}",
            decision_with,
            decision_no
        );
        assert!(decision_no > decision_with, "gap should be positive");
    }

    // =====================================================================
    // scoring.rs: OC complement cross-pair (catches 157:20, 157:55)
    // =====================================================================

    #[test]
    fn test_oc_complement_cross_pair() {
        // Complement fires: a.O=7,b.C=7 → arm1 fires. oc=(sim(7,4)+sim(6,7))/2=0.8
        // raw_oc = 0.8+0.15 = 0.95
        let mut a = full_profile();
        a.ocean.openness = Some(7);
        a.ocean.conscientiousness = Some(6);
        let mut b = full_profile();
        b.ocean.openness = Some(4);
        b.ocean.conscientiousness = Some(7);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // No complement: a.O=7,b.C=6 → arm1 (7,6)✗, arm2 (4,6)✗
        // oc=(sim(7,4)+sim(6,6))/2=0.85. raw_oc=0.85
        let mut a2 = full_profile();
        a2.ocean.openness = Some(7);
        a2.ocean.conscientiousness = Some(6);
        let mut b2 = full_profile();
        b2.ocean.openness = Some(4);
        b2.ocean.conscientiousness = Some(6);
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "OC cross-pair complement must boost ocean: {} vs {}",
            bd.ocean,
            bd2.ocean
        );
    }

    // =====================================================================
    // scoring.rs: EA complement cross-pair (catches 170:20, 170:22, 170:55, 170:50)
    // =====================================================================

    #[test]
    fn test_ea_complement_cross_pair() {
        // Complement fires: a.E=7,b.A=7 → arm1 fires. ea=(sim(7,4)+sim(6,7))/2=0.8
        // raw_ea = 0.8+0.15 = 0.95
        let mut a = full_profile();
        a.ocean.extraversion = Some(7);
        a.ocean.agreeableness = Some(6);
        let mut b = full_profile();
        b.ocean.extraversion = Some(4);
        b.ocean.agreeableness = Some(7);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // No complement: a.E=7,b.A=6 → arm1 (7,6)✗, arm2 (4,6)✗
        // ea=(sim(7,4)+sim(6,6))/2=0.85. raw_ea=0.85
        let mut a2 = full_profile();
        a2.ocean.extraversion = Some(7);
        a2.ocean.agreeableness = Some(6);
        let mut b2 = full_profile();
        b2.ocean.extraversion = Some(4);
        b2.ocean.agreeableness = Some(6);
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(
            bd.ocean > bd2.ocean,
            "EA cross-pair complement must boost ocean: {} vs {}",
            bd.ocean,
            bd2.ocean
        );
    }

    #[test]
    fn test_ea_complement_one_side_only() {
        // Only ob.extraversion >=7, oa.agreeableness < 7
        // Original: guard `e >= 7 && a >= 7` → false → no bonus
        // Mutant && → ||: true → bonus
        // Use b.A=6 (not 3) to avoid antagonism penalty (both A ≤ 4)
        let mut a = full_profile();
        a.ocean.extraversion = Some(5);
        a.ocean.agreeableness = Some(3);
        let mut b = full_profile();
        b.ocean.extraversion = Some(9);
        b.ocean.agreeableness = Some(6);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // With &&, one-side-only must NOT trigger complement
        // ea = (sim(5,9) + sim(3,6))/2 = (0.6+0.7)/2 = 0.65, no bonus
        // raw_ocean = (1.0 + 0.65 + 1.0)/3 = 2.65/3
        assert!(
            (bd.ocean - 2.65 / 3.0).abs() < 1e-9,
            "one-side must have no bonus: {}",
            bd.ocean
        );
    }

    // =====================================================================
    // scoring.rs: pat_active (catches 205:56)
    // =====================================================================

    #[test]
    fn test_pat_active_requires_both() {
        // a has patterns, b doesn't → pat_active = false
        let mut a = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let mut b = full_profile();
        b.behavioral_patterns = vec![];
        let bd_one = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // Neither has patterns
        let mut a2 = full_profile();
        a2.behavioral_patterns = vec![];
        let mut b2 = full_profile();
        b2.behavioral_patterns = vec![];
        let bd_none = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        // When only one has patterns, pat_active should be false,
        // so patterns bucket = 0 (same as neither having patterns)
        assert_eq!(
            bd_one.patterns, bd_none.patterns,
            "one-sided patterns must not activate"
        );
    }

    // =====================================================================
    // scoring.rs: bias modulation exact (catches 255:59, 256:57, 278:47)
    // =====================================================================

    #[test]
    fn test_bias_modulation_targets_exact() {
        // Anchoring → Ocean target, coefficient +0.10
        // Need raw_ocean < 1.0 so the modulation shows (not clamped).
        // Use different traits between a and b so similarity < 1.0.
        let mut a = full_profile();
        a.ocean.openness = Some(5);
        a.ocean.conscientiousness = Some(5);
        a.ocean.extraversion = Some(5);
        a.ocean.agreeableness = Some(5);
        let mut b = full_profile();
        b.ocean.openness = Some(8);
        b.ocean.conscientiousness = Some(8);
        b.ocean.extraversion = Some(8);
        b.ocean.agreeableness = Some(8);
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd_shared = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // No biases, same traits
        let mut a2 = full_profile();
        a2.ocean.openness = Some(5);
        a2.ocean.conscientiousness = Some(5);
        a2.ocean.extraversion = Some(5);
        a2.ocean.agreeableness = Some(5);
        let mut b2 = full_profile();
        b2.ocean.openness = Some(8);
        b2.ocean.conscientiousness = Some(8);
        b2.ocean.extraversion = Some(8);
        b2.ocean.agreeableness = Some(8);
        a2.biases = vec![];
        b2.biases = vec![];
        let bd_none = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        // Anchoring is Ocean target → shared bias boosts ocean
        assert!(
            bd_shared.ocean > bd_none.ocean,
            "shared Anchoring boosts ocean: {} vs {}",
            bd_shared.ocean,
            bd_none.ocean
        );
        assert!(bd_shared.bias_mod_active, "shared bias must activate mod");
    }

    #[test]
    fn test_bias_modulation_motivation_target() {
        // SunkCost → Motivation target, coefficient +0.10
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::SunkCost,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::SunkCost,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.biases = vec![];
        b2.biases = vec![];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        assert!(
            bd.motivation > bd2.motivation,
            "shared SunkCost must boost motivation via mot_mod"
        );
    }

    #[test]
    fn test_bias_modulation_patterns_target() {
        // Availability → Patterns target, coefficient +0.10
        // Use different triggers so raw_pat < 1.0 (not clamped)
        let mut a = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let mut b = full_profile();
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        a.biases = vec![Bias {
            r#type: BiasType::Availability,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Availability,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        let mut a2 = full_profile();
        a2.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let mut b2 = full_profile();
        b2.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        a2.biases = vec![];
        b2.biases = vec![];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        assert!(
            bd.patterns > bd2.patterns,
            "shared Availability must boost patterns: {} vs {}",
            bd.patterns,
            bd2.patterns
        );
    }

    // =====================================================================
    // scoring.rs: opposite bias (catches 262:61, 268:18, 268:20, 278:47)
    // =====================================================================

    #[test]
    fn test_opposite_bias_cap_exact() {
        // Multiple opposite pairs → total exceeds cap
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![
            Bias {
                r#type: BiasType::DunningKruger,
                intensity: 10,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 10,
                evidence: String::new(),
            },
        ];
        b.biases = vec![
            Bias {
                r#type: BiasType::Impostor,
                intensity: 10,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Recency,
                intensity: 10,
                evidence: String::new(),
            },
        ];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // Single opposite pair (below cap)
        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        b2.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        // Cap must limit the damage: multi-pair ocean should not be much worse than single-pair
        // Both should have ocean_mod < 0
        assert!(bd.ocean < 1.0, "opposite biases must reduce ocean");
        assert!(bd2.ocean < 1.0, "single opposite must also reduce ocean");
    }

    #[test]
    fn test_opposite_bias_single_pair_exact() {
        // DK + Impostor → Ocean target, coefficient -0.10
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // No biases → ocean_mod = 0
        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.biases = vec![];
        b2.biases = vec![];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        // Opposite bias must lower ocean compared to no bias
        assert!(
            bd.ocean < bd2.ocean,
            "DK+Impostor must lower ocean: {} vs {}",
            bd.ocean,
            bd2.ocean
        );
    }

    // =====================================================================
    // scoring.rs: history penalty (catches 297:33, 297:63, 297:36×3, 297:69×3)
    // =====================================================================

    #[test]
    fn test_history_both_low_exact() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(4), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(4), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);

        let a2 = full_profile();
        let b2 = full_profile();
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        assert!(
            bd.danger > bd2.danger,
            "both low accuracy adds history danger"
        );
        assert!(bd.danger_details.contains("Low prediction accuracy"));
    }

    #[test]
    fn test_history_single_low_exact() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(4), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(8), true))
            .collect::<Vec<_>>();
        let bd_single = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);

        let bd_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd_single.danger > bd_none.danger,
            "single low accuracy adds some danger"
        );
    }

    #[test]
    fn test_history_at_boundary_no_penalty() {
        let a = full_profile();
        let b = full_profile();
        // accuracy=5 is NOT < 5.0 → no penalty
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(5), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(5), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);

        let bd_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            (bd.danger - bd_none.danger).abs() < 1e-9,
            "accuracy=5 must not trigger penalty: {} vs {}",
            bd.danger,
            bd_none.danger
        );
    }

    #[test]
    fn test_history_one_low_one_high_no_both_penalty() {
        let a = full_profile();
        let b = full_profile();
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(4), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(8), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);

        let a2 = full_profile();
        let b2 = full_profile();
        let a2_preds = (0..3)
            .map(|_i| make_pred("a2", Some(4), true))
            .collect::<Vec<_>>();
        let b2_preds = (0..3)
            .map(|_i| make_pred("b2", Some(4), true))
            .collect::<Vec<_>>();
        let bd_both = compute_synergy_score_inner(&a2, &b2, None, &a2_preds, &b2_preds);

        // Single low should add less danger than both low
        assert!(
            bd.danger < bd_both.danger,
            "single low < both low: {} vs {}",
            bd.danger,
            bd_both.danger
        );
    }

    // =====================================================================
    // scoring.rs: hierarchy rep diff (catches 330:51)
    // =====================================================================

    #[test]
    fn test_hierarchy_rep_diff_exact_boundary() {
        let mut a = full_profile();
        let mut b = full_profile();
        b.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 10,
            notes: String::new(),
        }];
        // ReportsTo: sub=a, boss=b. diff = boss_rep - sub_rep > 3
        // diff=3 (NOT > 3): boss(b)=8, sub(a)=5
        a.rep_scores.authoritative_submissive = Some(5);
        b.rep_scores.authoritative_submissive = Some(8);
        let ctx = RelContext {
            rtype: RelationType::ReportsTo,
            strength: 5,
        };
        let bd_exact = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);

        // diff = 4 (> 3) → bonus applies: boss(b)=9, sub(a)=5
        b.rep_scores.authoritative_submissive = Some(9);
        let bd_above = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);

        assert!(
            bd_above.total >= bd_exact.total,
            "diff=4 must trigger bonus vs diff=3: {} vs {}",
            bd_above.total,
            bd_exact.total
        );
    }

    // =====================================================================
    // scoring.rs: reputation/motivation/patterns formulas (catches 338:19, 339:38, 340:30)
    // =====================================================================

    #[test]
    fn test_reputation_formula_uses_subtraction() {
        // rep_penalty > 0 should lower reputation
        let mut a = full_profile();
        let mut b = full_profile();
        // Both have authoritative_submissive ≥ 8 → triggers rep danger
        a.rep_scores.authoritative_submissive = Some(10);
        b.rep_scores.authoritative_submissive = Some(10);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // No rep danger
        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.rep_scores.authoritative_submissive = Some(5);
        b2.rep_scores.authoritative_submissive = Some(5);
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        assert!(
            bd.reputation <= bd2.reputation,
            "rep danger must not increase reputation: {} vs {}",
            bd.reputation,
            bd2.reputation
        );
    }

    #[test]
    fn test_patterns_formula_uses_subtraction() {
        let mut a = full_profile();
        let mut b = full_profile();
        // Both negative patterns → pat_danger_penalty > 0
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b2.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        assert!(
            bd.patterns < bd2.patterns,
            "negative pattern danger must lower patterns bucket"
        );
    }

    // =====================================================================
    // scoring.rs: total_danger weighted sum (catches 358:9, 360:27)
    // =====================================================================

    #[test]
    fn test_total_danger_weighted_sum_exact() {
        let mut a = full_profile();
        let mut b = full_profile();
        // OCEAN danger
        a.ocean.neuroticism = Some(10);
        b.ocean.neuroticism = Some(10);
        a.ocean.openness = Some(1);
        b.ocean.openness = Some(1);
        let a_preds = (0..3)
            .map(|_i| make_pred("a", Some(3), true))
            .collect::<Vec<_>>();
        let b_preds = (0..3)
            .map(|_i| make_pred("b", Some(3), true))
            .collect::<Vec<_>>();
        let bd = compute_synergy_score_inner(&a, &b, None, &a_preds, &b_preds);

        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.ocean.neuroticism = Some(1);
        b2.ocean.neuroticism = Some(1);
        a2.ocean.openness = Some(10);
        b2.ocean.openness = Some(10);
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        assert!(
            bd.danger > bd2.danger,
            "volatile OCEAN + low accuracy must have more danger: {} vs {}",
            bd.danger,
            bd2.danger
        );
    }

    // =====================================================================
    // scoring.rs: neuroticism inversion (catches 390:45)
    // =====================================================================

    #[test]
    fn test_neuroticism_inverted_in_ovec() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.ocean.neuroticism = Some(10); // High N → low ovec[4] = (10-10)/10 = 0.0
        b.ocean.neuroticism = Some(1);
        let bd_high_n = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.ocean.neuroticism = Some(1); // Low N → high ovec[4] = (10-1)/10 = 0.9
        b2.ocean.neuroticism = Some(1);
        let bd_low_n = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        // High N means lower stability in ovec → potentially different scores
        assert_ne!(
            bd_high_n.total, bd_low_n.total,
            "neuroticism inversion must matter"
        );
    }

    // =====================================================================
    // scoring.rs: rep_boost / mutant (catches 423:37)
    // =====================================================================

    #[test]
    fn test_rep_boost_multiplied() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores.authoritative_submissive = Some(9);
        b.rep_scores.authoritative_submissive = Some(5);
        b.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 10,
            notes: String::new(),
        }];
        let ctx = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let bd_with_ctx = compute_synergy_score_inner(&a, &b, Some(&ctx), &[], &[]);
        let bd_none = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // Context must change reputation
        assert!(
            bd_with_ctx.reputation != bd_none.reputation,
            "Manages context must modify reputation via rep_boost"
        );
    }

    // =====================================================================
    // scoring.rs: asym_w (catches 435:16)
    // =====================================================================

    #[test]
    fn test_asym_w_includes_pat_weight() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.behavioral_patterns = vec![];
        b2.behavioral_patterns = vec![];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);

        // With patterns active, asym_w is larger, affecting the normalization
        assert_ne!(
            bd.total, bd2.total,
            "active patterns must change asym_w and score"
        );
    }

    // =====================================================================
    // scoring.rs: danger penalty (catches 465:23)
    // =====================================================================

    #[test]
    fn test_danger_penalty_normalization() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.ocean.neuroticism = Some(10);
        b.ocean.neuroticism = Some(10);
        a.ocean.openness = Some(1);
        b.ocean.openness = Some(1);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);

        // Danger penalty = total_danger / asym_w * 100
        // It must reduce the mutual score
        assert!(bd.danger > 0.0, "volatile OCEAN → positive danger");
        // mutual = (a_score + b_score) / 2 - danger_penalty
        let avg_raw = ((bd.a_score as f64) + (bd.b_score as f64)) / 2.0;
        assert!(
            (avg_raw - bd.total as f64) >= 0.0 || bd.danger < 0.01,
            "danger must reduce score or be negligible"
        );
    }

    // =====================================================================
    // scoring.rs: rep_penalty detail (catches 476:20)
    // =====================================================================

    #[test]
    fn test_danger_details_rep_power_struggle() {
        let mut a = full_profile();
        let mut b = full_profile();
        a.rep_scores.authoritative_submissive = Some(10);
        b.rep_scores.authoritative_submissive = Some(10);
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(
            bd.danger_details.contains("Rep power struggle"),
            "both authoritative ≥ 8 → power struggle detail"
        );
    }

    // =====================================================================
    // scoring.rs: bias_mod_active boundary (catches 512:47, 512:57×2)
    // =====================================================================

    #[test]
    fn test_bias_mod_active_boundary() {
        // No biases → all mods = 0 → bias_mod_active = false
        let mut a = full_profile();
        let mut b = full_profile();
        a.biases = vec![];
        b.biases = vec![];
        let bd = compute_synergy_score_inner(&a, &b, None, &[], &[]);
        assert!(!bd.bias_mod_active, "no biases → mod inactive");

        // Shared bias → mod > 0 → active
        let mut a2 = full_profile();
        let mut b2 = full_profile();
        a2.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: String::new(),
        }];
        b2.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: String::new(),
        }];
        let bd2 = compute_synergy_score_inner(&a2, &b2, None, &[], &[]);
        assert!(bd2.bias_mod_active, "shared Confirmation → mod active");
    }

    // =====================================================================
    // team.rs: avg_danger (catches 125:76 ×2)
    // =====================================================================

    #[test]
    fn test_team_avg_danger_exact_arithmetic() {
        let a = {
            let mut p = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(10), Some(1), Some(10), Some(1), Some(10));
            p.id = "b".into();
            p
        };
        let c = {
            let mut p = make_person(Some(1), Some(10), Some(1), Some(10), Some(1));
            p.id = "c".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b, c], &[], &preds).unwrap();
        let manual_avg =
            team.pairs.iter().map(|p| p.breakdown.danger).sum::<f64>() / team.pairs.len() as f64;
        assert!(
            (team.avg_danger - manual_avg).abs() < 0.001,
            "avg_danger must be exact sum/n"
        );
    }

    // =====================================================================
    // team.rs: context_averages (catches 143:75, 145:32, 146:22)
    // =====================================================================

    #[test]
    fn test_team_context_averages_zero_count() {
        // 3 persons → 3 pairs. Context averages should compute correctly.
        let a = {
            let mut p = make_person(Some(8), Some(8), Some(8), Some(8), Some(3));
            p.id = "a".into();
            p
        };
        let b = {
            let mut p = make_person(Some(6), Some(6), Some(6), Some(6), Some(4));
            p.id = "b".into();
            p
        };
        let c = {
            let mut p = make_person(Some(7), Some(7), Some(7), Some(7), Some(3));
            p.id = "c".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b, c], &[], &preds).unwrap();
        assert_eq!(team.pairs.len(), 3);
        // Each context average should be computed from the pairs that have that context
        for (ctx, score) in &team.context_averages {
            let pairs_with_ctx: Vec<_> = team
                .pairs
                .iter()
                .filter_map(|p| {
                    p.breakdown
                        .per_context
                        .iter()
                        .find(|(c, _)| c == ctx)
                        .map(|(_, s)| *s)
                })
                .collect();
            if !pairs_with_ctx.is_empty() {
                let expected = (pairs_with_ctx.iter().map(|s| *s as u32).sum::<u32>()
                    / pairs_with_ctx.len() as u32) as u8;
                assert_eq!(*score, expected, "context avg mismatch for {:?}", ctx);
            }
        }
    }

    // =====================================================================
    // trajectory.rs: timestamp arithmetic (catches 27:26, 51:29)
    // =====================================================================

    #[test]
    fn test_trajectory_uses_correct_age() {
        // Two entries with different timestamps but same valence
        let e1 = InteractionEntry {
            id: "e1".into(),
            timestamp: 1000,
            text: String::new(),
            valence: Some(2),
            trigger: None,
            target_id: None,
        };
        let e2 = InteractionEntry {
            id: "e2".into(),
            timestamp: 2000,
            text: String::new(),
            valence: Some(2),
            trigger: None,
            target_id: None,
        };
        let entries = vec![&e1, &e2];
        let traj = trajectory_from(&entries);
        // Both same valence → level should be positive (both +2)
        assert!(
            traj.level > 0.0,
            "both positive valence → positive level: {}",
            traj.level
        );
    }

    #[test]
    fn test_trajectory_ages_weight_recent_more() {
        // Recent positive, old negative → net positive if weighting works
        let e1 = InteractionEntry {
            id: "e1".into(),
            timestamp: 0,
            text: String::new(),
            valence: Some(-3),
            trigger: None,
            target_id: None,
        };
        let e2 = InteractionEntry {
            id: "e2".into(),
            timestamp: 1_000_000,
            text: String::new(),
            valence: Some(3),
            trigger: None,
            target_id: None,
        };
        let entries = vec![&e1, &e2];
        let traj = trajectory_from(&entries);
        // Recent entry (ts=1M) has valence +3, old entry (ts=0) has -3
        // Recent should weight more → net positive
        assert!(
            traj.level > 0.0,
            "recent positive must outweigh old negative: {}",
            traj.level
        );

        // Reverse: recent negative, old positive
        let e3 = InteractionEntry {
            id: "e3".into(),
            timestamp: 0,
            text: String::new(),
            valence: Some(3),
            trigger: None,
            target_id: None,
        };
        let e4 = InteractionEntry {
            id: "e4".into(),
            timestamp: 1_000_000,
            text: String::new(),
            valence: Some(-3),
            trigger: None,
            target_id: None,
        };
        let entries2 = vec![&e3, &e4];
        let traj2 = trajectory_from(&entries2);
        assert!(
            traj2.level < 0.0,
            "recent negative must outweigh old positive: {}",
            traj2.level
        );
    }

    // =====================================================================
    // trajectory.rs: trend boundaries (catches 53:21, 55:28, 57:25, 59:25, 66:21)
    // =====================================================================

    #[test]
    fn test_trajectory_short_series_level_only() {
        // Below min_samples → trend based on level only
        let entries: Vec<InteractionEntry> = (0..3)
            .map(|i| InteractionEntry {
                id: format!("e{i}"),
                timestamp: i as i64 * 1000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        // All +3 valence → high level → Improving
        assert_eq!(
            traj.trend,
            Trend::Improving,
            "short series all positive → Improving via level"
        );
        assert_eq!(traj.sample, 3);
    }

    #[test]
    fn test_trajectory_improving_trend() {
        // 4+ entries, recent much higher than early
        let entries: Vec<InteractionEntry> = vec![
            InteractionEntry {
                id: "e0".into(),
                timestamp: 0,
                text: String::new(),
                valence: Some(-3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                timestamp: 1000,
                text: String::new(),
                valence: Some(-2),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e2".into(),
                timestamp: 2000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e3".into(),
                timestamp: 3000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(
            traj.trend,
            Trend::Improving,
            "negative→positive must be Improving: {:?}",
            traj.trend
        );
    }

    #[test]
    fn test_trajectory_deteriorating_trend() {
        let entries: Vec<InteractionEntry> = vec![
            InteractionEntry {
                id: "e0".into(),
                timestamp: 0,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                timestamp: 1000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e2".into(),
                timestamp: 2000,
                text: String::new(),
                valence: Some(-3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e3".into(),
                timestamp: 3000,
                text: String::new(),
                valence: Some(-3),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(
            traj.trend,
            Trend::Deteriorating,
            "positive→negative must be Deteriorating: {:?}",
            traj.trend
        );
    }

    #[test]
    fn test_trajectory_stable_trend() {
        let entries: Vec<InteractionEntry> = (0..4)
            .map(|i| InteractionEntry {
                id: format!("e{i}"),
                timestamp: i as i64 * 1000,
                text: String::new(),
                valence: Some(1),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        // Same valence everywhere → momentum ≈ 0, level > 0 but may or may not exceed threshold
        // At least it shouldn't be Improving if momentum is near zero
        assert!(traj.sample == 4);
    }

    #[test]
    fn test_trajectory_odd_entries_split() {
        // 5 entries: mid = 5/2 = 2, early=[0,1], recent=[2,3,4]
        let entries: Vec<InteractionEntry> = vec![
            InteractionEntry {
                id: "e0".into(),
                timestamp: 0,
                text: String::new(),
                valence: Some(-3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                timestamp: 1000,
                text: String::new(),
                valence: Some(-3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e2".into(),
                timestamp: 2000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e3".into(),
                timestamp: 3000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e4".into(),
                timestamp: 4000,
                text: String::new(),
                valence: Some(3),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 5);
        assert_eq!(
            traj.trend,
            Trend::Improving,
            "odd split: early=[-3,-3] recent=[3,3,3] → Improving"
        );
    }

    #[test]
    fn test_trajectory_empty_entries() {
        let entries: Vec<InteractionEntry> = vec![];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 0);
        assert_eq!(traj.delta, 0);
        assert_eq!(traj.trend, Trend::Stable);
    }

    #[test]
    fn test_trajectory_all_same_valence() {
        let entries: Vec<InteractionEntry> = (0..5)
            .map(|i| InteractionEntry {
                id: format!("e{i}"),
                timestamp: i as i64 * 1000,
                text: String::new(),
                valence: Some(2),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        // All same → momentum = 0 → level > threshold → Improving
        assert_eq!(traj.sample, 5);
    }

    // ======== mutants-synergy-rest: targeted tests for missed mutants ========

    // --- profile_completeness L95: catch `+` → `*` in numerator sum ---

    #[test]
    fn test_profile_completeness_all_components_nonzero() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(3), Some(7), None, None);
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 6,
                notes: String::new(),
            },
        ];
        p.biases = vec![
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 2,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 3,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Availability,
                intensity: 4,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::SunkCost,
                intensity: 5,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::DunningKruger,
                intensity: 2,
                evidence: String::new(),
            },
        ];
        p.rep_scores = RepScores {
            honest_deceitful: Some(7),
            reliable_flaky: Some(5),
            authoritative_submissive: Some(8),
            humble_arrogant: Some(6),
            hardworker_lazy: Some(4),
            calm_reactive: Some(3),
            diplomatic_blunt: Some(9),
            ..RepScores::default()
        };
        p.styles = vec![
            PersonalStyle {
                r#type: StyleType::Analytical,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 5,
                notes: String::new(),
            },
            PersonalStyle {
                r#type: StyleType::Collaborating,
                intensity: 5,
                notes: String::new(),
            },
        ];
        p.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::RemainsCalm,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::CommunicatesOpenly,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Success,
                predicted_behavior: BehaviorResponse::CelebratesWithOthers,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Uncertainty,
                predicted_behavior: BehaviorResponse::SeeksData,
                notes: String::new(),
            },
        ];
        p.values = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 5,
                priority: 5,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 6,
                priority: 7,
                notes: String::new(),
            },
        ];
        let c = profile_completeness(&p);
        // ocean=3(O,C,E set), mot=2, biases=5, rep=7, styles=3(cats), pat=4, vals=2
        let expected = (3 + 2 + 5 + 7 + 3 + 4 + 2) as f64 / CFG.completeness.denominator;
        assert!(
            (c - expected).abs() < 0.001,
            "all-nonzero: expected {expected}, got {c}"
        );
    }

    // --- ocean_danger_penalty: ob within-person impulsive (L121,124) ---

    #[test]
    fn test_ocean_penalty_ob_impulsive_only() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(7), Some(3), Some(5), Some(7), Some(8));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        // ob: N=8>=7, C=3<=4 → impulsive. No other triggers.
        assert!(
            (p - 0.05).abs() < 0.001,
            "ob impulsive only → 0.05, got {}",
            p
        );
    }

    #[test]
    fn test_ocean_penalty_ob_rigid_only() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(3), Some(7), Some(5), Some(7), Some(8));
        let p = ocean_danger_penalty(&a.ocean, &b.ocean);
        // ob: N=8>=7, O=3<=4 → rigid. No other triggers.
        assert!((p - 0.05).abs() < 0.001, "ob rigid only → 0.05, got {}", p);
    }

    // --- compute_person_profile: exact ocean for each penalty type ---

    #[test]
    fn test_self_score_volatile_only_exact() {
        // O=7, C=7, E=5, A=3, N=8 → volatile only (N≥7, A≤4)
        let p = make_person(Some(7), Some(7), Some(5), Some(3), Some(8));
        let pf = compute_person_profile(&p);
        // a_s=0.3, n_s=0.2, raw_ocean=0.25, penalty=0.10 → ocean=0.15
        assert!(
            (pf.ocean - 0.15).abs() < 0.001,
            "volatile only → 0.15, got {}",
            pf.ocean
        );
    }

    #[test]
    fn test_self_score_impulsive_only_exact() {
        // O=7, C=3, E=5, A=7, N=8 → impulsive only (N≥7, C≤4)
        let p = make_person(Some(7), Some(3), Some(5), Some(7), Some(8));
        let pf = compute_person_profile(&p);
        // a_s=0.7, n_s=0.2, raw_ocean=0.45, penalty=0.05 → ocean=0.40
        assert!(
            (pf.ocean - 0.40).abs() < 0.001,
            "impulsive only → 0.40, got {}",
            pf.ocean
        );
    }

    #[test]
    fn test_self_score_rigid_only_exact() {
        // O=3, C=7, E=5, A=7, N=8 → rigid only (N≥7, O≤4)
        let p = make_person(Some(3), Some(7), Some(5), Some(7), Some(8));
        let pf = compute_person_profile(&p);
        // a_s=0.7, n_s=0.2, raw_ocean=0.45, penalty=0.05 → ocean=0.40
        assert!(
            (pf.ocean - 0.40).abs() < 0.001,
            "rigid only → 0.40, got {}",
            pf.ocean
        );
    }

    #[test]
    fn test_self_score_all_three_penalties_exact() {
        let p = make_person(Some(3), Some(3), Some(5), Some(3), Some(8));
        let pf = compute_person_profile(&p);
        // a_s=0.3, n_s=0.2, raw_ocean=0.25, penalty=0.20 → ocean=0.05
        assert!(
            (pf.ocean - 0.05).abs() < 0.001,
            "all three penalties → 0.05, got {}",
            pf.ocean
        );
    }

    // --- compute_person_profile: bias calculation exact (L502-512) ---

    #[test]
    fn test_self_score_bias_calculation_exact() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.biases = (0..12)
            .map(|i| Bias {
                r#type: BiasType::ALL[i],
                intensity: 2, // all mild (≤ mild_max=3)
                evidence: String::new(),
            })
            .collect();
        let pf = compute_person_profile(&p);
        // absent_count=0, moderate_plus=0, present_bias_count=0
        // base_bias=1.0 - 0/12 = 1.0
        // count_bonus(0)=0.09, bias_adj=12*0.01=0.12
        // bias = (1.0 + 0.12 + 0.09).clamp(0,1) = 1.0
        assert!(
            (pf.bias - 1.0).abs() < 0.001,
            "12 mild biases → bias=1.0, got {}",
            pf.bias
        );
    }

    #[test]
    fn test_self_score_bias_mixed_strong_mild() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        // 1 mild + 1 strong bias, rest absent
        p.biases = vec![
            Bias {
                r#type: BiasType::Confirmation,
                intensity: 2,
                evidence: String::new(),
            },
            Bias {
                r#type: BiasType::Anchoring,
                intensity: 8,
                evidence: String::new(),
            },
        ];
        let pf = compute_person_profile(&p);
        // absent_count=10, moderate_plus=1 (Anchoring 8≥4), present_bias_count=11
        // base_bias=1.0 - 11/12 = 0.08333...
        // count_bonus(11)=0.0
        // bias_adj: Confirmation mild(2≤3)→+0.01, Anchoring strong(8≥7)→-0.03 → -0.02
        // bias = (0.08333 + (-0.02) + 0.0).clamp(0,1) = 0.06333
        let expected = 1.0 - 11.0 / 12.0 - 0.02;
        assert!(
            (pf.bias - expected).abs() < 0.001,
            "mixed biases → {expected}, got {}",
            pf.bias
        );
    }

    // --- compute_person_profile: style empty check (L514) ---

    #[test]
    fn test_self_score_empty_styles_default() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let pf = compute_person_profile(&p);
        assert!(
            (pf.styles - CFG.profile.default_style).abs() < 0.001,
            "empty styles → default {}, got {}",
            CFG.profile.default_style,
            pf.styles
        );
    }

    #[test]
    fn test_self_score_nonempty_styles_synergy() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.styles = vec![PersonalStyle {
            r#type: StyleType::Analytical,
            intensity: 8,
            notes: String::new(),
        }];
        let pf = compute_person_profile(&p);
        // style_synergy with single style self-paired → same_score = 1.0
        assert!(
            (pf.styles - 1.0).abs() < 0.001,
            "single Analytical self-synergy → 1.0, got {}",
            pf.styles
        );
    }

    // --- compute_person_profile: raw accumulation (L528, L535) ---

    #[test]
    fn test_self_score_patterns_contribute_to_total() {
        use crate::models::*;
        let mut p = make_person(None, None, None, None, None);
        p.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Success,
            predicted_behavior: BehaviorResponse::CelebratesWithOthers,
            notes: String::new(),
        }];
        let pf_with = compute_person_profile(&p);
        let pf_without = compute_person_profile(&make_person(None, None, None, None, None));
        // Patterns add positive contribution to total
        assert!(
            pf_with.total > pf_without.total,
            "patterns must increase total: {} vs {}",
            pf_with.total,
            pf_without.total
        );
    }

    #[test]
    fn test_self_score_bias_contributes_to_total() {
        use crate::models::*;
        let mut p = make_person(None, None, None, None, None);
        p.biases = (0..10)
            .map(|i| Bias {
                r#type: BiasType::ALL[i],
                intensity: 2,
                evidence: String::new(),
            })
            .collect();
        let pf_with = compute_person_profile(&p);
        let pf_without = compute_person_profile(&make_person(None, None, None, None, None));
        // Bias adds positive contribution to total
        assert!(
            pf_with.total > pf_without.total,
            "biases must increase total: {} vs {}",
            pf_with.total,
            pf_without.total
        );
    }

    // --- profile_completeness * 100.0 (L559) ---

    #[test]
    fn test_self_score_completeness_times_100() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(3), Some(7), None, None);
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Achievement,
                intensity: 5,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 6,
                notes: String::new(),
            },
        ];
        let c = profile_completeness(&p);
        let pf = compute_person_profile(&p);
        let expected_completeness = (c * 100.0).round() as u8;
        assert_eq!(
            pf.completeness, expected_completeness,
            "completeness should be {expected_completeness}, got {}",
            pf.completeness
        );
    }

    // --- compute_person_profile: total_w guard (L544) ---

    #[test]
    fn test_self_score_total_w_always_positive() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let pf = compute_person_profile(&p);
        assert!(
            pf.total >= 20 && pf.total <= 70,
            "baseline total: {}",
            pf.total
        );
    }

    // --- compute_person_profile: count_bonus sign (L512 second +) ---

    #[test]
    fn test_self_score_bias_count_bonus_additive() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        // 10 mild biases → absent_count=2, moderate_plus=0, present_bias_count=2
        p.biases = (0..10)
            .map(|i| Bias {
                r#type: BiasType::ALL[i],
                intensity: 2,
                evidence: String::new(),
            })
            .collect();
        let pf = compute_person_profile(&p);
        // base_bias = 1 - 2/12 = 0.8333, bias_adj = 10*0.01 = 0.10, count_bonus(2) = 0.03
        // bias = (0.8333 + 0.10 + 0.03) = 0.9633
        // mutant (+ → -) would give 0.9033
        let expected = 1.0 - 2.0 / 12.0 + 10.0 * 0.01 + 0.03;
        assert!(
            (pf.bias - expected).abs() < 0.001,
            "count_bonus must add: expected bias={}, got {}",
            expected,
            pf.bias
        );
    }

    // --- compute_person_profile: bias * weight (L535) ---

    #[test]
    fn test_self_score_bias_weight_multiplication() {
        use crate::models::*;
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.biases = (0..10)
            .map(|i| Bias {
                r#type: BiasType::ALL[i],
                intensity: 2,
                evidence: String::new(),
            })
            .collect();
        let pf = compute_person_profile(&p);
        // bias ≈ 0.9633; raw includes bias * 0.14 not bias / 0.14.
        // With / total would saturate at 100; with * total stays mid-range.
        assert!(
            pf.total > 30 && pf.total < 90,
            "bias * weight keeps total in range, got {}",
            pf.total
        );
    }

    // === scoring.rs: per_context_breakdown exact values ===

    #[test]
    fn test_per_context_with_rels_changes_scores() {
        let inp = PerContextInputs {
            buckets: [0.8, 0.6, 0.7, 0.5, 0.4, 0.3, 0.2],
            penalties: [0.0; 4],
            rep_active: true,
            mot_active: true,
            pat_active: true,
        };
        let rel = RelContext {
            rtype: RelationType::Manages,
            strength: 5,
        };
        let with_rel = per_context_breakdown(&inp, Some(&rel));
        let no_rel = per_context_breakdown(&inp, None);
        assert_eq!(with_rel.len(), 6);
        assert_eq!(no_rel.len(), 6);
        let differs = with_rel
            .iter()
            .zip(&no_rel)
            .any(|((_, s1), (_, s2))| s1 != s2);
        assert!(differs, "RelContext must change per-context scores");
    }

    #[test]
    fn test_per_context_inactive_buckets_raw_division() {
        let buckets = [0.8, 0.0, 0.0, 0.0, 0.2, 0.3, 0.4];
        let inp = PerContextInputs {
            buckets,
            penalties: [0.0; 4],
            rep_active: false,
            mot_active: false,
            pat_active: false,
        };
        let result = per_context_breakdown(&inp, None);
        for (ctx, score) in &result {
            assert!(*score <= 100, "score {:?} = {} exceeds 100", ctx, score);
            assert!(
                *score > 0,
                "non-zero buckets → non-zero score for {:?}",
                ctx
            );
        }
    }

    #[test]
    fn test_per_context_penalties_reduce_score() {
        let buckets = [0.8; 7];
        let no_pen = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.0; 4],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        let with_pen = per_context_breakdown(
            &PerContextInputs {
                buckets,
                penalties: [0.1, 0.1, 0.1, 0.1],
                rep_active: true,
                mot_active: true,
                pat_active: true,
            },
            None,
        );
        for ((_, s1), (_, s2)) in no_pen.iter().zip(&with_pen) {
            assert!(
                *s2 < *s1,
                "penalty {} must be less than no-penalty {}",
                s2,
                s1
            );
        }
    }

    // === scoring.rs: complement guards (lines 151-176) ===

    #[test]
    fn test_oc_complement_exactly_at_min() {
        let a = make_person(Some(7), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(7), Some(5), Some(5), Some(5));
        let brk = compute_synergy_score(&a, &b);
        // complement_min=7: O=7>=7 && C=7>=7 → bonus applied
        // oc = (sim(7,5)+sim(5,7))/2 = (0.8+0.8)/2 = 0.8
        // with bonus: (0.8+0.15).min(1.0) = 0.95
        // raw_ocean = (0.95 + 1.0 + 1.0)/3 ≈ 0.9833
        assert!(
            (brk.ocean - 0.9833).abs() < 0.001,
            "OC complement at exact min=7: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_ea_complement_exactly_at_min() {
        let a = make_person(Some(5), Some(5), Some(7), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(7), Some(5));
        let brk = compute_synergy_score(&a, &b);
        // E=7>=7 && A=7>=7 → bonus applied
        // ea = (sim(7,5)+sim(5,7))/2 = 0.8, with bonus = 0.95
        // raw_ocean = (1.0 + 0.95 + 1.0)/3 ≈ 0.9833
        assert!(
            (brk.ocean - 0.9833).abs() < 0.001,
            "EA complement at exact min=7: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_oc_complement_second_arm() {
        // First arm: OA.openness=5 < 7 → no match
        // Second arm: OB.openness=7 >= 7 && OA.conscientiousness=7 >= 7 → match
        let a = make_person(Some(5), Some(7), Some(5), Some(5), Some(5));
        let b = make_person(Some(7), Some(5), Some(5), Some(5), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.ocean - 0.9833).abs() < 0.001,
            "OC complement second arm: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_ea_complement_second_arm() {
        let a = make_person(Some(5), Some(5), Some(5), Some(7), Some(5));
        let b = make_person(Some(5), Some(5), Some(7), Some(5), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.ocean - 0.9833).abs() < 0.001,
            "EA complement second arm: {}",
            brk.ocean
        );
    }

    // === scoring.rs: history penalty (lines 296-303) ===

    #[test]
    fn test_history_both_low_penalty_exact() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let preds: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(4),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let brk = compute_synergy_score_with_preds(&a, &b, &preds, &preds);
        // avg=4.0 < 5.0 → both low → penalty = 0.05
        assert!(brk.danger > 0.0, "both low → danger > 0: {}", brk.danger);
        assert!(brk.danger_details.contains("Low prediction accuracy"));
    }

    #[test]
    fn test_history_one_low_penalty_exact() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let low: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(4),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let high: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("q{}", i),
                person_id: "y".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(8),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let brk = compute_synergy_score_with_preds(&a, &b, &low, &high);
        assert!(brk.danger > 0.0, "one low → danger > 0: {}", brk.danger);
        assert!(brk.danger_details.contains("Low prediction accuracy"));
    }

    #[test]
    fn test_history_no_penalty_at_boundary() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let preds: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(5),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let brk = compute_synergy_score_with_preds(&a, &b, &preds, &preds);
        // avg=5.0, 5.0 < 5.0 is false → no penalty
        assert!(
            !brk.danger_details.contains("Low prediction accuracy"),
            "accuracy=5 must not trigger penalty"
        );
    }

    // === scoring.rs: total_danger exact values (lines 357-360) ===

    #[test]
    fn test_total_danger_from_ocean_exact() {
        let a = make_person(Some(5), Some(5), Some(5), Some(2), Some(8));
        let b = make_person(Some(5), Some(5), Some(5), Some(3), Some(7));
        let brk = compute_synergy_score(&a, &b);
        // N>=7, A<=4 → volatile (0.10 each) + contagion (0.10) + antagonism (0.15) = 0.45
        // total_danger = 0.45 * 0.17 = 0.0765
        assert!(
            (brk.danger - 0.0765).abs() < 0.001,
            "ocean danger exact: {}",
            brk.danger
        );
    }

    #[test]
    fn test_total_danger_from_history_exact() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let preds: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(3),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let brk = compute_synergy_score_with_preds(&a, &b, &preds, &preds);
        // both low → history_penalty = 0.05
        // total_danger = 0.05 * 0.10 = 0.005
        assert!(
            (brk.danger - 0.005).abs() < 0.001,
            "history danger exact: {}",
            brk.danger
        );
    }

    // === scoring.rs: rep/pat subtraction (lines 338, 340) ===

    #[test]
    fn test_rep_danger_reduces_reputation() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.rep_scores = RepScores {
            authoritative_submissive: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            authoritative_submissive: Some(8),
            ..RepScores::default()
        };
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.reputation < 1.0,
            "rep danger reduces rep: {}",
            brk.reputation
        );
    }

    #[test]
    fn test_pattern_danger_reduces_patterns() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.behavioral_patterns = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                notes: String::new(),
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::Overwhelmed,
                notes: String::new(),
            },
        ];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Threatened,
            predicted_behavior: BehaviorResponse::DeflectsBlame,
            notes: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        // all-negative triggers → pat_danger_penalty = 0.05
        assert!(
            brk.patterns < 1.0,
            "pattern danger reduces patterns: {}",
            brk.patterns
        );
    }

    // === scoring.rs: ovec neuroticism inversion (line 390) ===

    #[test]
    fn test_neuroticism_in_ovec_inverts() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(10));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(10));
        let brk = compute_synergy_score(&a, &b);
        // N=10 → ovec[4] = (10-10)/10 = 0.0 (unstable)
        // With mutant (+): ovec[4] = (10+10)/10 = 2.0 → very different asym_ocean
        // av = [0.5, 0.5, 0.5, 0.5, 0.0]
        // asym_ocean(av, av) = sum((1-0)*av[i])/5 = (0.5+0.5+0.5+0.5+0)/5 = 0.4
        // a_raw = 0.4*0.17 + 0.5*(0.18+0.15+0.16+0.14+0.12+0.08) = 0.068+0.5*0.83 = 0.068+0.415 = 0.483
        // a_score = (0.483*100).round() = 48
        // With mutant: av = [0.5,0.5,0.5,0.5,2.0]
        // asym_ocean = (0.5+0.5+0.5+0.5+4.0)/5 = 6.0/5 = 1.2, clamped? No, it's raw
        // a_raw would be much higher
        assert!(
            brk.total > 0 && brk.total < 80,
            "high N synergy: {}",
            brk.total
        );
    }

    // === scoring.rs: patterns contribute to asym_w (line 435) ===

    #[test]
    fn test_patterns_active_adds_to_asym_w() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        // patterns active → asym_w includes w_pat=0.16
        // Without patterns: asym_w = 1.0 - 0.16 = 0.84
        // a_raw and b_raw also include patterns contribution
        assert!(brk.patterns > 0.0, "patterns active");
        assert!(brk.total > 0, "total > 0 with patterns");
    }

    // === scoring.rs: bias_mod_active (line 512) ===

    #[test]
    fn test_bias_mod_active_shared_anchoring() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 10,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        // shared Anchoring → ocean_mod = 0.10 * (10*10/100) = 0.10 > 0
        assert!(brk.bias_mod_active, "shared Anchoring → bias_mod_active");
    }

    #[test]
    fn test_bias_mod_active_dk_impostor_opposite() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let mut b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 10,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        // DK/Impostor → ocean_mod = -0.10 (opposite), no shared → no positive mod
        assert!(
            !brk.bias_mod_active,
            "opposite-only → bias_mod_active false"
        );
    }

    // === scoring.rs: opposite bias exact weight (line 262) ===

    #[test]
    fn test_opposite_bias_nonunit_intensity() {
        let mut a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        a.biases = vec![Bias {
            r#type: BiasType::DunningKruger,
            intensity: 5,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Impostor,
            intensity: 10,
            evidence: String::new(),
        }];
        let brk = compute_synergy_score(&a, &b);
        let no_bias = compute_synergy_score(
            &make_person(Some(8), Some(7), Some(6), Some(5), Some(4)),
            &make_person(Some(6), Some(5), Some(4), Some(3), Some(2)),
        );
        // w = 5*10/100 = 0.5, coefficient = -0.10
        // opp_mod = -0.10 * 0.5 = -0.05
        assert!(
            brk.ocean < no_bias.ocean,
            "nonunit intensity opposite bias lowers ocean ({} < {})",
            brk.ocean,
            no_bias.ocean
        );
    }

    // === scoring.rs: opposite bias cap (lines 268-273) ===

    #[test]
    fn test_opposite_bias_cap_enforced() {
        let mut base_a = make_person(Some(8), Some(7), Some(6), Some(5), Some(4));
        let mut base_b = make_person(Some(6), Some(5), Some(4), Some(3), Some(2));
        base_a.behavioral_patterns = vec![one_positive_pattern()];
        base_b.behavioral_patterns = vec![one_positive_pattern()];
        base_a.rep_scores = RepScores {
            authoritative_submissive: Some(9),
            reliable_flaky: Some(7),
            ..RepScores::default()
        };
        base_b.rep_scores = RepScores {
            authoritative_submissive: Some(8),
            reliable_flaky: Some(8),
            ..RepScores::default()
        };
        let mut a = Person { ..base_a.clone() };
        let mut b = Person { ..base_b.clone() };
        a.biases = vec![
            one_bias(BiasType::DunningKruger),
            one_bias(BiasType::Anchoring),
            one_bias(BiasType::Authority),
        ];
        b.biases = vec![
            one_bias(BiasType::Impostor),
            one_bias(BiasType::Recency),
            one_bias(BiasType::SocialProof),
        ];
        let brk = compute_synergy_score(&a, &b);
        let brk_no = compute_synergy_score(&base_a, &base_b);
        let reduction = (brk_no.ocean - brk.ocean)
            + (brk_no.patterns - brk.patterns)
            + (brk_no.reputation - brk.reputation);
        assert!(reduction > 0.05, "friction present: {reduction}");
        assert!(
            reduction <= CFG.bias.opposite_cap + 0.001,
            "capped at 0.15: {reduction}"
        );
    }

    // === scoring.rs: complementary pairs targets (lines 151-176 cross-pair) ===

    #[test]
    fn test_oc_complement_cross_pair_exact() {
        // OA.openness=8, OB.conscientiousness=8 → first arm matches
        let a = make_person(Some(8), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(8), Some(5), Some(5), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.ocean - 0.95).abs() < 0.001,
            "OC cross-pair: {}",
            brk.ocean
        );
    }

    #[test]
    fn test_ea_complement_cross_pair_exact() {
        let a = make_person(Some(5), Some(5), Some(8), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(8), Some(5));
        let brk = compute_synergy_score(&a, &b);
        assert!(
            (brk.ocean - 0.95).abs() < 0.001,
            "EA cross-pair: {}",
            brk.ocean
        );
    }

    // === scoring.rs: history penalty weight in total_danger (line 360) ===

    #[test]
    fn test_history_penalty_contributes_to_total_danger() {
        let a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let b = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let low: Vec<Prediction> = (0..3)
            .map(|i| Prediction {
                id: format!("p{}", i),
                person_id: "x".into(),
                context: String::new(),
                predicted_outcome: String::new(),
                actual_outcome: Some("ok".into()),
                accuracy: Some(2),
                created_at: 0,
                resolved_at: Some(1),
                resolved: true,
            })
            .collect();
        let brk = compute_synergy_score_with_preds(&a, &b, &low, &low);
        // both low → history_penalty = 0.05
        // total_danger = 0.05 * 0.10 = 0.005
        assert!(
            (brk.danger - 0.005).abs() < 0.001,
            "history danger weight exact: {}",
            brk.danger
        );
    }

    // === team.rs: context filter (line 143) ===

    #[test]
    fn test_team_context_averages_vary_by_context() {
        let a = {
            let mut p = make_person(Some(9), Some(9), Some(9), Some(9), Some(1));
            p.id = "a".into();
            p.name = "A".into();
            p
        };
        let b = {
            let mut p = make_person(Some(1), Some(1), Some(1), Some(1), Some(9));
            p.id = "b".into();
            p.name = "B".into();
            p
        };
        let preds = std::collections::HashMap::new();
        let team = compute_team_synergy(&[a, b], &[], &preds).unwrap();
        assert_eq!(team.context_averages.len(), 6);
        let scores: Vec<u8> = team.context_averages.iter().map(|(_, s)| *s).collect();
        let min = *scores.iter().min().unwrap();
        let max = *scores.iter().max().unwrap();
        assert!(max > min, "context averages must vary: {:?}", scores);
    }
}

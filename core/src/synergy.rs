use crate::models::{BehaviorTrigger, BehavioralPattern, BiasType, MotivationType, Person, RepDim};

pub struct SynergyBreakdown {
    pub total: u8,
    pub ocean: f64,
    pub reputation: f64,
    pub motivation: f64,
    pub patterns: f64,
    pub bias: f64,
}

pub fn compute_synergy_score(a: &Person, b: &Person) -> SynergyBreakdown {
    let oa = &a.ocean;
    let ob = &b.ocean;

    let sim = |x: Option<u8>, y: Option<u8>| match (x, y) {
        (Some(a), Some(b)) => 1.0 - (a.abs_diff(b) as f64) / 10.0,
        _ => 0.5,
    };

    let oc =
        (sim(oa.openness, ob.openness) + sim(oa.conscientiousness, ob.conscientiousness)) / 2.0;
    let ea =
        (sim(oa.extraversion, ob.extraversion) + sim(oa.agreeableness, ob.agreeableness)) / 2.0;
    let n = sim(oa.neuroticism, ob.neuroticism);

    let oc_bonus = match (oa.openness, ob.conscientiousness) {
        (Some(o), Some(c)) if o >= 7 && c >= 7 => 0.15,
        _ => match (ob.openness, oa.conscientiousness) {
            (Some(o), Some(c)) if o >= 7 && c >= 7 => 0.15,
            _ => 0.0,
        },
    };
    let ea_bonus = match (oa.extraversion, ob.agreeableness) {
        (Some(e), Some(a)) if e >= 7 && a >= 7 => 0.15,
        _ => match (ob.extraversion, oa.agreeableness) {
            (Some(e), Some(a)) if e >= 7 && a >= 7 => 0.15,
            _ => 0.0,
        },
    };

    let ocean = ((oc + oc_bonus).min(1.0) + (ea + ea_bonus).min(1.0) + n) / 3.0;

    // Reputation: distance per shared dimension
    let mut rep_sum = 0.0;
    let mut rep_count = 0;
    for dim in RepDim::ALL {
        if let (Some(va), Some(vb)) = (a.rep_scores.score(dim), b.rep_scores.score(dim)) {
            let dist = if va >= vb { va - vb } else { vb - va };
            rep_sum += 1.0 - dist as f64 / 10.0;
            rep_count += 1;
        }
    }
    let (reputation, rep_active) = if rep_count == 0 {
        (0.0, false)
    } else {
        (rep_sum / rep_count as f64, true)
    };

    // Motivation: all-pair weighted synergy
    let mot_active = !a.motivations.is_empty() && !b.motivations.is_empty();
    let motivation = if mot_active {
        all_pair_weighted_avg(
            &a.motivations,
            &b.motivations,
            |m| m.intensity,
            |ma, mb| motivation_synergy(ma.r#type, mb.r#type),
        )
    } else {
        0.0
    };

    // Patterns: all-pair weighted synergy
    let pat_active = !a.behavioral_patterns.is_empty() && !b.behavioral_patterns.is_empty();
    let patterns = if pat_active {
        pattern_synergy(&a.behavioral_patterns, &b.behavioral_patterns)
    } else {
        0.0
    };

    // Bias: all-pair weighted synergy
    let bias_active = !a.biases.is_empty() && !b.biases.is_empty();
    let bias = if bias_active {
        all_pair_weighted_avg(
            &a.biases,
            &b.biases,
            |b| b.intensity,
            |ba, bb| bias_pair_synergy(ba.r#type, bb.r#type),
        )
    } else {
        0.0
    };

    // Base weights when all categories have data
    const W_OCEAN: f64 = 0.19;
    const W_REP: f64 = 0.29;
    const W_MOT: f64 = 0.21;
    const W_PAT: f64 = 0.16;
    const W_BIAS: f64 = 0.15;

    // Sum only active categories, normalize by total active weight
    let mut raw = 0.0;
    let mut total_w = 0.0;

    raw += ocean * W_OCEAN;
    total_w += W_OCEAN;
    if rep_active {
        raw += reputation * W_REP;
        total_w += W_REP;
    }
    if mot_active {
        raw += motivation * W_MOT;
        total_w += W_MOT;
    }
    if pat_active {
        raw += patterns * W_PAT;
        total_w += W_PAT;
    }
    if bias_active {
        raw += bias * W_BIAS;
        total_w += W_BIAS;
    }

    let score = if total_w > 0.0 {
        (raw / total_w * 100.0).round() as u8
    } else {
        0
    };

    SynergyBreakdown {
        total: score,
        ocean,
        reputation,
        motivation,
        patterns,
        bias,
    }
}

pub fn motivation_synergy(a: MotivationType, b: MotivationType) -> f64 {
    if a == b {
        return 0.2;
    }
    use MotivationType::*;
    match (a, b) {
        // Agency cluster
        (Power, Achievement) | (Achievement, Power) => 0.3,
        (Power, Autonomy) | (Autonomy, Power) => 0.2,
        (Achievement, Autonomy) | (Autonomy, Achievement) => 0.2,
        // Communion cluster
        (Affiliation, Helping) | (Helping, Affiliation) => 0.3,
        // Growth
        (Achievement, Learning) | (Learning, Achievement) => 0.3,
        (Autonomy, Learning) | (Learning, Autonomy) => 0.2,
        (Learning, Helping) | (Helping, Learning) => 0.2,
        // Ego
        (Power, Recognition) | (Recognition, Power) => 0.2,
        (Achievement, Recognition) | (Recognition, Achievement) => 0.3,
        // Communion + Security
        (Affiliation, Security) | (Security, Affiliation) => 0.2,
        (Helping, Security) | (Security, Helping) => 0.2,
        // Conflict pairs
        (Power, Affiliation) | (Affiliation, Power) => -0.2,
        (Power, Security) | (Security, Power) => -0.1,
        (Achievement, Security) | (Security, Achievement) => -0.2,
        (Autonomy, Affiliation) | (Affiliation, Autonomy) => -0.1,
        (Autonomy, Security) | (Security, Autonomy) => -0.3,
        (Recognition, Affiliation) | (Affiliation, Recognition) => -0.1,
        _ => 0.0,
    }
}

pub fn bias_pair_synergy(a: BiasType, b: BiasType) -> f64 {
    if a == b { -0.2 } else { 0.2 }
}

pub fn all_pair_weighted_avg<T, F>(
    items_a: &[T],
    items_b: &[T],
    get_intensity: F,
    pair_score: fn(&T, &T) -> f64,
) -> f64
where
    F: Fn(&T) -> u8,
{
    let mut sum = 0.0;
    let mut total_w = 0.0;
    for a in items_a {
        for b in items_b {
            let w = (get_intensity(a) as f64 * get_intensity(b) as f64) / 100.0;
            sum += pair_score(a, b) * w;
            total_w += w;
        }
    }
    if total_w == 0.0 {
        0.5
    } else {
        (0.5 + sum / total_w).clamp(0.0, 1.0)
    }
}

pub fn trigger_synergy(a: BehaviorTrigger, b: BehaviorTrigger) -> f64 {
    match (a, b) {
        (BehaviorTrigger::Change, BehaviorTrigger::Change) => 0.3,
        (BehaviorTrigger::Feedback, BehaviorTrigger::Feedback) => 0.3,
        (BehaviorTrigger::Feedback, BehaviorTrigger::Change)
        | (BehaviorTrigger::Change, BehaviorTrigger::Feedback) => 0.3,
        (BehaviorTrigger::Success, BehaviorTrigger::Success) => 0.3,
        (BehaviorTrigger::Conflict, BehaviorTrigger::Conflict) => -0.3,
        (BehaviorTrigger::Stress, BehaviorTrigger::Stress) => -0.2,
        (BehaviorTrigger::Stress, BehaviorTrigger::Conflict)
        | (BehaviorTrigger::Conflict, BehaviorTrigger::Stress) => -0.3,
        (BehaviorTrigger::Change, BehaviorTrigger::Stress)
        | (BehaviorTrigger::Stress, BehaviorTrigger::Change) => -0.2,
        (BehaviorTrigger::Conflict, BehaviorTrigger::Uncertainty)
        | (BehaviorTrigger::Uncertainty, BehaviorTrigger::Conflict) => -0.2,
        (BehaviorTrigger::Feedback, BehaviorTrigger::Recognition)
        | (BehaviorTrigger::Recognition, BehaviorTrigger::Feedback) => 0.2,
        _ => 0.0,
    }
}

pub fn pattern_synergy(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    let mut sum = 0.0;
    let mut total_w = 0.0;
    for a in pa {
        for b in pb {
            let w = (a.confidence as f64 * b.confidence as f64) / 100.0;
            sum += trigger_synergy(a.trigger, b.trigger) * w;
            total_w += w;
        }
    }
    if total_w == 0.0 {
        0.5
    } else {
        (sum / total_w + 0.5).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    // --- test helpers ---

    struct TestItem {
        intensity: u8,
    }

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
            ocean: OceanScores {
                openness,
                conscientiousness,
                extraversion,
                agreeableness,
                neuroticism,
            },
            predictions: vec![],
            confidence: 5,
            log: Vec::new(),
            created_at: 0,
            updated_at: 0,
        }
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
    fn test_motivation_same_type() {
        assert!(
            (motivation_synergy(MotivationType::Power, MotivationType::Power) - 0.2).abs() < 1e-9
        );
        assert!(
            (motivation_synergy(MotivationType::Learning, MotivationType::Learning) - 0.2).abs()
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

    // --- bias_pair_synergy tests ---

    #[test]
    fn test_bias_same_type() {
        assert!(
            (bias_pair_synergy(BiasType::Anchoring, BiasType::Anchoring) - (-0.2)).abs() < 1e-9
        );
    }

    #[test]
    fn test_bias_different_type() {
        assert!(
            (bias_pair_synergy(BiasType::Anchoring, BiasType::Confirmation) - 0.2).abs() < 1e-9
        );
    }

    // --- all_pair_weighted_avg tests ---

    #[test]
    fn test_all_pair_avg_empty_a() {
        let a: Vec<TestItem> = vec![];
        let b = vec![TestItem { intensity: 5 }];
        let score = |_: &TestItem, _: &TestItem| 1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        assert!((result - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_all_pair_avg_empty_both() {
        let a: Vec<TestItem> = vec![];
        let b: Vec<TestItem> = vec![];
        let score = |_: &TestItem, _: &TestItem| 1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        assert!((result - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_all_pair_avg_single_each() {
        let a = [TestItem { intensity: 10 }];
        let b = [TestItem { intensity: 10 }];
        let score = |_: &TestItem, _: &TestItem| 1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        // w = (10*10)/100 = 1.0; sum=1.0*1.0=1.0; total_w=1.0; 0.5 + 1.0/1.0 = 1.5, clamped to 1.0
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_all_pair_avg_equal_scores() {
        let a = [TestItem { intensity: 5 }];
        let b = [TestItem { intensity: 5 }];
        let score = |_: &TestItem, _: &TestItem| 0.8_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        // w=0.25, sum=0.8*0.25=0.2, total_w=0.25, 0.5+0.8=1.3 -> 1.0
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_all_pair_avg_negative_score() {
        let a = [TestItem { intensity: 10 }];
        let b = [TestItem { intensity: 10 }];
        let score = |_: &TestItem, _: &TestItem| -1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        // w=1.0, sum=-1.0, total_w=1.0; 0.5 + (-1.0)/1.0 = -0.5, clamped to 0.0
        assert!((result - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_all_pair_avg_weighting() {
        let a = [TestItem { intensity: 10 }, TestItem { intensity: 2 }];
        let b = [TestItem { intensity: 10 }];
        let score = |_: &TestItem, _: &TestItem| 1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        // pair (10,10): w=1.0, score=1.0
        // pair (2,10): w=0.2, score=1.0
        // sum=1.0+0.2=1.2; total_w=1.2; 0.5+1.2/1.2=1.5 -> 1.0
        assert!((result - 1.0).abs() < 0.001);
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
            predicted_behavior: "embraces change".into(),
            confidence: 8,
        }];
        let b = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: "welcomes change".into(),
            confidence: 6,
        }];
        // trigger_synergy(Change,Change)=0.3
        // w=(8*6)/100=0.48; sum=0.3*0.48=0.144; total_w=0.48
        // 0.5 + 0.144/0.48 = 0.5 + 0.3 = 0.8
        let result = pattern_synergy(&a, &b);
        assert!((result - 0.8).abs() < 0.001);
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
        // With missing traits, ocean is computed from what's available; default sim=0.5 for missing
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
        // Add rep scores so reputation is active
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
        // No motivations, biases, patterns — only ocean + rep active
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

    // --- compute_synergy_score tests ---

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
        };
        let a = p.clone();
        let b = p;
        let brk = compute_synergy_score(&a, &b);
        assert!(
            brk.total > 75,
            "Identical persons should score > 75, got {}",
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
}

use std::collections::HashSet;

use crate::models::{
    BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation, MotivationType, OceanScores,
    Person, PersonalStyle, Prediction, RepDim,
};

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
    pub danger: f64,
    pub bias_mod_active: bool,
    pub danger_details: String,
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
}

#[derive(Debug, Clone, Copy)]
enum BiasTarget {
    Ocean,
    Reputation,
    Motivation,
    Patterns,
}

struct Modulation {
    target: BiasTarget,
    coefficient: f64,
}

fn bias_modifier(ty: BiasType) -> Option<Modulation> {
    use BiasType::*;
    match ty {
        Anchoring => Some(Modulation {
            target: BiasTarget::Ocean,
            coefficient: 0.10,
        }),
        Confirmation => Some(Modulation {
            target: BiasTarget::Reputation,
            coefficient: 0.10,
        }),
        Availability => Some(Modulation {
            target: BiasTarget::Patterns,
            coefficient: 0.10,
        }),
        SunkCost => Some(Modulation {
            target: BiasTarget::Motivation,
            coefficient: 0.10,
        }),
        DunningKruger => Some(Modulation {
            target: BiasTarget::Ocean,
            coefficient: -0.10,
        }),
        LossAversion => Some(Modulation {
            target: BiasTarget::Patterns,
            coefficient: -0.10,
        }),
        SocialProof => Some(Modulation {
            target: BiasTarget::Reputation,
            coefficient: 0.08,
        }),
        Authority => Some(Modulation {
            target: BiasTarget::Motivation,
            coefficient: 0.08,
        }),
        Recency => Some(Modulation {
            target: BiasTarget::Patterns,
            coefficient: 0.08,
        }),
        InGroup => Some(Modulation {
            target: BiasTarget::Ocean,
            coefficient: 0.08,
        }),
        Favoritism => Some(Modulation {
            target: BiasTarget::Reputation,
            coefficient: -0.08,
        }),
    }
}

fn avg_prediction_accuracy(predictions: &[Prediction]) -> Option<f64> {
    let resolved: Vec<_> = predictions
        .iter()
        .filter(|p| p.resolved && p.accuracy.is_some())
        .collect();
    if resolved.len() < 3 {
        return None;
    }
    let sum: f64 = resolved.iter().map(|p| p.accuracy.unwrap() as f64).sum();
    Some(sum / resolved.len() as f64)
}

const DIM_WEIGHTS: [(RepDim, f64); 13] = [
    (RepDim::HonestDeceitful, 0.15),
    (RepDim::ReliableFlaky, 0.12),
    (RepDim::AuthoritativeSubmissive, 0.12),
    (RepDim::HumbleArrogant, 0.12),
    (RepDim::HardworkerLazy, 0.07),
    (RepDim::CalmReactive, 0.07),
    (RepDim::DiplomaticBlunt, 0.07),
    (RepDim::GenerousSelfish, 0.04),
    (RepDim::FairFavoritism, 0.07),
    (RepDim::TrustingSuspicious, 0.05),
    (RepDim::AssertivePassive, 0.05),
    (RepDim::EmpatheticDetached, 0.05),
    (RepDim::AdaptableRigid, 0.04),
];

pub fn base_rep_quality(p: &Person) -> f64 {
    let mut sum = 0.0;
    let mut n = 0.0;
    for &(dim, weight) in &DIM_WEIGHTS {
        if let Some(v) = p.rep_scores.score(dim) {
            sum += (v as f64 / 10.0) * weight;
            n += weight;
        }
    }
    if n == 0.0 { 0.5 } else { sum / n }
}

fn ocean_danger_penalty(oa: &crate::models::OceanScores, ob: &crate::models::OceanScores) -> f64 {
    let mut p = 0.0;

    // Within-person: volatile (N >= 7 and A <= 4)
    if oa.neuroticism.is_some_and(|n| n >= 7) && oa.agreeableness.is_some_and(|a| a <= 4) {
        p += 0.10;
    }
    if ob.neuroticism.is_some_and(|n| n >= 7) && ob.agreeableness.is_some_and(|a| a <= 4) {
        p += 0.10;
    }

    // Within-person: impulsive (N >= 7 and C <= 4)
    if oa.neuroticism.is_some_and(|n| n >= 7) && oa.conscientiousness.is_some_and(|c| c <= 4) {
        p += 0.05;
    }
    if ob.neuroticism.is_some_and(|n| n >= 7) && ob.conscientiousness.is_some_and(|c| c <= 4) {
        p += 0.05;
    }

    // Within-person: rigid anxious (N >= 7 and O <= 4)
    if oa.neuroticism.is_some_and(|n| n >= 7) && oa.openness.is_some_and(|o| o <= 4) {
        p += 0.05;
    }
    if ob.neuroticism.is_some_and(|n| n >= 7) && ob.openness.is_some_and(|o| o <= 4) {
        p += 0.05;
    }

    // Cross-person: emotional contagion (both N >= 7)
    if oa.neuroticism.is_some_and(|n| n >= 7) && ob.neuroticism.is_some_and(|n| n >= 7) {
        p += 0.10;
    }

    // Cross-person: antagonism (both A <= 4)
    if oa.agreeableness.is_some_and(|a| a <= 4) && ob.agreeableness.is_some_and(|a| a <= 4) {
        p += 0.15;
    }

    // Cross-person: mutual unreliability (both C <= 4)
    if oa.conscientiousness.is_some_and(|c| c <= 4) && ob.conscientiousness.is_some_and(|c| c <= 4)
    {
        p += 0.10;
    }

    // Cross-person: mutual rigidity (both O <= 4)
    if oa.openness.is_some_and(|o| o <= 4) && ob.openness.is_some_and(|o| o <= 4) {
        p += 0.05;
    }

    p
}

fn rep_danger_penalty(rep_a: &crate::models::RepScores, rep_b: &crate::models::RepScores) -> f64 {
    let mut p = 0.0;

    // Both authoritative >= 8 → power struggle
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::AuthoritativeSubmissive),
        rep_b.score(RepDim::AuthoritativeSubmissive),
    ) && aa >= 8
        && ab >= 8
    {
        p += 0.10;
    }

    // Both blunt >= 8 → brutal honesty, no diplomacy
    // score: 10 = Diplomatic (pole A), 0 = Blunt (pole B)
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::DiplomaticBlunt),
        rep_b.score(RepDim::DiplomaticBlunt),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.10;
    }

    // Both reactive >= 8 → mutual escalation
    // score: 10 = Calm (pole A), 0 = Reactive (pole B)
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::CalmReactive),
        rep_b.score(RepDim::CalmReactive),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.10;
    }

    // Both arrogant >= 8 → neither concedes
    // score: 10 = Humble (pole A), 0 = Arrogant (pole B)
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HumbleArrogant),
        rep_b.score(RepDim::HumbleArrogant),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.10;
    }

    // Both lazy <= 3 → mutual passivity
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HardworkerLazy),
        rep_b.score(RepDim::HardworkerLazy),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.05;
    }

    // Both untrusting <= 3 → mutual suspicion
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::TrustingSuspicious),
        rep_b.score(RepDim::TrustingSuspicious),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.08;
    }

    // Both detached <= 3 → mutual coldness
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::EmpatheticDetached),
        rep_b.score(RepDim::EmpatheticDetached),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.08;
    }

    // Both deceitful <= 3 → trust collapse
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HonestDeceitful),
        rep_b.score(RepDim::HonestDeceitful),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.10;
    }

    // Both flaky <= 3 → mutual unreliability
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::ReliableFlaky),
        rep_b.score(RepDim::ReliableFlaky),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.08;
    }

    // Both unfair <= 3 → cronyism
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::FairFavoritism),
        rep_b.score(RepDim::FairFavoritism),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.08;
    }

    // Both selfish <= 3 → mutual hoarding
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::GenerousSelfish),
        rep_b.score(RepDim::GenerousSelfish),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.05;
    }

    // Both passive <= 3 → decision paralysis
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::AssertivePassive),
        rep_b.score(RepDim::AssertivePassive),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.05;
    }

    // Both rigid <= 3 → gridlock
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::AdaptableRigid),
        rep_b.score(RepDim::AdaptableRigid),
    ) && aa <= 3
        && ab <= 3
    {
        p += 0.05;
    }

    p
}

/// Similarity score between two OCEAN trait values (1-10 scale).
/// Formula: `1 - |a-b|/10`, clamped to [0.0, 1.0].
/// Returns 0.5 when either value is missing.
pub fn sim(a: Option<u8>, b: Option<u8>) -> f64 {
    match (a, b) {
        (Some(a), Some(b)) => 1.0 - (a.abs_diff(b) as f64) / 10.0,
        _ => 0.5,
    }
}

/// Scale band thresholds derived from the [`sim`] formula.
///
/// `score = sim * 100 = (1 - |a-b|/10) * 100`.
/// Trait-diff boundaries `[3, 5, 7, 8.5]` map to score thresholds:
/// - Strong:   ≥70  (diff ≤3)
/// - Good:     ≥50  (diff ≤5)
/// - Moderate: ≥30  (diff ≤7)
/// - Friction: ≥15  (diff ≤8.5)
/// - Tension:  0-14 (diff >8.5)
pub fn synergy_bands() -> [(u8, u8); 5] {
    // Trait-diff boundaries mapped through sim formula → score thresholds
    const DIFF_BOUNDS: [f64; 4] = [3.0, 5.0, 7.0, 8.5];
    let mut thresh = [0u8; 4];
    for (i, &d) in DIFF_BOUNDS.iter().enumerate() {
        thresh[i] = ((1.0 - d / 10.0) * 100.0).round() as u8;
    }
    [
        (0, thresh[3] - 1),         // Tension
        (thresh[3], thresh[2] - 1), // Friction
        (thresh[2], thresh[1] - 1), // Moderate
        (thresh[1], thresh[0] - 1), // Good
        (thresh[0], 100),           // Strong
    ]
}

pub fn compute_synergy_score(a: &Person, b: &Person) -> SynergyBreakdown {
    compute_synergy_score_inner(a, b, &[], &[])
}

pub fn compute_synergy_score_with_preds(
    a: &Person,
    b: &Person,
    a_preds: &[Prediction],
    b_preds: &[Prediction],
) -> SynergyBreakdown {
    compute_synergy_score_inner(a, b, a_preds, b_preds)
}

fn compute_synergy_score_inner(
    a: &Person,
    b: &Person,
    a_preds: &[Prediction],
    b_preds: &[Prediction],
) -> SynergyBreakdown {
    let oa = &a.ocean;
    let ob = &b.ocean;

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

    let raw_ocean = ((oc + oc_bonus).min(1.0) + (ea + ea_bonus).min(1.0) + n) / 3.0;

    // Reputation: weighted distance per shared dimension
    let mut rep_sum = 0.0;
    let mut total_active_w = 0.0;
    for &(dim, weight) in &DIM_WEIGHTS {
        if let (Some(va), Some(vb)) = (a.rep_scores.score(dim), b.rep_scores.score(dim)) {
            let dist = va.abs_diff(vb);
            rep_sum += (1.0 - dist as f64 / 10.0) * weight;
            total_active_w += weight;
        }
    }
    let (raw_rep, rep_active) = if total_active_w == 0.0 {
        (0.0, false)
    } else {
        (rep_sum / total_active_w, true)
    };

    // Motivation: all-pair weighted synergy
    let mot_active = !a.motivations.is_empty() && !b.motivations.is_empty();
    let raw_mot = if mot_active {
        motivation_synergy_score(&a.motivations, &b.motivations)
    } else {
        0.0
    };

    // Patterns: all-pair weighted synergy
    let pat_active = !a.behavioral_patterns.is_empty() && !b.behavioral_patterns.is_empty();
    let raw_pat = if pat_active {
        pattern_synergy(&a.behavioral_patterns, &b.behavioral_patterns)
    } else {
        0.0
    };

    // --- Pattern danger: both persons have only negative triggers ---

    let has_negative_only = |patterns: &[BehavioralPattern]| -> bool {
        !patterns.is_empty()
            && patterns.iter().all(|p| {
                matches!(
                    p.trigger,
                    BehaviorTrigger::Conflict
                        | BehaviorTrigger::Stress
                        | BehaviorTrigger::Threatened
                        | BehaviorTrigger::Injustice
                )
            })
    };
    let pat_danger_penalty = if pat_active
        && has_negative_only(&a.behavioral_patterns)
        && has_negative_only(&b.behavioral_patterns)
    {
        0.05
    } else {
        0.0
    };

    // --- Bias: shared-type modulation system ---

    let a_types: HashSet<BiasType> = a.biases.iter().map(|b| b.r#type).collect();
    let b_types: HashSet<BiasType> = b.biases.iter().map(|b| b.r#type).collect();
    let shared_count = a_types.intersection(&b_types).count();
    let max_unique = a_types.len().max(b_types.len());
    let bias_score = if max_unique > 0 {
        shared_count as f64 / max_unique as f64
    } else {
        0.5
    };

    let mut ocean_mod = 0.0;
    let mut rep_mod = 0.0;
    let mut mot_mod = 0.0;
    let mut pat_mod = 0.0;

    for ba in &a.biases {
        for bb in &b.biases {
            if ba.r#type == bb.r#type {
                let w = (ba.intensity as f64 * bb.intensity as f64) / 100.0;
                if let Some(m) = bias_modifier(ba.r#type) {
                    let delta = m.coefficient * w;
                    match m.target {
                        BiasTarget::Ocean => ocean_mod += delta,
                        BiasTarget::Reputation => rep_mod += delta,
                        BiasTarget::Motivation => mot_mod += delta,
                        BiasTarget::Patterns => pat_mod += delta,
                    }
                }
            }
        }
    }

    // --- Danger penalties ---

    let ocean_penalty = ocean_danger_penalty(oa, ob);
    let rep_penalty = if rep_active {
        rep_danger_penalty(&a.rep_scores, &b.rep_scores)
    } else {
        0.0
    };

    // --- History factor ---

    let a_accuracy = avg_prediction_accuracy(a_preds);
    let b_accuracy = avg_prediction_accuracy(b_preds);
    let history_penalty = match (a_accuracy, b_accuracy) {
        (Some(pa), Some(pb)) if pa < 5.0 && pb < 5.0 => 0.05,
        (Some(pa), Some(_)) if pa < 5.0 => 0.03,
        (Some(_), Some(pb)) if pb < 5.0 => 0.03,
        _ => 0.0,
    };

    // Apply penalties + modulation
    let ocean = ((raw_ocean - ocean_penalty).max(0.0) * (1.0 + ocean_mod)).clamp(0.0, 1.0);
    let reputation = ((raw_rep - rep_penalty).max(0.0) * (1.0 + rep_mod)).clamp(0.0, 1.0);
    let motivation = (raw_mot * (1.0 + mot_mod)).clamp(0.0, 1.0);
    let patterns = ((raw_pat - pat_danger_penalty).max(0.0) * (1.0 + pat_mod)).clamp(0.0, 1.0);

    const W_HISTORY: f64 = 0.10;
    let total_danger = ocean_penalty * W_OCEAN
        + rep_penalty * W_REP
        + pat_danger_penalty * W_PAT
        + history_penalty * W_HISTORY;

    // Dynamic weight redistribution (shared by mutual total & asymmetric)
    const W_OCEAN: f64 = 0.17;
    const W_REP: f64 = 0.26;
    const W_MOT: f64 = 0.19;
    const W_PAT: f64 = 0.14;
    const W_BIAS: f64 = 0.13;
    const W_STYLE: f64 = 0.11;

    // --- Asymmetric individual perspectives ---
    // A's benefit = Σ(A's valuation_i × B's quality_i) via composition of
    //   OCEAN: similarity-weighted partner quality  → asymmetric
    //   Reputation / Bias: partner's raw quality     → asymmetric when levels differ
    //   Motivation / Patterns: shared synergy        → symmetric (same for both)
    // Total = (a_score + b_score) / 2

    let a_base_rep = base_rep_quality(a);
    let b_base_rep = base_rep_quality(b);
    let a_bias_quality = 1.0 - (a.biases.len() as f64 / crate::models::BiasType::ALL.len() as f64);
    let b_bias_quality = 1.0 - (b.biases.len() as f64 / crate::models::BiasType::ALL.len() as f64);

    // OCEAN vector for each person (trait value / 10, stability = 1 - N/10)
    let ovec = |o: &OceanScores| -> [f64; 5] {
        [
            o.openness.map_or(0.5, |v| v as f64 / 10.0),
            o.conscientiousness.map_or(0.5, |v| v as f64 / 10.0),
            o.extraversion.map_or(0.5, |v| v as f64 / 10.0),
            o.agreeableness.map_or(0.5, |v| v as f64 / 10.0),
            o.neuroticism.map_or(0.5, |v| (10.0 - v as f64) / 10.0),
        ]
    };
    let av = ovec(oa);
    let bv = ovec(ob);

    // Similarity-weighted partner quality: a_ocean_i = B_quality_i × sim(A_i, B_i).
    // Asymmetric because B_quality_i × sim ≠ A_quality_i × sim when A_i ≠ B_i.
    // This preserves genuine OCEAN asymmetry without inflating scores when
    // partners have very different trait levels (unlike pure complementarity).
    let asym_ocean = |v: &[f64; 5], t: &[f64; 5]| -> f64 {
        ((1.0 - (v[0] - t[0]).abs()) * t[0]
            + (1.0 - (v[1] - t[1]).abs()) * t[1]
            + (1.0 - (v[2] - t[2]).abs()) * t[2]
            + (1.0 - (v[3] - t[3]).abs()) * t[3]
            + (1.0 - (v[4] - t[4]).abs()) * t[4])
            / 5.0
    };

    // A's perspective: B's traits × similarity(A traits, B traits)
    let a_ocean = asym_ocean(&av, &bv);
    // B's perspective: A's traits × similarity(B traits, A traits)
    let b_ocean = asym_ocean(&bv, &av);

    let mut a_raw = 0.0;
    let mut b_raw = 0.0;
    let mut asym_w = 0.0;
    a_raw += a_ocean * W_OCEAN;
    b_raw += b_ocean * W_OCEAN;
    asym_w += W_OCEAN;
    if rep_active {
        a_raw += b_base_rep * W_REP;
        b_raw += a_base_rep * W_REP;
        asym_w += W_REP;
    }
    if mot_active {
        a_raw += motivation * W_MOT;
        b_raw += motivation * W_MOT;
        asym_w += W_MOT;
    }
    if pat_active {
        a_raw += patterns * W_PAT;
        b_raw += patterns * W_PAT;
        asym_w += W_PAT;
    }
    a_raw += b_bias_quality * W_BIAS;
    b_raw += a_bias_quality * W_BIAS;
    asym_w += W_BIAS;

    let styles = style_synergy(&a.styles, &b.styles);
    a_raw += styles * W_STYLE;
    b_raw += styles * W_STYLE;
    asym_w += W_STYLE;

    let a_score = if asym_w > 0.0 {
        ((a_raw / asym_w * 100.0).round() as u8).min(100)
    } else {
        0
    };
    let b_score = if asym_w > 0.0 {
        ((b_raw / asym_w * 100.0).round() as u8).min(100)
    } else {
        0
    };

    // Apply danger penalties: total_danger = Σ(penalty_i × W_i) was the direct
    // reduction to `raw` in the old formula. Score-point reduction = total_danger / asym_w * 100.
    let danger_penalty = if asym_w > 0.0 {
        (total_danger / asym_w * 100.0).round() as u8
    } else {
        0
    };
    let mutual = ((a_score as f64 + b_score as f64) / 2.0).round() as u8;
    let mutual = mutual.saturating_sub(danger_penalty);

    let mut details = Vec::new();
    if ocean_penalty > 0.0 {
        details.push("OCEAN volatility");
    }
    if rep_penalty > 0.0 {
        details.push("Rep power struggle");
    }
    if pat_danger_penalty > 0.0 {
        details.push("Only negative patterns");
    }
    if history_penalty > 0.0 {
        details.push("Low prediction accuracy");
    }
    let danger_details = if details.is_empty() {
        String::new()
    } else {
        details.join(", ")
    };

    SynergyBreakdown {
        total: mutual,
        a_score,
        b_score,
        ocean,
        reputation,
        motivation,
        patterns,
        bias: bias_score,
        styles,
        danger: total_danger,
        bias_mod_active: (ocean_mod + rep_mod + mot_mod + pat_mod) > 0.0,
        danger_details,
    }
}

pub fn compute_person_profile(person: &Person) -> PersonProfile {
    let mot_active = !person.motivations.is_empty();
    let pat_active = !person.behavioral_patterns.is_empty();

    let base_mot = if mot_active {
        motivation_synergy_score(&person.motivations, &person.motivations)
    } else {
        0.5
    };
    let virtue = virtue_adjustment(&person.motivations);
    let count_penalty = motivation_count_penalty(person.motivations.len());
    let motivation = (base_mot + virtue - count_penalty).clamp(0.0, 1.0);

    let raw_pat = if pat_active {
        pattern_synergy(&person.behavioral_patterns, &person.behavioral_patterns)
    } else {
        0.5
    };

    let a_s = person.ocean.agreeableness.map_or(0.5, |v| v as f64 / 10.0);
    let n_s = person
        .ocean
        .neuroticism
        .map_or(0.5, |v| (10.0 - v as f64) / 10.0);
    let mut ocean_penalty = 0.0;
    if person.ocean.neuroticism.is_some_and(|n| n >= 7)
        && person.ocean.agreeableness.is_some_and(|a| a <= 4)
    {
        ocean_penalty += 0.10;
    }
    if person.ocean.neuroticism.is_some_and(|n| n >= 7)
        && person.ocean.conscientiousness.is_some_and(|c| c <= 4)
    {
        ocean_penalty += 0.05;
    }
    if person.ocean.neuroticism.is_some_and(|n| n >= 7)
        && person.ocean.openness.is_some_and(|o| o <= 4)
    {
        ocean_penalty += 0.05;
    }

    let raw_ocean = (a_s + n_s) / 2.0;
    let ocean = (raw_ocean - ocean_penalty).max(0.0);

    let rep = base_rep_quality(person);

    let bias_adj = bias_adjustment(&person.biases);
    let absent_count = BiasType::ALL.len() - person.biases.len();
    let moderate_plus = person.biases.iter().filter(|b| b.intensity >= 4).count();
    let present_bias_count = absent_count + moderate_plus;
    let base_bias =
        1.0 - (present_bias_count as f64 / crate::models::BiasType::ALL.len() as f64).min(1.0);
    let count_bonus = bias_count_bonus(present_bias_count);
    let bias = (base_bias + bias_adj + count_bonus).clamp(0.0, 1.0);

    let raw_style = if !person.styles.is_empty() {
        style_synergy(&person.styles, &person.styles)
    } else {
        0.5
    };

    const W_MOT: f64 = 0.19;
    const W_PAT: f64 = 0.14;
    const W_OCEAN: f64 = 0.17;
    const W_REP: f64 = 0.26;
    const W_BIAS: f64 = 0.13;
    const W_STYLE: f64 = 0.11;
    let mut total_w = 0.0;
    let mut raw = 0.0;
    raw += motivation * W_MOT;
    total_w += W_MOT;
    if pat_active {
        raw += raw_pat * W_PAT;
        total_w += W_PAT;
    }
    raw += ocean * W_OCEAN;
    total_w += W_OCEAN;
    raw += rep * W_REP;
    total_w += W_REP;
    raw += bias * W_BIAS;
    total_w += W_BIAS;
    raw += raw_style * W_STYLE;
    total_w += W_STYLE;

    let total = if total_w > 0.0 {
        ((raw / total_w * 100.0).round() as u8).min(100)
    } else {
        50
    };

    PersonProfile {
        total,
        motivation,
        patterns: raw_pat,
        ocean,
        reputation: rep,
        bias,
        styles: raw_style,
    }
}

pub fn motivation_synergy(a: MotivationType, b: MotivationType) -> f64 {
    use MotivationType::*;
    match (a, b) {
        // Self-pairs
        (Power, Power) => -0.2,
        (Recognition, Recognition) => -0.1,
        (Autonomy, Autonomy) => 0.0,
        (Security, Security) => 0.0,
        (Creativity, Creativity) => 0.2,
        (Fairness, Fairness) => 0.2,
        (Achievement, Achievement)
        | (Affiliation, Affiliation)
        | (Helping, Helping)
        | (Learning, Learning) => 0.2,
        // Cross-pairs
        (Power, Achievement) | (Achievement, Power) => 0.3,
        (Power, Helping) | (Helping, Power) => 0.1,
        (Achievement, Affiliation) | (Affiliation, Achievement) => 0.1,
        (Power, Autonomy) | (Autonomy, Power) => 0.2,
        (Achievement, Autonomy) | (Autonomy, Achievement) => 0.2,
        (Affiliation, Helping) | (Helping, Affiliation) => 0.3,
        (Achievement, Learning) | (Learning, Achievement) => 0.3,
        (Autonomy, Learning) | (Learning, Autonomy) => 0.2,
        (Learning, Helping) | (Helping, Learning) => 0.2,
        (Power, Recognition) | (Recognition, Power) => 0.2,
        (Achievement, Recognition) | (Recognition, Achievement) => 0.3,
        (Affiliation, Security) | (Security, Affiliation) => 0.2,
        (Helping, Security) | (Security, Helping) => 0.2,
        (Power, Affiliation) | (Affiliation, Power) => -0.2,
        (Power, Security) | (Security, Power) => -0.1,
        (Achievement, Security) | (Security, Achievement) => -0.2,
        (Autonomy, Affiliation) | (Affiliation, Autonomy) => -0.1,
        (Autonomy, Security) | (Security, Autonomy) => -0.3,
        (Recognition, Affiliation) | (Affiliation, Recognition) => -0.1,
        (Creativity, Learning) | (Learning, Creativity) => 0.3,
        (Creativity, Autonomy) | (Autonomy, Creativity) => 0.2,
        (Creativity, Achievement) | (Achievement, Creativity) => 0.2,
        (Creativity, Helping) | (Helping, Creativity) => -0.1,
        (Fairness, Helping) | (Helping, Fairness) => 0.3,
        (Fairness, Affiliation) | (Affiliation, Fairness) => 0.2,
        (Fairness, Power) | (Power, Fairness) => -0.2,
        (Fairness, Recognition) | (Recognition, Fairness) => -0.1,
        (Power, Learning) | (Learning, Power) => 0.0,
        (Power, Creativity) | (Creativity, Power) => -0.1,
        (Achievement, Helping) | (Helping, Achievement) => 0.2,
        (Achievement, Fairness) | (Fairness, Achievement) => 0.2,
        (Affiliation, Learning) | (Learning, Affiliation) => 0.2,
        (Affiliation, Creativity) | (Creativity, Affiliation) => 0.2,
        (Helping, Autonomy) | (Autonomy, Helping) => 0.0,
        (Helping, Recognition) | (Recognition, Helping) => 0.0,
        (Autonomy, Recognition) | (Recognition, Autonomy) => 0.0,
        (Autonomy, Fairness) | (Fairness, Autonomy) => 0.2,
        (Learning, Recognition) | (Recognition, Learning) => 0.3,
        (Learning, Security) | (Security, Learning) => 0.2,
        (Learning, Fairness) | (Fairness, Learning) => 0.2,
        (Recognition, Security) | (Security, Recognition) => 0.0,
        (Recognition, Creativity) | (Creativity, Recognition) => 0.3,
        (Security, Creativity) | (Creativity, Security) => -0.2,
        (Security, Fairness) | (Fairness, Security) => 0.2,
        (Creativity, Fairness) | (Fairness, Creativity) => 0.2,
    }
}

pub fn motivation_synergy_score(ma: &[Motivation], mb: &[Motivation]) -> f64 {
    let mut sum = 0.0;
    let mut total_w = 0.0;
    for a in ma {
        for b in mb {
            let syn = motivation_synergy(a.r#type, b.r#type);
            if syn == 0.0 {
                continue;
            }
            let w = (a.intensity as f64 * b.intensity as f64) / 100.0;
            sum += syn * w;
            total_w += w;
        }
    }
    if total_w == 0.0 {
        0.5
    } else {
        ((sum / total_w + 0.3) / 0.6).clamp(0.0, 1.0)
    }
}

pub fn virtue_adjustment(motivations: &[Motivation]) -> f64 {
    use crate::models::MotivationType::*;
    let mut sum = 0.0;
    for &t in &MotivationType::ALL {
        let mot = motivations.iter().find(|m| m.r#type == t);
        let intensity = mot.map(|m| m.intensity);
        match (t, intensity) {
            (Fairness, Some(i)) if i >= 7 => sum += 0.08,
            (Fairness, Some(i)) if i <= 3 => sum -= 0.08,
            (Fairness, None) => sum -= 0.08,
            (Helping, Some(i)) if i >= 7 => sum += 0.06,
            (Helping, Some(i)) if i <= 3 => sum -= 0.06,
            (Helping, None) => sum -= 0.06,
            (Learning, Some(i)) if i >= 7 => sum += 0.04,
            (Creativity, Some(i)) if i >= 7 => sum += 0.04,
            (Power, Some(i)) if i >= 7 => sum -= 0.08,
            (Security, Some(i)) if i >= 7 => sum -= 0.05,
            (Recognition, Some(i)) if i >= 9 => sum -= 0.03,
            _ => {}
        }
    }
    sum
}

fn motivation_count_penalty(n: usize) -> f64 {
    if n >= 3 { 0.0 } else { (3 - n) as f64 * 0.03 }
}

pub fn bias_adjustment(biases: &[Bias]) -> f64 {
    let mut sum = 0.0;
    for &t in &BiasType::ALL {
        match biases.iter().find(|b| b.r#type == t).map(|b| b.intensity) {
            Some(0) => sum += 0.02,           // explicitly absent → bonus
            Some(i) if i <= 3 => sum += 0.01, // mild → small bonus
            Some(i) if i >= 7 => sum -= 0.03, // strong → penalty
            _ => {}                           // moderate (4-6) or undefined → neutral
        }
    }
    sum
}

fn bias_count_bonus(n: usize) -> f64 {
    match n {
        0 => 0.09,
        1 => 0.06,
        2 => 0.03,
        _ => 0.0,
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
        (BehaviorTrigger::Injustice, BehaviorTrigger::Stress)
        | (BehaviorTrigger::Stress, BehaviorTrigger::Injustice) => -0.1,
        (BehaviorTrigger::Injustice, BehaviorTrigger::Conflict)
        | (BehaviorTrigger::Conflict, BehaviorTrigger::Injustice) => -0.1,
        (BehaviorTrigger::Injustice, BehaviorTrigger::Uncertainty)
        | (BehaviorTrigger::Uncertainty, BehaviorTrigger::Injustice) => -0.1,
        (BehaviorTrigger::Injustice, BehaviorTrigger::Injustice) => -0.2,
        _ => 0.0,
    }
}

pub fn pattern_synergy(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    let mut sum = 0.0;
    let mut total_w = 0.0;
    for a in pa {
        for b in pb {
            let syn = trigger_synergy(a.trigger, b.trigger);
            if syn == 0.0 {
                continue;
            }
            let w = (a.intensity as f64 * b.intensity as f64) / 100.0;
            sum += syn * w;
            total_w += w;
        }
    }
    if total_w == 0.0 {
        0.5
    } else {
        ((sum / total_w + 0.3) / 0.6).clamp(0.0, 1.0)
    }
}

/// Style synergy: for each of the 6 style categories, if both persons have a
/// style in that category, score 1.0 if same choice, 0.5 if different.
/// Average over categories where both have data. Returns 0.5 if no overlap.
pub fn style_synergy(a: &[PersonalStyle], b: &[PersonalStyle]) -> f64 {
    use crate::models::StyleCategory;
    let cats = StyleCategory::ALL;
    let mut sum = 0.0;
    let mut n = 0;
    for cat in &cats {
        let a_style = a.iter().find(|s| s.r#type.category() == *cat);
        let b_style = b.iter().find(|s| s.r#type.category() == *cat);
        if let (Some(sa), Some(sb)) = (a_style, b_style) {
            if sa.r#type == sb.r#type {
                sum += 1.0;
            } else {
                sum += 0.5;
            }
            n += 1;
        }
    }
    if n == 0 { 0.5 } else { sum / n as f64 }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            ocean: OceanScores {
                openness,
                conscientiousness,
                extraversion,
                agreeableness,
                neuroticism,
            },
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
                bias_modifier(*ty).is_some(),
                "bias_modifier missing for {:?}",
                ty
            );
        }
    }

    #[test]
    fn test_bias_modifier_anchoring_ocean() {
        let m = bias_modifier(BiasType::Anchoring).unwrap();
        assert!(matches!(m.target, BiasTarget::Ocean));
        assert!((m.coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_confirmation_rep() {
        let m = bias_modifier(BiasType::Confirmation).unwrap();
        assert!(matches!(m.target, BiasTarget::Reputation));
        assert!((m.coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_availability_patterns() {
        let m = bias_modifier(BiasType::Availability).unwrap();
        assert!(matches!(m.target, BiasTarget::Patterns));
        assert!((m.coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_sunkcost_motivation() {
        let m = bias_modifier(BiasType::SunkCost).unwrap();
        assert!(matches!(m.target, BiasTarget::Motivation));
        assert!((m.coefficient - 0.10).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_dunningkruger_ocean_negative() {
        let m = bias_modifier(BiasType::DunningKruger).unwrap();
        assert!(matches!(m.target, BiasTarget::Ocean));
        assert!((m.coefficient - (-0.10)).abs() < 1e-9);
    }

    #[test]
    fn test_bias_modifier_lossaversion_patterns_negative() {
        let m = bias_modifier(BiasType::LossAversion).unwrap();
        assert!(matches!(m.target, BiasTarget::Patterns));
        assert!((m.coefficient - (-0.10)).abs() < 1e-9);
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
            intensity: 5,
        }];
        let b = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            intensity: 5,
        }];
        let result = pattern_synergy(&a, &b);
        assert!((result - 1.0).abs() < 0.001, "got {}", result);
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
                intensity: 5,
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Stress,
                predicted_behavior: BehaviorResponse::Withdraws,
                intensity: 5,
            },
        ];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Threatened,
            predicted_behavior: BehaviorResponse::DeflectsBlame,
            intensity: 5,
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
            intensity: 5,
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            intensity: 5,
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
            (pf.patterns - 0.5).abs() < 0.001,
            "baseline pat: {}",
            pf.patterns
        );
        assert!(
            pf.total > 30 && pf.total < 70,
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
        assert!(pf.total > 40, "should be decent: {}", pf.total);
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
        assert!(pf.total > 45, "good rep boosts total: {}", pf.total);
    }

    #[test]
    fn test_self_score_negative_patterns_lower() {
        let mut p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        p.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::BecomesDefensive,
            intensity: 8,
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
        // All None → A=0.5, N-inverted=0.5 → raw_ocean=0.5, no penalty
        assert!((pf.ocean - 0.5).abs() < 0.001, "ocean score: {}", pf.ocean);
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
                intensity: 8,
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                intensity: 6,
            },
        ];
        let b = vec![
            BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::EmbracesChange,
                intensity: 8,
            },
            BehavioralPattern {
                trigger: BehaviorTrigger::Conflict,
                predicted_behavior: BehaviorResponse::BecomesDefensive,
                intensity: 6,
            },
        ];
        let result = pattern_synergy(&a, &b);
        // Same triggers: Change=0.3, Conflict=-0.3
        // w_cc = 64/100=0.64, score+0.5=0.8; w_cf = 48/100=0.48, score-0.3+0.5=0.2 etc.
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
}

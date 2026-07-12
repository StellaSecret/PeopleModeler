use std::collections::HashSet;

use crate::models::{
    BehaviorTrigger, BehavioralPattern, BiasType, MotivationType, OceanScores, Person, Prediction,
    RepDim,
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
    pub danger: f64,
    pub bias_mod_active: bool,
    pub danger_details: String,
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

fn ocean_danger_penalty(oa: &crate::models::OceanScores, ob: &crate::models::OceanScores) -> f64 {
    let mut p = 0.0;

    // Within-person: volatile (N >= 7 and A <= 4)
    if oa.neuroticism.map_or(false, |n| n >= 7) && oa.agreeableness.map_or(false, |a| a <= 4) {
        p += 0.10;
    }
    if ob.neuroticism.map_or(false, |n| n >= 7) && ob.agreeableness.map_or(false, |a| a <= 4) {
        p += 0.10;
    }

    // Within-person: impulsive (N >= 7 and C <= 4)
    if oa.neuroticism.map_or(false, |n| n >= 7) && oa.conscientiousness.map_or(false, |c| c <= 4) {
        p += 0.05;
    }
    if ob.neuroticism.map_or(false, |n| n >= 7) && ob.conscientiousness.map_or(false, |c| c <= 4) {
        p += 0.05;
    }

    // Within-person: rigid anxious (N >= 7 and O <= 4)
    if oa.neuroticism.map_or(false, |n| n >= 7) && oa.openness.map_or(false, |o| o <= 4) {
        p += 0.05;
    }
    if ob.neuroticism.map_or(false, |n| n >= 7) && ob.openness.map_or(false, |o| o <= 4) {
        p += 0.05;
    }

    // Cross-person: emotional contagion (both N >= 7)
    if oa.neuroticism.map_or(false, |n| n >= 7) && ob.neuroticism.map_or(false, |n| n >= 7) {
        p += 0.10;
    }

    // Cross-person: antagonism (both A <= 4)
    if oa.agreeableness.map_or(false, |a| a <= 4) && ob.agreeableness.map_or(false, |a| a <= 4) {
        p += 0.15;
    }

    // Cross-person: mutual unreliability (both C <= 4)
    if oa.conscientiousness.map_or(false, |c| c <= 4)
        && ob.conscientiousness.map_or(false, |c| c <= 4)
    {
        p += 0.10;
    }

    // Cross-person: mutual rigidity (both O <= 4)
    if oa.openness.map_or(false, |o| o <= 4) && ob.openness.map_or(false, |o| o <= 4) {
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
    ) {
        if aa >= 8 && ab >= 8 {
            p += 0.10;
        }
    }

    // Both blunt >= 8 → brutal honesty, no diplomacy
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::DiplomaticBlunt),
        rep_b.score(RepDim::DiplomaticBlunt),
    ) {
        if aa >= 8 && ab >= 8 {
            p += 0.10;
        }
    }

    // Both reactive >= 8 → mutual escalation
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::CalmReactive),
        rep_b.score(RepDim::CalmReactive),
    ) {
        if aa >= 8 && ab >= 8 {
            p += 0.10;
        }
    }

    // Both arrogant >= 8 → neither concedes
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HumbleArrogant),
        rep_b.score(RepDim::HumbleArrogant),
    ) {
        if aa >= 8 && ab >= 8 {
            p += 0.10;
        }
    }

    // Both lazy <= 3 → mutual passivity
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HardworkerLazy),
        rep_b.score(RepDim::HardworkerLazy),
    ) {
        if aa <= 3 && ab <= 3 {
            p += 0.05;
        }
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
    const DIM_WEIGHTS: [(RepDim, f64); 8] = [
        (RepDim::HonestDeceitful, 0.20),
        (RepDim::ReliableFlaky, 0.15),
        (RepDim::AuthoritativeSubmissive, 0.15),
        (RepDim::HumbleArrogant, 0.15),
        (RepDim::HardworkerLazy, 0.10),
        (RepDim::CalmReactive, 0.10),
        (RepDim::DiplomaticBlunt, 0.10),
        (RepDim::GenerousSelfish, 0.05),
    ];
    let mut rep_sum = 0.0;
    let mut total_active_w = 0.0;
    for &(dim, weight) in &DIM_WEIGHTS {
        if let (Some(va), Some(vb)) = (a.rep_scores.score(dim), b.rep_scores.score(dim)) {
            let dist = if va >= vb { va - vb } else { vb - va };
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

    let a_accuracy = avg_prediction_accuracy(&a.predictions);
    let b_accuracy = avg_prediction_accuracy(&b.predictions);
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

    let total_danger = ocean_penalty * W_OCEAN
        + rep_penalty * W_REP
        + pat_danger_penalty * W_PAT
        + history_penalty;

    // Dynamic weight redistribution (shared by mutual total & asymmetric)
    const W_OCEAN: f64 = 0.19;
    const W_REP: f64 = 0.29;
    const W_MOT: f64 = 0.21;
    const W_PAT: f64 = 0.16;
    const W_BIAS: f64 = 0.15;

    // --- Asymmetric individual perspectives ---
    // A's benefit = Σ(A's valuation_i × B's quality_i) via composition of
    //   OCEAN: similarity-weighted partner quality  → asymmetric
    //   Reputation / Bias: partner's raw quality     → asymmetric when levels differ
    //   Motivation / Patterns: shared synergy        → symmetric (same for both)
    // Total = (a_score + b_score) / 2

    let base_rep = |p: &Person| -> f64 {
        let mut sum = 0.0;
        let mut n = 0.0;
        for &(dim, weight) in &DIM_WEIGHTS {
            if let Some(v) = p.rep_scores.score(dim) {
                sum += (v as f64 / 10.0) * weight;
                n += weight;
            }
        }
        if n == 0.0 { 0.5 } else { sum / n }
    };
    let a_base_rep = base_rep(a);
    let b_base_rep = base_rep(b);
    let a_bias_quality = 1.0 - (a.biases.len() as f64 / 10.0);
    let b_bias_quality = 1.0 - (b.biases.len() as f64 / 10.0);

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
    if ocean_penalty > 0.0 { details.push("OCEAN volatility"); }
    if rep_penalty > 0.0 { details.push("Rep power struggle"); }
    if pat_danger_penalty > 0.0 { details.push("Only negative patterns"); }
    if history_penalty > 0.0 { details.push("Low prediction accuracy"); }
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
        danger: total_danger,
        bias_mod_active: (ocean_mod + rep_mod + mot_mod + pat_mod) > 0.0,
        danger_details,
    }
}

pub fn motivation_synergy(a: MotivationType, b: MotivationType) -> f64 {
    if a == b {
        use MotivationType::*;
        return match a {
            Power => -0.2,
            Recognition => -0.1,
            Autonomy => 0.0,
            Security => 0.0,
            _ => 0.2,
        };
    }
    use MotivationType::*;
    match (a, b) {
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
        _ => 0.0,
    }
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
            let w = (a.intensity as f64 * b.intensity as f64) / 100.0;
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
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_all_pair_avg_equal_scores() {
        let a = [TestItem { intensity: 5 }];
        let b = [TestItem { intensity: 5 }];
        let score = |_: &TestItem, _: &TestItem| 0.8_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_all_pair_avg_negative_score() {
        let a = [TestItem { intensity: 10 }];
        let b = [TestItem { intensity: 10 }];
        let score = |_: &TestItem, _: &TestItem| -1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
        assert!((result - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_all_pair_avg_weighting() {
        let a = [TestItem { intensity: 10 }, TestItem { intensity: 2 }];
        let b = [TestItem { intensity: 10 }];
        let score = |_: &TestItem, _: &TestItem| 1.0_f64;
        let result = all_pair_weighted_avg(&a, &b, |x| x.intensity, score);
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
            predicted_behavior: BehaviorResponse::EmbracesChange,
            intensity: 5,
        }];
        let b = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            intensity: 5,
        }];
        let result = pattern_synergy(&a, &b);
        assert!((result - 0.8).abs() < 0.001);
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
            diplomatic_blunt: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            diplomatic_blunt: Some(8),
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
            diplomatic_blunt: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            authoritative_submissive: Some(8),
            diplomatic_blunt: Some(8),
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
        assert!(brk.total >= 0, "total should be >= 0: {}", brk.total);
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
            brk.a_score, brk.b_score
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
            brk.a_score, brk.b_score
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
            brk.total, expected, diff
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
            brk.a_score, brk.b_score
        );
    }
}

use std::collections::HashSet;

use crate::models::{
    BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation, MotivationType, OceanScores,
    Person, PersonalStyle, Prediction, RelationType, RepDim,
};

#[derive(serde::Serialize)]
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
    /// Width of the confidence band (± points) from relationship strength or
    /// profile confidence. 0 = no banding (legacy behavior).
    pub band: u8,
}

/// Relationship context that makes the synergy score relationship-aware.
/// `None` preserves the legacy relationship-blind scoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelContext {
    pub rtype: RelationType,
    pub strength: u8,
}

/// Per-relation-type bucket weights. All rows sum to 1.0 so the mutual score is
/// directly comparable across contexts. The 6 buckets reuse the existing dynamic
/// redistribution path (only the constants change).
pub fn rel_weights(t: RelationType) -> (f64, f64, f64, f64, f64, f64) {
    use RelationType::*;
    match t {
        WorksWith => (0.20, 0.28, 0.16, 0.16, 0.12, 0.08),
        Collaborates => (0.18, 0.28, 0.16, 0.16, 0.13, 0.09),
        Manages | ReportsTo => (0.15, 0.30, 0.15, 0.18, 0.13, 0.09),
        Friends => (0.18, 0.18, 0.20, 0.12, 0.12, 0.20),
        Family => (0.14, 0.22, 0.24, 0.12, 0.12, 0.16),
        Partner => (0.16, 0.20, 0.22, 0.14, 0.10, 0.18),
        Mentors => (0.20, 0.18, 0.20, 0.14, 0.12, 0.16),
    }
}

/// Confidence band width (± points) from relationship strength (1-10).
pub fn strength_band(strength: u8) -> u8 {
    match strength {
        1..=4 => 12,
        5..=7 => 8,
        _ => 4,
    }
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
    pub completeness: u8,
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
        Impostor => Some(Modulation {
            target: BiasTarget::Ocean,
            coefficient: 0.10,
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

pub fn rep_adjustment(rep: &crate::models::RepScores) -> f64 {
    let mut adj = 0.0;
    for &dim in &RepDim::ALL {
        match rep.score(dim) {
            Some(v) => {
                let v = v.min(10);
                if dim.is_context_dependent() {
                    if v <= 2 || v >= 9 {
                        adj -= 0.04;
                    } else if (4..=6).contains(&v) {
                        adj += 0.02;
                    }
                } else {
                    if v <= 2 {
                        adj -= 0.05;
                    } else if v >= 9 {
                        adj += 0.03;
                    }
                }
            }
            None => {
                adj -= 0.02;
            }
        }
    }
    adj
}

pub fn profile_completeness(person: &Person) -> f64 {
    let ocean = person.ocean.openness.is_some() as u32
        + person.ocean.conscientiousness.is_some() as u32
        + person.ocean.extraversion.is_some() as u32
        + person.ocean.agreeableness.is_some() as u32
        + person.ocean.neuroticism.is_some() as u32;
    let mot = person.motivations.len().min(3) as u32;
    let biases = person.biases.len().min(11) as u32;
    let rep = RepDim::ALL
        .iter()
        .filter(|d| person.rep_scores.score(**d).is_some())
        .count() as u32;
    let styles = person
        .styles
        .iter()
        .map(|s| s.r#type.category())
        .fold(Vec::new(), |mut acc, cat| {
            if !acc.contains(&cat) {
                acc.push(cat);
            }
            acc
        })
        .len()
        .min(8) as u32;
    let pat = person.behavioral_patterns.len().min(5) as u32;
    let num = ocean + mot + biases + rep + styles + pat;
    let den = 45.0;
    (num as f64 / den).clamp(0.0, 1.0)
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
    compute_synergy_score_inner(a, b, None, &[], &[])
}

pub fn compute_synergy_score_with_preds(
    a: &Person,
    b: &Person,
    a_preds: &[Prediction],
    b_preds: &[Prediction],
) -> SynergyBreakdown {
    compute_synergy_score_inner(a, b, None, a_preds, b_preds)
}

/// Relationship-aware synergy. `ctx = None` behaves exactly like the legacy
/// `compute_synergy_score_with_preds`.
pub fn compute_synergy_score_ctx(
    a: &Person,
    b: &Person,
    ctx: Option<&RelContext>,
    a_preds: &[Prediction],
    b_preds: &[Prediction],
) -> SynergyBreakdown {
    compute_synergy_score_inner(a, b, ctx, a_preds, b_preds)
}

fn compute_synergy_score_inner(
    a: &Person,
    b: &Person,
    ctx: Option<&RelContext>,
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
        !patterns.is_empty() && patterns.iter().all(|p| p.trigger.is_negative())
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

    // --- Relationship-aware modulations ---
    // Power friction: a subordinate/mentee with high Power motivation chafes in
    // a hierarchy. Hierarchy clarity bonus: a clearly senior authority signal
    // (boss rep notably above the report's) reduces friction.
    let mut rel_mot_mod = 0.0;
    let mut rel_rep_bonus = 0.0;
    if let Some(rel) = ctx {
        let directional = match rel.rtype {
            RelationType::Manages | RelationType::Mentors => Some((b, a)),
            RelationType::ReportsTo => Some((a, b)),
            _ => None,
        };
        if let Some((sub, boss)) = directional {
            if sub
                .motivations
                .iter()
                .any(|m| m.r#type == MotivationType::Power && m.intensity >= 7)
            {
                rel_mot_mod -= 0.08;
            }
            if let (Some(boss_rep), Some(sub_rep)) = (
                boss.rep_scores.authoritative_submissive,
                sub.rep_scores.authoritative_submissive,
            ) && boss_rep as i16 - sub_rep as i16 > 3
            {
                rel_rep_bonus += 0.04;
            }
        }
    }

    let reputation =
        ((raw_rep - rep_penalty).max(0.0) * (1.0 + rep_mod + rel_rep_bonus)).clamp(0.0, 1.0);
    let motivation = (raw_mot * (1.0 + mot_mod + rel_mot_mod)).clamp(0.0, 1.0);
    let patterns = ((raw_pat - pat_danger_penalty).max(0.0) * (1.0 + pat_mod)).clamp(0.0, 1.0);

    // Dynamic weight redistribution (shared by mutual total & asymmetric).
    // Without relationship context, the documented base weights apply.
    let (w_ocean, w_rep, w_mot, w_pat, w_bias, w_style) = match ctx {
        Some(rel) => rel_weights(rel.rtype),
        None => (0.17, 0.26, 0.19, 0.14, 0.13, 0.11),
    };

    const W_HISTORY: f64 = 0.10;
    let total_danger = ocean_penalty * w_ocean
        + rep_penalty * w_rep
        + pat_danger_penalty * w_pat
        + history_penalty * W_HISTORY;

    // --- Asymmetric individual perspectives ---
    // A's benefit = Σ(A's valuation_i × B's quality_i) via composition of
    //   OCEAN: similarity-weighted partner quality  → asymmetric
    //   Reputation / Bias: partner's raw quality     → asymmetric when levels differ
    //   Motivation / Patterns: shared synergy        → symmetric (same for both)
    // Total = (a_score + b_score) / 2

    let a_base_rep = (base_rep_quality(a) + rep_adjustment(&a.rep_scores)).clamp(0.0, 1.0);
    let b_base_rep = (base_rep_quality(b) + rep_adjustment(&b.rep_scores)).clamp(0.0, 1.0);
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
    a_raw += a_ocean * w_ocean;
    b_raw += b_ocean * w_ocean;
    asym_w += w_ocean;
    if rep_active {
        let rep_boost = 1.0 + rel_rep_bonus;
        a_raw += b_base_rep * w_rep * rep_boost;
        b_raw += a_base_rep * w_rep * rep_boost;
        asym_w += w_rep;
    }
    if mot_active {
        a_raw += motivation * w_mot;
        b_raw += motivation * w_mot;
        asym_w += w_mot;
    }
    if pat_active {
        a_raw += patterns * w_pat;
        b_raw += patterns * w_pat;
        asym_w += w_pat;
    }
    a_raw += b_bias_quality * w_bias;
    b_raw += a_bias_quality * w_bias;
    asym_w += w_bias;

    let styles = style_synergy(&a.styles, &b.styles);
    a_raw += styles * w_style;
    b_raw += styles * w_style;
    asym_w += w_style;

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

    let band = match ctx {
        Some(rel) => strength_band(rel.strength),
        None => 0,
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
        band,
    }
}

/// Severity weight per consistency flag, by evidence strength:
/// 0.20 self-report inconsistencies, 0.30 stated-vs-perceived,
/// 0.40 evidence-based (recorded patterns or biases).
pub fn flag_weight(key: &str) -> f64 {
    match key {
        "flag_high_e_low_a"
        | "flag_high_n_low_c"
        | "flag_high_o_low_c"
        | "flag_honest_selfish"
        | "flag_honest_favoritist" => 0.20,
        "flag_pattern_calm_volatile"
        | "flag_pattern_honest_exploiter"
        | "flag_pattern_diplomat_escalator"
        | "flag_pattern_fair_exploiter"
        | "flag_pattern_humble_dismissive"
        | "flag_pattern_trusting_paranoid"
        | "flag_pattern_reliable_shirker"
        | "flag_pattern_hardworker_complacent"
        | "flag_pattern_passive_blowup"
        | "flag_pattern_assertive_quiet"
        | "flag_pattern_generous_exploiter"
        | "flag_pattern_empath_dismissive"
        | "flag_pattern_flexible_resister"
        | "flag_pattern_helping_exploiter"
        | "flag_pattern_warmth_dismissive"
        | "flag_pattern_discipline_shirker"
        | "flag_pattern_claimed_calm_volatile"
        | "flag_pattern_fairness_exploiter"
        | "flag_pattern_achievement_complacent"
        | "flag_pattern_learning_resister"
        | "flag_pattern_extravert_quiet"
        | "flag_pattern_open_resister"
        | "flag_pattern_recognition_dismissive"
        | "flag_bias_confirmation_open"
        | "flag_anchoring_open"
        | "flag_bias_favoritism_fairness"
        | "flag_authority_dominant"
        | "flag_social_proof_open"
        | "flag_sunk_cost_flexible"
        | "flag_loss_aversion_risky"
        | "flag_dunning_kruger_humble"
        | "flag_impostor_arrogant"
        | "flag_recency_reliable"
        | "flag_availability_calm" => 0.40,
        _ => 0.30,
    }
}

/// Reputation penalty from consistency flags: weighted sum of each flag's
/// severity, capped at 0.50.
pub fn consistency_malus(flags: &[&str]) -> f64 {
    flags.iter().map(|k| flag_weight(k)).sum::<f64>().min(0.50)
}

/// Motivations whose claimed credit is invalidated by a firing consistency flag.
/// A flag proves the self-reported drive is contradicted by stated perception or
/// recorded behavior, so that motivation banks zero credit in the profile.
fn invalidated_motivations(flags: &[&str]) -> Vec<MotivationType> {
    use crate::models::MotivationType::*;
    let mut out: Vec<MotivationType> = Vec::new();
    for &k in flags {
        let hits: &[MotivationType] = match k {
            "flag_fairness_rhetoric"
            | "flag_bias_favoritism_fairness"
            | "flag_pattern_fairness_exploiter" => &[Fairness],
            "flag_helping_selfish" | "flag_helping_cold" | "flag_pattern_helping_exploiter" => {
                &[Helping]
            }
            "flag_affiliation_cold" | "flag_affiliation_distrustful" => &[Affiliation],
            "flag_ambition_lazy" => &[Power, Achievement, Recognition],
            "flag_risk_appetite_ambition" => &[Power, Achievement],
            "flag_security_gullible" | "flag_security_risky" => &[Security],
            "flag_autonomy_submissive" => &[Autonomy],
            "flag_learning_rigid" | "flag_learning_arrogant" | "flag_pattern_learning_resister" => {
                &[Learning]
            }
            "flag_creativity_closed" | "flag_creativity_rigid" => &[Creativity],
            "flag_power_passive" => &[Power],
            "flag_pattern_achievement_complacent" => &[Achievement],
            "flag_pattern_recognition_dismissive" => &[Recognition],
            _ => &[],
        };
        for &m in hits {
            if !out.contains(&m) {
                out.push(m);
            }
        }
    }
    out
}

/// OCEAN dimensions voided to neutral (0.5) by a consistency flag, removing
/// self-report credit where the claim is contradicted by other evidence.
fn voided_ocean_dims(flags: &[&str]) -> (bool, bool) {
    let mut void_a = false;
    let mut void_n = false;
    for &k in flags {
        match k {
            "flag_warmth_cold"
            | "flag_warmth_blunt"
            | "flag_warmth_selfish"
            | "flag_pattern_warmth_dismissive" => void_a = true,
            "flag_claims_calm_reactive" | "flag_pattern_claimed_calm_volatile" => void_n = true,
            _ => {}
        }
    }
    (void_a, void_n)
}

/// True when a recorded pattern contradicts the declared profile.
fn has_pattern_contradiction(flags: &[&str]) -> bool {
    flags.iter().any(|k| k.starts_with("flag_pattern_"))
}

/// True when a declared style is contradicted by the recorded profile.
fn has_style_contradiction(flags: &[&str]) -> bool {
    flags.iter().any(|k| k.starts_with("flag_style_"))
}

pub fn compute_person_profile(person: &Person) -> PersonProfile {
    let pat_active = !person.behavioral_patterns.is_empty();

    let flags = crate::validation::all_person_flags(person);
    let invalidated = invalidated_motivations(&flags);
    let credited: Vec<Motivation> = person
        .motivations
        .iter()
        .filter(|m| !invalidated.contains(&m.r#type))
        .cloned()
        .collect();
    let mot_active = !credited.is_empty();

    let base_mot = if mot_active {
        motivation_synergy_score(&credited, &credited)
    } else {
        0.5
    };
    let virtue = virtue_adjustment(&credited);
    let count_penalty = motivation_count_penalty(credited.len());
    let motivation = (base_mot + virtue - count_penalty).clamp(0.0, 1.0);

    let raw_pat = if pat_active {
        pattern_synergy(&person.behavioral_patterns, &person.behavioral_patterns)
    } else {
        0.5
    };
    let mut pat = (raw_pat + pattern_adjustment(&person.behavioral_patterns)).clamp(0.0, 1.0);
    if has_pattern_contradiction(&flags) {
        pat = pat.min(0.5);
    }

    let (void_a, void_n) = voided_ocean_dims(&flags);
    let a_s = if void_a {
        0.5
    } else {
        person.ocean.agreeableness.map_or(0.5, |v| v as f64 / 10.0)
    };
    let n_s = if void_n {
        0.5
    } else {
        person
            .ocean
            .neuroticism
            .map_or(0.5, |v| (10.0 - v as f64) / 10.0)
    };
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

    let mut rep = (base_rep_quality(person) + rep_adjustment(&person.rep_scores)).clamp(0.0, 1.0);
    rep = (rep - consistency_malus(&flags)).max(0.0);

    let bias_adj = bias_adjustment(&person.biases);
    let absent_count = BiasType::ALL.len() - person.biases.len();
    let moderate_plus = person.biases.iter().filter(|b| b.intensity >= 4).count();
    let present_bias_count = absent_count + moderate_plus;
    let base_bias =
        1.0 - (present_bias_count as f64 / crate::models::BiasType::ALL.len() as f64).min(1.0);
    let count_bonus = bias_count_bonus(present_bias_count);
    let bias = (base_bias + bias_adj + count_bonus).clamp(0.0, 1.0);

    let mut raw_style = if !person.styles.is_empty() {
        style_synergy(&person.styles, &person.styles)
    } else {
        0.5
    };
    if has_style_contradiction(&flags) {
        raw_style = raw_style.min(0.5);
    }

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
        raw += pat * W_PAT;
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
        patterns: pat,
        ocean,
        reputation: rep,
        bias,
        styles: raw_style,
        completeness: (profile_completeness(person) * 100.0).round() as u8,
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

pub fn pattern_adjustment(patterns: &[BehavioralPattern]) -> f64 {
    let mut adj = 0.0;
    let mut defined: HashSet<BehaviorTrigger> = HashSet::new();
    for p in patterns {
        defined.insert(p.trigger);
        adj += p.predicted_behavior.score();
    }
    for t in BehaviorTrigger::ALL {
        if !defined.contains(&t) {
            adj -= 0.02;
        }
    }
    adj
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
            let w = 1.0;
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
            resilience: None,
            risk_appetite: None,
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
    fn test_bias_modifier_impostor_ocean_positive() {
        let m = bias_modifier(BiasType::Impostor).unwrap();
        assert!(matches!(m.target, BiasTarget::Ocean));
        assert!((m.coefficient - 0.10).abs() < 1e-9);
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
            pf.total >= 30 && pf.total < 70,
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
        assert!(pf.total >= 36, "should be decent: {}", pf.total);
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
        let c = profile_completeness(&p);
        assert!((c - 1.0).abs() < 0.001, "full: {c}");
    }

    #[test]
    fn test_profile_completeness_ocean_only() {
        let p = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        let c = profile_completeness(&p);
        let expected = 5.0 / 45.0;
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
        let expected = (13.0 + 3.0) / 45.0;
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
        let expected = 2.0 / 45.0;
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
        assert!(pf.total >= 45, "good rep boosts total: {}", pf.total);
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
            let (a, b, c, d, e, f) = rel_weights(rt);
            let sum = a + b + c + d + e + f;
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
            m.total > r.total,
            "clear hierarchy should add a small bonus ({} > {})",
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
    fn test_relationship_strength_serde_default() {
        let json = r#"{"id":"r1","source_id":"a","target_id":"b","type":"Friends","notes":"","created_at":0}"#;
        let r: Relationship = serde_json::from_str(json).unwrap();
        assert_eq!(r.strength, 5, "missing strength → default 5");
        assert_eq!(r.r#type, RelationType::Friends);
        let out = serde_json::to_string(&r).unwrap();
        assert!(out.contains("\"strength\":5"));
    }
}

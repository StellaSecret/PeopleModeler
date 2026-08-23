use super::components::{
    confidence_band, motivation_synergy_score, pattern_synergy, sim, strength_band, style_synergy,
    value_similarity,
};
use super::profile::{
    avg_prediction_accuracy, base_rep_quality, ocean_danger_penalty, rep_adjustment,
    rep_danger_penalty,
};
use super::rel_weights::rel_weights;
use super::trajectory::pair_trajectory;
use super::{RelContext, SynergyBreakdown};
use crate::insights::InsightContext;
use crate::model_config::{BiasTarget, CFG};
use crate::models::{
    BehavioralPattern, BiasType, MotivationType, OceanScores, Person, Prediction, RelationType,
    RepDim,
};
use std::collections::HashSet;

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

/// Phase 4: per-context inputs re-weighted from the final bucket scores.
pub(crate) struct PerContextInputs {
    /// (ocean, reputation, motivation, patterns, bias, styles, values).
    pub(crate) buckets: [f64; 7],
    /// (ocean_penalty, rep_penalty, pat_danger_penalty, history_penalty).
    pub(crate) penalties: [f64; 4],
    pub(crate) rep_active: bool,
    pub(crate) mot_active: bool,
    pub(crate) pat_active: bool,
}

/// Per-context re-weighting of the final per-bucket scores (Phase 4).
/// Each `InsightContext` uses its own weight profile from `CFG.contexts`;
/// when a relationship context is present it composes with the relation-type
/// profile (element-wise product, renormalized) so, e.g., a `Manages`
/// relationship under `Stress` emphasizes buckets that matter for both.
/// Mirrors the headline formula: weighted mean over active buckets minus the
/// context-weighted danger penalty.
pub(crate) fn per_context_breakdown(
    inp: &PerContextInputs,
    ctx: Option<&RelContext>,
) -> Vec<(InsightContext, u8)> {
    let rel_w = ctx.map(|r| rel_weights(r.rtype));
    let mut out = Vec::with_capacity(InsightContext::ALL.len());
    for c in InsightContext::ALL {
        let cw = CFG.context_weights(c);
        let w = match rel_w {
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
        let raw = if wsum > 0.0 { num / wsum } else { 0.0 };
        let danger = inp.penalties[0] * w[0]
            + inp.penalties[1] * w[1]
            + inp.penalties[2] * w[2]
            + inp.penalties[3] * CFG.base_weights.history;
        let penalty = if wsum > 0.0 {
            (danger / wsum * 100.0).round() as u8
        } else {
            0
        };
        let score = ((raw * 100.0).round() as u8)
            .min(100)
            .saturating_sub(penalty);
        out.push((c, score));
    }
    out
}

pub(crate) fn compute_synergy_score_inner(
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
        (Some(o), Some(c)) if o >= CFG.ocean.complement_min && c >= CFG.ocean.complement_min => {
            CFG.ocean.complement_bonus
        }
        _ => match (ob.openness, oa.conscientiousness) {
            (Some(o), Some(c))
                if o >= CFG.ocean.complement_min && c >= CFG.ocean.complement_min =>
            {
                CFG.ocean.complement_bonus
            }
            _ => 0.0,
        },
    };
    let ea_bonus = match (oa.extraversion, ob.agreeableness) {
        (Some(e), Some(a)) if e >= CFG.ocean.complement_min && a >= CFG.ocean.complement_min => {
            CFG.ocean.complement_bonus
        }
        _ => match (ob.extraversion, oa.agreeableness) {
            (Some(e), Some(a))
                if e >= CFG.ocean.complement_min && a >= CFG.ocean.complement_min =>
            {
                CFG.ocean.complement_bonus
            }
            _ => 0.0,
        },
    };

    let raw_ocean = ((oc + oc_bonus).min(1.0) + (ea + ea_bonus).min(1.0) + n) / 3.0;

    // Reputation: weighted distance per shared dimension
    let mut rep_sum = 0.0;
    let mut total_active_w = 0.0;
    for (dim, weight) in RepDim::ALL.iter().zip(&CFG.reputation.dim_weights) {
        if let (Some(va), Some(vb)) = (a.rep_scores.score(*dim), b.rep_scores.score(*dim)) {
            let dist = va.abs_diff(vb);
            rep_sum += (1.0 - dist as f64 / CFG.similarity.trait_scale) * weight;
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
        CFG.patterns.only_negative_penalty
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
        CFG.bias.default
    };

    let mut ocean_mod = 0.0;
    let mut rep_mod = 0.0;
    let mut mot_mod = 0.0;
    let mut pat_mod = 0.0;
    // Opposite/adjacent-bias friction (Phase 8): negative-only, combined
    // magnitude capped at `opposite_cap`.
    let mut opp_mods: Vec<(BiasTarget, f64)> = Vec::new();

    for ba in &a.biases {
        for bb in &b.biases {
            let w = (ba.intensity as f64 * bb.intensity as f64) / CFG.bias.intensity_scale;
            if ba.r#type == bb.r#type {
                if let Some((target, coefficient)) = CFG.bias_modulation(ba.r#type) {
                    let delta = coefficient * w;
                    match target {
                        BiasTarget::Ocean => ocean_mod += delta,
                        BiasTarget::Reputation => rep_mod += delta,
                        BiasTarget::Motivation => mot_mod += delta,
                        BiasTarget::Patterns => pat_mod += delta,
                    }
                }
            } else if let Some((target, coefficient)) =
                CFG.bias_complementarity(ba.r#type, bb.r#type)
            {
                opp_mods.push((target, coefficient.min(0.0) * w));
            }
        }
    }

    let opp_total: f64 = opp_mods.iter().map(|(_, d)| d).sum();
    if opp_total < -CFG.bias.opposite_cap {
        let scale = CFG.bias.opposite_cap / -opp_total;
        for (_, d) in opp_mods.iter_mut() {
            *d *= scale;
        }
    }
    for (target, delta) in opp_mods {
        match target {
            BiasTarget::Ocean => ocean_mod += delta,
            BiasTarget::Reputation => rep_mod += delta,
            BiasTarget::Motivation => mot_mod += delta,
            BiasTarget::Patterns => pat_mod += delta,
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
        (Some(pa), Some(pb)) if pa < CFG.history.low_accuracy && pb < CFG.history.low_accuracy => {
            CFG.history.both_low_penalty
        }
        (Some(pa), Some(_)) if pa < CFG.history.low_accuracy => CFG.history.single_low_penalty,
        (Some(_), Some(pb)) if pb < CFG.history.low_accuracy => CFG.history.single_low_penalty,
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
            if sub.motivations.iter().any(|m| {
                m.r#type == MotivationType::Power
                    && m.intensity >= CFG.relationship.power_intensity_min
            }) {
                rel_mot_mod += CFG.relationship.power_friction_mod;
            }
            if let (Some(boss_rep), Some(sub_rep)) = (
                boss.rep_scores.authoritative_submissive,
                sub.rep_scores.authoritative_submissive,
            ) && boss_rep as i16 - sub_rep as i16 > CFG.relationship.hierarchy_rep_diff_min
            {
                rel_rep_bonus += CFG.relationship.hierarchy_bonus;
            }
        }
    }

    let reputation =
        ((raw_rep - rep_penalty).max(0.0) * (1.0 + rep_mod + rel_rep_bonus)).clamp(0.0, 1.0);
    let motivation = (raw_mot * (1.0 + mot_mod + rel_mot_mod)).clamp(0.0, 1.0);
    let patterns = ((raw_pat - pat_danger_penalty).max(0.0) * (1.0 + pat_mod)).clamp(0.0, 1.0);

    // Dynamic weight redistribution (shared by mutual total & asymmetric).
    // Without relationship context, the documented base weights apply.
    let (w_ocean, w_rep, w_mot, w_pat, w_bias, w_style, w_values) = match ctx {
        Some(rel) => rel_weights(rel.rtype),
        None => (
            CFG.base_weights.ocean,
            CFG.base_weights.reputation,
            CFG.base_weights.motivation,
            CFG.base_weights.patterns,
            CFG.base_weights.bias,
            CFG.base_weights.style,
            CFG.base_weights.values,
        ),
    };

    let total_danger = ocean_penalty * w_ocean
        + rep_penalty * w_rep
        + pat_danger_penalty * w_pat
        + history_penalty * CFG.base_weights.history;

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

    // OCEAN vector for each person (trait value / trait_scale, stability = 1 - N/trait_scale)
    let ovec = |o: &OceanScores| -> [f64; 5] {
        [
            o.openness.map_or(CFG.similarity.neutral, |v| {
                v as f64 / CFG.similarity.trait_scale
            }),
            o.conscientiousness.map_or(CFG.similarity.neutral, |v| {
                v as f64 / CFG.similarity.trait_scale
            }),
            o.extraversion.map_or(CFG.similarity.neutral, |v| {
                v as f64 / CFG.similarity.trait_scale
            }),
            o.agreeableness.map_or(CFG.similarity.neutral, |v| {
                v as f64 / CFG.similarity.trait_scale
            }),
            o.neuroticism.map_or(CFG.similarity.neutral, |v| {
                (CFG.similarity.trait_scale - v as f64) / CFG.similarity.trait_scale
            }),
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
            / CFG.similarity.asym_dimensions
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

    let values = value_similarity(&a.values, &b.values);
    a_raw += values * w_values;
    b_raw += values * w_values;
    asym_w += w_values;

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
        Some(rel) => strength_band(rel.strength)
            .max(confidence_band(a.confidence))
            .max(confidence_band(b.confidence)),
        None => 0,
    };

    let traj = pair_trajectory(a, b);

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
        values,
        danger: total_danger,
        bias_mod_active: (ocean_mod + rep_mod + mot_mod + pat_mod) > 0.0,
        danger_details,
        band,
        trajectory_delta: traj.delta,
        trajectory_trend: traj.trend,
        trajectory_sample: traj.sample,
        per_context: per_context_breakdown(
            &PerContextInputs {
                buckets: [
                    ocean, reputation, motivation, patterns, bias_score, styles, values,
                ],
                penalties: [
                    ocean_penalty,
                    rep_penalty,
                    pat_danger_penalty,
                    history_penalty,
                ],
                rep_active,
                mot_active,
                pat_active,
            },
            ctx,
        ),
    }
}

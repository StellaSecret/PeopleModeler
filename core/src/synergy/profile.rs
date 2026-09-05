use super::PersonProfile;
use super::components::{
    bias_adjustment, bias_count_bonus, confidence_band, motivation_count_penalty,
    motivation_synergy_score, pattern_adjustment, pattern_synergy, style_synergy, value_self_score,
    virtue_adjustment,
};
use crate::model_config::CFG;
use crate::models::{BiasType, Motivation, MotivationType, Person, Prediction, RepDim};

pub(crate) fn avg_prediction_accuracy(predictions: &[Prediction]) -> Option<f64> {
    let resolved: Vec<_> = predictions
        .iter()
        .filter(|p| p.resolved && p.accuracy.is_some())
        .collect();
    if resolved.len() < CFG.history.min_samples {
        return None;
    }
    let sum: f64 = resolved.iter().map(|p| p.accuracy.unwrap() as f64).sum();
    Some(sum / resolved.len() as f64)
}

pub fn base_rep_quality(p: &Person) -> f64 {
    let mut sum = 0.0;
    let mut n = 0.0;
    for (dim, weight) in RepDim::ALL.iter().zip(&CFG.reputation.dim_weights) {
        if let Some(v) = p.rep_scores.score(*dim) {
            sum += (v as f64 / CFG.similarity.trait_scale) * weight;
            n += weight;
        }
    }
    if n == 0.0 { 0.5 } else { sum / n }
}

pub fn rep_adjustment(rep: &crate::models::RepScores) -> f64 {
    let adj = CFG.reputation.adjust;
    let mut total = 0.0;
    for &dim in &RepDim::ALL {
        match rep.score(dim) {
            Some(v) => {
                let v = v.min(10);
                if dim.is_context_dependent() {
                    if v <= adj.extreme_low || v >= adj.extreme_high {
                        total -= adj.context_extreme;
                    } else if (adj.mid_low..=adj.mid_high).contains(&v) {
                        total += adj.context_mid;
                    }
                } else {
                    if v <= adj.extreme_low {
                        total -= adj.non_context_low;
                    } else if v >= adj.extreme_high {
                        total += adj.non_context_high;
                    }
                }
            }
            None => {
                total -= adj.missing;
            }
        }
    }
    total
}

pub fn profile_completeness(person: &Person) -> f64 {
    let ocean = person.ocean.openness.is_some() as u32
        + person.ocean.conscientiousness.is_some() as u32
        + person.ocean.extraversion.is_some() as u32
        + person.ocean.agreeableness.is_some() as u32
        + person.ocean.neuroticism.is_some() as u32;
    let mot = person
        .motivations
        .len()
        .min(CFG.completeness.motivation_cap) as u32;
    let biases = person.biases.len().min(CFG.completeness.bias_cap) as u32;
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
        .min(CFG.completeness.style_cap) as u32;
    let pat = person
        .behavioral_patterns
        .len()
        .min(CFG.completeness.pattern_cap) as u32;
    let vals = person.values.len().min(CFG.completeness.values_cap) as u32;
    let num = ocean + mot + biases + rep + styles + pat + vals;
    let den = CFG.completeness.denominator;
    (num as f64 / den).clamp(0.0, 1.0)
}

pub(crate) fn ocean_danger_penalty(
    oa: &crate::models::OceanScores,
    ob: &crate::models::OceanScores,
) -> f64 {
    let d = CFG.ocean.danger;
    let mut p = 0.0;

    // Within-person: volatile (N high and A low)
    if oa.neuroticism.is_some_and(|n| n >= d.high) && oa.agreeableness.is_some_and(|a| a <= d.low) {
        p += d.within_volatile;
    }
    if ob.neuroticism.is_some_and(|n| n >= d.high) && ob.agreeableness.is_some_and(|a| a <= d.low) {
        p += d.within_volatile;
    }

    // Within-person: impulsive (N high and C low)
    if oa.neuroticism.is_some_and(|n| n >= d.high)
        && oa.conscientiousness.is_some_and(|c| c <= d.low)
    {
        p += d.within_impulsive;
    }
    if ob.neuroticism.is_some_and(|n| n >= d.high)
        && ob.conscientiousness.is_some_and(|c| c <= d.low)
    {
        p += d.within_impulsive;
    }

    // Within-person: rigid anxious (N high and O low)
    if oa.neuroticism.is_some_and(|n| n >= d.high) && oa.openness.is_some_and(|o| o <= d.low) {
        p += d.within_rigid;
    }
    if ob.neuroticism.is_some_and(|n| n >= d.high) && ob.openness.is_some_and(|o| o <= d.low) {
        p += d.within_rigid;
    }

    // Cross-person: emotional contagion (both N high)
    if oa.neuroticism.is_some_and(|n| n >= d.high) && ob.neuroticism.is_some_and(|n| n >= d.high) {
        p += d.contagion;
    }

    // Cross-person: antagonism (both A low)
    if oa.agreeableness.is_some_and(|a| a <= d.low) && ob.agreeableness.is_some_and(|a| a <= d.low)
    {
        p += d.antagonism;
    }

    // Cross-person: mutual unreliability (both C low)
    if oa.conscientiousness.is_some_and(|c| c <= d.low)
        && ob.conscientiousness.is_some_and(|c| c <= d.low)
    {
        p += d.unreliability;
    }

    // Cross-person: mutual rigidity (both O low)
    if oa.openness.is_some_and(|o| o <= d.low) && ob.openness.is_some_and(|o| o <= d.low) {
        p += d.rigidity;
    }

    p
}

pub(crate) fn rep_danger_penalty(
    rep_a: &crate::models::RepScores,
    rep_b: &crate::models::RepScores,
) -> f64 {
    let d = CFG.reputation.danger;
    let mut p = 0.0;

    // Both authoritative >= high → power struggle
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::AuthoritativeSubmissive),
        rep_b.score(RepDim::AuthoritativeSubmissive),
    ) && aa >= d.high
        && ab >= d.high
    {
        p += d.power_struggle;
    }

    // Both blunt → brutal honesty, no diplomacy
    // score: 10 = Diplomatic (pole A), 0 = Blunt (pole B)
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::DiplomaticBlunt),
        rep_b.score(RepDim::DiplomaticBlunt),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.brutal;
    }

    // Both reactive → mutual escalation
    // score: 10 = Calm (pole A), 0 = Reactive (pole B)
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::CalmReactive),
        rep_b.score(RepDim::CalmReactive),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.escalation;
    }

    // Both arrogant → neither concedes
    // score: 10 = Humble (pole A), 0 = Arrogant (pole B)
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HumbleArrogant),
        rep_b.score(RepDim::HumbleArrogant),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.no_concede;
    }

    // Both lazy → mutual passivity
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HardworkerLazy),
        rep_b.score(RepDim::HardworkerLazy),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.passivity;
    }

    // Both untrusting → mutual suspicion
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::TrustingSuspicious),
        rep_b.score(RepDim::TrustingSuspicious),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.suspicion;
    }

    // Both detached → mutual coldness
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::EmpatheticDetached),
        rep_b.score(RepDim::EmpatheticDetached),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.coldness;
    }

    // Both deceitful → trust collapse
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::HonestDeceitful),
        rep_b.score(RepDim::HonestDeceitful),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.trust_collapse;
    }

    // Both flaky → mutual unreliability
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::ReliableFlaky),
        rep_b.score(RepDim::ReliableFlaky),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.unreliability;
    }

    // Both unfair → cronyism
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::FairFavoritism),
        rep_b.score(RepDim::FairFavoritism),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.cronyism;
    }

    // Both selfish → mutual hoarding
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::GenerousSelfish),
        rep_b.score(RepDim::GenerousSelfish),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.hoarding;
    }

    // Both passive → decision paralysis
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::AssertivePassive),
        rep_b.score(RepDim::AssertivePassive),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.paralysis;
    }

    // Both rigid → gridlock
    if let (Some(aa), Some(ab)) = (
        rep_a.score(RepDim::AdaptableRigid),
        rep_b.score(RepDim::AdaptableRigid),
    ) && aa <= d.low
        && ab <= d.low
    {
        p += d.gridlock;
    }

    p
}

pub fn flag_weight(key: &str) -> f64 {
    match key {
        "flag_high_e_low_a"
        | "flag_high_n_low_c"
        | "flag_high_o_low_c"
        | "flag_honest_selfish"
        | "flag_honest_favoritist"
        | "flag_value_family_past"
        | "flag_value_stability_risk"
        | "flag_value_career_family"
        | "flag_value_loyalty_guarded"
        | "flag_value_health_risky"
        | "flag_value_wealth_generous"
        | "flag_value_faith_deceitful" => CFG.flags.self_report,
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
        | "flag_availability_calm" => CFG.flags.evidence,
        "flag_style_controlling"
        | "flag_style_manipulative"
        | "flag_style_passive_aggressive"
        | "flag_style_detached" => CFG.flags.style_consistent,
        _ => CFG.flags.stated_perceived,
    }
}

/// Reputation penalty from consistency flags: weighted sum of each flag's
/// severity, capped at the configured maximum.
pub fn consistency_malus(flags: &[&str]) -> f64 {
    flags
        .iter()
        .map(|k| flag_weight(k))
        .sum::<f64>()
        .min(CFG.flags.malus_cap)
}

/// Motivations whose claimed credit is invalidated by a firing consistency flag.
/// A flag proves the self-reported drive is contradicted by stated perception or
/// recorded behavior, so that motivation banks zero credit in the profile.
pub(crate) fn invalidated_motivations(flags: &[&str]) -> Vec<MotivationType> {
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
pub(crate) fn voided_ocean_dims(flags: &[&str]) -> (bool, bool) {
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
pub(crate) fn has_pattern_contradiction(flags: &[&str]) -> bool {
    flags.iter().any(|k| k.starts_with("flag_pattern_"))
}

/// True when a declared style is contradicted by the recorded profile.
pub(crate) fn has_style_contradiction(flags: &[&str]) -> bool {
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
        CFG.profile.default_motivation
    };
    let virtue = virtue_adjustment(&credited);
    let count_penalty = motivation_count_penalty(credited.len());
    let motivation = (base_mot + virtue - count_penalty).clamp(0.0, 1.0);

    let raw_pat = if pat_active {
        pattern_synergy(&person.behavioral_patterns, &person.behavioral_patterns)
    } else {
        CFG.profile.default_patterns
    };
    let mut pat = (raw_pat + pattern_adjustment(&person.behavioral_patterns)).clamp(0.0, 1.0);
    if has_pattern_contradiction(&flags) {
        pat = pat.min(CFG.profile.contradiction_cap);
    }

    let (void_a, void_n) = voided_ocean_dims(&flags);
    let neutral = CFG.similarity.neutral;
    let a_s = if void_a {
        neutral
    } else {
        person
            .ocean
            .agreeableness
            .map_or(0.0, |v| v as f64 / CFG.similarity.trait_scale)
    };
    let n_s = if void_n {
        neutral
    } else {
        person.ocean.neuroticism.map_or(0.0, |v| {
            (CFG.similarity.trait_scale - v as f64) / CFG.similarity.trait_scale
        })
    };
    let d = CFG.ocean.danger;
    let mut ocean_penalty = 0.0;
    if person.ocean.neuroticism.is_some_and(|n| n >= d.high)
        && person.ocean.agreeableness.is_some_and(|a| a <= d.low)
    {
        ocean_penalty += d.within_volatile;
    }
    if person.ocean.neuroticism.is_some_and(|n| n >= d.high)
        && person.ocean.conscientiousness.is_some_and(|c| c <= d.low)
    {
        ocean_penalty += d.within_impulsive;
    }
    if person.ocean.neuroticism.is_some_and(|n| n >= d.high)
        && person.ocean.openness.is_some_and(|o| o <= d.low)
    {
        ocean_penalty += d.within_rigid;
    }

    let raw_ocean = (a_s + n_s) / 2.0;
    let ocean = (raw_ocean - ocean_penalty).max(0.0);

    let mut rep = (base_rep_quality(person) + rep_adjustment(&person.rep_scores)).clamp(0.0, 1.0);
    rep = (rep - consistency_malus(&flags)).max(0.0);

    let bias_adj = bias_adjustment(&person.biases);
    let absent_count = BiasType::ALL.len() - person.biases.len();
    let moderate_plus = person
        .biases
        .iter()
        .filter(|b| b.intensity >= CFG.bias.moderate_min)
        .count();
    let present_bias_count = absent_count + moderate_plus;
    let base_bias =
        1.0 - (present_bias_count as f64 / crate::models::BiasType::ALL.len() as f64).min(1.0);
    let count_bonus = bias_count_bonus(present_bias_count);
    let bias = (base_bias + bias_adj + count_bonus).clamp(0.0, 1.0);

    let mut raw_style = if !person.styles.is_empty() {
        style_synergy(&person.styles, &person.styles)
    } else {
        CFG.profile.default_style
    };
    if has_style_contradiction(&flags) {
        raw_style = raw_style.min(CFG.profile.contradiction_cap);
    }

    let mut total_w = 0.0;
    let mut raw = 0.0;
    raw += motivation * CFG.base_weights.motivation;
    total_w += CFG.base_weights.motivation;
    if pat_active {
        raw += pat * CFG.base_weights.patterns;
        total_w += CFG.base_weights.patterns;
    }
    raw += ocean * CFG.base_weights.ocean;
    total_w += CFG.base_weights.ocean;
    raw += rep * CFG.base_weights.reputation;
    total_w += CFG.base_weights.reputation;
    raw += bias * CFG.base_weights.bias;
    total_w += CFG.base_weights.bias;
    raw += raw_style * CFG.base_weights.style;
    total_w += CFG.base_weights.style;

    let val = value_self_score(&person.values);
    raw += val * CFG.base_weights.values;
    total_w += CFG.base_weights.values;

    let total = ((raw / total_w * 100.0).round() as u8).min(100);

    PersonProfile {
        total,
        motivation,
        patterns: pat,
        ocean,
        reputation: rep,
        bias,
        styles: raw_style,
        values: val,
        completeness: (profile_completeness(person) * 100.0).round() as u8,
        band: confidence_band(person.confidence),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controlling_flag_weight_is_mild() {
        for flag in [
            "flag_style_controlling",
            "flag_style_manipulative",
            "flag_style_passive_aggressive",
            "flag_style_detached",
        ] {
            let w = flag_weight(flag);
            assert_eq!(w, CFG.flags.style_consistent);
            // A self-consistent style (control freak / confirmed manipulator / PA /
            // detached) is a real but milder signal than a self-perception
            // contradiction or a pattern flag.
            assert!(
                w < CFG.flags.stated_perceived,
                "{flag} should be milder than a stated/perceived gap"
            );
            assert!(w > 0.0);
        }
    }

    #[test]
    fn consistency_malus_counts_controlling_flag() {
        let before = consistency_malus(&[]);
        let after = consistency_malus(&["flag_style_controlling"]);
        assert!((after - before - CFG.flags.style_consistent).abs() < 1e-9);
        let both = consistency_malus(&[
            "flag_style_controlling",
            "flag_style_manipulative",
            "flag_style_passive_aggressive",
            "flag_style_detached",
        ]);
        assert!((both - before - 4.0 * CFG.flags.style_consistent).abs() < 1e-9);
    }

    #[test]
    fn value_flags_weight_to_self_report() {
        for flag in [
            "flag_value_health_risky",
            "flag_value_wealth_generous",
            "flag_value_faith_deceitful",
        ] {
            assert_eq!(flag_weight(flag), CFG.flags.self_report);
        }
    }
}

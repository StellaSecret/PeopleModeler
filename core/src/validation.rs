use crate::model_config::CFG;
use crate::models::{
    BehaviorResponse, BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation,
    MotivationType, OceanScores, Person, PersonalStyle, RepScores, StyleType,
};

const HIGH: u8 = CFG.validation.high;
const LOW: u8 = CFG.validation.low;
const MOTIVATION_HIGH: u8 = CFG.validation.motivation_high;
const BIAS_HIGH: u8 = CFG.validation.bias_high;
const STYLE_HIGH: u8 = CFG.validation.style_high;

pub fn ocean_rep_flags(ocean: &OceanScores, rep: &RepScores) -> Vec<&'static str> {
    let mut flags = Vec::new();

    if ocean.extraversion >= Some(HIGH) && ocean.agreeableness.is_some_and(|a| a <= LOW) {
        flags.push("flag_high_e_low_a");
    }
    if ocean.neuroticism >= Some(HIGH) && ocean.conscientiousness.is_some_and(|c| c <= LOW) {
        flags.push("flag_high_n_low_c");
    }
    if ocean.openness >= Some(HIGH) && ocean.conscientiousness.is_some_and(|c| c <= LOW) {
        flags.push("flag_high_o_low_c");
    }
    if rep.calm_reactive >= Some(HIGH) && ocean.neuroticism >= Some(HIGH) {
        flags.push("flag_calm_neurotic");
    }
    if rep.honest_deceitful >= Some(HIGH) && rep.generous_selfish.is_some_and(|g| g <= LOW) {
        flags.push("flag_honest_selfish");
    }
    if ocean.openness >= Some(HIGH) && rep.adaptable_rigid.is_some_and(|v| v <= LOW) {
        flags.push("flag_open_rigid");
    }
    if ocean.neuroticism.is_some_and(|v| v <= LOW) && rep.calm_reactive.is_some_and(|v| v <= LOW) {
        flags.push("flag_claims_calm_reactive");
    }
    if rep.honest_deceitful >= Some(HIGH) && rep.fair_favoritism.is_some_and(|v| v <= LOW) {
        flags.push("flag_honest_favoritist");
    }
    if ocean.agreeableness >= Some(HIGH) && rep.empathetic_detached.is_some_and(|v| v <= LOW) {
        flags.push("flag_warmth_cold");
    }
    if ocean.conscientiousness >= Some(HIGH) && rep.reliable_flaky.is_some_and(|v| v <= LOW) {
        flags.push("flag_discipline_flaky");
    }

    flags
}

/// True when a person professes fairness but is perceived as practicing favoritism:
/// "do as I say, not as I do."
pub fn fairness_rhetoric_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    motivations
        .iter()
        .any(|m| m.r#type == MotivationType::Fairness && m.intensity >= MOTIVATION_HIGH)
        && rep.fair_favoritism.is_some_and(|v| v <= LOW)
}

pub fn fairness_rhetoric_flag(motivations: &[Motivation], rep: &RepScores) -> Option<&'static str> {
    fairness_rhetoric_gap(motivations, rep).then_some("flag_fairness_rhetoric")
}

fn mot_high(motivations: &[Motivation], t: MotivationType) -> bool {
    motivations
        .iter()
        .any(|m| m.r#type == t && m.intensity >= MOTIVATION_HIGH)
}

/// Preaches helpfulness but is perceived as selfish.
pub fn helping_selfish_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Helping) && rep.generous_selfish.is_some_and(|v| v <= LOW)
}

/// Values closeness but is perceived as cold and detached.
pub fn affiliation_cold_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Affiliation)
        && rep.empathetic_detached.is_some_and(|v| v <= LOW)
}

/// Aspires to power, success, or recognition but is perceived as lazy.
pub fn ambition_lazy_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    [
        MotivationType::Power,
        MotivationType::Achievement,
        MotivationType::Recognition,
    ]
    .iter()
    .any(|t| mot_high(motivations, *t))
        && rep.hardworker_lazy.is_some_and(|v| v <= LOW)
}

/// Claims to value security yet is perceived as gullibly trusting.
pub fn security_gullible_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Security)
        && rep.trusting_suspicious.is_some_and(|v| v >= HIGH)
}

/// Self-image of discipline (OCEAN C) contradicted by a lazy reputation.
pub fn discipline_lazy_gap(ocean: &OceanScores, rep: &RepScores) -> bool {
    ocean.conscientiousness >= Some(HIGH) && rep.hardworker_lazy.is_some_and(|v| v <= LOW)
}

/// Self-image of warmth (OCEAN A) contradicted by a blunt reputation.
pub fn warmth_blunt_gap(ocean: &OceanScores, rep: &RepScores) -> bool {
    ocean.agreeableness >= Some(HIGH) && rep.diplomatic_blunt.is_some_and(|v| v <= LOW)
}

/// Values belonging but is perceived as suspicious and distrustful.
pub fn affiliation_distrustful_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Affiliation)
        && rep.trusting_suspicious.is_some_and(|v| v <= LOW)
}

const VOLATILE_TRIGGERS: [BehaviorTrigger; 3] = [
    BehaviorTrigger::Stress,
    BehaviorTrigger::Conflict,
    BehaviorTrigger::Threatened,
];

const VOLATILE_OUTCOMES: [BehaviorResponse; 8] = [
    BehaviorResponse::BecomesIrritable,
    BehaviorResponse::Panics,
    BehaviorResponse::BecomesPassiveAggressive,
    BehaviorResponse::BecomesDefensive,
    BehaviorResponse::Escalates,
    BehaviorResponse::Counterattacks,
    BehaviorResponse::BecomesParanoid,
    BehaviorResponse::BecomesBitter,
];

const EXPLOIT_OUTCOMES: [BehaviorResponse; 3] = [
    BehaviorResponse::ExploitsOpportunistically,
    BehaviorResponse::UnderminesOthers,
    BehaviorResponse::DeflectsBlame,
];

fn has_pattern_with_outcome(
    patterns: &[BehavioralPattern],
    triggers: &[BehaviorTrigger],
    outcomes: &[BehaviorResponse],
) -> bool {
    patterns
        .iter()
        .any(|p| triggers.contains(&p.trigger) && outcomes.contains(&p.predicted_behavior))
}

/// Reputation says calm, but recorded patterns show volatility under pressure.
pub fn pattern_calm_volatile_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.calm_reactive >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &VOLATILE_TRIGGERS, &VOLATILE_OUTCOMES)
}

/// Reputation says honest, but recorded patterns show exploitation.
pub fn pattern_honest_exploiter_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.honest_deceitful >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &BehaviorTrigger::ALL, &EXPLOIT_OUTCOMES)
}

const CONFLICT_ESCALATION_OUTCOMES: [BehaviorResponse; 3] = [
    BehaviorResponse::BecomesPassiveAggressive,
    BehaviorResponse::BecomesDefensive,
    BehaviorResponse::Escalates,
];

/// Reputation says diplomatic, but recorded patterns escalate conflict.
pub fn pattern_diplomat_escalator_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.diplomatic_blunt >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &[BehaviorTrigger::Conflict],
            &CONFLICT_ESCALATION_OUTCOMES,
        )
}

/// Reputation says fair, but recorded patterns exploit injustice.
pub fn pattern_fair_exploiter_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.fair_favoritism >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &[BehaviorTrigger::Injustice],
            &[BehaviorResponse::ExploitsOpportunistically],
        )
}

const HUMBLE_DISMISSIVE_TRIGGERS: [BehaviorTrigger; 3] = [
    BehaviorTrigger::Success,
    BehaviorTrigger::Recognition,
    BehaviorTrigger::Threatened,
];

const HUMBLE_DISMISSIVE_OUTCOMES: [BehaviorResponse; 4] = [
    BehaviorResponse::DismissesOthers,
    BehaviorResponse::DemandsAttention,
    BehaviorResponse::UnderminesOthers,
    BehaviorResponse::DeflectsBlame,
];

/// Reputation says humble, but recorded patterns put others down.
pub fn pattern_humble_dismissive_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.humble_arrogant >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &HUMBLE_DISMISSIVE_TRIGGERS,
            &HUMBLE_DISMISSIVE_OUTCOMES,
        )
}

/// Reputation says trusting, but recorded patterns turn paranoid under threat.
pub fn pattern_trusting_paranoid_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.trusting_suspicious >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &[BehaviorTrigger::Threatened],
            &[BehaviorResponse::BecomesParanoid],
        )
}

const SHIRK_OUTCOMES: [BehaviorResponse; 3] = [
    BehaviorResponse::DeflectsResponsibility,
    BehaviorResponse::DeflectsBlame,
    BehaviorResponse::Sabotages,
];

const SHIRK_TRIGGERS: [BehaviorTrigger; 3] = [
    BehaviorTrigger::Uncertainty,
    BehaviorTrigger::Threatened,
    BehaviorTrigger::Change,
];

/// Reputation says reliable, but recorded patterns dodge accountability.
pub fn pattern_reliable_shirker_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.reliable_flaky >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &SHIRK_TRIGGERS, &SHIRK_OUTCOMES)
}

const COMPLACENT_OUTCOMES: [BehaviorResponse; 2] = [
    BehaviorResponse::BecomesComplacent,
    BehaviorResponse::BecomesOverconfident,
];

/// Reputation says hardworking, but recorded patterns rest on past laurels.
pub fn pattern_hardworker_complacent_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.hardworker_lazy >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &[BehaviorTrigger::Success], &COMPLACENT_OUTCOMES)
}

const BLOWUP_TRIGGERS: [BehaviorTrigger; 3] = [
    BehaviorTrigger::Stress,
    BehaviorTrigger::Conflict,
    BehaviorTrigger::Threatened,
];

const BLOWUP_OUTCOMES: [BehaviorResponse; 6] = [
    BehaviorResponse::BecomesIrritable,
    BehaviorResponse::BecomesPassiveAggressive,
    BehaviorResponse::BecomesDefensive,
    BehaviorResponse::Escalates,
    BehaviorResponse::Counterattacks,
    BehaviorResponse::BecomesParanoid,
];

/// Reputation says passive, but recorded patterns blow up under pressure.
pub fn pattern_passive_blowup_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.assertive_passive.is_some_and(|v| v <= LOW)
        && has_pattern_with_outcome(patterns, &BLOWUP_TRIGGERS, &BLOWUP_OUTCOMES)
}

const QUIET_TRIGGERS: [BehaviorTrigger; 4] = [
    BehaviorTrigger::Conflict,
    BehaviorTrigger::Stress,
    BehaviorTrigger::Uncertainty,
    BehaviorTrigger::Injustice,
];

const QUIET_OUTCOMES: [BehaviorResponse; 4] = [
    BehaviorResponse::StaysSilent,
    BehaviorResponse::BecomesQuiet,
    BehaviorResponse::WaitsForClarity,
    BehaviorResponse::WithdrawsFromInjustice,
];

/// Reputation says assertive, but recorded patterns go quiet when it counts.
pub fn pattern_assertive_quiet_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.assertive_passive >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &QUIET_TRIGGERS, &QUIET_OUTCOMES)
}

const GENEROUS_EXPLOIT_TRIGGERS: [BehaviorTrigger; 3] = [
    BehaviorTrigger::Injustice,
    BehaviorTrigger::Recognition,
    BehaviorTrigger::Threatened,
];

const GENEROUS_EXPLOIT_OUTCOMES: [BehaviorResponse; 3] = [
    BehaviorResponse::ExploitsOpportunistically,
    BehaviorResponse::UnderminesOthers,
    BehaviorResponse::DeflectsBlame,
];

/// Reputation says generous, but recorded patterns exploit or deflect blame.
pub fn pattern_generous_exploiter_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.generous_selfish >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &GENEROUS_EXPLOIT_TRIGGERS,
            &GENEROUS_EXPLOIT_OUTCOMES,
        )
}

const EMPATH_DISMISSIVE_TRIGGERS: [BehaviorTrigger; 3] = [
    BehaviorTrigger::Success,
    BehaviorTrigger::Recognition,
    BehaviorTrigger::Threatened,
];

const EMPATH_DISMISSIVE_OUTCOMES: [BehaviorResponse; 4] = [
    BehaviorResponse::DismissesOthers,
    BehaviorResponse::DemandsAttention,
    BehaviorResponse::UnderminesOthers,
    BehaviorResponse::DeflectsBlame,
];

/// Reputation says empathetic, but recorded patterns put others down.
pub fn pattern_empath_dismissive_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.empathetic_detached >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &EMPATH_DISMISSIVE_TRIGGERS,
            &EMPATH_DISMISSIVE_OUTCOMES,
        )
}

const RESIST_TRIGGERS: [BehaviorTrigger; 2] = [BehaviorTrigger::Change, BehaviorTrigger::Feedback];

const RESIST_OUTCOMES: [BehaviorResponse; 4] = [
    BehaviorResponse::ResistsChange,
    BehaviorResponse::Sabotages,
    BehaviorResponse::IgnoresCompletely,
    BehaviorResponse::RejectsFeedback,
];

/// Reputation says flexible, but recorded patterns resist change and feedback.
pub fn pattern_flexible_resister_gap(patterns: &[BehavioralPattern], rep: &RepScores) -> bool {
    rep.adaptable_rigid >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &RESIST_TRIGGERS, &RESIST_OUTCOMES)
}

/// Preaches helpfulness yet recorded patterns show exploitation.
pub fn pattern_helping_exploiter_gap(
    patterns: &[BehavioralPattern],
    motivations: &[Motivation],
) -> bool {
    mot_high(motivations, MotivationType::Helping)
        && has_pattern_with_outcome(
            patterns,
            &GENEROUS_EXPLOIT_TRIGGERS,
            &GENEROUS_EXPLOIT_OUTCOMES,
        )
}

/// Self-image of warmth (OCEAN A) yet recorded patterns put others down.
pub fn pattern_warmth_dismissive_gap(patterns: &[BehavioralPattern], ocean: &OceanScores) -> bool {
    ocean.agreeableness >= Some(HIGH)
        && has_pattern_with_outcome(
            patterns,
            &EMPATH_DISMISSIVE_TRIGGERS,
            &EMPATH_DISMISSIVE_OUTCOMES,
        )
}

/// Self-image of discipline (OCEAN C) yet recorded patterns dodge accountability.
pub fn pattern_discipline_shirker_gap(patterns: &[BehavioralPattern], ocean: &OceanScores) -> bool {
    ocean.conscientiousness >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &SHIRK_TRIGGERS, &SHIRK_OUTCOMES)
}

/// Self-reports calm (OCEAN N low) yet recorded patterns show volatility.
pub fn pattern_claimed_calm_volatile_gap(
    patterns: &[BehavioralPattern],
    ocean: &OceanScores,
) -> bool {
    ocean.neuroticism.is_some_and(|n| n <= LOW)
        && has_pattern_with_outcome(patterns, &VOLATILE_TRIGGERS, &VOLATILE_OUTCOMES)
}

/// Preaches fairness yet recorded patterns exploit injustice.
pub fn pattern_fairness_exploiter_gap(
    patterns: &[BehavioralPattern],
    motivations: &[Motivation],
) -> bool {
    mot_high(motivations, MotivationType::Fairness)
        && has_pattern_with_outcome(
            patterns,
            &[BehaviorTrigger::Injustice],
            &[BehaviorResponse::ExploitsOpportunistically],
        )
}

/// Aspires to achievement yet recorded patterns rest on laurels.
pub fn pattern_achievement_complacent_gap(
    patterns: &[BehavioralPattern],
    motivations: &[Motivation],
) -> bool {
    mot_high(motivations, MotivationType::Achievement)
        && has_pattern_with_outcome(patterns, &[BehaviorTrigger::Success], &COMPLACENT_OUTCOMES)
}

/// Preaches learning yet recorded patterns reject change and feedback.
pub fn pattern_learning_resister_gap(
    patterns: &[BehavioralPattern],
    motivations: &[Motivation],
) -> bool {
    mot_high(motivations, MotivationType::Learning)
        && has_pattern_with_outcome(patterns, &RESIST_TRIGGERS, &RESIST_OUTCOMES)
}

/// Self-image of extraversion yet recorded patterns go quiet when it counts.
pub fn pattern_extravert_quiet_gap(patterns: &[BehavioralPattern], ocean: &OceanScores) -> bool {
    ocean.extraversion >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &QUIET_TRIGGERS, &QUIET_OUTCOMES)
}

/// Self-image of openness yet recorded patterns resist change and feedback.
pub fn pattern_open_resister_gap(patterns: &[BehavioralPattern], ocean: &OceanScores) -> bool {
    ocean.openness >= Some(HIGH)
        && has_pattern_with_outcome(patterns, &RESIST_TRIGGERS, &RESIST_OUTCOMES)
}

/// Aspires to recognition yet recorded patterns put others down to win.
pub fn pattern_recognition_dismissive_gap(
    patterns: &[BehavioralPattern],
    motivations: &[Motivation],
) -> bool {
    mot_high(motivations, MotivationType::Recognition)
        && has_pattern_with_outcome(
            patterns,
            &EMPATH_DISMISSIVE_TRIGGERS,
            &EMPATH_DISMISSIVE_OUTCOMES,
        )
}

/// Claims a taste for risk yet is loss-averse.
pub fn loss_aversion_risky_gap(biases: &[Bias], risk_appetite: Option<u8>) -> bool {
    bias_high(biases, BiasType::LossAversion, BIAS_HIGH) && risk_appetite.is_some_and(|v| v >= HIGH)
}

/// Overestimates their competence yet is perceived as humble.
pub fn dunning_kruger_humble_gap(biases: &[Bias], rep: &RepScores) -> bool {
    bias_high(biases, BiasType::DunningKruger, BIAS_HIGH)
        && rep.humble_arrogant.is_some_and(|v| v <= LOW)
}

/// Underestimates their competence yet is perceived as arrogant.
pub fn impostor_arrogant_gap(biases: &[Bias], rep: &RepScores) -> bool {
    bias_high(biases, BiasType::Impostor, BIAS_HIGH)
        && rep.humble_arrogant.is_some_and(|v| v >= HIGH)
}

/// Perceived as steady yet swings with the latest news.
pub fn recency_reliable_gap(biases: &[Bias], rep: &RepScores) -> bool {
    bias_high(biases, BiasType::Recency, BIAS_HIGH) && rep.reliable_flaky.is_some_and(|v| v >= HIGH)
}

/// Perceived as unflappable yet overweights dramatic recent events.
pub fn availability_calm_gap(biases: &[Bias], rep: &RepScores) -> bool {
    bias_high(biases, BiasType::Availability, BIAS_HIGH)
        && rep.calm_reactive.is_some_and(|v| v >= HIGH)
}

/// Admits fragility yet appears unflappable — hides it well.
pub fn resilient_hides_gap(resilience: Option<u8>, rep: &RepScores) -> bool {
    resilience.is_some_and(|v| v <= LOW) && rep.calm_reactive.is_some_and(|v| v >= HIGH)
}

fn bias_high(biases: &[Bias], t: BiasType, min: u8) -> bool {
    biases.iter().any(|b| b.r#type == t && b.intensity >= min)
}

/// Claims open-mindedness (OCEAN O) yet shows confirmation bias.
pub fn bias_confirmation_open_gap(biases: &[Bias], ocean: &OceanScores) -> bool {
    bias_high(biases, BiasType::Confirmation, BIAS_HIGH) && ocean.openness >= Some(HIGH)
}

/// Preaches fairness yet shows favoritism or in-group bias.
pub fn bias_favoritism_fairness_gap(biases: &[Bias], motivations: &[Motivation]) -> bool {
    (bias_high(biases, BiasType::Favoritism, BIAS_HIGH)
        || bias_high(biases, BiasType::InGroup, BIAS_HIGH))
        && mot_high(motivations, MotivationType::Fairness)
}

/// Perceived as a leader yet blindly defers to authority figures.
pub fn authority_dominant_gap(biases: &[Bias], rep: &RepScores) -> bool {
    bias_high(biases, BiasType::Authority, BIAS_HIGH)
        && rep.authoritative_submissive.is_some_and(|v| v >= HIGH)
}

/// Claims independent thinking yet follows the herd.
pub fn social_proof_open_gap(biases: &[Bias], ocean: &OceanScores) -> bool {
    bias_high(biases, BiasType::SocialProof, BIAS_HIGH) && ocean.openness >= Some(HIGH)
}

/// Claims open-mindedness yet clings to first impressions.
pub fn anchoring_open_gap(biases: &[Bias], ocean: &OceanScores) -> bool {
    bias_high(biases, BiasType::Anchoring, BIAS_HIGH) && ocean.openness >= Some(HIGH)
}

/// Perceived as flexible yet clings to sunk costs.
pub fn sunk_cost_flexible_gap(biases: &[Bias], rep: &RepScores) -> bool {
    bias_high(biases, BiasType::SunkCost, BIAS_HIGH)
        && rep.adaptable_rigid.is_some_and(|v| v >= HIGH)
}

/// Preaches caution and security yet self-reports a taste for risk.
pub fn security_risky_gap(motivations: &[Motivation], risk_appetite: Option<u8>) -> bool {
    mot_high(motivations, MotivationType::Security) && risk_appetite.is_some_and(|v| v >= HIGH)
}

/// Claims high resilience yet is perceived as reactive.
pub fn resilient_reactive_gap(resilience: Option<u8>, rep: &RepScores) -> bool {
    resilience.is_some_and(|v| v >= HIGH) && rep.calm_reactive.is_some_and(|v| v <= LOW)
}

/// Aspires to power or achievement yet self-reports avoiding all risk.
pub fn risk_appetite_ambition_gap(motivations: &[Motivation], risk_appetite: Option<u8>) -> bool {
    [MotivationType::Power, MotivationType::Achievement]
        .iter()
        .any(|t| mot_high(motivations, *t))
        && risk_appetite.is_some_and(|v| v <= LOW)
}

/// Flags from recorded behavioral/cognitive evidence contradicting stated traits.
pub fn evidence_flags(person: &Person) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if pattern_calm_volatile_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_calm_volatile");
    }
    if pattern_honest_exploiter_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_honest_exploiter");
    }
    if pattern_diplomat_escalator_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_diplomat_escalator");
    }
    if pattern_fair_exploiter_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_fair_exploiter");
    }
    if pattern_humble_dismissive_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_humble_dismissive");
    }
    if pattern_trusting_paranoid_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_trusting_paranoid");
    }
    if pattern_reliable_shirker_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_reliable_shirker");
    }
    if pattern_hardworker_complacent_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_hardworker_complacent");
    }
    if pattern_passive_blowup_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_passive_blowup");
    }
    if pattern_assertive_quiet_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_assertive_quiet");
    }
    if pattern_generous_exploiter_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_generous_exploiter");
    }
    if pattern_empath_dismissive_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_empath_dismissive");
    }
    if pattern_flexible_resister_gap(&person.behavioral_patterns, &person.rep_scores) {
        flags.push("flag_pattern_flexible_resister");
    }
    if pattern_helping_exploiter_gap(&person.behavioral_patterns, &person.motivations) {
        flags.push("flag_pattern_helping_exploiter");
    }
    if pattern_warmth_dismissive_gap(&person.behavioral_patterns, &person.ocean) {
        flags.push("flag_pattern_warmth_dismissive");
    }
    if pattern_discipline_shirker_gap(&person.behavioral_patterns, &person.ocean) {
        flags.push("flag_pattern_discipline_shirker");
    }
    if pattern_claimed_calm_volatile_gap(&person.behavioral_patterns, &person.ocean) {
        flags.push("flag_pattern_claimed_calm_volatile");
    }
    if pattern_fairness_exploiter_gap(&person.behavioral_patterns, &person.motivations) {
        flags.push("flag_pattern_fairness_exploiter");
    }
    if pattern_achievement_complacent_gap(&person.behavioral_patterns, &person.motivations) {
        flags.push("flag_pattern_achievement_complacent");
    }
    if pattern_learning_resister_gap(&person.behavioral_patterns, &person.motivations) {
        flags.push("flag_pattern_learning_resister");
    }
    if pattern_extravert_quiet_gap(&person.behavioral_patterns, &person.ocean) {
        flags.push("flag_pattern_extravert_quiet");
    }
    if pattern_open_resister_gap(&person.behavioral_patterns, &person.ocean) {
        flags.push("flag_pattern_open_resister");
    }
    if pattern_recognition_dismissive_gap(&person.behavioral_patterns, &person.motivations) {
        flags.push("flag_pattern_recognition_dismissive");
    }
    if bias_confirmation_open_gap(&person.biases, &person.ocean) {
        flags.push("flag_bias_confirmation_open");
    }
    if anchoring_open_gap(&person.biases, &person.ocean) {
        flags.push("flag_anchoring_open");
    }
    if bias_favoritism_fairness_gap(&person.biases, &person.motivations) {
        flags.push("flag_bias_favoritism_fairness");
    }
    if authority_dominant_gap(&person.biases, &person.rep_scores) {
        flags.push("flag_authority_dominant");
    }
    if social_proof_open_gap(&person.biases, &person.ocean) {
        flags.push("flag_social_proof_open");
    }
    if sunk_cost_flexible_gap(&person.biases, &person.rep_scores) {
        flags.push("flag_sunk_cost_flexible");
    }
    if loss_aversion_risky_gap(&person.biases, person.risk_appetite) {
        flags.push("flag_loss_aversion_risky");
    }
    if dunning_kruger_humble_gap(&person.biases, &person.rep_scores) {
        flags.push("flag_dunning_kruger_humble");
    }
    if impostor_arrogant_gap(&person.biases, &person.rep_scores) {
        flags.push("flag_impostor_arrogant");
    }
    if recency_reliable_gap(&person.biases, &person.rep_scores) {
        flags.push("flag_recency_reliable");
    }
    if availability_calm_gap(&person.biases, &person.rep_scores) {
        flags.push("flag_availability_calm");
    }
    if security_risky_gap(&person.motivations, person.risk_appetite) {
        flags.push("flag_security_risky");
    }
    if resilient_reactive_gap(person.resilience, &person.rep_scores) {
        flags.push("flag_resilient_reactive");
    }
    if risk_appetite_ambition_gap(&person.motivations, person.risk_appetite) {
        flags.push("flag_risk_appetite_ambition");
    }
    if resilient_hides_gap(person.resilience, &person.rep_scores) {
        flags.push("flag_resilient_hides");
    }
    flags
}

/// Values independence yet is perceived as submissive.
pub fn autonomy_submissive_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Autonomy)
        && rep.authoritative_submissive.is_some_and(|v| v <= LOW)
}

/// Values growth and learning yet is perceived as rigid.
pub fn learning_rigid_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Learning) && rep.adaptable_rigid.is_some_and(|v| v <= LOW)
}

/// Values creativity yet self-reports little openness to novelty.
pub fn creativity_closed_gap(motivations: &[Motivation], ocean: &OceanScores) -> bool {
    mot_high(motivations, MotivationType::Creativity) && ocean.openness.is_some_and(|v| v <= LOW)
}

/// Values creativity yet is perceived as rigid.
pub fn creativity_rigid_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Creativity)
        && rep.adaptable_rigid.is_some_and(|v| v <= LOW)
}

/// Aspires to power yet is perceived as passive.
pub fn power_passive_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Power) && rep.assertive_passive.is_some_and(|v| v <= LOW)
}

/// Preaches helpfulness yet is perceived as emotionally cold and detached.
pub fn helping_cold_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Helping)
        && rep.empathetic_detached.is_some_and(|v| v <= LOW)
}

/// Preaches growth and learning yet is perceived as too arrogant to take advice.
pub fn learning_arrogant_gap(motivations: &[Motivation], rep: &RepScores) -> bool {
    mot_high(motivations, MotivationType::Learning) && rep.humble_arrogant.is_some_and(|v| v <= LOW)
}

/// Self-image of warmth (OCEAN A) contradicted by a selfish reputation.
pub fn warmth_selfish_gap(ocean: &OceanScores, rep: &RepScores) -> bool {
    ocean.agreeableness >= Some(HIGH) && rep.generous_selfish.is_some_and(|v| v <= LOW)
}

fn style_high(styles: &[PersonalStyle], types: &[StyleType], min: u8) -> bool {
    styles
        .iter()
        .any(|s| types.contains(&s.r#type) && s.intensity >= min)
}

/// Claims to communicate directly yet is perceived as diplomatic.
pub fn style_direct_diplomatic_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::DirectCommunicator], STYLE_HIGH)
        && rep.diplomatic_blunt.is_some_and(|v| v >= HIGH)
}

/// Claims a diplomatic style yet is perceived as blunt.
pub fn style_diplomatic_blunt_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::DiplomaticCommunicator], STYLE_HIGH)
        && rep.diplomatic_blunt.is_some_and(|v| v <= LOW)
}

/// Claims a competitive conflict style yet is perceived as passive.
pub fn style_competing_passive_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::Competing], STYLE_HIGH)
        && rep.assertive_passive.is_some_and(|v| v <= LOW)
}

/// Claims an autocratic or controlling style yet is perceived as submissive.
pub fn style_dominant_submissive_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[StyleType::Autocratic, StyleType::Controlling],
        STYLE_HIGH,
    ) && rep.authoritative_submissive.is_some_and(|v| v <= LOW)
}

/// Claims a controlling or autocratic style and is perceived as authoritative
/// (self-image matches reputation). Flags the "control freak" pattern: someone
/// who genuinely dominates and micromanages, not just one who aspires to it.
pub fn style_controlling_consistent(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[StyleType::Controlling, StyleType::Autocratic],
        STYLE_HIGH,
    ) && rep.authoritative_submissive.is_some_and(|v| v >= HIGH)
}

/// Claims to operate opportunistically yet is perceived as honest.
pub fn style_manipulative_honest_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[
            StyleType::Opportunistic,
            StyleType::Manipulative,
            StyleType::Intrusive,
        ],
        STYLE_HIGH,
    ) && rep.honest_deceitful.is_some_and(|v| v >= HIGH)
}

/// Claims a manipulative, opportunistic, or intrusive style and is perceived
/// as deceitful (self-image matches reputation). Flags the confirmed
/// manipulator: someone who openly operates dirty and the model's reputation
/// already distrusts them — the mirror of `style_manipulative_honest_gap`.
pub fn style_manipulative_consistent(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[
            StyleType::Opportunistic,
            StyleType::Manipulative,
            StyleType::Intrusive,
        ],
        STYLE_HIGH,
    ) && rep.honest_deceitful.is_some_and(|v| v <= LOW)
}

/// Claims an empathetic, respectful, supportive, or nurturing conduct style yet
/// is perceived as cold.
pub fn style_empathetic_cold_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[
            StyleType::Empathetic,
            StyleType::Respectful,
            StyleType::Supportive,
            StyleType::Nurturing,
        ],
        STYLE_HIGH,
    ) && rep.empathetic_detached.is_some_and(|v| v <= LOW)
}

/// Claims a guarded or verifying trust style yet is perceived as trusting.
pub fn style_guarded_trusting_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[StyleType::Guarded, StyleType::VerifiesTrust],
        STYLE_HIGH,
    ) && rep.trusting_suspicious.is_some_and(|v| v >= HIGH)
}

/// Claims a servant leadership style yet is perceived as authoritative.
pub fn style_servant_authoritative_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::Servant], STYLE_HIGH)
        && rep.authoritative_submissive.is_some_and(|v| v >= HIGH)
}

/// Claims a consensus-driven style yet is perceived as authoritative.
pub fn style_consensus_authoritative_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(
        styles,
        &[StyleType::Participatory, StyleType::ConsensusDriven],
        STYLE_HIGH,
    ) && rep.authoritative_submissive.is_some_and(|v| v >= HIGH)
}

/// Claims to trust freely yet is perceived as suspicious.
pub fn style_trusts_freely_suspicious_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::ExtendsTrustFreely], STYLE_HIGH)
        && rep.trusting_suspicious.is_some_and(|v| v <= LOW)
}

/// Claims to repair trust yet is perceived as deceitful.
pub fn style_repairs_trust_deceitful_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::RepairsTrustActively], STYLE_HIGH)
        && rep.honest_deceitful.is_some_and(|v| v <= LOW)
}

/// Claims a rules-based approach yet is perceived as playing favorites.
pub fn style_rulebased_favoritist_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::RuleBased], STYLE_HIGH)
        && rep.fair_favoritism.is_some_and(|v| v <= LOW)
}

/// Claims a virtue-based approach yet is perceived as deceitful.
pub fn style_virtuebased_deceitful_gap(styles: &[PersonalStyle], rep: &RepScores) -> bool {
    style_high(styles, &[StyleType::VirtueBased], STYLE_HIGH)
        && rep.honest_deceitful.is_some_and(|v| v <= LOW)
}

/// All "declared style vs perceived" gaps: a self-described work or conduct
/// style contradicted by reputation.
pub fn style_gap_flags(styles: &[PersonalStyle], rep: &RepScores) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if style_direct_diplomatic_gap(styles, rep) {
        flags.push("flag_style_direct_diplomatic");
    }
    if style_diplomatic_blunt_gap(styles, rep) {
        flags.push("flag_style_diplomatic_blunt");
    }
    if style_competing_passive_gap(styles, rep) {
        flags.push("flag_style_competing_passive");
    }
    if style_dominant_submissive_gap(styles, rep) {
        flags.push("flag_style_dominant_submissive");
    }
    if style_controlling_consistent(styles, rep) {
        flags.push("flag_style_controlling");
    }
    if style_manipulative_honest_gap(styles, rep) {
        flags.push("flag_style_manipulative_honest");
    }
    if style_manipulative_consistent(styles, rep) {
        flags.push("flag_style_manipulative");
    }
    if style_empathetic_cold_gap(styles, rep) {
        flags.push("flag_style_empathetic_cold");
    }
    if style_guarded_trusting_gap(styles, rep) {
        flags.push("flag_style_guarded_trusting");
    }
    if style_servant_authoritative_gap(styles, rep) {
        flags.push("flag_style_servant_authoritative");
    }
    if style_consensus_authoritative_gap(styles, rep) {
        flags.push("flag_style_consensus_authoritative");
    }
    if style_trusts_freely_suspicious_gap(styles, rep) {
        flags.push("flag_style_trusts_freely_suspicious");
    }
    if style_repairs_trust_deceitful_gap(styles, rep) {
        flags.push("flag_style_repairs_trust_deceitful");
    }
    if style_rulebased_favoritist_gap(styles, rep) {
        flags.push("flag_style_rulebased_favoritist");
    }
    if style_virtuebased_deceitful_gap(styles, rep) {
        flags.push("flag_style_virtuebased_deceitful");
    }
    flags
}

/// All "says vs does" gaps: a stated value or self-image contradicted by reputation.
pub fn rhetoric_gap_flags(
    ocean: &OceanScores,
    rep: &RepScores,
    motivations: &[Motivation],
) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if fairness_rhetoric_gap(motivations, rep) {
        flags.push("flag_fairness_rhetoric");
    }
    if helping_selfish_gap(motivations, rep) {
        flags.push("flag_helping_selfish");
    }
    if affiliation_cold_gap(motivations, rep) {
        flags.push("flag_affiliation_cold");
    }
    if ambition_lazy_gap(motivations, rep) {
        flags.push("flag_ambition_lazy");
    }
    if security_gullible_gap(motivations, rep) {
        flags.push("flag_security_gullible");
    }
    if discipline_lazy_gap(ocean, rep) {
        flags.push("flag_discipline_lazy");
    }
    if warmth_blunt_gap(ocean, rep) {
        flags.push("flag_warmth_blunt");
    }
    if affiliation_distrustful_gap(motivations, rep) {
        flags.push("flag_affiliation_distrustful");
    }
    if autonomy_submissive_gap(motivations, rep) {
        flags.push("flag_autonomy_submissive");
    }
    if learning_rigid_gap(motivations, rep) {
        flags.push("flag_learning_rigid");
    }
    if creativity_closed_gap(motivations, ocean) {
        flags.push("flag_creativity_closed");
    }
    if creativity_rigid_gap(motivations, rep) {
        flags.push("flag_creativity_rigid");
    }
    if power_passive_gap(motivations, rep) {
        flags.push("flag_power_passive");
    }
    if helping_cold_gap(motivations, rep) {
        flags.push("flag_helping_cold");
    }
    if learning_arrogant_gap(motivations, rep) {
        flags.push("flag_learning_arrogant");
    }
    if warmth_selfish_gap(ocean, rep) {
        flags.push("flag_warmth_selfish");
    }
    flags
}

pub fn value_flags(
    values: &[crate::models::Value],
    risk_appetite: Option<u8>,
    styles: &[crate::models::PersonalStyle],
) -> Vec<&'static str> {
    use crate::models::{StyleCategory, StyleType, ValueType};
    let mut flags = Vec::new();
    let val = |vt: ValueType| -> Option<u8> {
        values.iter().find(|v| v.r#type == vt).map(|v| v.intensity)
    };
    if val(ValueType::Family).is_some_and(|i| i >= 7)
        && !styles.iter().any(|s| s.r#type == StyleType::PastOriented)
    {
        flags.push("flag_value_family_past");
    }
    if val(ValueType::Stability).is_some_and(|i| i >= 8) && risk_appetite.is_some_and(|r| r >= 8) {
        flags.push("flag_value_stability_risk");
    }
    if val(ValueType::Career).is_some_and(|i| i >= 8)
        && val(ValueType::Family).is_some_and(|i| i >= 8)
    {
        flags.push("flag_value_career_family");
    }
    if val(ValueType::Loyalty).is_some_and(|i| i >= 8)
        && let Some(trust) = styles
            .iter()
            .find(|s| s.r#type.category() == StyleCategory::TrustStyle)
        && trust.r#type == StyleType::Guarded
    {
        flags.push("flag_value_loyalty_guarded");
    }
    flags
}

pub fn all_person_flags(person: &Person) -> Vec<&'static str> {
    let mut flags = ocean_rep_flags(&person.ocean, &person.rep_scores);
    flags.extend(rhetoric_gap_flags(
        &person.ocean,
        &person.rep_scores,
        &person.motivations,
    ));
    flags.extend(evidence_flags(person));
    flags.extend(style_gap_flags(&person.styles, &person.rep_scores));
    flags.extend(value_flags(
        &person.values,
        person.risk_appetite,
        &person.styles,
    ));
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        BehaviorResponse, BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation,
        MotivationType, OceanScores, Person, RepScores,
    };

    #[test]
    fn test_no_flags_default() {
        let o = OceanScores::default();
        let r = RepScores::default();
        assert!(ocean_rep_flags(&o, &r).is_empty());
    }

    #[test]
    fn test_high_e_low_a() {
        let o = OceanScores {
            extraversion: Some(8),
            agreeableness: Some(2),
            ..Default::default()
        };
        let r = RepScores::default();
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_high_e_low_a"));
    }

    #[test]
    fn test_high_n_low_c() {
        let o = OceanScores {
            neuroticism: Some(9),
            conscientiousness: Some(2),
            ..Default::default()
        };
        let r = RepScores::default();
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_high_n_low_c"));
    }

    #[test]
    fn test_calm_neurotic() {
        let o = OceanScores {
            neuroticism: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            calm_reactive: Some(8),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_calm_neurotic"));
    }

    #[test]
    fn test_honest_selfish() {
        let o = OceanScores::default();
        let r = RepScores {
            honest_deceitful: Some(9),
            generous_selfish: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_honest_selfish"));
    }

    #[test]
    fn test_threshold_bounds() {
        let o = OceanScores {
            extraversion: Some(7),
            agreeableness: Some(4),
            ..Default::default()
        };
        let r = RepScores::default();
        assert!(ocean_rep_flags(&o, &r).is_empty());
    }

    #[test]
    fn test_fairness_rhetoric_gap_detected() {
        let mot = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 7,
            notes: String::new(),
        }];
        let r = RepScores {
            fair_favoritism: Some(2),
            ..Default::default()
        };
        assert!(fairness_rhetoric_gap(&mot, &r));
        assert_eq!(
            fairness_rhetoric_flag(&mot, &r),
            Some("flag_fairness_rhetoric")
        );
    }

    #[test]
    fn test_fairness_rhetoric_gap_no_fairness() {
        let mot = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: String::new(),
        }];
        let r = RepScores {
            fair_favoritism: Some(2),
            ..Default::default()
        };
        assert!(!fairness_rhetoric_gap(&mot, &r));
    }

    #[test]
    fn test_fairness_rhetoric_gap_boundary() {
        let mot = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 5,
            notes: String::new(),
        }];
        let r = RepScores {
            fair_favoritism: Some(3),
            ..Default::default()
        };
        assert!(!fairness_rhetoric_gap(&mot, &r));
        let r2 = RepScores {
            fair_favoritism: Some(4),
            ..Default::default()
        };
        assert!(!fairness_rhetoric_gap(&mot, &r2));
    }

    fn mk_mot(t: MotivationType, intensity: u8) -> Motivation {
        Motivation {
            r#type: t,
            intensity,
            notes: String::new(),
        }
    }

    fn mk_bias(t: BiasType, intensity: u8) -> Bias {
        Bias {
            r#type: t,
            intensity,
            evidence: String::new(),
        }
    }

    #[test]
    fn test_helping_selfish_gap() {
        let mot = vec![mk_mot(MotivationType::Helping, 7)];
        let r = RepScores {
            generous_selfish: Some(2),
            ..Default::default()
        };
        assert!(helping_selfish_gap(&mot, &r));
        let consistent = RepScores {
            generous_selfish: Some(7),
            ..Default::default()
        };
        assert!(!helping_selfish_gap(&mot, &consistent));
        assert!(!helping_selfish_gap(
            &[mk_mot(MotivationType::Power, 8)],
            &r
        ));
        let undefined = RepScores::default();
        assert!(!helping_selfish_gap(&mot, &undefined));
    }

    #[test]
    fn test_affiliation_cold_gap() {
        let mot = vec![mk_mot(MotivationType::Affiliation, 6)];
        let r = RepScores {
            empathetic_detached: Some(2),
            ..Default::default()
        };
        assert!(affiliation_cold_gap(&mot, &r));
        let warm = RepScores {
            empathetic_detached: Some(8),
            ..Default::default()
        };
        assert!(!affiliation_cold_gap(&mot, &warm));
        assert!(!affiliation_cold_gap(
            &[mk_mot(MotivationType::Affiliation, 5)],
            &r
        ));
    }

    #[test]
    fn test_ambition_lazy_gap() {
        for t in [
            MotivationType::Power,
            MotivationType::Achievement,
            MotivationType::Recognition,
        ] {
            let mot = vec![mk_mot(t, 7)];
            let r = RepScores {
                hardworker_lazy: Some(2),
                ..Default::default()
            };
            assert!(ambition_lazy_gap(&mot, &r), "missing gap for {t:?}");
        }
        let r = RepScores {
            hardworker_lazy: Some(8),
            ..Default::default()
        };
        assert!(!ambition_lazy_gap(
            &[mk_mot(MotivationType::Achievement, 8)],
            &r
        ));
        assert!(!ambition_lazy_gap(
            &[mk_mot(MotivationType::Learning, 8)],
            &r
        ));
        let undefined = RepScores::default();
        assert!(!ambition_lazy_gap(
            &[mk_mot(MotivationType::Power, 8)],
            &undefined
        ));
    }

    #[test]
    fn test_security_gullible_gap() {
        let mot = vec![mk_mot(MotivationType::Security, 8)];
        let r = RepScores {
            trusting_suspicious: Some(9),
            ..Default::default()
        };
        assert!(security_gullible_gap(&mot, &r));
        let cautious = RepScores {
            trusting_suspicious: Some(2),
            ..Default::default()
        };
        assert!(!security_gullible_gap(&mot, &cautious));
        assert!(!security_gullible_gap(
            &[mk_mot(MotivationType::Helping, 8)],
            &r
        ));
        let undefined = RepScores::default();
        assert!(!security_gullible_gap(&mot, &undefined));
    }

    #[test]
    fn test_discipline_lazy_gap() {
        let o = OceanScores {
            conscientiousness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            hardworker_lazy: Some(2),
            ..Default::default()
        };
        assert!(discipline_lazy_gap(&o, &r));
        let diligent = RepScores {
            hardworker_lazy: Some(8),
            ..Default::default()
        };
        assert!(!discipline_lazy_gap(&o, &diligent));
        let low_c = OceanScores {
            conscientiousness: Some(4),
            ..Default::default()
        };
        assert!(!discipline_lazy_gap(&low_c, &r));
    }

    #[test]
    fn test_warmth_blunt_gap() {
        let o = OceanScores {
            agreeableness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            diplomatic_blunt: Some(2),
            ..Default::default()
        };
        assert!(warmth_blunt_gap(&o, &r));
        let warm = RepScores {
            diplomatic_blunt: Some(8),
            ..Default::default()
        };
        assert!(!warmth_blunt_gap(&o, &warm));
        let low_a = OceanScores {
            agreeableness: Some(4),
            ..Default::default()
        };
        assert!(!warmth_blunt_gap(&low_a, &r));
    }

    #[test]
    fn test_rhetoric_gap_flags_aggregates() {
        let o = OceanScores {
            agreeableness: Some(9),
            conscientiousness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            fair_favoritism: Some(2),
            generous_selfish: Some(2),
            empathetic_detached: Some(2),
            hardworker_lazy: Some(2),
            trusting_suspicious: Some(9),
            diplomatic_blunt: Some(2),
            ..Default::default()
        };
        let m = vec![
            mk_mot(MotivationType::Fairness, 7),
            mk_mot(MotivationType::Helping, 7),
            mk_mot(MotivationType::Affiliation, 7),
            mk_mot(MotivationType::Achievement, 7),
            mk_mot(MotivationType::Security, 7),
        ];
        let flags = rhetoric_gap_flags(&o, &r, &m);
        for key in [
            "flag_fairness_rhetoric",
            "flag_helping_selfish",
            "flag_affiliation_cold",
            "flag_ambition_lazy",
            "flag_security_gullible",
            "flag_discipline_lazy",
            "flag_warmth_blunt",
        ] {
            assert!(flags.contains(&key), "missing {key}: {flags:?}");
        }
    }

    #[test]
    fn test_open_rigid() {
        let o = OceanScores {
            openness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            adaptable_rigid: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_open_rigid"));
        let flexible = RepScores {
            adaptable_rigid: Some(8),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&o, &flexible).contains(&"flag_open_rigid"));
        let low_o = OceanScores {
            openness: Some(4),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&low_o, &r).contains(&"flag_open_rigid"));
        assert!(!ocean_rep_flags(&OceanScores::default(), &r).contains(&"flag_open_rigid"));
    }

    #[test]
    fn test_claims_calm_reactive() {
        let o = OceanScores {
            neuroticism: Some(2),
            ..Default::default()
        };
        let r = RepScores {
            calm_reactive: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_claims_calm_reactive"));
        let calm = RepScores {
            calm_reactive: Some(8),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&o, &calm).contains(&"flag_claims_calm_reactive"));
        let high_n = OceanScores {
            neuroticism: Some(8),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&high_n, &r).contains(&"flag_claims_calm_reactive"));
        let both_high = ocean_rep_flags(&high_n, &calm);
        assert!(both_high.contains(&"flag_calm_neurotic"));
        assert!(!both_high.contains(&"flag_claims_calm_reactive"));
        assert!(
            !ocean_rep_flags(&OceanScores::default(), &r).contains(&"flag_claims_calm_reactive")
        );
    }

    #[test]
    fn test_honest_favoritist() {
        let r = RepScores {
            honest_deceitful: Some(9),
            fair_favoritism: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&OceanScores::default(), &r);
        assert!(flags.contains(&"flag_honest_favoritist"));
        let fair = RepScores {
            honest_deceitful: Some(9),
            fair_favoritism: Some(8),
            ..Default::default()
        };
        assert!(
            !ocean_rep_flags(&OceanScores::default(), &fair).contains(&"flag_honest_favoritist")
        );
        let low_honest = RepScores {
            honest_deceitful: Some(4),
            fair_favoritism: Some(2),
            ..Default::default()
        };
        assert!(
            !ocean_rep_flags(&OceanScores::default(), &low_honest)
                .contains(&"flag_honest_favoritist")
        );
        assert!(
            !ocean_rep_flags(&OceanScores::default(), &RepScores::default())
                .contains(&"flag_honest_favoritist")
        );
    }

    #[test]
    fn test_affiliation_distrustful_gap() {
        let m = vec![mk_mot(MotivationType::Affiliation, 7)];
        let r = RepScores {
            trusting_suspicious: Some(2),
            ..Default::default()
        };
        assert!(affiliation_distrustful_gap(&m, &r));
        let trusting = RepScores {
            trusting_suspicious: Some(8),
            ..Default::default()
        };
        assert!(!affiliation_distrustful_gap(&m, &trusting));
        assert!(!affiliation_distrustful_gap(
            &[mk_mot(MotivationType::Helping, 7)],
            &r
        ));
        assert!(!affiliation_distrustful_gap(&m, &RepScores::default()));
        let flags = rhetoric_gap_flags(&OceanScores::default(), &r, &m);
        assert!(flags.contains(&"flag_affiliation_distrustful"));
    }

    #[test]
    fn test_warmth_cold() {
        let o = OceanScores {
            agreeableness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            empathetic_detached: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_warmth_cold"));
        let warm = RepScores {
            empathetic_detached: Some(8),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&o, &warm).contains(&"flag_warmth_cold"));
        let low_a = OceanScores {
            agreeableness: Some(4),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&low_a, &r).contains(&"flag_warmth_cold"));
    }

    #[test]
    fn test_discipline_flaky() {
        let o = OceanScores {
            conscientiousness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            reliable_flaky: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_discipline_flaky"));
        let reliable = RepScores {
            reliable_flaky: Some(8),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&o, &reliable).contains(&"flag_discipline_flaky"));
        let low_c = OceanScores {
            conscientiousness: Some(4),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&low_c, &r).contains(&"flag_discipline_flaky"));
    }

    fn pattern(trigger: BehaviorTrigger, response: BehaviorResponse) -> BehavioralPattern {
        BehavioralPattern {
            trigger,
            predicted_behavior: response,
            notes: String::new(),
        }
    }

    #[test]
    fn test_pattern_calm_volatile() {
        let r = RepScores {
            calm_reactive: Some(9),
            ..Default::default()
        };
        let panic_stress = vec![pattern(BehaviorTrigger::Stress, BehaviorResponse::Panics)];
        assert!(pattern_calm_volatile_gap(&panic_stress, &r));
        let escalate_conflict = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::Escalates,
        )];
        assert!(pattern_calm_volatile_gap(&escalate_conflict, &r));
        let calm_under_stress = vec![pattern(
            BehaviorTrigger::Stress,
            BehaviorResponse::RemainsCalm,
        )];
        assert!(!pattern_calm_volatile_gap(&calm_under_stress, &r));
        let volatile_success = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::DismissesOthers,
        )];
        assert!(!pattern_calm_volatile_gap(&volatile_success, &r));
        assert!(!pattern_calm_volatile_gap(
            &panic_stress,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_honest_exploiter() {
        let r = RepScores {
            honest_deceitful: Some(9),
            ..Default::default()
        };
        let exploit = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::ExploitsOpportunistically,
        )];
        assert!(pattern_honest_exploiter_gap(&exploit, &r));
        let undermine = vec![pattern(
            BehaviorTrigger::Recognition,
            BehaviorResponse::UnderminesOthers,
        )];
        assert!(pattern_honest_exploiter_gap(&undermine, &r));
        let honest_pattern = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::SeeksRestoration,
        )];
        assert!(!pattern_honest_exploiter_gap(&honest_pattern, &r));
        assert!(!pattern_honest_exploiter_gap(
            &exploit,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_bias_confirmation_open() {
        let b = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: String::new(),
        }];
        let o = OceanScores {
            openness: Some(9),
            ..Default::default()
        };
        assert!(bias_confirmation_open_gap(&b, &o));
        let low_open = OceanScores {
            openness: Some(4),
            ..Default::default()
        };
        assert!(!bias_confirmation_open_gap(&b, &low_open));
        let low_bias = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 4,
            evidence: String::new(),
        }];
        assert!(!bias_confirmation_open_gap(&low_bias, &o));
    }

    #[test]
    fn test_bias_favoritism_fairness() {
        let m = vec![mk_mot(MotivationType::Fairness, 7)];
        let favor = vec![Bias {
            r#type: BiasType::Favoritism,
            intensity: 8,
            evidence: String::new(),
        }];
        assert!(bias_favoritism_fairness_gap(&favor, &m));
        let ingroup = vec![Bias {
            r#type: BiasType::InGroup,
            intensity: 9,
            evidence: String::new(),
        }];
        assert!(bias_favoritism_fairness_gap(&ingroup, &m));
        let unfair = vec![mk_mot(MotivationType::Helping, 7)];
        assert!(!bias_favoritism_fairness_gap(&favor, &unfair));
        let low_bias = vec![Bias {
            r#type: BiasType::Favoritism,
            intensity: 4,
            evidence: String::new(),
        }];
        assert!(!bias_favoritism_fairness_gap(&low_bias, &m));
    }

    #[test]
    fn test_ocean_rep_flags_high_e_low_a_both_sides() {
        let o_e_high_a_high = OceanScores {
            extraversion: Some(9),
            agreeableness: Some(9),
            ..Default::default()
        };
        assert!(ocean_rep_flags(&o_e_high_a_high, &RepScores::default()).is_empty());
        let o_e_low_a_low = OceanScores {
            extraversion: Some(4),
            agreeableness: Some(2),
            ..Default::default()
        };
        assert!(ocean_rep_flags(&o_e_low_a_low, &RepScores::default()).is_empty());
    }

    #[test]
    fn test_ocean_rep_flags_high_o_low_c_both_sides() {
        let o = OceanScores {
            openness: Some(9),
            conscientiousness: Some(9),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&o, &RepScores::default()).contains(&"flag_high_o_low_c"));
        let o2 = OceanScores {
            openness: Some(4),
            conscientiousness: Some(2),
            ..Default::default()
        };
        assert!(!ocean_rep_flags(&o2, &RepScores::default()).contains(&"flag_high_o_low_c"));
    }

    #[test]
    fn test_style_diplomatic_blunt_gap_both_sides() {
        let styles = vec![PersonalStyle {
            r#type: StyleType::DiplomaticCommunicator,
            intensity: 8,
            notes: String::new(),
        }];
        let r_high_blunt = RepScores {
            diplomatic_blunt: Some(2),
            ..Default::default()
        };
        assert!(style_diplomatic_blunt_gap(&styles, &r_high_blunt));
        let r_not_blunt = RepScores {
            diplomatic_blunt: Some(9),
            ..Default::default()
        };
        assert!(!style_diplomatic_blunt_gap(&styles, &r_not_blunt));
        let no_style = vec![];
        assert!(!style_diplomatic_blunt_gap(&no_style, &r_high_blunt));
    }

    #[test]
    fn test_style_dominant_submissive_gap_both_sides() {
        let styles = vec![PersonalStyle {
            r#type: StyleType::Autocratic,
            intensity: 8,
            notes: String::new(),
        }];
        let r_submissive = RepScores {
            authoritative_submissive: Some(2),
            ..Default::default()
        };
        assert!(style_dominant_submissive_gap(&styles, &r_submissive));
        let r_dominant = RepScores {
            authoritative_submissive: Some(9),
            ..Default::default()
        };
        assert!(!style_dominant_submissive_gap(&styles, &r_dominant));
        let no_style = vec![];
        assert!(!style_dominant_submissive_gap(&no_style, &r_submissive));
    }

    #[test]
    fn test_style_guarded_trusting_gap_both_sides() {
        let styles = vec![PersonalStyle {
            r#type: StyleType::Guarded,
            intensity: 8,
            notes: String::new(),
        }];
        let r_trusting = RepScores {
            trusting_suspicious: Some(9),
            ..Default::default()
        };
        assert!(style_guarded_trusting_gap(&styles, &r_trusting));
        let r_suspicious = RepScores {
            trusting_suspicious: Some(2),
            ..Default::default()
        };
        assert!(!style_guarded_trusting_gap(&styles, &r_suspicious));
        let no_style = vec![];
        assert!(!style_guarded_trusting_gap(&no_style, &r_trusting));
    }

    #[test]
    fn test_style_consensus_authoritative_gap_both_sides() {
        let styles = vec![PersonalStyle {
            r#type: StyleType::ConsensusDriven,
            intensity: 8,
            notes: String::new(),
        }];
        let r_auth = RepScores {
            authoritative_submissive: Some(9),
            ..Default::default()
        };
        assert!(style_consensus_authoritative_gap(&styles, &r_auth));
        let r_sub = RepScores {
            authoritative_submissive: Some(2),
            ..Default::default()
        };
        assert!(!style_consensus_authoritative_gap(&styles, &r_sub));
        let no_style = vec![];
        assert!(!style_consensus_authoritative_gap(&no_style, &r_auth));
    }

    #[test]
    fn test_style_repairs_trust_deceitful_gap_both_sides() {
        let styles = vec![PersonalStyle {
            r#type: StyleType::RepairsTrustActively,
            intensity: 8,
            notes: String::new(),
        }];
        let r_deceitful = RepScores {
            honest_deceitful: Some(2),
            ..Default::default()
        };
        assert!(style_repairs_trust_deceitful_gap(&styles, &r_deceitful));
        let r_honest = RepScores {
            honest_deceitful: Some(9),
            ..Default::default()
        };
        assert!(!style_repairs_trust_deceitful_gap(&styles, &r_honest));
        let no_style = vec![];
        assert!(!style_repairs_trust_deceitful_gap(&no_style, &r_deceitful));
    }

    #[test]
    fn test_value_flags_empty() {
        let flags = value_flags(&[], None, &[]);
        assert!(flags.is_empty());
    }

    #[test]
    fn test_value_flags_family_past() {
        use crate::models::{PersonalStyle, StyleType, Value, ValueType};
        let values = vec![Value {
            r#type: ValueType::Family,
            intensity: 8,
            priority: 5,
            notes: String::new(),
        }];
        let styles = vec![PersonalStyle {
            r#type: StyleType::PresentOriented,
            intensity: 8,
            notes: String::new(),
        }];
        let flags = value_flags(&values, None, &styles);
        assert!(flags.contains(&"flag_value_family_past"));
        let past_style = vec![PersonalStyle {
            r#type: StyleType::PastOriented,
            intensity: 8,
            notes: String::new(),
        }];
        assert!(!value_flags(&values, None, &past_style).contains(&"flag_value_family_past"));
        let low_val = vec![Value {
            r#type: ValueType::Family,
            intensity: 5,
            priority: 5,
            notes: String::new(),
        }];
        assert!(!value_flags(&low_val, None, &styles).contains(&"flag_value_family_past"));
    }

    #[test]
    fn test_value_flags_stability_risk() {
        use crate::models::{Value, ValueType};
        let values = vec![Value {
            r#type: ValueType::Stability,
            intensity: 9,
            priority: 5,
            notes: String::new(),
        }];
        assert!(value_flags(&values, Some(9), &[]).contains(&"flag_value_stability_risk"));
        assert!(value_flags(&values, Some(8), &[]).contains(&"flag_value_stability_risk"));
        assert!(!value_flags(&values, Some(7), &[]).contains(&"flag_value_stability_risk"));
        assert!(!value_flags(&values, None, &[]).contains(&"flag_value_stability_risk"));
        let low_val = vec![Value {
            r#type: ValueType::Stability,
            intensity: 5,
            priority: 5,
            notes: String::new(),
        }];
        assert!(!value_flags(&low_val, Some(9), &[]).contains(&"flag_value_stability_risk"));
    }

    #[test]
    fn test_value_flags_career_family() {
        use crate::models::{Value, ValueType};
        let values = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 9,
                priority: 5,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 9,
                priority: 5,
                notes: String::new(),
            },
        ];
        assert!(value_flags(&values, None, &[]).contains(&"flag_value_career_family"));
        let only_career = vec![Value {
            r#type: ValueType::Career,
            intensity: 9,
            priority: 5,
            notes: String::new(),
        }];
        assert!(!value_flags(&only_career, None, &[]).contains(&"flag_value_career_family"));
        let low_both = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 5,
                priority: 5,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 9,
                priority: 5,
                notes: String::new(),
            },
        ];
        assert!(!value_flags(&low_both, None, &[]).contains(&"flag_value_career_family"));
    }

    #[test]
    fn test_value_flags_loyalty_guarded() {
        use crate::models::{PersonalStyle, StyleType, Value, ValueType};
        let values = vec![Value {
            r#type: ValueType::Loyalty,
            intensity: 9,
            priority: 5,
            notes: String::new(),
        }];
        let guarded = vec![PersonalStyle {
            r#type: StyleType::Guarded,
            intensity: 8,
            notes: String::new(),
        }];
        assert!(value_flags(&values, None, &guarded).contains(&"flag_value_loyalty_guarded"));
        let free_trust = vec![PersonalStyle {
            r#type: StyleType::ExtendsTrustFreely,
            intensity: 8,
            notes: String::new(),
        }];
        assert!(!value_flags(&values, None, &free_trust).contains(&"flag_value_loyalty_guarded"));
        let low_val = vec![Value {
            r#type: ValueType::Loyalty,
            intensity: 5,
            priority: 5,
            notes: String::new(),
        }];
        assert!(!value_flags(&low_val, None, &guarded).contains(&"flag_value_loyalty_guarded"));
    }

    #[test]
    fn test_all_person_flags_includes_evidence() {
        let person = Person {
            id: String::new(),
            name: String::new(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: String::new(),
            tags: Vec::new(),
            notes: String::new(),
            motivations: Vec::new(),
            biases: vec![Bias {
                r#type: BiasType::Confirmation,
                intensity: 8,
                evidence: String::new(),
            }],
            rep_scores: RepScores {
                calm_reactive: Some(9),
                ..Default::default()
            },
            behavioral_patterns: vec![pattern(
                BehaviorTrigger::Threatened,
                BehaviorResponse::BecomesParanoid,
            )],
            styles: Vec::new(),
            values: Vec::new(),
            ocean: OceanScores {
                openness: Some(9),
                ..Default::default()
            },
            resilience: None,
            risk_appetite: None,
            log: Vec::new(),
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        };
        let flags = all_person_flags(&person);
        assert!(flags.contains(&"flag_pattern_calm_volatile"));
        assert!(flags.contains(&"flag_bias_confirmation_open"));
    }

    #[test]
    fn test_security_risky_gap() {
        let m = vec![mk_mot(MotivationType::Security, 7)];
        assert!(security_risky_gap(&m, Some(9)));
        assert!(security_risky_gap(&m, Some(8)));
        assert!(!security_risky_gap(&m, Some(7)));
        assert!(!security_risky_gap(&m, None));
        assert!(!security_risky_gap(
            &[mk_mot(MotivationType::Helping, 8)],
            Some(9)
        ));
    }

    #[test]
    fn test_resilient_reactive_gap() {
        let r = RepScores {
            calm_reactive: Some(2),
            ..Default::default()
        };
        assert!(resilient_reactive_gap(Some(9), &r));
        assert!(!resilient_reactive_gap(Some(7), &r));
        assert!(!resilient_reactive_gap(None, &r));
        let calm = RepScores {
            calm_reactive: Some(8),
            ..Default::default()
        };
        assert!(!resilient_reactive_gap(Some(9), &calm));
    }

    #[test]
    fn test_autonomy_submissive_gap() {
        let m = vec![mk_mot(MotivationType::Autonomy, 7)];
        let r = RepScores {
            authoritative_submissive: Some(2),
            ..Default::default()
        };
        assert!(autonomy_submissive_gap(&m, &r));
        let dominant = RepScores {
            authoritative_submissive: Some(8),
            ..Default::default()
        };
        assert!(!autonomy_submissive_gap(&m, &dominant));
        assert!(!autonomy_submissive_gap(
            &[mk_mot(MotivationType::Power, 8)],
            &r
        ));
        assert!(!autonomy_submissive_gap(&m, &RepScores::default()));
        let flags = rhetoric_gap_flags(&OceanScores::default(), &r, &m);
        assert!(flags.contains(&"flag_autonomy_submissive"));
    }

    #[test]
    fn test_learning_rigid_gap() {
        let m = vec![mk_mot(MotivationType::Learning, 7)];
        let r = RepScores {
            adaptable_rigid: Some(2),
            ..Default::default()
        };
        assert!(learning_rigid_gap(&m, &r));
        let flexible = RepScores {
            adaptable_rigid: Some(8),
            ..Default::default()
        };
        assert!(!learning_rigid_gap(&m, &flexible));
        assert!(!learning_rigid_gap(&m, &RepScores::default()));
        let flags = rhetoric_gap_flags(&OceanScores::default(), &r, &m);
        assert!(flags.contains(&"flag_learning_rigid"));
    }

    #[test]
    fn test_creativity_closed_gap() {
        let m = vec![mk_mot(MotivationType::Creativity, 7)];
        let o = OceanScores {
            openness: Some(2),
            ..Default::default()
        };
        assert!(creativity_closed_gap(&m, &o));
        let open = OceanScores {
            openness: Some(8),
            ..Default::default()
        };
        assert!(!creativity_closed_gap(&m, &open));
        assert!(!creativity_closed_gap(
            &[mk_mot(MotivationType::Learning, 7)],
            &o
        ));
        assert!(!creativity_closed_gap(&m, &OceanScores::default()));
        let flags = rhetoric_gap_flags(&o, &RepScores::default(), &m);
        assert!(flags.contains(&"flag_creativity_closed"));
    }

    #[test]
    fn test_creativity_rigid_gap() {
        let m = vec![mk_mot(MotivationType::Creativity, 7)];
        let r = RepScores {
            adaptable_rigid: Some(2),
            ..Default::default()
        };
        assert!(creativity_rigid_gap(&m, &r));
        let flexible = RepScores {
            adaptable_rigid: Some(8),
            ..Default::default()
        };
        assert!(!creativity_rigid_gap(&m, &flexible));
        assert!(!creativity_rigid_gap(&m, &RepScores::default()));
        let flags = rhetoric_gap_flags(&OceanScores::default(), &r, &m);
        assert!(flags.contains(&"flag_creativity_rigid"));
    }

    #[test]
    fn test_authority_dominant_gap() {
        let b = vec![mk_bias(BiasType::Authority, 8)];
        let r = RepScores {
            authoritative_submissive: Some(9),
            ..Default::default()
        };
        assert!(authority_dominant_gap(&b, &r));
        let submissive = RepScores {
            authoritative_submissive: Some(4),
            ..Default::default()
        };
        assert!(!authority_dominant_gap(&b, &submissive));
        let low_bias = vec![mk_bias(BiasType::Authority, 4)];
        assert!(!authority_dominant_gap(&low_bias, &r));
        assert!(!authority_dominant_gap(&b, &RepScores::default()));
        assert!(!authority_dominant_gap(
            &[mk_bias(BiasType::Anchoring, 8)],
            &r
        ));
    }

    #[test]
    fn test_social_proof_open_gap() {
        let b = vec![mk_bias(BiasType::SocialProof, 8)];
        let o = OceanScores {
            openness: Some(9),
            ..Default::default()
        };
        assert!(social_proof_open_gap(&b, &o));
        let closed = OceanScores {
            openness: Some(5),
            ..Default::default()
        };
        assert!(!social_proof_open_gap(&b, &closed));
        assert!(!social_proof_open_gap(&b, &OceanScores::default()));
        assert!(!social_proof_open_gap(&[mk_bias(BiasType::Recency, 8)], &o));
    }

    #[test]
    fn test_sunk_cost_flexible_gap() {
        let b = vec![mk_bias(BiasType::SunkCost, 8)];
        let r = RepScores {
            adaptable_rigid: Some(9),
            ..Default::default()
        };
        assert!(sunk_cost_flexible_gap(&b, &r));
        let rigid = RepScores {
            adaptable_rigid: Some(3),
            ..Default::default()
        };
        assert!(!sunk_cost_flexible_gap(&b, &rigid));
        assert!(!sunk_cost_flexible_gap(&b, &RepScores::default()));
        assert!(!sunk_cost_flexible_gap(
            &[mk_bias(BiasType::Availability, 8)],
            &r
        ));
    }

    #[test]
    fn test_pattern_diplomat_escalator() {
        let r = RepScores {
            diplomatic_blunt: Some(9),
            ..Default::default()
        };
        let escalate = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::Escalates,
        )];
        assert!(pattern_diplomat_escalator_gap(&escalate, &r));
        let passive_agg = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::BecomesPassiveAggressive,
        )];
        assert!(pattern_diplomat_escalator_gap(&passive_agg, &r));
        let constructive = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::SeeksCompromise,
        )];
        assert!(!pattern_diplomat_escalator_gap(&constructive, &r));
        assert!(!pattern_diplomat_escalator_gap(
            &escalate,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_fair_exploiter() {
        let r = RepScores {
            fair_favoritism: Some(9),
            ..Default::default()
        };
        let exploit = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::ExploitsOpportunistically,
        )];
        assert!(pattern_fair_exploiter_gap(&exploit, &r));
        let restore = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::SeeksRestoration,
        )];
        assert!(!pattern_fair_exploiter_gap(&restore, &r));
        assert!(!pattern_fair_exploiter_gap(&exploit, &RepScores::default()));
    }

    #[test]
    fn test_pattern_humble_dismissive() {
        let r = RepScores {
            humble_arrogant: Some(9),
            ..Default::default()
        };
        let dismiss = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::DismissesOthers,
        )];
        assert!(pattern_humble_dismissive_gap(&dismiss, &r));
        let demands = vec![pattern(
            BehaviorTrigger::Recognition,
            BehaviorResponse::DemandsAttention,
        )];
        assert!(pattern_humble_dismissive_gap(&demands, &r));
        let blame = vec![pattern(
            BehaviorTrigger::Threatened,
            BehaviorResponse::DeflectsBlame,
        )];
        assert!(pattern_humble_dismissive_gap(&blame, &r));
        let celebrates = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::CelebratesWithOthers,
        )];
        assert!(!pattern_humble_dismissive_gap(&celebrates, &r));
        assert!(!pattern_humble_dismissive_gap(
            &dismiss,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_trusting_paranoid() {
        let r = RepScores {
            trusting_suspicious: Some(9),
            ..Default::default()
        };
        let paranoid = vec![pattern(
            BehaviorTrigger::Threatened,
            BehaviorResponse::BecomesParanoid,
        )];
        assert!(pattern_trusting_paranoid_gap(&paranoid, &r));
        let cautious = vec![pattern(
            BehaviorTrigger::Threatened,
            BehaviorResponse::BecomesCautious,
        )];
        assert!(!pattern_trusting_paranoid_gap(&cautious, &r));
        assert!(!pattern_trusting_paranoid_gap(
            &paranoid,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_reliable_shirker() {
        let r = RepScores {
            reliable_flaky: Some(9),
            ..Default::default()
        };
        let deflect = vec![pattern(
            BehaviorTrigger::Uncertainty,
            BehaviorResponse::DeflectsResponsibility,
        )];
        assert!(pattern_reliable_shirker_gap(&deflect, &r));
        let blame = vec![pattern(
            BehaviorTrigger::Threatened,
            BehaviorResponse::DeflectsBlame,
        )];
        assert!(pattern_reliable_shirker_gap(&blame, &r));
        let sabotage = vec![pattern(
            BehaviorTrigger::Change,
            BehaviorResponse::Sabotages,
        )];
        assert!(pattern_reliable_shirker_gap(&sabotage, &r));
        let focuses = vec![pattern(
            BehaviorTrigger::Uncertainty,
            BehaviorResponse::SeeksData,
        )];
        assert!(!pattern_reliable_shirker_gap(&focuses, &r));
        assert!(!pattern_reliable_shirker_gap(
            &deflect,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_hardworker_complacent() {
        let r = RepScores {
            hardworker_lazy: Some(9),
            ..Default::default()
        };
        let complacent = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::BecomesComplacent,
        )];
        assert!(pattern_hardworker_complacent_gap(&complacent, &r));
        let overconfident = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::BecomesOverconfident,
        )];
        assert!(pattern_hardworker_complacent_gap(&overconfident, &r));
        let goals = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::SetsNewGoals,
        )];
        assert!(!pattern_hardworker_complacent_gap(&goals, &r));
        assert!(!pattern_hardworker_complacent_gap(
            &complacent,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_risk_appetite_ambition_gap() {
        let power = vec![mk_mot(MotivationType::Power, 7)];
        assert!(risk_appetite_ambition_gap(&power, Some(2)));
        assert!(risk_appetite_ambition_gap(&power, Some(3)));
        assert!(!risk_appetite_ambition_gap(&power, Some(4)));
        assert!(!risk_appetite_ambition_gap(&power, None));
        let achievement = vec![mk_mot(MotivationType::Achievement, 7)];
        assert!(risk_appetite_ambition_gap(&achievement, Some(2)));
        assert!(!risk_appetite_ambition_gap(
            &[mk_mot(MotivationType::Helping, 8)],
            Some(2)
        ));
    }

    #[test]
    fn test_power_passive_gap() {
        let m = vec![mk_mot(MotivationType::Power, 7)];
        let r = RepScores {
            assertive_passive: Some(2),
            ..Default::default()
        };
        assert!(power_passive_gap(&m, &r));
        let assertive = RepScores {
            assertive_passive: Some(8),
            ..Default::default()
        };
        assert!(!power_passive_gap(&m, &assertive));
        assert!(!power_passive_gap(&m, &RepScores::default()));
        let flags = rhetoric_gap_flags(&OceanScores::default(), &r, &m);
        assert!(flags.contains(&"flag_power_passive"));
    }

    #[test]
    fn test_helping_cold_gap() {
        let m = vec![mk_mot(MotivationType::Helping, 7)];
        let r = RepScores {
            empathetic_detached: Some(2),
            ..Default::default()
        };
        assert!(helping_cold_gap(&m, &r));
        let warm = RepScores {
            empathetic_detached: Some(8),
            ..Default::default()
        };
        assert!(!helping_cold_gap(&m, &warm));
        assert!(!helping_cold_gap(
            &[mk_mot(MotivationType::Security, 7)],
            &r
        ));
        assert!(!helping_cold_gap(&m, &RepScores::default()));
        let flags = rhetoric_gap_flags(&OceanScores::default(), &r, &m);
        assert!(flags.contains(&"flag_helping_cold"));
    }

    #[test]
    fn test_pattern_passive_blowup() {
        let r = RepScores {
            assertive_passive: Some(2),
            ..Default::default()
        };
        let escalate = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::Escalates,
        )];
        assert!(pattern_passive_blowup_gap(&escalate, &r));
        let irritable = vec![pattern(
            BehaviorTrigger::Stress,
            BehaviorResponse::BecomesIrritable,
        )];
        assert!(pattern_passive_blowup_gap(&irritable, &r));
        let calm = vec![pattern(
            BehaviorTrigger::Stress,
            BehaviorResponse::RemainsCalm,
        )];
        assert!(!pattern_passive_blowup_gap(&calm, &r));
        let assertive = RepScores {
            assertive_passive: Some(8),
            ..Default::default()
        };
        assert!(!pattern_passive_blowup_gap(&escalate, &assertive));
    }

    #[test]
    fn test_pattern_assertive_quiet() {
        let r = RepScores {
            assertive_passive: Some(9),
            ..Default::default()
        };
        let silent = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::StaysSilent,
        )];
        assert!(pattern_assertive_quiet_gap(&silent, &r));
        let withdraw = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::WithdrawsFromInjustice,
        )];
        assert!(pattern_assertive_quiet_gap(&withdraw, &r));
        let firm = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::ProtestsFirmly,
        )];
        assert!(!pattern_assertive_quiet_gap(&firm, &r));
        let passive = RepScores {
            assertive_passive: Some(3),
            ..Default::default()
        };
        assert!(!pattern_assertive_quiet_gap(&silent, &passive));
    }

    #[test]
    fn test_loss_aversion_risky_gap() {
        let b = vec![mk_bias(BiasType::LossAversion, 8)];
        assert!(loss_aversion_risky_gap(&b, Some(9)));
        assert!(!loss_aversion_risky_gap(&b, Some(6)));
        assert!(!loss_aversion_risky_gap(&b, None));
        assert!(!loss_aversion_risky_gap(
            &[mk_bias(BiasType::Anchoring, 8)],
            Some(9)
        ));
    }

    #[test]
    fn test_dunning_kruger_humble_gap() {
        let b = vec![mk_bias(BiasType::DunningKruger, 8)];
        let r = RepScores {
            humble_arrogant: Some(2),
            ..Default::default()
        };
        assert!(dunning_kruger_humble_gap(&b, &r));
        let arrogant = RepScores {
            humble_arrogant: Some(8),
            ..Default::default()
        };
        assert!(!dunning_kruger_humble_gap(&b, &arrogant));
        assert!(!dunning_kruger_humble_gap(&b, &RepScores::default()));
        assert!(!dunning_kruger_humble_gap(
            &[mk_bias(BiasType::Recency, 8)],
            &r
        ));
    }

    #[test]
    fn test_impostor_arrogant_gap() {
        let b = vec![mk_bias(BiasType::Impostor, 8)];
        let arrogant = RepScores {
            humble_arrogant: Some(9),
            ..Default::default()
        };
        assert!(impostor_arrogant_gap(&b, &arrogant));
        let humble = RepScores {
            humble_arrogant: Some(2),
            ..Default::default()
        };
        assert!(!impostor_arrogant_gap(&b, &humble));
        assert!(!impostor_arrogant_gap(&b, &RepScores::default()));
        assert!(!impostor_arrogant_gap(
            &[mk_bias(BiasType::DunningKruger, 8)],
            &arrogant
        ));
    }

    #[test]
    fn test_recency_reliable_gap() {
        let b = vec![mk_bias(BiasType::Recency, 8)];
        let r = RepScores {
            reliable_flaky: Some(9),
            ..Default::default()
        };
        assert!(recency_reliable_gap(&b, &r));
        let flaky = RepScores {
            reliable_flaky: Some(3),
            ..Default::default()
        };
        assert!(!recency_reliable_gap(&b, &flaky));
        assert!(!recency_reliable_gap(&b, &RepScores::default()));
        assert!(!recency_reliable_gap(
            &[mk_bias(BiasType::Authority, 8)],
            &r
        ));
    }

    #[test]
    fn test_resilient_hides_gap() {
        let r = RepScores {
            calm_reactive: Some(9),
            ..Default::default()
        };
        assert!(resilient_hides_gap(Some(2), &r));
        assert!(resilient_hides_gap(Some(3), &r));
        assert!(!resilient_hides_gap(Some(5), &r));
        assert!(!resilient_hides_gap(None, &r));
        let reactive = RepScores {
            calm_reactive: Some(3),
            ..Default::default()
        };
        assert!(!resilient_hides_gap(Some(2), &reactive));
    }

    fn mk_style(t: StyleType, intensity: u8) -> PersonalStyle {
        PersonalStyle {
            r#type: t,
            intensity,
            notes: String::new(),
        }
    }

    #[test]
    fn test_pattern_generous_exploiter_gap() {
        let r = RepScores {
            generous_selfish: Some(9),
            ..Default::default()
        };
        let exploit = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::ExploitsOpportunistically,
        )];
        assert!(pattern_generous_exploiter_gap(&exploit, &r));
        let undermine = vec![pattern(
            BehaviorTrigger::Recognition,
            BehaviorResponse::UnderminesOthers,
        )];
        assert!(pattern_generous_exploiter_gap(&undermine, &r));
        let selfish = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::SeeksRestoration,
        )];
        assert!(!pattern_generous_exploiter_gap(&selfish, &r));
        assert!(!pattern_generous_exploiter_gap(
            &exploit,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_empath_dismissive_gap() {
        let r = RepScores {
            empathetic_detached: Some(9),
            ..Default::default()
        };
        let dismiss = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::DismissesOthers,
        )];
        assert!(pattern_empath_dismissive_gap(&dismiss, &r));
        let attention = vec![pattern(
            BehaviorTrigger::Recognition,
            BehaviorResponse::DemandsAttention,
        )];
        assert!(pattern_empath_dismissive_gap(&attention, &r));
        let kind = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::CelebratesWithOthers,
        )];
        assert!(!pattern_empath_dismissive_gap(&kind, &r));
        assert!(!pattern_empath_dismissive_gap(
            &dismiss,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_flexible_resister_gap() {
        let r = RepScores {
            adaptable_rigid: Some(9),
            ..Default::default()
        };
        let resist = vec![pattern(
            BehaviorTrigger::Change,
            BehaviorResponse::ResistsChange,
        )];
        assert!(pattern_flexible_resister_gap(&resist, &r));
        let ignore = vec![pattern(
            BehaviorTrigger::Feedback,
            BehaviorResponse::IgnoresCompletely,
        )];
        assert!(pattern_flexible_resister_gap(&ignore, &r));
        let adapt = vec![pattern(
            BehaviorTrigger::Change,
            BehaviorResponse::EmbracesChange,
        )];
        assert!(!pattern_flexible_resister_gap(&adapt, &r));
        assert!(!pattern_flexible_resister_gap(
            &resist,
            &RepScores::default()
        ));
    }

    #[test]
    fn test_pattern_helping_exploiter_gap() {
        let mot = vec![mk_mot(MotivationType::Helping, 7)];
        let exploit = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::ExploitsOpportunistically,
        )];
        assert!(pattern_helping_exploiter_gap(&exploit, &mot));
        let undermine = vec![pattern(
            BehaviorTrigger::Recognition,
            BehaviorResponse::UnderminesOthers,
        )];
        assert!(pattern_helping_exploiter_gap(&undermine, &mot));
        assert!(!pattern_helping_exploiter_gap(
            &exploit,
            &[mk_mot(MotivationType::Power, 8)]
        ));
        assert!(!pattern_helping_exploiter_gap(
            &[pattern(
                BehaviorTrigger::Injustice,
                BehaviorResponse::SeeksRestoration
            )],
            &mot
        ));
    }

    #[test]
    fn test_pattern_warmth_dismissive_gap() {
        let o = OceanScores {
            agreeableness: Some(9),
            ..Default::default()
        };
        let dismiss = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::DismissesOthers,
        )];
        assert!(pattern_warmth_dismissive_gap(&dismiss, &o));
        let low_a = OceanScores {
            agreeableness: Some(5),
            ..Default::default()
        };
        assert!(!pattern_warmth_dismissive_gap(&dismiss, &low_a));
        assert!(!pattern_warmth_dismissive_gap(
            &[pattern(
                BehaviorTrigger::Success,
                BehaviorResponse::CelebratesWithOthers
            )],
            &o
        ));
    }

    #[test]
    fn test_pattern_discipline_shirker_gap() {
        let o = OceanScores {
            conscientiousness: Some(9),
            ..Default::default()
        };
        let shirk = vec![pattern(
            BehaviorTrigger::Uncertainty,
            BehaviorResponse::DeflectsResponsibility,
        )];
        assert!(pattern_discipline_shirker_gap(&shirk, &o));
        let low_c = OceanScores {
            conscientiousness: Some(5),
            ..Default::default()
        };
        assert!(!pattern_discipline_shirker_gap(&shirk, &low_c));
        assert!(!pattern_discipline_shirker_gap(
            &[pattern(
                BehaviorTrigger::Uncertainty,
                BehaviorResponse::WaitsForClarity
            )],
            &o
        ));
    }

    #[test]
    fn test_pattern_claimed_calm_volatile_gap() {
        let o = OceanScores {
            neuroticism: Some(2),
            ..Default::default()
        };
        let volatile = vec![pattern(BehaviorTrigger::Stress, BehaviorResponse::Panics)];
        assert!(pattern_claimed_calm_volatile_gap(&volatile, &o));
        let mid = OceanScores {
            neuroticism: Some(5),
            ..Default::default()
        };
        assert!(!pattern_claimed_calm_volatile_gap(&volatile, &mid));
        assert!(!pattern_claimed_calm_volatile_gap(
            &[pattern(
                BehaviorTrigger::Stress,
                BehaviorResponse::RemainsCalm
            )],
            &o
        ));
        assert!(!pattern_claimed_calm_volatile_gap(
            &volatile,
            &OceanScores::default()
        ));
    }

    #[test]
    fn test_pattern_fairness_exploiter_gap() {
        let mot = vec![mk_mot(MotivationType::Fairness, 7)];
        let exploit = vec![pattern(
            BehaviorTrigger::Injustice,
            BehaviorResponse::ExploitsOpportunistically,
        )];
        assert!(pattern_fairness_exploiter_gap(&exploit, &mot));
        assert!(!pattern_fairness_exploiter_gap(
            &exploit,
            &[mk_mot(MotivationType::Helping, 8)]
        ));
        assert!(!pattern_fairness_exploiter_gap(
            &[pattern(
                BehaviorTrigger::Injustice,
                BehaviorResponse::SeeksRestoration
            )],
            &mot
        ));
        assert!(!pattern_fairness_exploiter_gap(
            &[pattern(
                BehaviorTrigger::Recognition,
                BehaviorResponse::ExploitsOpportunistically
            )],
            &mot
        ));
    }

    #[test]
    fn test_pattern_achievement_complacent_gap() {
        let mot = vec![mk_mot(MotivationType::Achievement, 7)];
        let complacent = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::BecomesComplacent,
        )];
        assert!(pattern_achievement_complacent_gap(&complacent, &mot));
        let overconfident = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::BecomesOverconfident,
        )];
        assert!(pattern_achievement_complacent_gap(&overconfident, &mot));
        assert!(!pattern_achievement_complacent_gap(
            &[pattern(
                BehaviorTrigger::Success,
                BehaviorResponse::CelebratesWithOthers
            )],
            &mot
        ));
        assert!(!pattern_achievement_complacent_gap(
            &complacent,
            &[mk_mot(MotivationType::Power, 8)]
        ));
    }

    #[test]
    fn test_pattern_learning_resister_gap() {
        let mot = vec![mk_mot(MotivationType::Learning, 7)];
        let resist = vec![pattern(
            BehaviorTrigger::Feedback,
            BehaviorResponse::RejectsFeedback,
        )];
        assert!(pattern_learning_resister_gap(&resist, &mot));
        let ignore = vec![pattern(
            BehaviorTrigger::Change,
            BehaviorResponse::IgnoresCompletely,
        )];
        assert!(pattern_learning_resister_gap(&ignore, &mot));
        assert!(!pattern_learning_resister_gap(
            &resist,
            &[mk_mot(MotivationType::Achievement, 8)]
        ));
        assert!(!pattern_learning_resister_gap(
            &[pattern(
                BehaviorTrigger::Feedback,
                BehaviorResponse::SeeksFeedback
            )],
            &mot
        ));
    }

    #[test]
    fn test_pattern_extravert_quiet_gap() {
        let o = OceanScores {
            extraversion: Some(9),
            ..Default::default()
        };
        let quiet = vec![pattern(
            BehaviorTrigger::Conflict,
            BehaviorResponse::StaysSilent,
        )];
        assert!(pattern_extravert_quiet_gap(&quiet, &o));
        let mid = OceanScores {
            extraversion: Some(5),
            ..Default::default()
        };
        assert!(!pattern_extravert_quiet_gap(&quiet, &mid));
        assert!(!pattern_extravert_quiet_gap(
            &[pattern(
                BehaviorTrigger::Conflict,
                BehaviorResponse::Escalates
            )],
            &o
        ));
        assert!(!pattern_extravert_quiet_gap(
            &quiet,
            &OceanScores::default()
        ));
    }

    #[test]
    fn test_pattern_open_resister_gap() {
        let o = OceanScores {
            openness: Some(9),
            ..Default::default()
        };
        let resist = vec![pattern(
            BehaviorTrigger::Change,
            BehaviorResponse::ResistsChange,
        )];
        assert!(pattern_open_resister_gap(&resist, &o));
        let ignore = vec![pattern(
            BehaviorTrigger::Feedback,
            BehaviorResponse::IgnoresCompletely,
        )];
        assert!(pattern_open_resister_gap(&ignore, &o));
        let mid = OceanScores {
            openness: Some(5),
            ..Default::default()
        };
        assert!(!pattern_open_resister_gap(&resist, &mid));
        assert!(!pattern_open_resister_gap(
            &[pattern(
                BehaviorTrigger::Change,
                BehaviorResponse::EmbracesChange
            )],
            &o
        ));
    }

    #[test]
    fn test_pattern_recognition_dismissive_gap() {
        let mot = vec![mk_mot(MotivationType::Recognition, 7)];
        let dismiss = vec![pattern(
            BehaviorTrigger::Recognition,
            BehaviorResponse::DemandsAttention,
        )];
        assert!(pattern_recognition_dismissive_gap(&dismiss, &mot));
        let undermine = vec![pattern(
            BehaviorTrigger::Success,
            BehaviorResponse::UnderminesOthers,
        )];
        assert!(pattern_recognition_dismissive_gap(&undermine, &mot));
        assert!(!pattern_recognition_dismissive_gap(
            &dismiss,
            &[mk_mot(MotivationType::Helping, 8)]
        ));
        assert!(!pattern_recognition_dismissive_gap(
            &[pattern(
                BehaviorTrigger::Success,
                BehaviorResponse::CelebratesWithOthers
            )],
            &mot
        ));
    }

    #[test]
    fn test_availability_calm_gap() {
        let b = vec![mk_bias(BiasType::Availability, 8)];
        let r = RepScores {
            calm_reactive: Some(9),
            ..Default::default()
        };
        assert!(availability_calm_gap(&b, &r));
        let reactive = RepScores {
            calm_reactive: Some(3),
            ..Default::default()
        };
        assert!(!availability_calm_gap(&b, &reactive));
        assert!(!availability_calm_gap(&b, &RepScores::default()));
        assert!(!availability_calm_gap(&[mk_bias(BiasType::Recency, 8)], &r));
    }

    #[test]
    fn test_style_gaps_extended() {
        let authoritative = RepScores {
            authoritative_submissive: Some(9),
            ..Default::default()
        };
        assert!(style_servant_authoritative_gap(
            &[mk_style(StyleType::Servant, 7)],
            &authoritative
        ));
        for t in [StyleType::Participatory, StyleType::ConsensusDriven] {
            assert!(style_consensus_authoritative_gap(
                &[mk_style(t, 6)],
                &authoritative
            ));
        }
        let submissive = RepScores {
            authoritative_submissive: Some(2),
            ..Default::default()
        };
        assert!(!style_servant_authoritative_gap(
            &[mk_style(StyleType::Servant, 7)],
            &submissive
        ));

        let suspicious = RepScores {
            trusting_suspicious: Some(2),
            ..Default::default()
        };
        assert!(style_trusts_freely_suspicious_gap(
            &[mk_style(StyleType::ExtendsTrustFreely, 6)],
            &suspicious
        ));
        let trusting = RepScores {
            trusting_suspicious: Some(9),
            ..Default::default()
        };
        assert!(!style_trusts_freely_suspicious_gap(
            &[mk_style(StyleType::ExtendsTrustFreely, 6)],
            &trusting
        ));

        let deceitful = RepScores {
            honest_deceitful: Some(2),
            ..Default::default()
        };
        assert!(style_repairs_trust_deceitful_gap(
            &[mk_style(StyleType::RepairsTrustActively, 6)],
            &deceitful
        ));

        let favoritist = RepScores {
            fair_favoritism: Some(2),
            ..Default::default()
        };
        assert!(style_rulebased_favoritist_gap(
            &[mk_style(StyleType::RuleBased, 6)],
            &favoritist
        ));
        let fair = RepScores {
            fair_favoritism: Some(9),
            ..Default::default()
        };
        assert!(!style_rulebased_favoritist_gap(
            &[mk_style(StyleType::RuleBased, 6)],
            &fair
        ));

        let deceitful = RepScores {
            honest_deceitful: Some(2),
            ..Default::default()
        };
        assert!(style_virtuebased_deceitful_gap(
            &[mk_style(StyleType::VirtueBased, 6)],
            &deceitful
        ));
        let honest = RepScores {
            honest_deceitful: Some(9),
            ..Default::default()
        };
        assert!(!style_virtuebased_deceitful_gap(
            &[mk_style(StyleType::VirtueBased, 6)],
            &honest
        ));
    }

    #[test]
    fn test_anchoring_open_gap() {
        let b = vec![mk_bias(BiasType::Anchoring, 8)];
        let o = OceanScores {
            openness: Some(9),
            ..Default::default()
        };
        assert!(anchoring_open_gap(&b, &o));
        let closed = OceanScores {
            openness: Some(5),
            ..Default::default()
        };
        assert!(!anchoring_open_gap(&b, &closed));
        assert!(!anchoring_open_gap(&b, &OceanScores::default()));
        assert!(!anchoring_open_gap(
            &[mk_bias(BiasType::Availability, 8)],
            &o
        ));
    }

    #[test]
    fn test_learning_arrogant_gap() {
        let mot = vec![mk_mot(MotivationType::Learning, 7)];
        let r = RepScores {
            humble_arrogant: Some(2),
            ..Default::default()
        };
        assert!(learning_arrogant_gap(&mot, &r));
        let humble = RepScores {
            humble_arrogant: Some(8),
            ..Default::default()
        };
        assert!(!learning_arrogant_gap(&mot, &humble));
        assert!(!learning_arrogant_gap(
            &[mk_mot(MotivationType::Power, 8)],
            &r
        ));
        assert!(!learning_arrogant_gap(&mot, &RepScores::default()));
    }

    #[test]
    fn test_warmth_selfish_gap() {
        let o = OceanScores {
            agreeableness: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            generous_selfish: Some(2),
            ..Default::default()
        };
        assert!(warmth_selfish_gap(&o, &r));
        let generous = RepScores {
            generous_selfish: Some(8),
            ..Default::default()
        };
        assert!(!warmth_selfish_gap(&o, &generous));
        let low_a = OceanScores {
            agreeableness: Some(5),
            ..Default::default()
        };
        assert!(!warmth_selfish_gap(&low_a, &r));
        assert!(!warmth_selfish_gap(&o, &RepScores::default()));
    }

    #[test]
    fn test_style_gaps() {
        let diplomatic = RepScores {
            diplomatic_blunt: Some(9),
            ..Default::default()
        };
        let direct = vec![mk_style(StyleType::DirectCommunicator, 7)];
        assert!(style_direct_diplomatic_gap(&direct, &diplomatic));
        let weak = vec![mk_style(StyleType::DirectCommunicator, 5)];
        assert!(!style_direct_diplomatic_gap(&weak, &diplomatic));
        assert!(!style_direct_diplomatic_gap(&direct, &RepScores::default()));

        let blunt = RepScores {
            diplomatic_blunt: Some(2),
            ..Default::default()
        };
        assert!(style_diplomatic_blunt_gap(
            &[mk_style(StyleType::DiplomaticCommunicator, 6)],
            &blunt
        ));

        let passive = RepScores {
            assertive_passive: Some(2),
            ..Default::default()
        };
        assert!(style_competing_passive_gap(
            &[mk_style(StyleType::Competing, 8)],
            &passive
        ));

        let submissive = RepScores {
            authoritative_submissive: Some(2),
            ..Default::default()
        };
        for t in [StyleType::Autocratic, StyleType::Controlling] {
            assert!(style_dominant_submissive_gap(
                &[mk_style(t, 6)],
                &submissive
            ));
        }

        let honest = RepScores {
            honest_deceitful: Some(9),
            ..Default::default()
        };
        for t in [
            StyleType::Opportunistic,
            StyleType::Manipulative,
            StyleType::Intrusive,
        ] {
            assert!(style_manipulative_honest_gap(&[mk_style(t, 6)], &honest));
        }

        let cold = RepScores {
            empathetic_detached: Some(2),
            ..Default::default()
        };
        for t in [
            StyleType::Empathetic,
            StyleType::Respectful,
            StyleType::Supportive,
            StyleType::Nurturing,
        ] {
            assert!(style_empathetic_cold_gap(&[mk_style(t, 6)], &cold));
        }
        assert!(!style_empathetic_cold_gap(
            &[mk_style(StyleType::Competing, 8)],
            &cold
        ));

        let trusting = RepScores {
            trusting_suspicious: Some(9),
            ..Default::default()
        };
        for t in [StyleType::Guarded, StyleType::VerifiesTrust] {
            assert!(style_guarded_trusting_gap(&[mk_style(t, 6)], &trusting));
        }
    }

    #[test]
    fn test_style_gap_flags_aggregator() {
        let styles = vec![
            mk_style(StyleType::Competing, 7),
            mk_style(StyleType::Empathetic, 8),
        ];
        let rep = RepScores {
            assertive_passive: Some(2),
            empathetic_detached: Some(2),
            ..Default::default()
        };
        let flags = style_gap_flags(&styles, &rep);
        assert!(flags.contains(&"flag_style_competing_passive"));
        assert!(flags.contains(&"flag_style_empathetic_cold"));
        assert_eq!(flags.len(), 2);
        assert!(style_gap_flags(&[], &rep).is_empty());
        assert!(style_gap_flags(&styles, &RepScores::default()).is_empty());
    }

    #[test]
    fn test_style_controlling_consistent_fires_on_match() {
        let styles = vec![mk_style(StyleType::Controlling, 8)];
        let rep = RepScores {
            authoritative_submissive: Some(9),
            ..Default::default()
        };
        assert!(style_controlling_consistent(&styles, &rep));
        let flags = style_gap_flags(&styles, &rep);
        assert!(flags.contains(&"flag_style_controlling"));
    }

    #[test]
    fn test_style_controlling_consistent_quiescent_when_submissive_or_low() {
        let controlling_style = vec![mk_style(StyleType::Controlling, 8)];
        // Perceived as submissive (low authoritative) → aspiration gap, not consistent
        let submissive = RepScores {
            authoritative_submissive: Some(2),
            ..Default::default()
        };
        assert!(!style_controlling_consistent(
            &controlling_style,
            &submissive
        ));
        assert!(
            !style_gap_flags(&controlling_style, &submissive).contains(&"flag_style_controlling")
        );
        // No controlling style → no flag even if perceived authoritative
        let authoritative = RepScores {
            authoritative_submissive: Some(9),
            ..Default::default()
        };
        assert!(!style_controlling_consistent(&[], &authoritative));
        assert!(!style_gap_flags(&[], &authoritative).contains(&"flag_style_controlling"));
    }

    #[test]
    fn test_style_manipulative_consistent_fires_on_match() {
        for t in [
            StyleType::Opportunistic,
            StyleType::Manipulative,
            StyleType::Intrusive,
        ] {
            let styles = vec![mk_style(t, 8)];
            let rep = RepScores {
                honest_deceitful: Some(2),
                ..Default::default()
            };
            assert!(style_manipulative_consistent(&styles, &rep));
            assert!(style_gap_flags(&styles, &rep).contains(&"flag_style_manipulative"));
        }
    }

    #[test]
    fn test_style_manipulative_consistent_quiescent_when_honest_or_low() {
        let manipulative_style = vec![mk_style(StyleType::Manipulative, 8)];
        // Perceived as honest → aspiration gap (manipulative_honest), not consistent
        let honest = RepScores {
            honest_deceitful: Some(9),
            ..Default::default()
        };
        assert!(!style_manipulative_consistent(&manipulative_style, &honest));
        assert!(
            !style_gap_flags(&manipulative_style, &honest).contains(&"flag_style_manipulative")
        );
        // No manipulative style → no flag even if perceived deceitful
        let deceitful = RepScores {
            honest_deceitful: Some(2),
            ..Default::default()
        };
        assert!(!style_manipulative_consistent(&[], &deceitful));
        assert!(!style_gap_flags(&[], &deceitful).contains(&"flag_style_manipulative"));
    }

    #[test]
    fn test_all_person_flags_includes_styles() {
        let person = Person {
            id: String::new(),
            name: String::new(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: String::new(),
            tags: Vec::new(),
            notes: String::new(),
            motivations: Vec::new(),
            biases: Vec::new(),
            rep_scores: RepScores {
                diplomatic_blunt: Some(9),
                ..Default::default()
            },
            behavioral_patterns: Vec::new(),
            styles: vec![mk_style(StyleType::DirectCommunicator, 7)],
            values: Vec::new(),
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            log: Vec::new(),
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        };
        assert!(all_person_flags(&person).contains(&"flag_style_direct_diplomatic"));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    Fr,
    En,
}

fn detect_from_strings(
    stored: Option<&str>,
    navigator: Option<&str>,
    env_lang: Option<&str>,
) -> Lang {
    if let Some(l) = stored {
        return if l == "en" { Lang::En } else { Lang::Fr };
    }
    if let Some(nav) = navigator
        && nav.starts_with("en")
    {
        return Lang::En;
    }
    if let Some(l) = env_lang
        && l.starts_with("en")
    {
        return Lang::En;
    }
    Lang::Fr
}

impl Lang {
    pub fn detect() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let stored = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item("pm_lang").ok())
                .flatten();
            let nav = web_sys::window().and_then(|w| w.navigator().language());
            detect_from_strings(stored.as_deref(), nav.as_deref(), None)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let env_lang = std::env::var("LANG").ok();
            detect_from_strings(None, None, env_lang.as_deref())
        }
    }

    pub fn persist(self) {
        let s = match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        };
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item("pm_lang", s);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::write(
                std::env::current_dir()
                    .unwrap_or_else(|_| ".".into())
                    .join(".pm_lang"),
                s,
            );
        }
    }
}

pub fn tr(key: &'static str, lang: Lang) -> &'static str {
    match lang {
        Lang::Fr => fr(key),
        Lang::En => en(key),
    }
}

pub fn tr_danger_details(details: &str, lang: Lang) -> String {
    if details.is_empty() {
        return String::new();
    }
    fn key(s: &str) -> &'static str {
        match s {
            "OCEAN volatility" => "OCEAN volatility",
            "Rep power struggle" => "Rep power struggle",
            "Only negative patterns" => "Only negative patterns",
            "Low prediction accuracy" => "Low prediction accuracy",
            _ => "Unknown",
        }
    }
    details
        .split(", ")
        .map(|d| tr(key(d), lang))
        .collect::<Vec<_>>()
        .join(", ")
}

fn en(key: &'static str) -> &'static str {
    match key {
        // Nav
        "nav_people" => "People",
        "nav_relationships" => "Relationships",
        "nav_timeline" => "Timeline",
        "nav_sync" => "Sync",

        // People list
        "search_placeholder" => "Search people...",
        "no_people_yet" => "No people yet. Tap + to add someone.",
        "pl_name" => "Name",
        "no_people_insights" => "No persons yet. Add someone to see insights.",
        "toast_saved" => "Saved",
        "toast_deleted" => "Deleted",
        "toast_error" => "Something went wrong",

        // Person detail
        "person_not_found" => "Person not found",
        "edit_btn" => "✏ Edit",
        "delete_btn" => "🗑 Delete",
        "motivations_title" => "Motivations",
        "no_motivations" => "No motivations recorded.",
        "biases_title" => "Biases",
        "no_biases" => "No biases recorded.",
        "reputation_title" => "Reputation",
        "no_reputation" => "No reputation traits recorded.",

        "patterns_title" => "Behavioral Patterns",
        "no_patterns" => "No behavioral patterns recorded.",
        "ocean_title" => "OCEAN Scores",
        "confidence_label" => "Profile confidence",
        "confidence_hint" => {
            "How reliable is this profile? 1 = rough sketch, 10 = built from real observations."
        }
        "reliability_title" => "Data quality",
        "score_band" => "±{}",
        "resilience_label" => "Resilience",
        "risk_appetite_label" => "Risk appetite",
        // Person edit form
        "form_new_title" => "New Person",
        "form_edit_title" => "Edit Person",
        "template_title" => "Quick Template",
        "template_blank" => "Blank (start from scratch)",
        "form_name" => "Name",
        "form_role" => "Role",
        "form_context" => "Context",
        "form_avatar" => "Avatar",
        "form_tags" => "Tags (comma separated)",
        "form_notes" => "Notes",
        "form_confidence" => "Profile confidence (1-10)",
        "form_resilience" => "Resilience (1-10)",
        "form_risk_appetite" => "Risk appetite (1-10)",
        "form_ocean_title" => "OCEAN Scores (1-10)",
        "form_save" => "💾 Save",
        "form_cancel" => "Cancel",

        // Ocean labels
        "ocean_openness" => "Openness",
        "ocean_conscientiousness" => "Conscientiousness",
        "ocean_extraversion" => "Extraversion",
        "ocean_agreeableness" => "Agreeableness",
        "ocean_neuroticism" => "Neuroticism",
        "ocean_o" => "O — Openness",
        "ocean_c" => "C — Conscientiousness",
        "ocean_e" => "E — Extraversion",
        "ocean_a" => "A — Agreeableness",
        "ocean_n" => "N — Neuroticism",
        "ocean_o_high" => "very open to new ideas, creative and curious",
        "ocean_o_low" => "pragmatic, prefers routines and concrete things",
        "ocean_c_high" => "organized, reliable, results and detail-oriented",
        "ocean_c_low" => "flexible and spontaneous, may lack rigor",
        "ocean_e_high" => "extraverted, energetic, seeks social stimulation",
        "ocean_e_low" => "introverted, thoughtful, prefers limited interactions",
        "ocean_a_high" => "cooperative, empathetic, seeks harmony",
        "ocean_a_low" => "direct or abrasive, puts goals before relationships",
        "ocean_n_high" => "emotionally reactive, prone to stress, sensitive to criticism",
        "ocean_n_low" => "emotionally stable, calm under pressure",

        // Consistency flags
        "flag_high_e_low_a" => {
            "Very outgoing but low agreeableness — may be assertive to the point of abrasiveness."
        }
        "flag_high_n_low_c" => {
            "High emotional reactivity with low conscientiousness — may struggle with structure under stress."
        }
        "flag_high_o_low_c" => {
            "Highly creative but unstructured — may have many ideas with difficulty following through."
        }
        "flag_calm_neurotic" => {
            "Reported as calm under pressure but OCEAN indicates high reactivity — review for consistency."
        }
        "flag_honest_selfish" => {
            "Principled honesty paired with low generosity — may indicate a rigid moral stance."
        }
        "flag_fairness_rhetoric" => {
            "Talks about fairness and justice but practices favoritism — do as I say, not as I do."
        }
        "flag_helping_selfish" => {
            "Preaches helpfulness but is perceived as selfish — do as I say, not as I do."
        }
        "flag_affiliation_cold" => {
            "Values closeness but is perceived as cold and detached — do as I say, not as I do."
        }
        "flag_ambition_lazy" => {
            "Aspires to power, success, or recognition but is perceived as lazy — do as I say, not as I do."
        }
        "flag_security_gullible" => {
            "Claims to value security yet is perceived as gullibly trusting — do as I say, not as I do."
        }
        "flag_discipline_lazy" => {
            "Self-image of discipline contradicted by a lazy reputation — they don't know themselves."
        }
        "flag_warmth_blunt" => {
            "Self-image of warmth contradicted by a blunt reputation — they don't know themselves."
        }
        "flag_open_rigid" => {
            "Thinks they're open-minded but comes across as rigid — they don't know themselves."
        }
        "flag_claims_calm_reactive" => {
            "Claims to be calm and stable but is perceived as reactive — they don't know themselves."
        }
        "flag_honest_favoritist" => {
            "Principled honesty paired with perceived favoritism — may enforce fairness only for some."
        }
        "flag_affiliation_distrustful" => {
            "Values closeness but is perceived as suspicious — do as I say, not as I do."
        }
        "flag_warmth_cold" => {
            "Thinks they're warm-hearted but comes across cold — they don't know themselves."
        }
        "flag_discipline_flaky" => {
            "Sees themselves as disciplined but comes across as flaky — they don't know themselves."
        }
        "flag_pattern_calm_volatile" => {
            "Perceived as calm under pressure, but recorded patterns show volatility — the calm may be an act."
        }
        "flag_pattern_honest_exploiter" => {
            "Perceived as honest, but recorded patterns show exploitation or blame-shifting — do as I say, not as I do."
        }
        "flag_bias_confirmation_open" => {
            "Claims open-mindedness yet shows confirmation bias — they don't know themselves."
        }
        "flag_bias_favoritism_fairness" => {
            "Preaches fairness yet shows favoritism or in-group bias — do as I say, not as I do."
        }
        "flag_security_risky" => {
            "Preaches caution and security yet self-reports a taste for risk — do as I say, not as I do."
        }
        "flag_resilient_reactive" => {
            "Claims high resilience but is perceived as reactive — they don't know themselves."
        }
        "flag_autonomy_submissive" => {
            "Preaches independence yet is perceived as submissive — do as I say, not as I do."
        }
        "flag_learning_rigid" => {
            "Preaches growth and learning yet is perceived as rigid — do as I say, not as I do."
        }
        "flag_creativity_closed" => {
            "Preaches creativity yet self-reports little openness to novelty."
        }
        "flag_creativity_rigid" => {
            "Preaches creativity yet is perceived as rigid — do as I say, not as I do."
        }
        "flag_authority_dominant" => "Perceived as a leader yet blindly defers to authority.",
        "flag_social_proof_open" => {
            "Claims independent thinking yet follows the herd — do as I say, not as I do."
        }
        "flag_sunk_cost_flexible" => "Perceived as flexible yet clings to sunk costs.",
        "flag_pattern_diplomat_escalator" => "Perceived as diplomatic yet escalates conflict.",
        "flag_pattern_fair_exploiter" => {
            "Perceived as fair yet exploits injustice for personal gain."
        }
        "flag_pattern_humble_dismissive" => "Perceived as humble yet puts others down.",
        "flag_pattern_trusting_paranoid" => {
            "Perceived as trusting yet turns paranoid under threat."
        }
        "flag_pattern_reliable_shirker" => "Perceived as reliable yet dodges accountability.",
        "flag_pattern_hardworker_complacent" => {
            "Perceived as hardworking yet rests on past laurels."
        }
        "flag_risk_appetite_ambition" => "Aspires to power or achievement yet avoids all risk.",
        "flag_power_passive" => "Aspires to power yet is perceived as a pushover.",
        "flag_helping_cold" => "Preaches helpfulness yet reads as emotionally cold.",
        "flag_pattern_passive_blowup" => "Perceived as passive yet blows up under pressure.",
        "flag_pattern_assertive_quiet" => "Perceived as assertive yet goes quiet when it counts.",
        "flag_loss_aversion_risky" => "Claims a taste for risk yet is loss-averse.",
        "flag_dunning_kruger_humble" => "Overestimates their competence yet is seen as humble.",
        "flag_impostor_arrogant" => "Underestimates their competence yet is seen as arrogant.",
        "flag_recency_reliable" => "Perceived as steady yet swings with the latest news.",
        "flag_resilient_hides" => "Admits fragility yet appears unflappable — they hide it.",
        "flag_pattern_generous_exploiter" => "Perceived as generous yet exploits others.",
        "flag_pattern_empath_dismissive" => "Perceived as empathetic yet puts others down.",
        "flag_pattern_flexible_resister" => {
            "Perceived as flexible yet resists change and feedback."
        }
        "flag_anchoring_open" => "Claims open-mindedness yet clings to first impressions.",
        "flag_learning_arrogant" => "Preaches growth yet is too arrogant to take advice.",
        "flag_warmth_selfish" => "Claims warmth yet is perceived as selfish.",
        "flag_style_direct_diplomatic" => "Claims to be direct yet comes across as diplomatic.",
        "flag_style_diplomatic_blunt" => "Claims a diplomatic style yet comes across as blunt.",
        "flag_style_competing_passive" => "Claims a competitive style yet comes across as passive.",
        "flag_style_dominant_submissive" => {
            "Claims an autocratic style yet comes across as submissive."
        }
        "flag_style_manipulative_honest" => "Claims to play dirty yet comes across as honest.",
        "flag_style_empathetic_cold" => "Claims empathy yet comes across as cold.",
        "flag_style_guarded_trusting" => "Claims to be guarded yet comes across as trusting.",
        "flag_pattern_helping_exploiter" => {
            "Preaches helpfulness yet recorded patterns show exploitation."
        }
        "flag_pattern_warmth_dismissive" => {
            "Self-image of warmth yet recorded patterns put others down."
        }
        "flag_pattern_discipline_shirker" => {
            "Self-image of discipline yet recorded patterns dodge accountability."
        }
        "flag_pattern_claimed_calm_volatile" => {
            "Self-reports calm yet recorded patterns show volatility."
        }
        "flag_style_servant_authoritative" => {
            "Claims servant leadership yet comes across as a commander."
        }
        "flag_style_consensus_authoritative" => {
            "Claims consensus-driven yet comes across as a dictator."
        }
        "flag_style_trusts_freely_suspicious" => {
            "Claims to trust freely yet comes across as suspicious."
        }
        "flag_style_repairs_trust_deceitful" => {
            "Claims to repair trust yet comes across as deceitful."
        }
        "flag_style_rulebased_favoritist" => "Claims a rules-based approach yet plays favorites.",
        "flag_pattern_fairness_exploiter" => {
            "Preaches fairness yet recorded patterns exploit injustice."
        }
        "flag_pattern_achievement_complacent" => {
            "Aspires to achievement yet recorded patterns rest on laurels."
        }
        "flag_pattern_learning_resister" => {
            "Preaches learning yet recorded patterns reject feedback."
        }
        "flag_pattern_extravert_quiet" => {
            "Self-image of extraversion yet recorded patterns go quiet."
        }
        "flag_style_virtuebased_deceitful" => {
            "Claims a virtue-based approach yet comes across as deceitful."
        }
        "flag_availability_calm" => "Perceived as unflappable yet overweights dramatic events.",
        "flag_pattern_open_resister" => "Claims openness yet recorded patterns resist change.",
        "flag_pattern_recognition_dismissive" => {
            "Seeks recognition yet puts others down to win it."
        }
        "flag_value_family_past" => {
            "Values family highly yet shows no past-oriented time orientation."
        }
        "flag_value_stability_risk" => {
            "Craves stability yet has a very high risk appetite — contradictory."
        }
        "flag_value_career_family" => {
            "Both career and family rated as top priorities — expect tension."
        }
        "flag_value_loyalty_guarded" => {
            "Values loyalty yet adopts a guarded, distrustful trust style."
        }

        // Edit form sections
        "edit_motivations" => "Motivations",
        "edit_biases" => "Biases",
        "bias_undefined_warning" => "Undefined biases count as present. Set 0 to mark as absent.",
        "rep_undefined_warning" => {
            "Undefined traits penalize reputation. Extreme values (≤2 or ≥9) trigger adjustments."
        }
        "mot_undefined_warning" => {
            "Fewer than 3 motivations penalizes (−0.03 each). Missing Fairness/Helping also hurts."
        }
        "profile_completeness" => "Compl.",
        "edit_reputation" => "Reputation",
        "edit_patterns" => "Behavioral Patterns",
        "edit_styles" => "Personal Styles",
        "edit_notes_placeholder" => "Notes",
        "edit_evidence_placeholder" => "Evidence",
        "add_btn" => "＋",
        "edit_update_btn" => "💾",

        "mot_helper_achievement" => "Driven to excel and meet goals",
        "mot_helper_power" => "Seeks influence, control, and status",
        "mot_helper_affiliation" => "Values belonging, connection, and harmony",
        "mot_helper_security" => "Prioritizes stability, safety, and predictability",
        "mot_helper_autonomy" => "Cherishes independence, freedom, and self-direction",
        "mot_helper_recognition" => "Desires acknowledgment, praise, and visibility",
        "mot_helper_learning" => "Thirst for knowledge, growth, and mastery",
        "mot_helper_helping" => "Fulfilled by supporting, mentoring, and serving others",
        "mot_helper_creativity" => "Driven to create, innovate, and express ideas",
        "mot_helper_fairness" => "Motivated by justice, equity, and fair treatment",

        "bias_helper_confirmation" => "Favors info that confirms existing beliefs",
        "bias_helper_anchoring" => "Over-relies on the first piece of info received",
        "bias_helper_availability" => "Overestimates likelihood of easily recalled events",
        "bias_helper_sunk_cost" => "Continues investing due to past sunken resources",
        "bias_helper_dunning_kruger" => "Overestimates own competence in a domain",
        "bias_helper_impostor" => "Underestimates own competence in a domain",
        "bias_helper_loss_aversion" => "Fears losses more than values equivalent gains",
        "bias_helper_social_proof" => "Follows others' behavior in uncertainty",
        "bias_helper_authority" => "Defers excessively to authority figures",
        "bias_helper_recency" => "Overweighs recent events over older ones",
        "bias_helper_in_group" => "Favors own group members over outsiders",
        "bias_helper_favoritism" => "Shows preferential treatment to certain individuals",

        "pattern_helper_stress" => "How they react under pressure or tight deadlines",
        "pattern_helper_conflict" => "How they handle disagreements and confrontation",
        "pattern_helper_success" => "How they respond to achievements and wins",
        "pattern_helper_uncertainty" => "How they navigate ambiguity and unknown outcomes",
        "pattern_helper_recognition" => "How they seek and respond to acknowledgment",
        "pattern_helper_threat" => "How they defend themselves when feeling attacked",
        "pattern_helper_change" => "How they adapt to transitions and new situations",
        "pattern_helper_feedback" => "How they receive and process input from others",
        "pattern_helper_injustice" => {
            "How they react when treated unfairly or witnessing unfairness"
        }

        // Context labels
        "ctx_stress" => "Stress",
        "ctx_decision" => "Decision",
        "ctx_team" => "Team",
        "ctx_communication" => "Communication",
        "ctx_leadership" => "Leadership",
        "ctx_growth" => "Growth",
        "ctx_conflict" => "Conflict",
        "ctx_success" => "Success",
        "ctx_uncertainty" => "Uncertainty",
        "ctx_recognition" => "Recognition",
        "ctx_threatened" => "Threatened",
        "ctx_change" => "Change",
        "ctx_feedback" => "Feedback",
        "ctx_injustice" => "Injustice",

        // Predictions
        "pred_all_title" => "All Predictions",
        "pred_for" => "🔮 Predictions for",
        "pred_title" => "Predictions",
        "pred_context_placeholder" => "Context...",
        "pred_outcome_placeholder" => "Predicted outcome...",
        "pred_add_btn" => "Add",
        "pred_none" => "No predictions yet.",
        "pred_predicted_label" => "Predicted",
        "pred_actual_label" => "Actual",
        "pred_resolve_btn" => "Resolve",
        "pred_delete_btn" => "Delete",
        "pred_actual_placeholder" => "Actual outcome...",
        "pred_accuracy_label" => "Accuracy",
        "pred_resolve_submit" => "✓ Resolve",
        "pred_cancel_btn" => "Cancel",

        // Insights
        "insights_title" => "📊 Insights",
        "insights_select_person" => "Select a person to view behavioral insights.",
        "insights_observed" => "Observed Patterns",
        "log_title" => "📋 Log",
        "log_placeholder" => "What happened?",
        "log_add" => "Add entry",
        "log_empty" => "No entries yet.",
        "log_valence" => "Valence",
        "log_trigger" => "Trigger",
        "log_target" => "With",
        "log_no_trigger" => "No trigger",
        "log_no_target" => "Self note (no target)",
        "trend_improving" => "Improving",
        "trend_stable" => "Stable",
        "trend_deteriorating" => "Deteriorating",
        "trend_hint" => "From recent logged interactions",

        // Insight strategies
        "strategy_stress_label" => "Under stress",
        "strategy_conflict_label" => "In conflict",
        "strategy_success_label" => "In success",
        "strategy_uncertainty_label" => "In uncertainty",
        "strategy_recognition_label" => "Seeking recognition",
        "strategy_threat_label" => "Feeling threatened",
        "strategy_change_label" => "Facing change",
        "strategy_feedback_label" => "Receiving feedback",
        "strategy_when" => "When {name} is {trigger}:\n\n{advice}",
        "more_recs" => "More recommendations",

        "strategy_stress_high_n" => "High neuroticism — provide reassurance and clear structure.",
        "strategy_stress_high_e" => "High extraversion — allow verbal processing of stress.",
        "strategy_stress_low_e" => "Low extraversion — give quiet space to decompress.",
        "strategy_stress_high_c" => {
            "High conscientiousness — break problems into actionable steps."
        }
        "strategy_stress_low_a" => {
            "Low agreeableness — may become short or irritable under pressure."
        }
        "strategy_stress_low_c" => "Low conscientiousness — may become disorganized or avoidant.",
        "strategy_stress_high_o" => {
            "High openness — may overthink and spiral into worst-case scenarios."
        }
        "strategy_stress_power" => "Power-driven — let them regain control in one domain.",
        "strategy_stress_security" => "Security-driven — reinforce stability and routine.",
        "strategy_stress_ambition_rhetoric" => {
            "They talk ambition but are perceived as lazy — don't reward the rhetoric; focus on effort and follow-through."
        }
        "strategy_stress_security_rhetoric" => {
            "They claim to value security yet are gullibly trusting — don't rely on their stated caution; verify safeguards yourself."
        }
        "strategy_stress_fallback" => "Monitor stress signals and adjust environment.",

        "strategy_conflict_low_a" => "Low agreeableness — address conflict directly with facts.",
        "strategy_conflict_high_a" => {
            "High agreeableness — soften confrontation, focus on harmony."
        }
        "strategy_conflict_high_n" => {
            "High neuroticism — de-escalate and provide emotional safety."
        }
        "strategy_conflict_high_e" => "High extraversion — let them talk it through.",
        "strategy_conflict_high_c" => {
            "High conscientiousness — may rigidly insist on rules and procedures."
        }
        "strategy_conflict_low_e" => {
            "Low extraversion — may withdraw or stonewall instead of engaging."
        }
        "strategy_conflict_fallback" => "Mediate with balanced communication.",
        "strategy_conflict_affiliation_rhetoric" => {
            "They value closeness yet come across cold — don't appeal to their stated need for connection; address the detachment directly."
        }
        "strategy_conflict_affiliation_trust_rhetoric" => {
            "They value closeness yet come across distrustful — don't appeal to their stated need for connection; earn credibility before seeking rapport."
        }

        "strategy_success_high_o" => {
            "High openness — channel success into new creative challenges."
        }
        "strategy_success_high_c" => {
            "High conscientiousness — leverage success as validation of process."
        }
        "strategy_success_low_e" => "Low extraversion — may feel overwhelmed by public attention.",
        "strategy_success_high_a" => {
            "High agreeableness — may deflect credit to avoid standing out."
        }
        "strategy_success_recognition" => {
            "Recognition-driven — publicly acknowledge their achievement."
        }
        "strategy_success_power" => "Power-driven — give them ownership of the next initiative.",
        "strategy_success_ambition_rhetoric" => {
            "They talk ambition but are perceived as lazy — don't celebrate their plans; require delivery."
        }
        "strategy_success_fallback" => "Celebrate success and identify growth areas.",

        "strategy_uncertainty_high_n" => {
            "High neuroticism — provide clear timelines and frequent updates."
        }
        "strategy_uncertainty_low_n" => {
            "Low neuroticism — they handle ambiguity well; trust their resilience."
        }
        "strategy_uncertainty_high_o" => "High openness — frame uncertainty as opportunity.",
        "strategy_uncertainty_low_o" => {
            "Low openness — provide concrete examples and familiar frameworks."
        }
        "strategy_uncertainty_high_c" => {
            "High conscientiousness — needs a concrete plan immediately."
        }
        "strategy_uncertainty_high_e" => {
            "High extraversion — may over-socialize to cope with ambiguity."
        }
        "strategy_uncertainty_fallback" => {
            "Acknowledge uncertainty and provide available information."
        }

        "strategy_recognition_high" => "Strong recognition drive — give frequent, specific praise.",
        "strategy_recognition_mid" => {
            "Moderate recognition drive — acknowledge contributions regularly."
        }
        "strategy_recognition_low" => "Low recognition need — avoid over-praising.",
        "strategy_recognition_high_e" => "High extraversion — public recognition is effective.",
        "strategy_recognition_low_e" => {
            "Low extraversion — prefer private, written acknowledgment."
        }
        "strategy_recognition_fallback" => "Match recognition style to their comfort level.",

        "strategy_threat_low_a" => {
            "Low agreeableness — they may push back; address concerns calmly."
        }
        "strategy_threat_high_a" => {
            "High agreeableness — they may concede too easily; check true feelings."
        }
        "strategy_threat_high_n" => {
            "High neuroticism — perceived threats are amplified; offer reassurance."
        }
        "strategy_threat_power" => {
            "Power-driven — threat to status is serious; involve them in decisions."
        }
        "strategy_threat_fallback" => "Listen actively and validate their concerns.",

        "strategy_change_high_n" => {
            "High neuroticism — may resist change; provide stability anchors."
        }
        "strategy_change_low_n" => "Low neuroticism — adapts well; leverage as change champion.",
        "strategy_change_high_c" => "High conscientiousness — needs a clear transition roadmap.",
        "strategy_change_low_e" => "Low extraversion — needs time to process change privately.",
        "strategy_change_high_o" => {
            "High openness — embrace change; give them a role in shaping it."
        }
        "strategy_change_fallback" => "Communicate the why and involve them in the transition.",
        "strategy_change_discipline_rhetoric" => {
            "They see themselves as disciplined yet are perceived as lazy — don't appeal to their organized self-image; check actual output."
        }

        "strategy_feedback_high_n" => {
            "High neuroticism — may take feedback personally; use gentle framing."
        }
        "strategy_feedback_low_n" => "Low neuroticism — handles critical feedback well; be direct.",
        "strategy_feedback_low_a" => "Low agreeableness — may reject feedback; focus on data.",
        "strategy_feedback_low_e" => "Low extraversion — prefers private, written feedback.",
        "strategy_feedback_high_c" => {
            "High conscientiousness — values detailed, actionable feedback."
        }
        "strategy_feedback_fallback" => {
            "Balance praise and constructive input with specific examples."
        }
        "strategy_feedback_helping_rhetoric" => {
            "They preach helpfulness yet are perceived as selfish — don't frame feedback around helping others; name the self-interest behind the advice."
        }
        "strategy_feedback_warmth_rhetoric" => {
            "They see themselves as warm yet are perceived as blunt — don't rely on soft delivery; be clear and specific about the behavior."
        }

        "strategy_injustice_label" => "Facing injustice",
        "strategy_injustice_high_a" => {
            "High agreeableness — may feel personally wounded by unfairness."
        }
        "strategy_injustice_high_n" => {
            "High neuroticism — may ruminate and escalate perceived slights."
        }
        "strategy_injustice_fairness" => {
            "Fairness-driven — will fight for what they believe is right, even at personal cost."
        }
        "strategy_injustice_fairness_rhetoric" => {
            "Speaks of fairness but acts with favoritism — don't appeal to their justice rhetoric; address the real driver instead."
        }
        "strategy_injustice_power" => {
            "Power-driven — may leverage authority to correct the perceived wrong."
        }
        "strategy_injustice_ambition_rhetoric" => {
            "They talk ambition but are perceived as lazy — don't expect them to fight for the cause; frame the outcome as serving their status instead."
        }
        "strategy_injustice_fallback" => {
            "Acknowledge their concern and clarify the path to resolution."
        }

        // Sync / Drive
        "sync_title" => "☁ Sync & Backup",
        "sync_gdrive_title" => "Google Drive Sync",
        "sync_token_loaded" => "✓ Token loaded",
        "sync_token_cleared" => "Token cleared",
        "sync_clear_btn" => "Clear",
        "sync_sign_in" => "🔐 Sign in with Google",
        "sync_no_token" => "No token. Sign in first.",
        "sync_backing_up" => "Backing up...",
        "sync_backed_up" => "✅ Backed up",
        "sync_backup_btn" => "☁ Backup to Drive",
        "sync_restoring" => "Restoring...",
        "sync_restored" => "✅ Restored",
        "sync_restore_btn" => "☁ Restore from Drive",
        "sync_not_configured" => {
            "Google Drive backup not configured at build time. Set GOOGLE_CLIENT_ID env var before building."
        }
        "sync_local_title" => "Local Backup",
        "sync_local_desc" => "Export all data as JSON or import from a previous backup.",
        "sync_exported" => "✅ Exported",
        "sync_export_btn" => "📥 Export JSON",
        "sync_import_btn" => "📤 Import JSON",
        "sync_passphrase_label" => "Encrypt backup with passphrase (optional)",
        "sync_passphrase_placeholder" => "Enter passphrase...",
        "sync_passphrase_show" => "Show",
        "sync_passphrase_hide" => "Hide",
        "sync_wrong_passphrase" => "❌ Wrong passphrase or corrupted data",
        "sync_token_instruction_1" => "1. Tap 'Sign in with Google' — opens your browser",
        "sync_token_instruction_2" => "2. Sign in and grant access",
        "sync_token_instruction_3" => {
            "3. Browser redirects to the web app — copy the token from the address bar before the page loads"
        }
        "sync_token_instruction_4" => "4. Paste the URL below and tap Save",
        "sync_paste_placeholder" => "Paste the full redirect URL here",
        "sync_token_saved" => "✅ Token saved",
        "sync_save_token_btn" => "Save Token",
        "sync_no_data_warn" => "No people data to back up. Add people first!",
        "sync_view_backup" => "🔎 View backups in your browser (appDataFolder Browser)",

        // Common
        "common_save" => "Save",
        "common_cancel" => "Cancel",
        "common_delete" => "Delete",
        "common_add" => "Add",
        "common_edit" => "Edit",
        "common_back" => "← Back",
        "compare_title" => "Compare Persons",
        "compare_btn" => "Compare",
        "compare_sub" => "Identify synergies and friction points between two people",
        "compare_vs" => "VS",
        "compare_top_mot" => "Top Motivation",
        "compare_bias_main" => "Main Bias",
        "compare_ocean" => "OCEAN Profile",
        "compare_analysis_title" => "Dynamic Analysis",
        "compare_synergies" => "Synergies",
        "compare_friction" => "Friction Points",
        "compare_strategy" => "Interaction Strategy",
        "compare_breakdown" => "Breakdown",
        "compare_ctx_title" => "By situation",
        "compare_cat_ocean" => "OCEAN",
        "compare_cat_reputation" => "Reputation",
        "compare_cat_motivation" => "Motivation",
        "compare_cat_patterns" => "Patterns",
        "compare_cat_bias" => "Bias",
        "compare_cat_styles" => "Styles",
        "compare_cat_values" => "Values",
        "compare_risk_mitigation" => "Risks & Mitigations",
        "values_title" => "Values",
        "no_values" => "No values defined",
        "edit_values" => "Values",
        "edit_priority" => "P",
        "compare_rel_title" => "Relationship Context",
        "compare_rel_none" => "General (no context)",
        "compare_rel_strength" => "Strength",
        "compare_band_hint" => "±{}% (relationship + profile confidence)",
        "person_self_score" => "Profile Score",
        "Rep power struggle" => "Power struggle (Reputation)",
        "compare_asymmetric" => "Mutual benefit",
        "compare_benefit_more" => "benefits more",
        "compare_balanced" => "Balanced",
        "compare_ethics" => {
            "These are probabilistic models, not absolute truths. Use them to understand better, never to manipulate."
        }

        // Scale bands
        "scale_strong" => "Strong",
        "scale_good" => "Good",
        "scale_moderate" => "Moderate",
        "scale_friction" => "Friction",
        "scale_tension" => "Tension",

        // Relationships
        "rel_title" => "Relationships",
        "rel_notes" => "Notes",
        "rel_strength" => "Strength",
        "rel_none" => "No relationships yet.",
        "rel_open_add" => "＋ Add",
        "rel_close_add" => "− Cancel",
        "rel_search_placeholder" => "Search person…",
        "rel_confirm_delete" => "Delete this relationship?",
        "confirm_delete" => "Delete this person?",
        "confirm_delete_log" => "Delete this entry?",
        "confirm_delete_pred" => "Delete this prediction?",
        "no_search_results" => "No results for \"{0}\".",
        "rel_person_rel" => "Relationships",

        // Timeline
        "tl_title" => "Timeline",
        "tl_empty" => "No interaction entries yet.",

        // Style helpers
        "style_no_styles" => "No personal styles recorded.",
        "style_panel_title" => "Personal Styles",

        // Tags

        // Tutorial
        "tut_step" => "Step",
        "tut_welcome_title" => "Welcome to PeopleModeler!",
        "tut_welcome_body" => {
            "This app helps you model and understand the people in your life using personality frameworks like OCEAN (Big Five), motivations, cognitive biases, and behavioral patterns.\n\nYou can compare people side by side, track predictions over time, map relationships, and explore synergy scores."
        }
        "tut_people_title" => "Your People",
        "tut_people_body" => {
            "The main page shows everyone you've created. Use the search bar to find someone, sort by name / recent / OCEAN score, and click the + button to add someone new."
        }
        "tut_create_title" => "Creating a Person",
        "tut_create_body" => {
            "The person form is divided into sections: basic info (name, role, context), OCEAN personality scores, motivations, cognitive biases, reputation dimensions, and behavioral patterns.\n\nEach section captures a different facet of someone's personality — fill in what you know, leave the rest blank."
        }
        "tut_ocean_title" => "OCEAN Model (Big Five)",
        "tut_ocean_body" => {
            "OCEAN measures personality across five dimensions from 1 to 10:\n• Openness — curiosity vs. caution\n• Conscientiousness — organization vs. flexibility\n• Extraversion — sociability vs. solitude\n• Agreeableness — cooperation vs. competition\n• Neuroticism — sensitivity vs. emotional stability\n\nThese scores power the comparison engine and help predict behaviour."
        }
        "tut_mot_bias_title" => "Motivations & Biases",
        "tut_mot_bias_body" => {
            "Motivations capture what drives a person — their goals, fears, and values (Achievement, Power, Affiliation, Security, Autonomy, etc.).\n\nBiases represent mental shortcuts that shape their decisions (Confirmation bias, Anchoring, Overconfidence, etc.). Together they give you a deeper understanding of why people act the way they do."
        }
        "tut_rep_pattern_title" => "Reputation & Patterns",
        "tut_rep_pattern_body" => {
            "Reputation scores capture how others perceive this person across bipolar scales (hardworking vs. lazy, honest vs. deceitful, etc.).\n\nBehavioral patterns let you record how they typically react to specific triggers (stress, criticism, success, conflict, etc.). This helps anticipate their responses in future situations."
        }
        "tut_compare_title" => "Comparisons & More",
        "tut_compare_body" => {
            "Once you have at least two people, you can compare them side by side to see their synergy score, friction points, and interaction strategies.\n\nYou can also track predictions (guess an outcome, then check if you were right), build a relationship map, and log interactions on a timeline."
        }
        "tut_done_title" => "You're Ready!",
        "tut_done_body" => {
            "You can replay this tutorial anytime from the navigation bar.\n\nQuick tips:\n• Create at least two people to unlock comparisons\n• Use the Sync page to back up your data\n• Tag people to organise them by group\n\nGo ahead and start modeling the people in your world!"
        }

        // Team page
        "nav_teams" => "Teams",
        "teams_title" => "Teams",
        "teams_all" => "All People",
        "teams_create" => "New Team",
        "teams_delete" => "Delete team?",
        "teams_members" => "{0} members",
        "team_title" => "Team Synergy",
        "team_empty" => "Add at least 2 people to see team synergy.",
        "team_size" => "Team size",
        "team_avg_score" => "Avg score",
        "team_strongest" => "Strongest link",
        "team_weakest" => "Weakest link",
        "team_max_danger" => "Max danger",
        "team_avg_danger" => "Avg danger",
        "team_ctx_avg" => "Average by situation",
        "team_pairs" => "All pairs",
        "team_no_danger" => "None",
        "team_tab_synergy" => "Synergy",
        "team_tab_members" => "Members",
        "team_all_no_edit" => "All People includes everyone automatically",
        "team_members_count" => "{0} members",
        "confirm_delete_team" => "Delete this team?",
        "team_rename" => "Rename",
        "team_icon" => "Icon",
        "team_edit" => "Edit",

        // Common (tutorial)
        "common_next" => "Next →",
        "common_skip" => "Skip",
        "common_finish" => "Finish",

        _ => key,
    }
}

fn fr(key: &'static str) -> &'static str {
    match key {
        "nav_people" => "Personnes",
        "nav_relationships" => "Relations",
        "nav_timeline" => "Chrono",
        "nav_sync" => "Sync",

        "search_placeholder" => "Rechercher...",
        "no_people_yet" => "Aucune personne. Appuyez sur + pour ajouter.",
        "pl_name" => "Nom",
        "no_people_insights" => "Aucune personne encore. Ajoutez quelqu'un pour voir les analyses.",
        "toast_saved" => "Enregistré",
        "toast_deleted" => "Supprimé",
        "toast_error" => "Une erreur est survenue",
        "person_not_found" => "Personne introuvable",
        "edit_btn" => "✏ Modifier",
        "delete_btn" => "🗑 Supprimer",
        "motivations_title" => "Motivations",
        "no_motivations" => "Aucune motivation enregistrée.",
        "biases_title" => "Biais",
        "no_biases" => "Aucun biais enregistré.",
        "reputation_title" => "Réputation",
        "no_reputation" => "Aucun trait de réputation enregistré.",

        "patterns_title" => "Patterns comportementaux",
        "no_patterns" => "Aucun pattern comportemental enregistré.",
        "ocean_title" => "Scores OCEAN",
        "confidence_label" => "Fiabilité du profil",
        "confidence_hint" => {
            "À quel point ce profil est fiable ? 1 = ébauche, 10 = fondé sur des observations réelles."
        }
        "reliability_title" => "Qualité des données",
        "score_band" => "±{}",
        "resilience_label" => "Résilience",
        "risk_appetite_label" => "Appétence risque",
        "form_new_title" => "Nouvelle personne",
        "form_edit_title" => "Modifier la personne",
        "template_title" => "Modèle rapide",
        "template_blank" => "Vierge (commencer de zéro)",
        "form_name" => "Nom",
        "form_role" => "Rôle",
        "form_context" => "Contexte",
        "form_avatar" => "Avatar",
        "form_tags" => "Tags (séparés par des virgules)",
        "form_notes" => "Notes",
        "form_confidence" => "Fiabilité du profil (1-10)",
        "form_resilience" => "Résilience (1-10)",
        "form_risk_appetite" => "Appétence pour le risque (1-10)",
        "form_ocean_title" => "Scores OCEAN (1-10)",
        "form_save" => "💾 Enregistrer",
        "form_cancel" => "Annuler",

        "ocean_openness" => "Ouverture",
        "ocean_conscientiousness" => "Conscienciosité",
        "ocean_extraversion" => "Extraversion",
        "ocean_agreeableness" => "Agréabilité",
        "ocean_neuroticism" => "Névrosisme",
        "ocean_o" => "O — Ouverture",
        "ocean_c" => "C — Conscienciosité",
        "ocean_e" => "E — Extraversion",
        "ocean_a" => "A — Agréabilité",
        "ocean_n" => "N — Névrosisme",
        "ocean_o_high" => "très ouvert aux nouvelles idées, créatif et curieux",
        "ocean_o_low" => "pragmatique, préfère les routines et le concret",
        "ocean_c_high" => "organisé, fiable, orienté résultats et détails",
        "ocean_c_low" => "flexible et spontané, peut manquer de rigueur",
        "ocean_e_high" => "extraverti, énergique, cherche la stimulation sociale",
        "ocean_e_low" => "introverti, réfléchi, préfère les interactions limitées",
        "ocean_a_high" => "coopératif, empathique, cherche l'harmonie",
        "ocean_a_low" => "direct voire abrasif, met ses objectifs avant les relations",
        "ocean_n_high" => "émotionnellement réactif, stressable, sensible aux critiques",
        "ocean_n_low" => "stable émotionnellement, calme sous pression",

        // Consistency flags
        "flag_high_e_low_a" => {
            "Très extraverti mais faible agréabilité — peut être assertif jusqu'à l'abrasivité."
        }
        "flag_high_n_low_c" => {
            "Réactivité émotionnelle élevée avec faible conscience — peut avoir du mal sous stress."
        }
        "flag_high_o_low_c" => {
            "Très créatif mais désorganisé — beaucoup d'idées mais difficulté à les concrétiser."
        }
        "flag_calm_neurotic" => {
            "Décrit comme calme sous pression mais l'OCEAN indique une forte réactivité — à vérifier."
        }
        "flag_honest_selfish" => {
            "Honnêteté de principe associée à une faible générosité — peut indiquer une position morale rigide."
        }
        "flag_fairness_rhetoric" => {
            "Parle d'équité et de justice mais pratique le favoritisme — fait ce que je dis, pas ce que je fais."
        }
        "flag_helping_selfish" => {
            "Prêche l'entraide mais est perçu comme égoïste — fait ce que je dis, pas ce que je fais."
        }
        "flag_affiliation_cold" => {
            "Revendique la proximité mais est perçu comme froid et distant — fait ce que je dis, pas ce que je fais."
        }
        "flag_ambition_lazy" => {
            "Aspire au pouvoir, au succès ou à la reconnaissance mais est perçu comme paresseux — fait ce que je dis, pas ce que je fais."
        }
        "flag_security_gullible" => {
            "Revendique un besoin de sécurité mais est perçu comme naïvement confiant — fait ce que je dis, pas ce que je fais."
        }
        "flag_discipline_lazy" => {
            "Image de soi disciplinée contredite par une réputation de paresse — ne se connaît pas."
        }
        "flag_warmth_blunt" => {
            "Image de soi chaleureuse contredite par une réputation de franchise brutale — ne se connaît pas."
        }
        "flag_open_rigid" => "Se croit ouvert d'esprit mais paraît rigide — ne se connaît pas.",
        "flag_claims_calm_reactive" => {
            "Se prétend calme et stable mais est perçu comme réactif — ne se connaît pas."
        }
        "flag_honest_favoritist" => {
            "Honnêteté de principe associée à un favoritisme perçu — l'équité ne vaut peut-être que pour certains."
        }
        "flag_affiliation_distrustful" => {
            "Revendique la proximité mais est perçu comme méfiant — fait ce que je dis, pas ce que je fais."
        }
        "flag_warmth_cold" => "Se croit chaleureux mais paraît froid — ne se connaît pas.",
        "flag_discipline_flaky" => "Se voit discipliné mais paraît inconstant — ne se connaît pas.",
        "flag_pattern_calm_volatile" => {
            "Perçu comme calme sous pression, mais les schémas enregistrés montrent de la volatilité — ce calme n'est peut-être qu'un masque."
        }
        "flag_pattern_honest_exploiter" => {
            "Perçu comme honnête, mais les schémas montrent de l'exploitation ou des rejets de responsabilité — fait ce que je dis, pas ce que je fais."
        }
        "flag_bias_confirmation_open" => {
            "Se dit ouvert d'esprit mais présente un biais de confirmation — ne se connaît pas."
        }
        "flag_bias_favoritism_fairness" => {
            "Prêche l'équité mais montre un biais de favoritisme ou de groupe — fait ce que je dis, pas ce que je fais."
        }
        "flag_security_risky" => {
            "Prêche la prudence et la sécurité mais déclare aimer le risque — fait ce que je dis, pas ce que je fais."
        }
        "flag_resilient_reactive" => {
            "Se dit très résilient mais est perçu comme réactif — ne se connaît pas."
        }
        "flag_autonomy_submissive" => {
            "Prêche l'indépendance mais est perçu comme soumis — fait ce que je dis, pas ce que je fais."
        }
        "flag_learning_rigid" => {
            "Prêche l'apprentissage et la croissance mais est perçu comme rigide — fait ce que je dis, pas ce que je fais."
        }
        "flag_creativity_closed" => "Prêche la créativité mais se dit peu ouvert à la nouveauté.",
        "flag_creativity_rigid" => {
            "Prêche la créativité mais est perçu comme rigide — fait ce que je dis, pas ce que je fais."
        }
        "flag_authority_dominant" => {
            "Perçu comme un leader mais se soumet aveuglément à l'autorité."
        }
        "flag_social_proof_open" => {
            "Se dit indépendant d'esprit mais suit le troupeau — fait ce que je dis, pas ce que je fais."
        }
        "flag_sunk_cost_flexible" => {
            "Perçu comme flexible mais s'accroche aux coûts irrécupérables."
        }
        "flag_pattern_diplomat_escalator" => "Perçu comme diplomate mais escalade les conflits.",
        "flag_pattern_fair_exploiter" => {
            "Perçu comme équitable mais exploite l'injustice à son profit."
        }
        "flag_pattern_humble_dismissive" => "Perçu comme humble mais rabaisse les autres.",
        "flag_pattern_trusting_paranoid" => {
            "Perçu comme confiant mais devient paranoïaque sous la menace."
        }
        "flag_pattern_reliable_shirker" => "Perçu comme fiable mais esquive ses responsabilités.",
        "flag_pattern_hardworker_complacent" => {
            "Perçu comme travailleur mais se repose sur ses lauriers."
        }
        "flag_risk_appetite_ambition" => {
            "Aspire au pouvoir ou à la réussite mais évite tout risque."
        }
        "flag_power_passive" => "Aspire au pouvoir mais est perçu comme une carpette.",
        "flag_helping_cold" => "Prêche l'aide aux autres mais paraît émotionnellement froid.",
        "flag_pattern_passive_blowup" => "Perçu comme passif mais explose sous la pression.",
        "flag_pattern_assertive_quiet" => "Perçu comme affirmé mais se tait quand il le faut.",
        "flag_loss_aversion_risky" => "Se dit amateur de risque mais est averse à la perte.",
        "flag_dunning_kruger_humble" => "Surestime ses compétences mais paraît humble.",
        "flag_impostor_arrogant" => "Sous-estime ses compétences mais paraît arrogant.",
        "flag_recency_reliable" => "Perçu comme stable mais ballotté par l'actualité.",
        "flag_resilient_hides" => "Admet sa fragilité mais paraît imperturbable — il la cache.",
        "flag_pattern_generous_exploiter" => "Perçu comme généreux mais exploite les autres.",
        "flag_pattern_empath_dismissive" => "Perçu comme empathique mais rabaisse les autres.",
        "flag_pattern_flexible_resister" => {
            "Perçu comme flexible mais résiste au changement et au feedback."
        }
        "flag_anchoring_open" => {
            "Se dit ouvert d'esprit mais s'accroche aux premières impressions."
        }
        "flag_learning_arrogant" => {
            "Prêche la croissance mais est trop arrogant pour écouter les conseils."
        }
        "flag_warmth_selfish" => "Se dit chaleureux mais est perçu comme égoïste.",
        "flag_style_direct_diplomatic" => "Se dit direct mais passe pour un diplomate.",
        "flag_style_diplomatic_blunt" => "Se dit diplomate mais passe pour brutal.",
        "flag_style_competing_passive" => "Se dit compétitif mais passe pour passif.",
        "flag_style_dominant_submissive" => "Se dit autocratique mais passe pour soumis.",
        "flag_style_manipulative_honest" => "Se dit roublard mais passe pour honnête.",
        "flag_style_empathetic_cold" => "Se dit empathique mais paraît froid.",
        "flag_style_guarded_trusting" => "Se dit méfiant mais paraît confiant.",
        "flag_pattern_helping_exploiter" => {
            "Prêche l'aide aux autres mais les patterns montrent l'exploitation."
        }
        "flag_pattern_warmth_dismissive" => {
            "Image de chaleur mais les patterns rabaissent les autres."
        }
        "flag_pattern_discipline_shirker" => {
            "Image de discipline mais les patterns esquivent les responsabilités."
        }
        "flag_pattern_claimed_calm_volatile" => {
            "Se dit calme mais les patterns montrent de la volatilité."
        }
        "flag_style_servant_authoritative" => {
            "Se dit leader serviteur mais passe pour un commandant."
        }
        "flag_style_consensus_authoritative" => {
            "Se dit axé consensus mais passe pour un dictateur."
        }
        "flag_style_trusts_freely_suspicious" => "Se dit confiant mais passe pour méfiant.",
        "flag_style_repairs_trust_deceitful" => {
            "Se dit réparateur de confiance mais passe pour trompeur."
        }
        "flag_style_rulebased_favoritist" => "Se dit basé sur des règles mais joue les favoris.",
        "flag_pattern_fairness_exploiter" => {
            "Prêche l'équité mais les patterns exploitent l'injustice."
        }
        "flag_pattern_achievement_complacent" => {
            "Aspire à la réussite mais les patterns se reposent sur les lauriers."
        }
        "flag_pattern_learning_resister" => {
            "Prêche l'apprentissage mais les patterns rejettent le feedback."
        }
        "flag_pattern_extravert_quiet" => "Image d'extraversion mais les patterns se taisent.",
        "flag_style_virtuebased_deceitful" => "Se dit basé sur la vertu mais passe pour trompeur.",
        "flag_availability_calm" => {
            "Perçu comme imperturbable mais surpondère les événements dramatiques."
        }
        "flag_pattern_open_resister" => "Se dit ouvert mais les patterns résistent au changement.",
        "flag_pattern_recognition_dismissive" => {
            "Cherche la reconnaissance mais rabaisse les autres pour la gagner."
        }
        "flag_value_family_past" => {
            "Valorise la famille mais n'a aucune orientation temporelle passée."
        }
        "flag_value_stability_risk" => {
            "Aspire à la stabilité mais a un très fort appétit pour le risque — contradictoire."
        }
        "flag_value_career_family" => {
            "Carrière et famille tous deux en priorité — attendez-vous à des tensions."
        }
        "flag_value_loyalty_guarded" => {
            "Valorise la loyauté mais adopte un style de confiance défiant."
        }

        "edit_motivations" => "Motivations",
        "edit_biases" => "Biais",
        "bias_undefined_warning" => {
            "Les biais non définis comptent comme présents. Mettez 0 pour les marquer absents."
        }
        "rep_undefined_warning" => {
            "Les traits non définis pénalisent la réputation. Les valeurs extrêmes (≤2 ou ≥9) déclenchent des ajustements."
        }
        "mot_undefined_warning" => {
            "Moins de 3 motivations pénalise (−0.03 chaque). L'absence de Justice/Aide aussi."
        }
        "profile_completeness" => "Compl.",
        "edit_reputation" => "Réputation",
        "edit_patterns" => "Patterns comportementaux",
        "edit_styles" => "Styles personnels",
        "edit_notes_placeholder" => "Notes",
        "edit_evidence_placeholder" => "Preuve",
        "add_btn" => "＋",
        "edit_update_btn" => "💾",

        "mot_helper_achievement" => "Cherche à exceller et atteindre ses objectifs",
        "mot_helper_power" => "Recherche influence, contrôle et statut",
        "mot_helper_affiliation" => "Valorise l'appartenance, la connexion et l'harmonie",
        "mot_helper_security" => "Priorise la stabilité, la sécurité et la prévisibilité",
        "mot_helper_autonomy" => "Chérit l'indépendance, la liberté et l'autonomie",
        "mot_helper_recognition" => "Désire reconnaissance, éloges et visibilité",
        "mot_helper_learning" => "Soif de connaissance, de croissance et de maîtrise",
        "mot_helper_helping" => "S'épanouit en soutenant, mentorant et servant les autres",
        "mot_helper_creativity" => "Cherche à créer, innover et exprimer des idées",
        "mot_helper_fairness" => "Motivé par la justice, l'équité et le traitement juste",

        "bias_helper_confirmation" => "Favorise les infos qui confirment ses croyances",
        "bias_helper_anchoring" => "Se fie trop à la première information reçue",
        "bias_helper_availability" => "Surestime la probabilité d'événements récents",
        "bias_helper_sunk_cost" => "Continue d'investir à cause des ressources déjà dépensées",
        "bias_helper_dunning_kruger" => "Surestime sa propre compétence dans un domaine",
        "bias_helper_impostor" => "Sous-estime sa propre compétence dans un domaine",
        "bias_helper_loss_aversion" => "Craint plus les pertes qu'il ne valorise les gains",
        "bias_helper_social_proof" => "Suit le comportement des autres en cas d'incertitude",
        "bias_helper_authority" => "Se soumet excessivement aux figures d'autorité",
        "bias_helper_recency" => "Accorde trop de poids aux événements récents",
        "bias_helper_in_group" => "Favorise les membres de son propre groupe",
        "bias_helper_favoritism" => "Accorde un traitement préférentiel à certains",

        "pattern_helper_stress" => "Comment il réagit sous pression ou délais serrés",
        "pattern_helper_conflict" => "Comment il gère les désaccords et confrontations",
        "pattern_helper_success" => "Comment il répond aux réussites et victoires",
        "pattern_helper_uncertainty" => "Comment il navigue l'ambiguïté et l'incertain",
        "pattern_helper_recognition" => "Comment il cherche et réagit à la reconnaissance",
        "pattern_helper_threat" => "Comment il se défend quand il se sent attaqué",
        "pattern_helper_change" => "Comment il s'adapte aux transitions et nouveautés",
        "pattern_helper_feedback" => "Comment il reçoit et traite les retours des autres",
        "pattern_helper_injustice" => {
            "Comment il réagit face à l'injustice ou au traitement inéquitable"
        }

        "ctx_stress" => "Stress",
        "ctx_decision" => "Décision",
        "ctx_team" => "Équipe",
        "ctx_communication" => "Communication",
        "ctx_leadership" => "Leadership",
        "ctx_growth" => "Croissance",
        "ctx_conflict" => "Conflit",
        "ctx_success" => "Réussite",
        "ctx_uncertainty" => "Incertitude",
        "ctx_recognition" => "Reconnaissance",
        "ctx_threatened" => "Menacé",
        "ctx_change" => "Changement",
        "ctx_feedback" => "Feedback",
        "ctx_injustice" => "Injustice",

        "pred_all_title" => "Toutes les prédictions",
        "pred_for" => "🔮 Prédictions pour",
        "pred_title" => "Prédictions",
        "pred_context_placeholder" => "Contexte...",
        "pred_outcome_placeholder" => "Comportement prédit...",
        "pred_add_btn" => "Ajouter",
        "pred_none" => "Aucune prédiction.",
        "pred_predicted_label" => "Prédit",
        "pred_actual_label" => "Réel",
        "pred_resolve_btn" => "Résoudre",
        "pred_delete_btn" => "Supprimer",
        "pred_actual_placeholder" => "Résultat réel...",
        "pred_accuracy_label" => "Précision",
        "pred_resolve_submit" => "✓ Résoudre",
        "pred_cancel_btn" => "Annuler",

        "insights_title" => "📊 Analyses",
        "insights_select_person" => {
            "Sélectionnez une personne pour voir les analyses comportementales."
        }
        "insights_observed" => "Patterns observés",
        "log_title" => "📋 Journal",
        "log_placeholder" => "Que s'est-il passé ?",
        "log_add" => "Ajouter",
        "log_empty" => "Aucune entrée.",
        "log_valence" => "Valence",
        "log_trigger" => "Déclencheur",
        "log_target" => "Avec",
        "log_no_trigger" => "Aucun déclencheur",
        "log_no_target" => "Note perso (sans cible)",
        "trend_improving" => "Amélioration",
        "trend_stable" => "Stable",
        "trend_deteriorating" => "Détérioration",
        "trend_hint" => "Basé sur les interactions journalisées récentes",

        "strategy_stress_label" => "Sous stress",
        "strategy_conflict_label" => "En conflit",
        "strategy_success_label" => "En réussite",
        "strategy_uncertainty_label" => "Dans l'incertitude",
        "strategy_recognition_label" => "Cherchant la reconnaissance",
        "strategy_threat_label" => "Se sentant menacé",
        "strategy_change_label" => "Face au changement",
        "strategy_feedback_label" => "Recevoir du feedback",
        "strategy_when" => "Quand {name} est {trigger} :\n\n{advice}",
        "more_recs" => "Plus de recommandations",

        "strategy_stress_high_n" => {
            "Névrosisme élevé — offrez du soutien émotionnel avant les solutions."
        }
        "strategy_stress_high_e" => {
            "Extraversion élevée — permettez l'expression verbale du stress."
        }
        "strategy_stress_low_e" => "Faible extraversion — laissez de l'espace pour décompresser.",
        "strategy_stress_high_c" => {
            "Conscienciosité élevée — décomposez les problèmes en étapes actionnables."
        }
        "strategy_stress_low_a" => "Faible agréabilité — peut devenir irritable sous pression.",
        "strategy_stress_low_c" => "Faible conscienciosité — peut devenir désorganisé ou éviter.",
        "strategy_stress_high_o" => "Haute ouverture — peut trop réfléchir et imaginer le pire.",
        "strategy_stress_power" => {
            "Motivé par le pouvoir — laissez-lui reprendre le contrôle sur un domaine."
        }
        "strategy_stress_security" => {
            "Motivé par la sécurité — renforcez la stabilité et la routine."
        }
        "strategy_stress_ambition_rhetoric" => {
            "Il parle d'ambition mais est perçu comme paresseux — ne récompensez pas le discours ; concentrez-vous sur l'effort et la concrétisation."
        }
        "strategy_stress_security_rhetoric" => {
            "Il revendique la sécurité mais est naïvement confiant — ne vous fiez pas à sa prudence affichée ; vérifiez vous-même les garde-fous."
        }
        "strategy_stress_fallback" => {
            "Surveillez les signaux de stress et ajustez l'environnement."
        }

        "strategy_conflict_low_a" => {
            "Faible agréabilité — abordez le conflit directement avec des faits."
        }
        "strategy_conflict_high_a" => {
            "Haute agréabilité — adoucissez la confrontation, concentrez-vous sur l'harmonie."
        }
        "strategy_conflict_high_n" => {
            "Névrosisme élevé — désamorcez et offrez un espace de sécurité émotionnelle."
        }
        "strategy_conflict_high_e" => "Extraversion élevée — laissez-les parler pour évacuer.",
        "strategy_conflict_high_c" => {
            "Haute conscienciosité — peut insister rigidement sur les règles."
        }
        "strategy_conflict_low_e" => "Faible extraversion — peut se retirer au lieu de s'engager.",
        "strategy_conflict_fallback" => "Médiateur avec une communication équilibrée.",
        "strategy_conflict_affiliation_rhetoric" => {
            "Il revendique la proximité mais paraît froid — ne faites pas appel à son besoin déclaré de connexion ; traitez directement la distance."
        }
        "strategy_conflict_affiliation_trust_rhetoric" => {
            "Il revendique la proximité mais se montre méfiant — ne faites pas appel à son besoin déclaré de connexion ; gagnez sa confiance avant de chercher la complicité."
        }

        "strategy_success_high_o" => {
            "Haute ouverture — canalisez le succès vers de nouveaux défis créatifs."
        }
        "strategy_success_high_c" => {
            "Haute conscienciosité — utilisez le succès comme validation du processus."
        }
        "strategy_success_low_e" => {
            "Faible extraversion — peut se sentir submergé par l'attention publique."
        }
        "strategy_success_high_a" => {
            "Haute agréabilité — peut détourner le crédit pour éviter de se démarquer."
        }
        "strategy_success_recognition" => {
            "Motivé par la reconnaissance — reconnaissez publiquement leur accomplissement."
        }
        "strategy_success_power" => {
            "Motivé par le pouvoir — donnez-leur la propriété de la prochaine initiative."
        }
        "strategy_success_ambition_rhetoric" => {
            "Il parle d'ambition mais est perçu comme paresseux — ne célébrez pas ses plans ; exigez des résultats."
        }
        "strategy_success_fallback" => "Célébrez le succès et identifiez les axes de croissance.",

        "strategy_uncertainty_high_n" => {
            "Névrosisme élevé — fournissez des échéances claires et des mises à jour fréquentes."
        }
        "strategy_uncertainty_low_n" => {
            "Faible névrosisme — gère bien l'ambiguïté ; faites confiance à sa résilience."
        }
        "strategy_uncertainty_high_o" => {
            "Haute ouverture — cadrez l'incertitude comme une opportunité."
        }
        "strategy_uncertainty_low_o" => {
            "Faible ouverture — fournissez des exemples concrets et des cadres familiers."
        }
        "strategy_uncertainty_high_c" => {
            "Haute conscienciosité — a besoin d'un plan concret immédiatement."
        }
        "strategy_uncertainty_high_e" => {
            "Haute extraversion — peut trop socialiser pour gérer l'ambiguïté."
        }
        "strategy_uncertainty_fallback" => {
            "Reconnaissez l'incertitude et fournissez les informations disponibles."
        }

        "strategy_recognition_high" => {
            "Fort besoin de reconnaissance — donnez des éloges fréquents et spécifiques."
        }
        "strategy_recognition_mid" => {
            "Besoin modéré de reconnaissance — reconnaissez les contributions régulièrement."
        }
        "strategy_recognition_low" => {
            "Faible besoin de reconnaissance — évitez les éloges excessifs."
        }
        "strategy_recognition_high_e" => {
            "Extraversion élevée — la reconnaissance publique est efficace."
        }
        "strategy_recognition_low_e" => {
            "Faible extraversion — préférez une reconnaissance privée et écrite."
        }
        "strategy_recognition_fallback" => {
            "Adaptez le style de reconnaissance à leur niveau de confort."
        }

        "strategy_threat_low_a" => {
            "Faible agréabilité — peut réagir ; abordez les préoccupations calmement."
        }
        "strategy_threat_high_a" => {
            "Haute agréabilité — peut céder trop facilement ; vérifiez les vrais sentiments."
        }
        "strategy_threat_high_n" => {
            "Névrosisme élevé — les menaces perçues sont amplifiées ; offrez du réconfort."
        }
        "strategy_threat_power" => {
            "Motivé par le pouvoir — la menace au statut est sérieuse ; impliquez-le dans les décisions."
        }
        "strategy_threat_fallback" => "Écoutez activement et validez ses préoccupations.",

        "strategy_change_high_n" => {
            "Névrosisme élevé — peut résister au changement ; offrez des points d'ancrage."
        }
        "strategy_change_low_n" => {
            "Faible névrosisme — s'adapte bien ; exploitez comme champion du changement."
        }
        "strategy_change_high_c" => {
            "Haute conscienciosité — a besoin d'une feuille de route claire."
        }
        "strategy_change_low_e" => {
            "Faible extraversion — a besoin de temps pour digérer le changement en privé."
        }
        "strategy_change_high_o" => {
            "Haute ouverture — embrasse le changement ; donnez-lui un rôle actif."
        }
        "strategy_change_fallback" => "Expliquez le pourquoi et impliquez-les dans la transition.",
        "strategy_change_discipline_rhetoric" => {
            "Il se voit discipliné mais est perçu comme paresseux — ne faites pas appel à son image organisée ; vérifiez la production réelle."
        }

        "strategy_feedback_high_n" => {
            "Névrosisme élevé — peut prendre le feedback personnellement ; utilisez un ton doux."
        }
        "strategy_feedback_low_n" => "Faible névrosisme — gère bien les critiques ; soyez direct.",
        "strategy_feedback_low_a" => {
            "Faible agréabilité — peut rejeter le feedback ; basez-vous sur des faits."
        }
        "strategy_feedback_low_e" => "Faible extraversion — préfère un feedback écrit et privé.",
        "strategy_feedback_high_c" => {
            "Haute conscienciosité — apprécie un feedback détaillé et actionnable."
        }
        "strategy_feedback_fallback" => {
            "Équilibrez éloges et critiques constructives avec des exemples précis."
        }
        "strategy_feedback_helping_rhetoric" => {
            "Il prêche l'entraide mais est perçu comme égoïste — ne formulez pas le retour autour de l'aide aux autres ; nommez l'intérêt personnel derrière le conseil."
        }
        "strategy_feedback_warmth_rhetoric" => {
            "Il se voit chaleureux mais est perçu comme brutal — ne comptez pas sur un ton doux ; soyez clair et précis sur le comportement."
        }

        "strategy_injustice_label" => "Face à l'injustice",
        "strategy_injustice_high_a" => {
            "Haute agréabilité — peut se sentir personnellement blessé par l'injustice."
        }
        "strategy_injustice_high_n" => {
            "Névrosisme élevé — peut ruminer et amplifier les affronts perçus."
        }
        "strategy_injustice_fairness" => {
            "Motivé par l'équité — se battra pour ce qu'il croit juste, même à titre personnel."
        }
        "strategy_injustice_fairness_rhetoric" => {
            "Parle d'équité mais agit avec favoritisme — ne faites pas appel à son discours sur la justice ; adressez-vous au vrai moteur."
        }
        "strategy_injustice_power" => {
            "Motivé par le pouvoir — peut utiliser son autorité pour corriger le tort perçu."
        }
        "strategy_injustice_ambition_rhetoric" => {
            "Il parle d'ambition mais est perçu comme paresseux — n'attendez pas qu'il se batte pour la cause ; présentez l'issue comme servant son statut."
        }
        "strategy_injustice_fallback" => {
            "Reconnaissez leur préoccupation et clarifiez la voie vers la résolution."
        }

        "sync_title" => "☁ Sync & Sauvegarde",
        "sync_gdrive_title" => "Synchronisation Google Drive",
        "sync_token_loaded" => "✓ Jeton chargé",
        "sync_token_cleared" => "Jeton effacé",
        "sync_clear_btn" => "Effacer",
        "sync_sign_in" => "🔐 Connexion Google",
        "sync_no_token" => "Aucun jeton. Connectez-vous d'abord.",
        "sync_backing_up" => "Sauvegarde en cours...",
        "sync_backed_up" => "✅ Sauvegardé",
        "sync_backup_btn" => "☁ Sauvegarder sur Drive",
        "sync_restoring" => "Restauration en cours...",
        "sync_restored" => "✅ Restauré",
        "sync_restore_btn" => "☁ Restaurer depuis Drive",
        "sync_not_configured" => {
            "Sauvegarde Google Drive non configurée. Définissez GOOGLE_CLIENT_ID avant de compiler."
        }
        "sync_local_title" => "Sauvegarde locale",
        "sync_local_desc" => {
            "Exportez toutes les données en JSON ou importez depuis une sauvegarde."
        }
        "sync_exported" => "✅ Exporté",
        "sync_export_btn" => "📥 Exporter JSON",
        "sync_import_btn" => "📤 Importer JSON",
        "sync_passphrase_label" => "Chiffrer la sauvegarde avec une phrase de passe (optionnel)",
        "sync_passphrase_placeholder" => "Entrez la phrase de passe...",
        "sync_passphrase_show" => "Afficher",
        "sync_passphrase_hide" => "Masquer",
        "sync_wrong_passphrase" => "❌ Mauvaise phrase de passe ou données corrompues",
        "sync_token_instruction_1" => "1. Appuyez sur « Connexion Google » — le navigateur s'ouvre",
        "sync_token_instruction_2" => "2. Connectez-vous et autorisez l'accès",
        "sync_token_instruction_3" => {
            "3. Le navigateur redirige vers l'app web — copiez le jeton depuis la barre d'adresse avant que la page ne charge"
        }
        "sync_token_instruction_4" => "4. Collez l'URL ci-dessous et appuyez sur Enregistrer",
        "sync_paste_placeholder" => "Collez l'URL de redirection complète ici",
        "sync_token_saved" => "✅ Jeton enregistré",
        "sync_save_token_btn" => "Enregistrer",
        "sync_no_data_warn" => "Aucune personne à sauvegarder. Ajoutez des personnes d'abord !",
        "sync_view_backup" => "🔎 Voir les sauvegardes dans le navigateur (appDataFolder Browser)",

        "common_save" => "Enregistrer",
        "common_cancel" => "Annuler",
        "common_delete" => "Supprimer",
        "common_add" => "Ajouter",
        "common_edit" => "Modifier",
        "common_back" => "← Retour",
        "compare_title" => "Comparer des personnes",
        "compare_btn" => "Comparer",
        "compare_sub" => "Identifiez synergies et points de friction entre deux personnes",
        "compare_vs" => "VS",
        "compare_top_mot" => "Motivation principale",
        "compare_bias_main" => "Biais principal",
        "compare_ocean" => "Profil OCEAN",
        "compare_analysis_title" => "Analyse dynamique",
        "compare_synergies" => "Synergies",
        "compare_friction" => "Points de friction",
        "compare_strategy" => "Stratégie d'interaction",
        "compare_breakdown" => "Détail",
        "compare_ctx_title" => "Par situation",
        "compare_cat_ocean" => "OCÉAN",
        "compare_cat_reputation" => "Réputation",
        "compare_cat_motivation" => "Motivation",
        "compare_cat_patterns" => "Patterns",
        "compare_cat_bias" => "Biais",
        "compare_cat_styles" => "Styles",
        "compare_cat_values" => "Valeurs",
        "compare_risk_mitigation" => "Risques & Mitigations",
        "values_title" => "Valeurs",
        "no_values" => "Aucune valeur définie",
        "edit_values" => "Valeurs",
        "edit_priority" => "P",
        "compare_rel_title" => "Contexte de relation",
        "compare_rel_none" => "Général (sans contexte)",
        "compare_rel_strength" => "Intensité",
        "compare_band_hint" => "±{}% (relation + fiabilité du profil)",
        "person_self_score" => "Score de profil",
        "Rep power struggle" => "Lutte de pouvoir (réputation)",
        "OCEAN volatility" => "Volatilité OCEAN",
        "Only negative patterns" => "Patterns négatifs uniquement",
        "Low prediction accuracy" => "Faible précision prédictive",
        "compare_asymmetric" => "Bénéfice mutuel",
        "compare_benefit_more" => "bénéficie plus",
        "compare_balanced" => "Équilibré",
        "compare_ethics" => {
            "Ce sont des modèles probabilistes, pas des vérités absolues. Utilisez-les pour mieux comprendre, jamais pour manipuler."
        }

        // Scale bands
        "scale_strong" => "Fort",
        "scale_good" => "Bon",
        "scale_moderate" => "Moyen",
        "scale_friction" => "Friction",
        "scale_tension" => "Tension",

        // Relationships
        "rel_title" => "Relations",
        "rel_notes" => "Notes",
        "rel_strength" => "Intensité",
        "rel_none" => "Aucune relation.",
        "rel_open_add" => "＋ Ajouter",
        "rel_close_add" => "− Annuler",
        "rel_search_placeholder" => "Rechercher une personne…",
        "rel_confirm_delete" => "Supprimer cette relation ?",
        "confirm_delete" => "Supprimer cette personne ?",
        "confirm_delete_log" => "Supprimer cette entrée ?",
        "confirm_delete_pred" => "Supprimer cette prédiction ?",
        "no_search_results" => "Aucun résultat pour «{0}».",
        "rel_person_rel" => "Relations",

        // Timeline
        "tl_title" => "Chronologie",
        "tl_empty" => "Aucune entrée d'interaction.",

        // Style helpers
        "style_no_styles" => "Aucun style personnel enregistré.",
        "style_panel_title" => "Styles personnels",

        // Tags

        // Tutorial
        "tut_step" => "Étape",
        "tut_welcome_title" => "Bienvenue sur PeopleModeler !",
        "tut_welcome_body" => {
            "Cette application vous aide à modéliser et comprendre les personnes de votre vie en utilisant des cadres de personnalité comme l'OCEAN (Big Five), les motivations, les biais cognitifs et les schémas comportementaux.\n\nVous pouvez comparer des personnes côte à côte, suivre des prédictions dans le temps, cartographier les relations et explorer les scores de synergie."
        }
        "tut_people_title" => "Vos Personnes",
        "tut_people_body" => {
            "La page principale montre toutes les personnes que vous avez créées. Utilisez la barre de recherche pour trouver quelqu'un, triez par nom / récent / score OCEAN, et cliquez sur le bouton + pour ajouter une nouvelle personne."
        }
        "tut_create_title" => "Créer une Personne",
        "tut_create_body" => {
            "Le formulaire personne est divisé en sections : infos de base (nom, rôle, contexte), scores de personnalité OCEAN, motivations, biais cognitifs, dimensions de réputation et schémas comportementaux.\n\nChaque section capture une facette différente de la personnalité — remplissez ce que vous savez, laissez le reste vide."
        }
        "tut_ocean_title" => "Modèle OCEAN (Big Five)",
        "tut_ocean_body" => {
            "L'OCEAN mesure la personnalité sur cinq dimensions de 1 à 10 :\n• Ouverture — curiosité vs. prudence\n• Conscience — organisation vs. flexibilité\n• Extraversion — sociabilité vs. solitude\n• Agréabilité — coopération vs. compétition\n• Névrosisme — sensibilité vs. stabilité émotionnelle\n\nCes scores alimentent le moteur de comparaison et aident à prédire le comportement."
        }
        "tut_mot_bias_title" => "Motivations & Biais",
        "tut_mot_bias_body" => {
            "Les motivations capturent ce qui anime une personne — ses objectifs, ses peurs et ses valeurs (Réussite, Pouvoir, Affiliation, Sécurité, Autonomie, etc.).\n\nLes biais représentent des raccourcis mentaux qui influencent ses décisions (biais de confirmation, ancrage, excès de confiance, etc.). Ensemble, ils vous donnent une compréhension plus profonde de pourquoi les gens agissent comme ils le font."
        }
        "tut_rep_pattern_title" => "Réputation & Schémas",
        "tut_rep_pattern_body" => {
            "Les scores de réputation capturent comment les autres perçoivent cette personne sur des échelles bipolaires (travailleur vs. paresseux, honnête vs. trompeur, etc.).\n\nLes schémas comportementaux vous permettent d'enregistrer comment elle réagit typiquement à des déclencheurs spécifiques (stress, critique, succès, conflit, etc.). Cela aide à anticiper ses réponses dans des situations futures."
        }
        "tut_compare_title" => "Comparaisons & Plus",
        "tut_compare_body" => {
            "Une fois que vous avez au moins deux personnes, vous pouvez les comparer côte à côte pour voir leur score de synergie, leurs points de friction et leurs stratégies d'interaction.\n\nVous pouvez aussi suivre des prédictions (devinez un résultat, puis vérifiez si vous aviez raison), construire une carte des relations et journaliser les interactions sur une chronologie."
        }
        "tut_done_title" => "Prêt à Commencer !",
        "tut_done_body" => {
            "Vous pouvez rejouer ce tutoriel à tout moment depuis la barre de navigation.\n\nConseils rapides :\n• Créez au moins deux personnes pour débloquer les comparaisons\n• Utilisez la page Sync pour sauvegarder vos données\n• Utilisez les tags pour organiser les personnes par groupe\n\nAllez-y et commencez à modéliser les personnes de votre monde !"
        }

        // Team page
        "nav_teams" => "Équipes",
        "teams_title" => "Équipes",
        "teams_all" => "Toutes les personnes",
        "teams_create" => "Nouvelle équipe",
        "teams_delete" => "Supprimer l'équipe ?",
        "teams_members" => "{0} membres",
        "team_title" => "Synergie d'équipe",
        "team_empty" => "Ajoutez au moins 2 personnes pour voir la synergie d'équipe.",
        "team_size" => "Taille",
        "team_avg_score" => "Score moyen",
        "team_strongest" => "Lien le plus fort",
        "team_weakest" => "Lien le plus faible",
        "team_max_danger" => "Danger max",
        "team_avg_danger" => "Danger moyen",
        "team_ctx_avg" => "Moyenne par situation",
        "team_pairs" => "Toutes les paires",
        "team_no_danger" => "Aucun",
        "team_tab_synergy" => "Synergie",
        "team_tab_members" => "Membres",
        "team_all_no_edit" => "Toutes les personnes inclut tout le monde automatiquement",
        "team_members_count" => "{0} membres",
        "confirm_delete_team" => "Supprimer cette équipe ?",
        "team_rename" => "Renommer",
        "team_icon" => "Icône",
        "team_edit" => "Modifier",

        // Common (tutorial)
        "common_next" => "Suivant →",
        "common_skip" => "Passer",
        "common_finish" => "Terminer",

        _ => key,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KEYS: &[&str] = &[
        "add_btn",
        "bias_helper_anchoring",
        "bias_helper_authority",
        "bias_helper_availability",
        "bias_helper_confirmation",
        "bias_helper_dunning_kruger",
        "bias_helper_favoritism",
        "bias_helper_impostor",
        "bias_helper_in_group",
        "bias_helper_loss_aversion",
        "bias_helper_recency",
        "bias_helper_social_proof",
        "bias_helper_sunk_cost",
        "bias_undefined_warning",
        "biases_title",
        "common_add",
        "common_back",
        "common_cancel",
        "common_delete",
        "common_edit",
        "common_finish",
        "common_next",
        "common_save",
        "common_skip",
        "compare_analysis_title",
        "compare_asymmetric",
        "compare_balanced",
        "compare_band_hint",
        "compare_benefit_more",
        "compare_bias_main",
        "compare_breakdown",
        "compare_btn",
        "compare_cat_bias",
        "compare_cat_motivation",
        "compare_cat_ocean",
        "compare_cat_patterns",
        "compare_cat_reputation",
        "compare_cat_styles",
        "compare_cat_values",
        "compare_ctx_title",
        "compare_ethics",
        "compare_friction",
        "compare_ocean",
        "compare_rel_none",
        "compare_rel_strength",
        "compare_rel_title",
        "compare_risk_mitigation",
        "compare_strategy",
        "compare_sub",
        "compare_synergies",
        "compare_title",
        "compare_top_mot",
        "compare_vs",
        "confidence_hint",
        "confidence_label",
        "confirm_delete",
        "confirm_delete_log",
        "confirm_delete_pred",
        "confirm_delete_team",
        "ctx_change",
        "ctx_communication",
        "ctx_conflict",
        "ctx_decision",
        "ctx_feedback",
        "ctx_growth",
        "ctx_injustice",
        "ctx_leadership",
        "ctx_recognition",
        "ctx_stress",
        "ctx_success",
        "ctx_team",
        "ctx_threatened",
        "ctx_uncertainty",
        "delete_btn",
        "edit_biases",
        "edit_btn",
        "edit_evidence_placeholder",
        "edit_motivations",
        "edit_notes_placeholder",
        "edit_patterns",
        "edit_priority",
        "edit_reputation",
        "edit_styles",
        "edit_update_btn",
        "edit_values",
        "flag_affiliation_cold",
        "flag_affiliation_distrustful",
        "flag_ambition_lazy",
        "flag_anchoring_open",
        "flag_authority_dominant",
        "flag_autonomy_submissive",
        "flag_availability_calm",
        "flag_bias_confirmation_open",
        "flag_bias_favoritism_fairness",
        "flag_calm_neurotic",
        "flag_claims_calm_reactive",
        "flag_creativity_closed",
        "flag_creativity_rigid",
        "flag_discipline_flaky",
        "flag_discipline_lazy",
        "flag_dunning_kruger_humble",
        "flag_fairness_rhetoric",
        "flag_helping_cold",
        "flag_helping_selfish",
        "flag_high_e_low_a",
        "flag_high_n_low_c",
        "flag_high_o_low_c",
        "flag_honest_favoritist",
        "flag_honest_selfish",
        "flag_impostor_arrogant",
        "flag_learning_arrogant",
        "flag_learning_rigid",
        "flag_loss_aversion_risky",
        "flag_open_rigid",
        "flag_pattern_achievement_complacent",
        "flag_pattern_assertive_quiet",
        "flag_pattern_calm_volatile",
        "flag_pattern_claimed_calm_volatile",
        "flag_pattern_diplomat_escalator",
        "flag_pattern_discipline_shirker",
        "flag_pattern_empath_dismissive",
        "flag_pattern_extravert_quiet",
        "flag_pattern_fair_exploiter",
        "flag_pattern_fairness_exploiter",
        "flag_pattern_flexible_resister",
        "flag_pattern_generous_exploiter",
        "flag_pattern_hardworker_complacent",
        "flag_pattern_helping_exploiter",
        "flag_pattern_honest_exploiter",
        "flag_pattern_humble_dismissive",
        "flag_pattern_learning_resister",
        "flag_pattern_open_resister",
        "flag_pattern_passive_blowup",
        "flag_pattern_recognition_dismissive",
        "flag_pattern_reliable_shirker",
        "flag_pattern_trusting_paranoid",
        "flag_pattern_warmth_dismissive",
        "flag_power_passive",
        "flag_recency_reliable",
        "flag_resilient_hides",
        "flag_resilient_reactive",
        "flag_risk_appetite_ambition",
        "flag_security_gullible",
        "flag_security_risky",
        "flag_social_proof_open",
        "flag_style_competing_passive",
        "flag_style_consensus_authoritative",
        "flag_style_diplomatic_blunt",
        "flag_style_direct_diplomatic",
        "flag_style_dominant_submissive",
        "flag_style_empathetic_cold",
        "flag_style_guarded_trusting",
        "flag_style_manipulative_honest",
        "flag_style_repairs_trust_deceitful",
        "flag_style_rulebased_favoritist",
        "flag_style_servant_authoritative",
        "flag_style_trusts_freely_suspicious",
        "flag_style_virtuebased_deceitful",
        "flag_sunk_cost_flexible",
        "flag_value_career_family",
        "flag_value_family_past",
        "flag_value_loyalty_guarded",
        "flag_value_stability_risk",
        "flag_warmth_blunt",
        "flag_warmth_cold",
        "flag_warmth_selfish",
        "form_avatar",
        "form_cancel",
        "form_confidence",
        "form_context",
        "form_edit_title",
        "form_name",
        "form_new_title",
        "form_notes",
        "form_ocean_title",
        "form_resilience",
        "form_risk_appetite",
        "form_role",
        "form_save",
        "form_tags",
        "insights_observed",
        "insights_select_person",
        "insights_title",
        "log_add",
        "log_empty",
        "log_no_target",
        "log_no_trigger",
        "log_placeholder",
        "log_target",
        "log_title",
        "log_trigger",
        "log_valence",
        "more_recs",
        "mot_helper_achievement",
        "mot_helper_affiliation",
        "mot_helper_autonomy",
        "mot_helper_creativity",
        "mot_helper_fairness",
        "mot_helper_helping",
        "mot_helper_learning",
        "mot_helper_power",
        "mot_helper_recognition",
        "mot_helper_security",
        "mot_undefined_warning",
        "motivations_title",
        "nav_people",
        "nav_relationships",
        "nav_sync",
        "nav_teams",
        "nav_timeline",
        "no_biases",
        "no_motivations",
        "no_patterns",
        "no_people_insights",
        "no_people_yet",
        "no_reputation",
        "no_search_results",
        "no_values",
        "ocean_a",
        "ocean_a_high",
        "ocean_a_low",
        "ocean_agreeableness",
        "ocean_c",
        "ocean_c_high",
        "ocean_c_low",
        "ocean_conscientiousness",
        "ocean_e",
        "ocean_e_high",
        "ocean_e_low",
        "ocean_extraversion",
        "ocean_n",
        "ocean_n_high",
        "ocean_n_low",
        "ocean_neuroticism",
        "ocean_o",
        "ocean_o_high",
        "ocean_o_low",
        "ocean_openness",
        "ocean_title",
        "pattern_helper_change",
        "pattern_helper_conflict",
        "pattern_helper_feedback",
        "pattern_helper_injustice",
        "pattern_helper_recognition",
        "pattern_helper_stress",
        "pattern_helper_success",
        "pattern_helper_threat",
        "pattern_helper_uncertainty",
        "patterns_title",
        "person_not_found",
        "person_self_score",
        "pl_name",
        "pred_accuracy_label",
        "pred_actual_label",
        "pred_actual_placeholder",
        "pred_add_btn",
        "pred_all_title",
        "pred_cancel_btn",
        "pred_context_placeholder",
        "pred_delete_btn",
        "pred_for",
        "pred_none",
        "pred_outcome_placeholder",
        "pred_predicted_label",
        "pred_resolve_btn",
        "pred_resolve_submit",
        "pred_title",
        "profile_completeness",
        "rel_close_add",
        "rel_confirm_delete",
        "rel_none",
        "rel_notes",
        "rel_open_add",
        "rel_person_rel",
        "rel_search_placeholder",
        "rel_strength",
        "rel_title",
        "reliability_title",
        "rep_undefined_warning",
        "reputation_title",
        "resilience_label",
        "risk_appetite_label",
        "scale_friction",
        "scale_good",
        "scale_moderate",
        "scale_strong",
        "scale_tension",
        "score_band",
        "search_placeholder",
        "strategy_change_discipline_rhetoric",
        "strategy_change_fallback",
        "strategy_change_high_c",
        "strategy_change_high_n",
        "strategy_change_high_o",
        "strategy_change_label",
        "strategy_change_low_e",
        "strategy_change_low_n",
        "strategy_conflict_affiliation_rhetoric",
        "strategy_conflict_affiliation_trust_rhetoric",
        "strategy_conflict_fallback",
        "strategy_conflict_high_a",
        "strategy_conflict_high_c",
        "strategy_conflict_high_e",
        "strategy_conflict_high_n",
        "strategy_conflict_label",
        "strategy_conflict_low_a",
        "strategy_conflict_low_e",
        "strategy_feedback_fallback",
        "strategy_feedback_helping_rhetoric",
        "strategy_feedback_high_c",
        "strategy_feedback_high_n",
        "strategy_feedback_label",
        "strategy_feedback_low_a",
        "strategy_feedback_low_e",
        "strategy_feedback_low_n",
        "strategy_feedback_warmth_rhetoric",
        "strategy_injustice_ambition_rhetoric",
        "strategy_injustice_fairness",
        "strategy_injustice_fairness_rhetoric",
        "strategy_injustice_fallback",
        "strategy_injustice_high_a",
        "strategy_injustice_high_n",
        "strategy_injustice_label",
        "strategy_injustice_power",
        "strategy_recognition_fallback",
        "strategy_recognition_high",
        "strategy_recognition_high_e",
        "strategy_recognition_label",
        "strategy_recognition_low",
        "strategy_recognition_low_e",
        "strategy_recognition_mid",
        "strategy_stress_ambition_rhetoric",
        "strategy_stress_fallback",
        "strategy_stress_high_c",
        "strategy_stress_high_e",
        "strategy_stress_high_n",
        "strategy_stress_high_o",
        "strategy_stress_label",
        "strategy_stress_low_a",
        "strategy_stress_low_c",
        "strategy_stress_low_e",
        "strategy_stress_power",
        "strategy_stress_security",
        "strategy_stress_security_rhetoric",
        "strategy_success_ambition_rhetoric",
        "strategy_success_fallback",
        "strategy_success_high_a",
        "strategy_success_high_c",
        "strategy_success_high_o",
        "strategy_success_label",
        "strategy_success_low_e",
        "strategy_success_power",
        "strategy_success_recognition",
        "strategy_threat_fallback",
        "strategy_threat_high_a",
        "strategy_threat_high_n",
        "strategy_threat_label",
        "strategy_threat_low_a",
        "strategy_threat_power",
        "strategy_uncertainty_fallback",
        "strategy_uncertainty_high_c",
        "strategy_uncertainty_high_e",
        "strategy_uncertainty_high_n",
        "strategy_uncertainty_high_o",
        "strategy_uncertainty_label",
        "strategy_uncertainty_low_n",
        "strategy_uncertainty_low_o",
        "strategy_when",
        "style_no_styles",
        "style_panel_title",
        "sync_backed_up",
        "sync_backing_up",
        "sync_backup_btn",
        "sync_clear_btn",
        "sync_export_btn",
        "sync_exported",
        "sync_gdrive_title",
        "sync_import_btn",
        "sync_local_desc",
        "sync_local_title",
        "sync_no_data_warn",
        "sync_no_token",
        "sync_not_configured",
        "sync_passphrase_hide",
        "sync_passphrase_label",
        "sync_passphrase_placeholder",
        "sync_passphrase_show",
        "sync_paste_placeholder",
        "sync_restore_btn",
        "sync_restored",
        "sync_restoring",
        "sync_save_token_btn",
        "sync_sign_in",
        "sync_title",
        "sync_token_cleared",
        "sync_token_instruction_1",
        "sync_token_instruction_2",
        "sync_token_instruction_3",
        "sync_token_instruction_4",
        "sync_token_loaded",
        "sync_token_saved",
        "sync_view_backup",
        "sync_wrong_passphrase",
        "team_all_no_edit",
        "team_avg_danger",
        "team_avg_score",
        "team_ctx_avg",
        "team_edit",
        "team_empty",
        "team_icon",
        "team_max_danger",
        "team_members_count",
        "team_no_danger",
        "team_pairs",
        "team_rename",
        "team_size",
        "team_strongest",
        "team_tab_members",
        "team_tab_synergy",
        "team_title",
        "team_weakest",
        "teams_all",
        "teams_create",
        "teams_delete",
        "teams_members",
        "teams_title",
        "template_blank",
        "template_title",
        "tl_empty",
        "tl_title",
        "toast_deleted",
        "toast_error",
        "toast_saved",
        "trend_deteriorating",
        "trend_hint",
        "trend_improving",
        "trend_stable",
        "tut_compare_body",
        "tut_compare_title",
        "tut_create_body",
        "tut_create_title",
        "tut_done_body",
        "tut_done_title",
        "tut_mot_bias_body",
        "tut_mot_bias_title",
        "tut_ocean_body",
        "tut_ocean_title",
        "tut_people_body",
        "tut_people_title",
        "tut_rep_pattern_body",
        "tut_rep_pattern_title",
        "tut_step",
        "tut_welcome_body",
        "tut_welcome_title",
        "values_title",
    ];

    const IDENTITY_KEYS: &[&str] = &[
        "OCEAN volatility",
        "Rep power struggle",
        "Only negative patterns",
        "Low prediction accuracy",
    ];

    #[test]
    fn all_keys_translate_en() {
        for &key in ALL_KEYS {
            let result = tr(key, Lang::En);
            assert!(!result.is_empty(), "tr({key}, En) returned empty");
            assert_ne!(
                result, key,
                "tr({key}, En) returned key itself (arm deleted?)"
            );
            assert_ne!(result, "xyzzy", "tr({key}, En) returned sentinel 'xyzzy'");
        }
        for &key in IDENTITY_KEYS {
            let result = tr(key, Lang::En);
            assert!(!result.is_empty(), "tr({key}, En) returned empty");
            assert_ne!(result, "xyzzy", "tr({key}, En) returned sentinel 'xyzzy'");
        }
    }

    #[test]
    fn all_keys_translate_fr() {
        for &key in ALL_KEYS {
            let result = tr(key, Lang::Fr);
            assert!(!result.is_empty(), "tr({key}, Fr) returned empty");
            assert_ne!(
                result, key,
                "tr({key}, Fr) returned key itself (arm deleted?)"
            );
            assert_ne!(result, "xyzzy", "tr({key}, Fr) returned sentinel 'xyzzy'");
        }
        for &key in IDENTITY_KEYS {
            let result = tr(key, Lang::Fr);
            assert!(!result.is_empty(), "tr({key}, Fr) returned empty");
            assert_ne!(
                result, key,
                "tr({key}, Fr) returned key itself (arm deleted?)"
            );
            assert_ne!(result, "xyzzy", "tr({key}, Fr) returned sentinel 'xyzzy'");
        }
    }

    #[test]
    fn tr_empty_details() {
        assert_eq!(tr_danger_details("", Lang::En), "");
        assert_eq!(tr_danger_details("", Lang::Fr), "");
    }

    #[test]
    fn tr_danger_details_en_individual() {
        assert_eq!(
            tr_danger_details("OCEAN volatility", Lang::En),
            "OCEAN volatility"
        );
        assert_eq!(
            tr_danger_details("Rep power struggle", Lang::En),
            "Power struggle (Reputation)"
        );
        assert_eq!(
            tr_danger_details("Only negative patterns", Lang::En),
            "Only negative patterns"
        );
        assert_eq!(
            tr_danger_details("Low prediction accuracy", Lang::En),
            "Low prediction accuracy"
        );
    }

    #[test]
    fn tr_danger_details_fr_individual() {
        assert_eq!(
            tr_danger_details("OCEAN volatility", Lang::Fr),
            "Volatilité OCEAN"
        );
        assert_eq!(
            tr_danger_details("Rep power struggle", Lang::Fr),
            "Lutte de pouvoir (réputation)"
        );
        assert_eq!(
            tr_danger_details("Only negative patterns", Lang::Fr),
            "Patterns négatifs uniquement"
        );
        assert_eq!(
            tr_danger_details("Low prediction accuracy", Lang::Fr),
            "Faible précision prédictive"
        );
    }

    #[test]
    fn tr_danger_details_multi() {
        assert_eq!(
            tr_danger_details("OCEAN volatility, Rep power struggle", Lang::En),
            "OCEAN volatility, Power struggle (Reputation)"
        );
        assert_eq!(
            tr_danger_details("OCEAN volatility, Rep power struggle", Lang::Fr),
            "Volatilité OCEAN, Lutte de pouvoir (réputation)"
        );
    }

    #[test]
    fn tr_danger_details_unknown() {
        assert_eq!(
            tr_danger_details("Some unknown detail", Lang::En),
            "Unknown"
        );
        assert_eq!(
            tr_danger_details("Some unknown detail", Lang::Fr),
            "Unknown"
        );
    }

    #[test]
    fn lang_detect_nonwasm() {
        let lang = Lang::detect();
        assert!(lang == Lang::En || lang == Lang::Fr);
    }

    #[test]
    fn lang_display() {
        assert_eq!(format!("{:?}", Lang::En), "En");
        assert_eq!(format!("{:?}", Lang::Fr), "Fr");
    }

    #[test]
    fn lang_persist_writes_file() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let path = std::env::current_dir().unwrap().join(".pm_lang");
        let _ = std::fs::remove_file(&path);
        Lang::En.persist();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "en");
        Lang::Fr.persist();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "fr");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn lang_equality() {
        assert_eq!(Lang::En, Lang::En);
        assert_eq!(Lang::Fr, Lang::Fr);
        assert_ne!(Lang::En, Lang::Fr);
    }

    #[test]
    fn lang_clone_copy() {
        let a = Lang::En;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn detect_from_strings_stored_en() {
        assert_eq!(detect_from_strings(Some("en"), None, None), Lang::En);
    }

    #[test]
    fn detect_from_strings_stored_fr() {
        assert_eq!(detect_from_strings(Some("fr"), None, None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_stored_other() {
        assert_eq!(detect_from_strings(Some("de"), None, None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_navigator_en() {
        assert_eq!(detect_from_strings(None, Some("en-CA"), None), Lang::En);
    }

    #[test]
    fn detect_from_strings_navigator_fr() {
        assert_eq!(detect_from_strings(None, Some("fr-CA"), None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_env_en() {
        assert_eq!(
            detect_from_strings(None, None, Some("en_US.UTF-8")),
            Lang::En
        );
    }

    #[test]
    fn detect_from_strings_env_fr() {
        assert_eq!(
            detect_from_strings(None, None, Some("fr_CA.UTF-8")),
            Lang::Fr
        );
    }

    #[test]
    fn detect_from_strings_all_none() {
        assert_eq!(detect_from_strings(None, None, None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_stored_takes_priority() {
        assert_eq!(
            detect_from_strings(Some("fr"), Some("en-CA"), Some("en_US.UTF-8")),
            Lang::Fr
        );
    }
}

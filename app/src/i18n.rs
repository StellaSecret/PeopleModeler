#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    Fr,
    En,
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
            if let Some(l) = stored {
                if l == "en" {
                    return Lang::En;
                }
                return Lang::Fr;
            }
            if let Some(nav) = web_sys::window().and_then(|w| w.navigator().language()) {
                if nav.starts_with("en") {
                    return Lang::En;
                }
            }
            Lang::Fr
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(l) = std::env::var("LANG") && l.starts_with("en") {
                return Lang::En;
            }
            Lang::Fr
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
            "Rep power struggle" => "Power struggle (Reputation)",
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
        "confidence_label" => "Confidence",
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
        "form_confidence" => "Assessment confidence (1-10)",
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

        // Edit form sections
        "edit_motivations" => "Motivations",
        "edit_biases" => "Biases",
        "edit_reputation" => "Reputation",
        "edit_patterns" => "Behavioral Patterns",
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

        "bias_helper_confirmation" => "Favors info that confirms existing beliefs",
        "bias_helper_anchoring" => "Over-relies on the first piece of info received",
        "bias_helper_availability" => "Overestimates likelihood of easily recalled events",
        "bias_helper_sunk_cost" => "Continues investing due to past sunken resources",
        "bias_helper_dunning_kruger" => "Overestimates own competence in a domain",
        "bias_helper_loss_aversion" => "Fears losses more than values equivalent gains",
        "bias_helper_social_proof" => "Follows others' behavior in uncertainty",
        "bias_helper_authority" => "Defers excessively to authority figures",
        "bias_helper_recency" => "Overweighs recent events over older ones",
        "bias_helper_in_group" => "Favors own group members over outsiders",

        "pattern_helper_stress" => "How they react under pressure or tight deadlines",
        "pattern_helper_conflict" => "How they handle disagreements and confrontation",
        "pattern_helper_success" => "How they respond to achievements and wins",
        "pattern_helper_uncertainty" => "How they navigate ambiguity and unknown outcomes",
        "pattern_helper_recognition" => "How they seek and respond to acknowledgment",
        "pattern_helper_threat" => "How they defend themselves when feeling attacked",
        "pattern_helper_change" => "How they adapt to transitions and new situations",
        "pattern_helper_feedback" => "How they receive and process input from others",

        // Context labels
        "ctx_stress" => "Stress",
        "ctx_conflict" => "Conflict",
        "ctx_success" => "Success",
        "ctx_uncertainty" => "Uncertainty",
        "ctx_recognition" => "Recognition",
        "ctx_threatened" => "Threatened",
        "ctx_change" => "Change",
        "ctx_feedback" => "Feedback",

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

        // Insight strategies
        "strategy_stress_label" => "Under stress",
        "strategy_conflict_label" => "In conflict",
        "strategy_success_label" => "In success",
        "strategy_uncertainty_label" => "In uncertainty",
        "strategy_recognition_label" => "Seeking recognition",
        "strategy_threat_label" => "Feeling threatened",
        "strategy_change_label" => "Facing change",
        "strategy_feedback_label" => "Receiving feedback",

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

        // Common
        "common_save" => "Save",
        "common_cancel" => "Cancel",
        "common_delete" => "Delete",
        "common_confirm" => "Confirm",
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
        "compare_cat_ocean" => "OCEAN",
        "compare_cat_reputation" => "Reputation",
        "compare_cat_motivation" => "Motivation",
        "compare_cat_patterns" => "Patterns",
        "compare_cat_bias" => "Bias",
        "person_self_score" => "Profile Score",
        "OCEAN volatility" => "OCEAN volatility",
        "Rep power struggle" => "Rep power struggle",
        "Only negative patterns" => "Only negative patterns",
        "Low prediction accuracy" => "Low prediction accuracy",
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
        "rel_add" => "＋ Add Relationship",
        "rel_from" => "From",
        "rel_to" => "To",
        "rel_type" => "Type",
        "rel_notes" => "Notes",
        "rel_none" => "No relationships yet.",
        "rel_open_add" => "＋ Add",
        "rel_close_add" => "− Cancel",
        "rel_search_placeholder" => "Search person…",
        "rel_type_placeholder" => "e.g. works_with, manages, friends, family",
        "rel_confirm_delete" => "Delete this relationship?",
        "rel_person_rel" => "Relationships",

        // Timeline
        "tl_title" => "Timeline",
        "tl_empty" => "No interaction entries yet.",

        // Tags

        // Tutorial
        "tut_welcome_title" => "Welcome to PeopleModeler!",
        "tut_welcome_body" => "This app helps you model and understand the people in your life using personality frameworks like OCEAN (Big Five), motivations, cognitive biases, and behavioral patterns.\n\nYou can compare people side by side, track predictions over time, map relationships, and explore synergy scores.",
        "tut_people_title" => "Your People",
        "tut_people_body" => "The main page shows everyone you've created. Use the search bar to find someone, sort by name / recent / OCEAN score, and click the + button to add someone new.",
        "tut_create_title" => "Creating a Person",
        "tut_create_body" => "The person form is divided into sections: basic info (name, role, context), OCEAN personality scores, motivations, cognitive biases, reputation dimensions, and behavioral patterns.\n\nEach section captures a different facet of someone's personality — fill in what you know, leave the rest blank.",
        "tut_ocean_title" => "OCEAN Model (Big Five)",
        "tut_ocean_body" => "OCEAN measures personality across five dimensions from 1 to 10:\n• Openness — curiosity vs. caution\n• Conscientiousness — organization vs. flexibility\n• Extraversion — sociability vs. solitude\n• Agreeableness — cooperation vs. competition\n• Neuroticism — sensitivity vs. emotional stability\n\nThese scores power the comparison engine and help predict behaviour.",
        "tut_mot_bias_title" => "Motivations & Biases",
        "tut_mot_bias_body" => "Motivations capture what drives a person — their goals, fears, and values (Achievement, Power, Affiliation, Security, Autonomy, etc.).\n\nBiases represent mental shortcuts that shape their decisions (Confirmation bias, Anchoring, Overconfidence, etc.). Together they give you a deeper understanding of why people act the way they do.",
        "tut_rep_pattern_title" => "Reputation & Patterns",
        "tut_rep_pattern_body" => "Reputation scores capture how others perceive this person across bipolar scales (hardworking vs. lazy, honest vs. deceitful, etc.).\n\nBehavioral patterns let you record how they typically react to specific triggers (stress, criticism, success, conflict, etc.). This helps anticipate their responses in future situations.",
        "tut_compare_title" => "Comparisons & More",
        "tut_compare_body" => "Once you have at least two people, you can compare them side by side to see their synergy score, friction points, and interaction strategies.\n\nYou can also track predictions (guess an outcome, then check if you were right), build a relationship map, and log interactions on a timeline.",
        "tut_done_title" => "You're Ready!",
        "tut_done_body" => "You can replay this tutorial anytime from the navigation bar.\n\nQuick tips:\n• Create at least two people to unlock comparisons\n• Use the Sync page to back up your data\n• Tag people to organise them by group\n\nGo ahead and start modeling the people in your world!",

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
        "confidence_label" => "Confiance",
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
        "form_confidence" => "Confiance estimation (1-10)",
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

        "edit_motivations" => "Motivations",
        "edit_biases" => "Biais",
        "edit_reputation" => "Réputation",
        "edit_patterns" => "Patterns comportementaux",
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

        "bias_helper_confirmation" => "Favorise les infos qui confirment ses croyances",
        "bias_helper_anchoring" => "Se fie trop à la première information reçue",
        "bias_helper_availability" => "Surestime la probabilité d'événements récents",
        "bias_helper_sunk_cost" => "Continue d'investir à cause des ressources déjà dépensées",
        "bias_helper_dunning_kruger" => "Surestime sa propre compétence dans un domaine",
        "bias_helper_loss_aversion" => "Craint plus les pertes qu'il ne valorise les gains",
        "bias_helper_social_proof" => "Suit le comportement des autres en cas d'incertitude",
        "bias_helper_authority" => "Se soumet excessivement aux figures d'autorité",
        "bias_helper_recency" => "Accorde trop de poids aux événements récents",
        "bias_helper_in_group" => "Favorise les membres de son propre groupe",

        "pattern_helper_stress" => "Comment il réagit sous pression ou délais serrés",
        "pattern_helper_conflict" => "Comment il gère les désaccords et confrontations",
        "pattern_helper_success" => "Comment il répond aux réussites et victoires",
        "pattern_helper_uncertainty" => "Comment il navigue l'ambiguïté et l'incertain",
        "pattern_helper_recognition" => "Comment il cherche et réagit à la reconnaissance",
        "pattern_helper_threat" => "Comment il se défend quand il se sent attaqué",
        "pattern_helper_change" => "Comment il s'adapte aux transitions et nouveautés",
        "pattern_helper_feedback" => "Comment il reçoit et traite les retours des autres",

        "ctx_stress" => "Stress",
        "ctx_conflict" => "Conflit",
        "ctx_success" => "Réussite",
        "ctx_uncertainty" => "Incertitude",
        "ctx_recognition" => "Reconnaissance",
        "ctx_threatened" => "Menacé",
        "ctx_change" => "Changement",
        "ctx_feedback" => "Feedback",

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

        "strategy_stress_label" => "Sous stress",
        "strategy_conflict_label" => "En conflit",
        "strategy_success_label" => "En réussite",
        "strategy_uncertainty_label" => "Dans l'incertitude",
        "strategy_recognition_label" => "Cherchant la reconnaissance",
        "strategy_threat_label" => "Se sentant menacé",
        "strategy_change_label" => "Face au changement",
        "strategy_feedback_label" => "Recevoir du feedback",

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

        "common_save" => "Enregistrer",
        "common_cancel" => "Annuler",
        "common_delete" => "Supprimer",
        "common_confirm" => "Confirmer",
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
        "compare_cat_ocean" => "OCÉAN",
        "compare_cat_reputation" => "Réputation",
        "compare_cat_motivation" => "Motivation",
        "compare_cat_patterns" => "Patterns",
        "compare_cat_bias" => "Biais",
        "person_self_score" => "Score de profil",
        "OCEAN volatility" => "Volatilité OCEAN",
        "Rep power struggle" => "Lutte de pouvoir (réputation)",
        "Only negative patterns" => "Patterns négatifs uniquement",
        "Low prediction accuracy" => "Faible précision prédictive",
        "compare_asymmetric" => "Bénéfice mutuel",
        "compare_benefit_more" => "bénéficie plus",
        "compare_balanced" => "Équilibré",
        "compare_ethics" => {
            "Ce sont des modèles probabilistes, pas des vérités absolues. Utilisez-les pour mieux comprendre, jamais pour manipuler."
        },

        // Scale bands
        "scale_strong" => "Fort",
        "scale_good" => "Bon",
        "scale_moderate" => "Moyen",
        "scale_friction" => "Friction",
        "scale_tension" => "Tension",

        // Relationships
        "rel_title" => "Relations",
        "rel_add" => "＋ Ajouter une relation",
        "rel_from" => "De",
        "rel_to" => "Vers",
        "rel_type" => "Type",
        "rel_notes" => "Notes",
        "rel_none" => "Aucune relation.",
        "rel_open_add" => "＋ Ajouter",
        "rel_close_add" => "− Annuler",
        "rel_search_placeholder" => "Rechercher une personne…",
        "rel_type_placeholder" => "ex: travaille_avec, dirige, amis, famille",
        "rel_confirm_delete" => "Supprimer cette relation ?",
        "rel_person_rel" => "Relations",

        // Timeline
        "tl_title" => "Chronologie",
        "tl_empty" => "Aucune entrée d'interaction.",

        // Tags


        // Tutorial
        "tut_welcome_title" => "Bienvenue sur PeopleModeler !",
        "tut_welcome_body" => "Cette application vous aide à modéliser et comprendre les personnes de votre vie en utilisant des cadres de personnalité comme l'OCEAN (Big Five), les motivations, les biais cognitifs et les schémas comportementaux.\n\nVous pouvez comparer des personnes côte à côte, suivre des prédictions dans le temps, cartographier les relations et explorer les scores de synergie.",
        "tut_people_title" => "Vos Personnes",
        "tut_people_body" => "La page principale montre toutes les personnes que vous avez créées. Utilisez la barre de recherche pour trouver quelqu'un, triez par nom / récent / score OCEAN, et cliquez sur le bouton + pour ajouter une nouvelle personne.",
        "tut_create_title" => "Créer une Personne",
        "tut_create_body" => "Le formulaire personne est divisé en sections : infos de base (nom, rôle, contexte), scores de personnalité OCEAN, motivations, biais cognitifs, dimensions de réputation et schémas comportementaux.\n\nChaque section capture une facette différente de la personnalité — remplissez ce que vous savez, laissez le reste vide.",
        "tut_ocean_title" => "Modèle OCEAN (Big Five)",
        "tut_ocean_body" => "L'OCEAN mesure la personnalité sur cinq dimensions de 1 à 10 :\n• Ouverture — curiosité vs. prudence\n• Conscience — organisation vs. flexibilité\n• Extraversion — sociabilité vs. solitude\n• Agréabilité — coopération vs. compétition\n• Névrosisme — sensibilité vs. stabilité émotionnelle\n\nCes scores alimentent le moteur de comparaison et aident à prédire le comportement.",
        "tut_mot_bias_title" => "Motivations & Biais",
        "tut_mot_bias_body" => "Les motivations capturent ce qui anime une personne — ses objectifs, ses peurs et ses valeurs (Réussite, Pouvoir, Affiliation, Sécurité, Autonomie, etc.).\n\nLes biais représentent des raccourcis mentaux qui influencent ses décisions (biais de confirmation, ancrage, excès de confiance, etc.). Ensemble, ils vous donnent une compréhension plus profonde de pourquoi les gens agissent comme ils le font.",
        "tut_rep_pattern_title" => "Réputation & Schémas",
        "tut_rep_pattern_body" => "Les scores de réputation capturent comment les autres perçoivent cette personne sur des échelles bipolaires (travailleur vs. paresseux, honnête vs. trompeur, etc.).\n\nLes schémas comportementaux vous permettent d'enregistrer comment elle réagit typiquement à des déclencheurs spécifiques (stress, critique, succès, conflit, etc.). Cela aide à anticiper ses réponses dans des situations futures.",
        "tut_compare_title" => "Comparaisons & Plus",
        "tut_compare_body" => "Une fois que vous avez au moins deux personnes, vous pouvez les comparer côte à côte pour voir leur score de synergie, leurs points de friction et leurs stratégies d'interaction.\n\nVous pouvez aussi suivre des prédictions (devinez un résultat, puis vérifiez si vous aviez raison), construire une carte des relations et journaliser les interactions sur une chronologie.",
        "tut_done_title" => "Prêt à Commencer !",
        "tut_done_body" => "Vous pouvez rejouer ce tutoriel à tout moment depuis la barre de navigation.\n\nConseils rapides :\n• Créez au moins deux personnes pour débloquer les comparaisons\n• Utilisez la page Sync pour sauvegarder vos données\n• Utilisez les tags pour organiser les personnes par groupe\n\nAllez-y et commencez à modéliser les personnes de votre monde !",

        // Common (tutorial)
        "common_next" => "Suivant →",
        "common_skip" => "Passer",
        "common_finish" => "Terminer",

        _ => key,
    }
}

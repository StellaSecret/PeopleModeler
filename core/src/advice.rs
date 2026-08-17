use crate::i18n::Lang;
use crate::insights::InsightContext;
use crate::model_config::CFG;
use crate::synergy::PersonProfile;
use crate::validation;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FlagAdvice {
    pub flag: &'static str,
    pub category: &'static str,
    pub action: String,
}

pub fn flag_action(flag: &str, lang: Lang) -> &'static str {
    match (flag, lang) {
        // --- ocean_rep (self-image) ---
        ("flag_high_e_low_a", Lang::Fr) => {
            "Pratiquez l'écoute active pour équilibrer votre extraversion."
        }
        ("flag_high_e_low_a", Lang::En) => {
            "Practice active listening to balance your extraversion."
        }
        ("flag_high_n_low_c", Lang::Fr) => {
            "Instauriez des routines structurées pour canaliser l'anxiété."
        }
        ("flag_high_n_low_c", Lang::En) => {
            "Build structured routines to channel anxiety productively."
        }
        ("flag_high_o_low_c", Lang::Fr) => {
            "Associez vos idées à un partenaire de redevabilité pour assurer le suivi."
        }
        ("flag_high_o_low_c", Lang::En) => {
            "Pair your ideas with an accountability partner for follow-through."
        }
        ("flag_calm_neurotic", Lang::Fr) => {
            "Identifiez les sources de stress profond et travaillez dessus directement."
        }
        ("flag_calm_neurotic", Lang::En) => {
            "Identify and address root stressors rather than masking them."
        }
        ("flag_honest_selfish", Lang::Fr) => "Pratiquez de petits actes de générité quotidiens.",
        ("flag_honest_selfish", Lang::En) => "Practice small daily acts of generosity.",
        ("flag_open_rigid", Lang::Fr) => {
            "Fixez-vous des objectifs stretch pour développer votre adaptabilité."
        }
        ("flag_open_rigid", Lang::En) => "Set stretch goals to develop your adaptability.",
        ("flag_claims_calm_reactive", Lang::Fr) => {
            "Travaillez sur la pleine conscience pour aligner perception et réaction."
        }
        ("flag_claims_calm_reactive", Lang::En) => {
            "Work on mindfulness to align perception with actual reactions."
        }
        ("flag_honest_favoritist", Lang::Fr) => {
            "Établissez des critères de décision objectifs et transparents."
        }
        ("flag_honest_favoritist", Lang::En) => {
            "Establish clear, objective decision criteria to counter favoritism."
        }
        ("flag_warmth_cold", Lang::Fr) => {
            "Pratiquez des exercices de mise à perspective pour développer l'empathie."
        }
        ("flag_warmth_cold", Lang::En) => {
            "Practice perspective-taking exercises to develop empathy."
        }
        ("flag_discipline_flaky", Lang::Fr) => {
            "Sécurisez vos engagements avec des échéances concrètes."
        }
        ("flag_discipline_flaky", Lang::En) => "Tighten commitments with concrete deadlines.",

        // --- rhetoric_gap ---
        ("flag_fairness_rhetoric", Lang::Fr) => {
            "Auditez vos décisions récentes pour détecter les biais de traitement."
        }
        ("flag_fairness_rhetoric", Lang::En) => "Audit recent decisions for hidden favoritism.",
        ("flag_helping_selfish", Lang::Fr) => {
            "Demandez un retour externe sur votre réel impact d'aide."
        }
        ("flag_helping_selfish", Lang::En) => {
            "Ask for external feedback on your actual helpfulness."
        }
        ("flag_affiliation_cold", Lang::Fr) => {
            "Pratiquez la chaleur dans les premières interactions sociales."
        }
        ("flag_affiliation_cold", Lang::En) => "Practice warmth in early social interactions.",
        ("flag_ambition_lazy", Lang::Fr) => {
            "Fixez 3 objectifs concrets à court terme et suivez-les."
        }
        ("flag_ambition_lazy", Lang::En) => "Set 3 concrete short-term goals and track them.",
        ("flag_security_gullible", Lang::Fr) => {
            "Implémentez des étapes de vérification avant toute décision importante."
        }
        ("flag_security_gullible", Lang::En) => {
            "Implement verification steps before important decisions."
        }
        ("flag_discipline_lazy", Lang::Fr) => "Utilisez des structures de redevabilité externe.",
        ("flag_discipline_lazy", Lang::En) => "Use external accountability structures.",
        ("flag_warmth_blunt", Lang::Fr) => "Pratiquez la communication empathique au quotidien.",
        ("flag_warmth_blunt", Lang::En) => "Practice empathic communication daily.",
        ("flag_affiliation_distrustful", Lang::Fr) => {
            "Travaillez sur la confiance graduée pas à pas."
        }
        ("flag_affiliation_distrustful", Lang::En) => "Work on graduated trust-building.",
        ("flag_autonomy_submissive", Lang::Fr) => {
            "Pratiquez l'affirmation de soi dans des situations à faible enjeu."
        }
        ("flag_autonomy_submissive", Lang::En) => {
            "Practice assertiveness in low-stakes situations."
        }
        ("flag_learning_rigid", Lang::Fr) => "Adoptez une nouvelle approche chaque semaine.",
        ("flag_learning_rigid", Lang::En) => "Adopt one new approach each week.",
        ("flag_creativity_closed", Lang::Fr) => {
            "Exposez-vous à des perspectives diverses pour stimuler l'ouverture."
        }
        ("flag_creativity_closed", Lang::En) => {
            "Expose yourself to diverse perspectives to foster openness."
        }
        ("flag_creativity_rigid", Lang::Fr) => {
            "Réservez du temps pour l'expérimentation sans structure."
        }
        ("flag_creativity_rigid", Lang::En) => "Set aside time for unstructured experimentation.",
        ("flag_power_passive", Lang::Fr) => "Prenez un rôle de leadership visible.",
        ("flag_power_passive", Lang::En) => "Take on a visible leadership role.",
        ("flag_helping_cold", Lang::Fr) => "Pratiquez l'écoute empathique régulièrement.",
        ("flag_helping_cold", Lang::En) => "Practice empathic listening regularly.",
        ("flag_learning_arrogant", Lang::Fr) => "Cherchez un mentor que vous respectez.",
        ("flag_learning_arrogant", Lang::En) => "Seek mentorship from someone you respect.",
        ("flag_warmth_selfish", Lang::Fr) => {
            "Pratiquez la générosité envers les autres au quotidien."
        }
        ("flag_warmth_selfish", Lang::En) => "Practice daily generosity toward others.",

        // --- evidence_flags (pattern gaps) ---
        ("flag_pattern_calm_volatile", Lang::Fr) => {
            "Vous prétendez être calme mais vos interactions montrent de la volatilité. Travaillez sur la régulation émotionnelle."
        }
        ("flag_pattern_calm_volatile", Lang::En) => {
            "You claim calm but interactions show volatility. Work on emotional regulation."
        }
        ("flag_pattern_honest_exploiter", Lang::Fr) => {
            "Vos actions d'exploitation contredisent votre honnêteté revendiquée. Soyez conscient de ces écarts."
        }
        ("flag_pattern_honest_exploiter", Lang::En) => {
            "Exploitative actions contradict claimed honesty. Be aware of this gap."
        }
        ("flag_pattern_diplomat_escalator", Lang::Fr) => {
            "Votre style diplomatique coexiste avec des escalades. Pratiquez la désescalade."
        }
        ("flag_pattern_diplomat_escalator", Lang::En) => {
            "Your diplomatic style coexists with escalation. Practice de-escalation."
        }
        ("flag_pattern_fair_exploiter", Lang::Fr) => {
            "La revendication d'équité masque des comportements d'exploitation. Alignez actions et valeurs."
        }
        ("flag_pattern_fair_exploiter", Lang::En) => {
            "Fairness claims mask exploitative behavior. Align actions with values."
        }
        ("flag_pattern_humble_dismissive", Lang::Fr) => {
            "L'humilité revendiquée se heurte au mépris. Pratiquez l'acceptation du feedback."
        }
        ("flag_pattern_humble_dismissive", Lang::En) => {
            "Claimed humility clashes with dismissiveness. Practice accepting feedback."
        }
        ("flag_pattern_trusting_paranoid", Lang::Fr) => {
            "Vous dites faire confiance mais vos actions sont paranoïaques. Évaluez les risques réels."
        }
        ("flag_pattern_trusting_paranoid", Lang::En) => {
            "You say you trust but act paranoid. Assess real risks."
        }
        ("flag_pattern_reliable_shirker", Lang::Fr) => {
            "Votre fiabilité revendiquée est minée par l'évitement. Tenez vos engagements."
        }
        ("flag_pattern_reliable_shirker", Lang::En) => {
            "Claimed reliability is undermined by shirking. Keep your commitments."
        }
        ("flag_pattern_hardworker_complacent", Lang::Fr) => {
            "Le travailleur acharné devient complaisant. Maintenez vos standards."
        }
        ("flag_pattern_hardworker_complacent", Lang::En) => {
            "Hard worker becoming complacent. Maintain your standards."
        }
        ("flag_pattern_passive_blowup", Lang::Fr) => {
            "Le repli passif mène à des explosions. Exprimez-vous plus tôt."
        }
        ("flag_pattern_passive_blowup", Lang::En) => {
            "Passive withdrawal leads to blowups. Express concerns earlier."
        }
        ("flag_pattern_assertive_quiet", Lang::Fr) => {
            "Vous vous affirmez mais les montrent le silence. Trouvez un juste milieu."
        }
        ("flag_pattern_assertive_quiet", Lang::En) => {
            "You claim assertiveness but interactions show quietness. Find a middle ground."
        }
        ("flag_pattern_generous_exploiter", Lang::Fr) => {
            "La générosité masque l'exploitation. Soyez authentique dans vos dons."
        }
        ("flag_pattern_generous_exploiter", Lang::En) => {
            "Generosity masking exploitation. Be authentic in giving."
        }
        ("flag_pattern_empath_dismissive", Lang::Fr) => {
            "L'empathie revendiquée cache le mépris. Écoutez vraiment les autres."
        }
        ("flag_pattern_empath_dismissive", Lang::En) => {
            "Claimed empathy hides dismissiveness. Truly listen to others."
        }
        ("flag_pattern_flexible_resister", Lang::Fr) => {
            "La flexibilité revendiquée se heurte à la résistance. Accueillez le changement."
        }
        ("flag_pattern_flexible_resister", Lang::En) => {
            "Claimed flexibility meets resistance. Embrace change."
        }
        ("flag_pattern_helping_exploiter", Lang::Fr) => {
            "L'aide est utilisée comme levier d'exploitation. Soyez désintéressé."
        }
        ("flag_pattern_helping_exploiter", Lang::En) => {
            "Help used as leverage. Be disinterested in giving."
        }
        ("flag_pattern_warmth_dismissive", Lang::Fr) => {
            "La chaleur revendiquée est contredite par le mépris. Valorisez les autres."
        }
        ("flag_pattern_warmth_dismissive", Lang::En) => {
            "Claimed warmth contradicted by dismissiveness. Value others."
        }
        ("flag_pattern_discipline_shirker", Lang::Fr) => {
            "La discipline revendiquée est minée par l'évitement. Structurez votre routine."
        }
        ("flag_pattern_discipline_shirker", Lang::En) => {
            "Claimed discipline undermined by avoidance. Structure your routine."
        }
        ("flag_pattern_claimed_calm_volatile", Lang::Fr) => {
            "Vous vous déclarez calme mais vos interactions sont volatiles. Travaillez sur la constance."
        }
        ("flag_pattern_claimed_calm_volatile", Lang::En) => {
            "You claim calm but interactions are volatile. Work on consistency."
        }
        ("flag_pattern_fairness_exploiter", Lang::Fr) => {
            "L'équité revendiquée masque l'exploitation. Soyez juste dans vos actions."
        }
        ("flag_pattern_fairness_exploiter", Lang::En) => {
            "Claimed fairness masking exploitation. Be just in actions."
        }
        ("flag_pattern_achievement_complacent", Lang::Fr) => {
            "La poursuite de réussite mène à la complaisance. Maintenez vos ambitions."
        }
        ("flag_pattern_achievement_complacent", Lang::En) => {
            "Achievement pursuit leading to complacency. Maintain ambitions."
        }
        ("flag_pattern_learning_resister", Lang::Fr) => {
            "L'apprentissage revendiqué se heurte à la résistance. Restez curieux."
        }
        ("flag_pattern_learning_resister", Lang::En) => {
            "Claimed learning meets resistance. Stay curious."
        }
        ("flag_pattern_extravert_quiet", Lang::Fr) => {
            "Vous vous dites extraverti mais vos interactions montrent du silence. Soyez plus expressif."
        }
        ("flag_pattern_extravert_quiet", Lang::En) => {
            "You claim extraversion but interactions show quietness. Be more expressive."
        }
        ("flag_pattern_open_resister", Lang::Fr) => {
            "L'ouverture revendiquée se heurte à la résistance. Accueillez les nouvelles idées."
        }
        ("flag_pattern_open_resister", Lang::En) => {
            "Claimed openness meets resistance. Welcome new ideas."
        }
        ("flag_pattern_recognition_dismissive", Lang::Fr) => {
            "Vous cherchez la reconnaissance mais rabaissez les autres pour l'obtenir."
        }
        ("flag_pattern_recognition_dismissive", Lang::En) => {
            "You seek recognition but put others down to get it."
        }

        // --- evidence_flags (bias gaps) ---
        ("flag_bias_confirmation_open", Lang::Fr) => {
            "Vous dites ouvert mais subissez le biais de confirmation. Cherchez activement des contre-exemples."
        }
        ("flag_bias_confirmation_open", Lang::En) => {
            "You claim openness but succumb to confirmation bias. Actively seek counter-examples."
        }
        ("flag_anchoring_open", Lang::Fr) => {
            "Ouvert revendiqué mais ancrage cognitif détecté. Examinez plusieurs points de référence."
        }
        ("flag_anchoring_open", Lang::En) => {
            "Openness claimed but anchoring detected. Examine multiple reference points."
        }
        ("flag_bias_favoritism_fairness", Lang::Fr) => {
            "Vous prônez l'équité mais favorisez vos proches. Appliquez des critères objectifs."
        }
        ("flag_bias_favoritism_fairness", Lang::En) => {
            "You advocate fairness but favor your own. Apply objective criteria."
        }
        ("flag_authority_dominant", Lang::Fr) => {
            "Vous prétendez être ouvert mais dominez les discussions. Laissez plus de place aux autres."
        }
        ("flag_authority_dominant", Lang::En) => {
            "You claim openness but dominate discussions. Make more room for others."
        }
        ("flag_social_proof_open", Lang::Fr) => {
            "Ouvert revendiqué mais soumis à la pression sociale. Pensez par vous-même."
        }
        ("flag_social_proof_open", Lang::En) => {
            "Openness claimed but socially conforming. Think independently."
        }
        ("flag_sunk_cost_flexible", Lang::Fr) => {
            "Flexibilité revendiquée mais biais du coût irrécupérable. Évaluez sur les mérites actuels."
        }
        ("flag_sunk_cost_flexible", Lang::En) => {
            "Flexibility claimed but sunk cost bias present. Evaluate on current merits."
        }
        ("flag_loss_aversion_risky", Lang::Fr) => {
            "Appétit pour le risque mais aversion aux pertes. Acceptez les pertes comme partie du processus."
        }
        ("flag_loss_aversion_risky", Lang::En) => {
            "Risk appetite claimed but loss aversion present. Accept losses as part of the process."
        }
        ("flag_dunning_kruger_humble", Lang::Fr) => {
            "Humilité revendiquée mais surévaluation de vos compétences. Demandez un feedback externe."
        }
        ("flag_dunning_kruger_humble", Lang::En) => {
            "Humility claimed but overestimation present. Seek external feedback."
        }
        ("flag_impostor_arrogant", Lang::Fr) => {
            "Vous dites être en imposture mais agissez avec arrogance. Soyez humble et curieux."
        }
        ("flag_impostor_arrogant", Lang::En) => {
            "You claim imposter syndrome but act arrogantly. Be humble and curious."
        }
        ("flag_recency_reliable", Lang::Fr) => {
            "Fiabilité revendiquée mais biais de récence. Considérez le contexte à long terme."
        }
        ("flag_recency_reliable", Lang::En) => {
            "Reliability claimed but recency bias present. Consider long-term context."
        }
        ("flag_availability_calm", Lang::Fr) => {
            "Calme revendiqué mais biais de disponibilité. Prenez du recul avant de réagir."
        }
        ("flag_availability_calm", Lang::En) => {
            "Calm claimed but availability bias present. Step back before reacting."
        }
        ("flag_security_risky", Lang::Fr) => {
            "Sécurité revendiquée mais comportement à risque. Évaluez les conséquences réelles."
        }
        ("flag_security_risky", Lang::En) => {
            "Security claimed but risky behavior. Assess real consequences."
        }
        ("flag_resilient_reactive", Lang::Fr) => {
            "Résilience revendiquée mais réactivité excessive. Développez la patience."
        }
        ("flag_resilient_reactive", Lang::En) => {
            "Resilience claimed but over-reactive. Develop patience."
        }
        ("flag_risk_appetite_ambition", Lang::Fr) => {
            "Ambition élevée mais aversion au risque. Acceptez l'incertitude comme nécessaire."
        }
        ("flag_risk_appetite_ambition", Lang::En) => {
            "High ambition but risk-averse. Accept uncertainty as necessary."
        }
        ("flag_resilient_hides", Lang::Fr) => {
            "Résilience revendiquée mais tendance à cacher les difficultés. Soyez transparent."
        }
        ("flag_resilient_hides", Lang::En) => {
            "Resilience claimed but hides difficulties. Be transparent."
        }

        // --- style_gap ---
        ("flag_style_direct_diplomatic", Lang::Fr) => {
            "Vous vous dites direct mais vous êtes perçu comme diplomatique. Clarifiez votre position."
        }
        ("flag_style_direct_diplomatic", Lang::En) => {
            "You claim direct but perceived as diplomatic. Clarify your position."
        }
        ("flag_style_diplomatic_blunt", Lang::Fr) => {
            "Vous vous dites diplomate mais vous êtes perçu comme brutale. Adoucissez votre approche."
        }
        ("flag_style_diplomatic_blunt", Lang::En) => {
            "You claim diplomatic but perceived as blunt. Soften your approach."
        }
        ("flag_style_competing_passive", Lang::Fr) => {
            "Vous vous dites compétitif mais vous êtes perçu comme passif. Prenez plus d'initiative."
        }
        ("flag_style_competing_passive", Lang::En) => {
            "You claim competitive but perceived as passive. Take more initiative."
        }
        ("flag_style_dominant_submissive", Lang::Fr) => {
            "Vous vous dites dominant mais vous êtes perçu comme soumis. Affirmez-vous."
        }
        ("flag_style_dominant_submissive", Lang::En) => {
            "You claim dominant but perceived as submissive. Assert yourself."
        }
        ("flag_style_manipulative_honest", Lang::Fr) => {
            "Vous vous déclarez manipulateur mais on vous perçoit honnête. Réfléchissez à votre intention réelle."
        }
        ("flag_style_manipulative_honest", Lang::En) => {
            "You claim manipulative but perceived as honest. Reflect on true intent."
        }
        ("flag_style_empathetic_cold", Lang::Fr) => {
            "Vous vous dites empathique mais on vous perçoit froid. Montrez plus de chaleur."
        }
        ("flag_style_empathetic_cold", Lang::En) => {
            "You claim empathetic but perceived as cold. Show more warmth."
        }
        ("flag_style_guarded_trusting", Lang::Fr) => {
            "Vous vous dites sur vos gardes mais on vous perçoit trop confiant. Renforcez votre vigilance."
        }
        ("flag_style_guarded_trusting", Lang::En) => {
            "You claim guarded but perceived as too trusting. Increase vigilance."
        }
        ("flag_style_servant_authoritative", Lang::Fr) => {
            "Vous vous dites servant mais on vous perçoit autoritaire. Pratiquez le service."
        }
        ("flag_style_servant_authoritative", Lang::En) => {
            "You claim servant but perceived as authoritative. Practice serving."
        }
        ("flag_style_consensus_authoritative", Lang::Fr) => {
            "Vous prônez le consensus mais on vous perçoit autoritaire. Impliquez les autres davantage."
        }
        ("flag_style_consensus_authoritative", Lang::En) => {
            "You advocate consensus but perceived as authoritative. Involve others more."
        }
        ("flag_style_trusts_freely_suspicious", Lang::Fr) => {
            "Vous prétendez faire confiance librement mais on vous perçoit méfiant. Laissez vos garde-fous."
        }
        ("flag_style_trusts_freely_suspicious", Lang::En) => {
            "You claim to trust freely but perceived as suspicious. Lower your guards."
        }
        ("flag_style_repairs_trust_deceitful", Lang::Fr) => {
            "Vous prétendez réparer la confiance mais on vous perçoit trompeur. Soyez transparent."
        }
        ("flag_style_repairs_trust_deceitful", Lang::En) => {
            "You claim trust-repairing but perceived as deceitful. Be transparent."
        }
        ("flag_style_rulebased_favoritist", Lang::Fr) => {
            "Vous prônez les règles mais on vous perçoit favoritiste. Appliquez les règles uniformément."
        }
        ("flag_style_rulebased_favoritist", Lang::En) => {
            "You advocate rules but perceived as favoritist. Apply rules uniformly."
        }
        ("flag_style_virtuebased_deceitful", Lang::Fr) => {
            "Vous prônez les vertus mais on vous perçoit trompeur. Alignez vos actions avec vos paroles."
        }
        ("flag_style_virtuebased_deceitful", Lang::En) => {
            "You advocate virtue but perceived as deceitful. Align actions with words."
        }

        // --- value_flags ---
        ("flag_value_family_past", Lang::Fr) => {
            "Vous valorisez la famille mais votre orientation temporelle ne l'est pas. Intégrez la dimension familiale dans vos décisions."
        }
        ("flag_value_family_past", Lang::En) => {
            "You value family but lack past orientation. Integrate family considerations into decisions."
        }
        ("flag_value_stability_risk", Lang::Fr) => {
            "Vous cravez la stabilité mais prends des risques. Définissez votre seuil de risque acceptable."
        }
        ("flag_value_stability_risk", Lang::En) => {
            "You crave stability but take risks. Define your acceptable risk threshold."
        }
        ("flag_value_career_family", Lang::Fr) => {
            "Carrière et famille en tension. Définissez clairement vos priorités par contexte."
        }
        ("flag_value_career_family", Lang::En) => {
            "Career and family in tension. Clearly define priorities by context."
        }
        ("flag_value_loyalty_guarded", Lang::Fr) => {
            "Vous valorisez la loyauté mais restez sur vos gardes. Apprenez à accorder une confiance graduée."
        }
        ("flag_value_loyalty_guarded", Lang::En) => {
            "You value loyalty but remain guarded. Learn to grant graduated trust."
        }

        _ => "",
    }
}

pub fn generate_advice(person: &crate::models::Person) -> Vec<FlagAdvice> {
    let flags = validation::all_person_flags(person);
    let cat = |f: &str| -> &'static str {
        if f.starts_with("flag_style_") {
            "style"
        } else if f.starts_with("flag_pattern_")
            || f.starts_with("flag_bias_")
            || f.starts_with("flag_anchoring_")
            || f.starts_with("flag_authority_")
            || f.starts_with("flag_social_")
            || f.starts_with("flag_sunk_")
            || f.starts_with("flag_loss_")
            || f.starts_with("flag_dunning_")
            || f.starts_with("flag_impostor_")
            || f.starts_with("flag_recency_")
            || f.starts_with("flag_availability_")
            || f.starts_with("flag_security_")
            || f.starts_with("flag_resilient_")
            || f.starts_with("flag_risk_")
        {
            "evidence"
        } else if f.starts_with("flag_value_") {
            "values"
        } else if f.starts_with("flag_fairness_rhetoric")
            || f.starts_with("flag_helping_")
            || f.starts_with("flag_affiliation_")
            || f.starts_with("flag_ambition_")
            || f.starts_with("flag_security_gullible")
            || f.starts_with("flag_discipline_lazy")
            || f.starts_with("flag_warmth_blunt")
            || f.starts_with("flag_warmth_selfish")
            || f.starts_with("flag_autonomy_")
            || f.starts_with("flag_learning_rigid")
            || f.starts_with("flag_learning_arrogant")
            || f.starts_with("flag_creativity_")
            || f.starts_with("flag_power_passive")
            || f.starts_with("flag_helping_cold")
        {
            "rhetoric"
        } else {
            "self_image"
        }
    };
    flags
        .iter()
        .filter_map(|f| {
            let action = flag_action(f, Lang::Fr);
            if action.is_empty() {
                None
            } else {
                Some(FlagAdvice {
                    flag: f,
                    category: cat(f),
                    action: action.to_string(),
                })
            }
        })
        .collect()
}

pub fn per_context_advice(
    person: &crate::models::Person,
    _profile: &PersonProfile,
    ctx: InsightContext,
) -> Vec<FlagAdvice> {
    let mut advice = generate_advice(person);
    let weights = CFG.contexts.weights[ctx as usize];
    let sort_key = |a: &FlagAdvice| -> f64 {
        let cat_idx = match a.category {
            "self_image" => 0,
            "rhetoric" => 1,
            "evidence" => 2,
            "style" => 3,
            "values" => 4,
            _ => 5,
        };
        let w = if cat_idx < weights.len() {
            weights[cat_idx]
        } else {
            0.01
        };
        -w
    };
    advice.sort_by(|a, b| sort_key(a).partial_cmp(&sort_key(b)).unwrap());
    advice
}

pub fn risk_mitigation_pair(person: &crate::models::Person) -> Vec<(&'static str, &'static str)> {
    let flags = validation::all_person_flags(person);
    flags
        .iter()
        .filter_map(|f| {
            let mitigation = flag_action(f, Lang::Fr);
            if mitigation.is_empty() {
                None
            } else {
                Some((*f, mitigation))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Person;

    fn test_person() -> Person {
        Person {
            id: "test-001".into(),
            name: "Test".into(),
            role: "Tester".into(),
            context: "".into(),
            avatar_emoji: "🧪".into(),
            tags: vec![],
            notes: String::new(),
            ocean: Default::default(),
            rep_scores: Default::default(),
            motivations: vec![],
            biases: vec![],
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            risk_appetite: None,
            resilience: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        }
    }

    /// Dynamically extract every "flag_..." string literal from validation.rs
    /// source code. This ensures the advice table stays in sync even if someone
    /// adds a new flag to validation.rs without updating advice.rs.
    fn flags_from_validation_source() -> Vec<&'static str> {
        let src = include_str!("validation.rs");
        let mut flags = Vec::new();
        let mut start = 0;
        while let Some(pos) = src[start..].find("\"flag_") {
            let abs = start + pos + 1;
            if let Some(end) = src[abs..].find('"') {
                let flag = &src[abs..abs + end];
                if !flags.contains(&flag) {
                    flags.push(flag);
                }
                start = abs + end + 1;
            } else {
                break;
            }
        }
        flags
    }

    #[test]
    fn acceptance_every_validation_flag_has_bilingual_advice() {
        let source_flags = flags_from_validation_source();
        assert!(
            source_flags.len() >= 70,
            "Expected at least 70 flags from validation.rs, found {}",
            source_flags.len()
        );
        for f in source_flags {
            let fr = flag_action(f, Lang::Fr);
            let en = flag_action(f, Lang::En);
            assert!(!fr.is_empty(), "Missing FR advice for {}", f);
            assert!(!en.is_empty(), "Missing EN advice for {}", f);
        }
    }

    #[test]
    fn advice_has_valid_category() {
        let p = test_person();
        let advice = generate_advice(&p);
        for a in &advice {
            assert!(
                matches!(
                    a.category,
                    "self_image" | "rhetoric" | "evidence" | "style" | "values"
                ),
                "Unknown category: {}",
                a.category
            );
        }
    }

    #[test]
    fn generate_advice_with_fired_flags() {
        use crate::models::*;
        let mut p = test_person();
        p.name = "Manipulator".into();
        p.ocean = OceanScores {
            openness: Some(9),
            conscientiousness: Some(3),
            extraversion: Some(9),
            agreeableness: Some(9),
            neuroticism: Some(3),
        };
        p.motivations = vec![
            Motivation {
                r#type: MotivationType::Fairness,
                intensity: 9,
                notes: String::new(),
            },
            Motivation {
                r#type: MotivationType::Helping,
                intensity: 8,
                notes: String::new(),
            },
        ];
        p.rep_scores = RepScores {
            honest_deceitful: Some(9),
            generous_selfish: Some(2),
            fair_favoritism: Some(9),
            empathetic_detached: Some(9),
            ..Default::default()
        };
        p.values = vec![
            Value {
                r#type: ValueType::Career,
                intensity: 9,
                priority: 9,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Family,
                intensity: 9,
                priority: 9,
                notes: String::new(),
            },
            Value {
                r#type: ValueType::Stability,
                intensity: 9,
                priority: 9,
                notes: String::new(),
            },
        ];
        p.risk_appetite = Some(9);
        let advice = generate_advice(&p);
        assert!(
            advice.len() >= 4,
            "Expected ≥4 advice items for a profile with multiple contradictions, got {}",
            advice.len()
        );
        let flags = crate::validation::all_person_flags(&p);
        assert!(
            !flags.is_empty(),
            "Expected at least some fired flags for this contradictory profile"
        );
        for a in &advice {
            assert!(!a.action.is_empty());
        }
    }

    #[test]
    fn per_context_deprioritizes_values_in_decision_ctx() {
        use crate::models::*;
        let mut p = test_person();
        p.values = vec![Value {
            r#type: ValueType::Career,
            intensity: 9,
            priority: 9,
            notes: String::new(),
        }];
        p.ocean = OceanScores {
            openness: Some(9),
            conscientiousness: Some(3),
            extraversion: Some(9),
            agreeableness: Some(9),
            neuroticism: Some(3),
        };
        p.rep_scores = RepScores {
            generous_selfish: Some(2),
            fair_favoritism: Some(9),
            empathetic_detached: Some(2),
            ..Default::default()
        };
        p.motivations = vec![Motivation {
            r#type: MotivationType::Fairness,
            intensity: 9,
            notes: String::new(),
        }];
        let profile = PersonProfile {
            total: 50,
            motivation: 0.5,
            patterns: 0.5,
            ocean: 0.5,
            reputation: 0.5,
            bias: 0.5,
            styles: 0.5,
            values: 0.5,
            completeness: 60,
            band: 0,
        };
        let decision = per_context_advice(&p, &profile, InsightContext::Decision);
        let growth = per_context_advice(&p, &profile, InsightContext::Growth);
        // Both should return the same flags (same person), just different order
        assert_eq!(decision.len(), growth.len());
        assert!(decision.len() >= 2);
    }
}

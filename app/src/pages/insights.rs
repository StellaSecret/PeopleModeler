use dioxus::prelude::*;
use peoplemodeler_core::models::{BehaviorTrigger, Person};

pub(crate) const ALL_TRIGGERS: [BehaviorTrigger; 9] = [
    BehaviorTrigger::Stress,
    BehaviorTrigger::Conflict,
    BehaviorTrigger::Success,
    BehaviorTrigger::Uncertainty,
    BehaviorTrigger::Recognition,
    BehaviorTrigger::Threatened,
    BehaviorTrigger::Change,
    BehaviorTrigger::Feedback,
    BehaviorTrigger::Injustice,
];

use crate::Route;
use crate::db;
use crate::i18n::Lang;

pub(crate) struct InsightOutput {
    pub top: String,
    pub secondary: Vec<String>,
    pub has_secondary: bool,
}

#[component]
pub fn Insights() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let persons = db::all_persons();
    let title = crate::i18n::tr("insights_title", lang());
    let hint = crate::i18n::tr("insights_select_person", lang());
    let empty = crate::i18n::tr("no_people_insights", lang());
    rsx! {
        div { class: "page",
            h2 { "{title}" }
            p { "{hint}" }
            if persons.is_empty() {
                div { class: "empty-state",
                    div { class: "empty-icon", "📊" }
                    p { "{empty}" }
                }
            } else {
                div { class: "person-list",
                    for p in &persons {
                        Link {
                            to: Route::PersonDetail { id: p.id.clone() },
                            class: "person-card",
                            span { "{p.avatar_emoji} {p.name}" }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn trigger_label(t: &BehaviorTrigger, lang: Lang) -> &'static str {
    match t {
        BehaviorTrigger::Stress => crate::i18n::tr("strategy_stress_label", lang),
        BehaviorTrigger::Conflict => crate::i18n::tr("strategy_conflict_label", lang),
        BehaviorTrigger::Success => crate::i18n::tr("strategy_success_label", lang),
        BehaviorTrigger::Uncertainty => crate::i18n::tr("strategy_uncertainty_label", lang),
        BehaviorTrigger::Recognition => crate::i18n::tr("strategy_recognition_label", lang),
        BehaviorTrigger::Threatened => crate::i18n::tr("strategy_threat_label", lang),
        BehaviorTrigger::Change => crate::i18n::tr("strategy_change_label", lang),
        BehaviorTrigger::Feedback => crate::i18n::tr("strategy_feedback_label", lang),
        BehaviorTrigger::Injustice => crate::i18n::tr("strategy_injustice_label", lang),
    }
}

pub(crate) fn generate_insight(p: &Person, trigger: &BehaviorTrigger, lang: Lang) -> InsightOutput {
    let all_recs = match trigger {
        BehaviorTrigger::Stress => stress_strategy(p, lang),
        BehaviorTrigger::Conflict => conflict_strategy(p, lang),
        BehaviorTrigger::Success => success_strategy(p, lang),
        BehaviorTrigger::Uncertainty => uncertainty_strategy(p, lang),
        BehaviorTrigger::Recognition => recognition_strategy(p, lang),
        BehaviorTrigger::Threatened => threatened_strategy(p, lang),
        BehaviorTrigger::Change => change_strategy(p, lang),
        BehaviorTrigger::Feedback => feedback_strategy(p, lang),
        BehaviorTrigger::Injustice => injustice_strategy(p, lang),
    };

    let top = build_top_rec(p, trigger, &all_recs, lang);
    let has_secondary = all_recs.len() > 1;
    InsightOutput {
        top,
        secondary: all_recs,
        has_secondary,
    }
}

fn build_top_rec(p: &Person, trigger: &BehaviorTrigger, recs: &[String], lang: Lang) -> String {
    let tl = trigger_label(trigger, lang);
    let base = recs.first().map_or(String::new(), |s| s.clone());
    let role_info = if !p.role.is_empty() || !p.context.is_empty() {
        let mut parts = vec![];
        if !p.role.is_empty() {
            parts.push(p.role.as_str());
        }
        if !p.context.is_empty() {
            parts.push(p.context.as_str());
        }
        format!(" ({})", parts.join(", "))
    } else {
        String::new()
    };
    crate::i18n::tr("strategy_when", lang)
        .replace("{name}", &format!("{}{}", p.name, role_info))
        .replace("{trigger}", &tl.to_lowercase())
        .replace("{advice}", &base)
}

fn stress_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_n", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_e", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_stress_low_e", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_c", lang).into());
    }
    if p.ocean.agreeableness.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_stress_low_a", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_stress_low_c", lang).into());
    }
    if p.ocean.openness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_o", lang).into());
    }
    if let Some(m) = p.top_motivation() {
        match m.r#type {
            peoplemodeler_core::models::MotivationType::Power => {
                if peoplemodeler_core::validation::ambition_lazy_gap(&p.motivations, &p.rep_scores)
                {
                    s.push(crate::i18n::tr("strategy_stress_ambition_rhetoric", lang).into())
                } else {
                    s.push(crate::i18n::tr("strategy_stress_power", lang).into())
                }
            }
            peoplemodeler_core::models::MotivationType::Security => {
                if peoplemodeler_core::validation::security_gullible_gap(
                    &p.motivations,
                    &p.rep_scores,
                ) {
                    s.push(crate::i18n::tr("strategy_stress_security_rhetoric", lang).into())
                } else {
                    s.push(crate::i18n::tr("strategy_stress_security", lang).into())
                }
            }
            _ => {}
        }
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_stress_fallback", lang).into());
    }
    s
}

fn conflict_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.agreeableness.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_conflict_low_a", lang).into());
    }
    if p.ocean.agreeableness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_a", lang).into());
    }
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_n", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_e", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_c", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_conflict_low_e", lang).into());
    }
    if peoplemodeler_core::validation::affiliation_cold_gap(&p.motivations, &p.rep_scores) {
        s.push(crate::i18n::tr("strategy_conflict_affiliation_rhetoric", lang).into());
    }
    if peoplemodeler_core::validation::affiliation_distrustful_gap(&p.motivations, &p.rep_scores) {
        s.push(crate::i18n::tr("strategy_conflict_affiliation_trust_rhetoric", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_conflict_fallback", lang).into());
    }
    s
}

fn success_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.openness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_success_high_o", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_success_high_c", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_success_low_e", lang).into());
    }
    if p.ocean.agreeableness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_success_high_a", lang).into());
    }
    if let Some(m) = p.top_motivation() {
        if m.r#type == peoplemodeler_core::models::MotivationType::Recognition && m.intensity >= 7 {
            if peoplemodeler_core::validation::ambition_lazy_gap(&p.motivations, &p.rep_scores) {
                s.push(crate::i18n::tr("strategy_success_ambition_rhetoric", lang).into());
            } else {
                s.push(crate::i18n::tr("strategy_success_recognition", lang).into());
            }
        }
        if m.r#type == peoplemodeler_core::models::MotivationType::Power && m.intensity >= 7 {
            if peoplemodeler_core::validation::ambition_lazy_gap(&p.motivations, &p.rep_scores) {
                s.push(crate::i18n::tr("strategy_success_ambition_rhetoric", lang).into());
            } else {
                s.push(crate::i18n::tr("strategy_success_power", lang).into());
            }
        }
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_success_fallback", lang).into());
    }
    s
}

fn uncertainty_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_n", lang).into());
    }
    if p.ocean.neuroticism.is_some_and(|v| v <= 3) {
        s.push(crate::i18n::tr("strategy_uncertainty_low_n", lang).into());
    }
    if p.ocean.openness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_o", lang).into());
    }
    if p.ocean.openness.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_uncertainty_low_o", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_c", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_e", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_uncertainty_fallback", lang).into());
    }
    s
}

fn recognition_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if let Some(m) = p.top_motivation()
        && m.r#type == peoplemodeler_core::models::MotivationType::Recognition
    {
        match m.intensity {
            8.. => s.push(crate::i18n::tr("strategy_recognition_high", lang).into()),
            5.. => s.push(crate::i18n::tr("strategy_recognition_mid", lang).into()),
            _ => s.push(crate::i18n::tr("strategy_recognition_low", lang).into()),
        }
    }
    if p.ocean.extraversion.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_recognition_high_e", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_recognition_low_e", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_recognition_fallback", lang).into());
    }
    s
}

fn threatened_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.agreeableness.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_threat_low_a", lang).into());
    }
    if p.ocean.agreeableness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_threat_high_a", lang).into());
    }
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_threat_high_n", lang).into());
    }
    if let Some(m) = p.top_motivation()
        && m.r#type == peoplemodeler_core::models::MotivationType::Power
        && m.intensity >= 7
    {
        s.push(crate::i18n::tr("strategy_threat_power", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_threat_fallback", lang).into());
    }
    s
}

fn change_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_change_high_n", lang).into());
    }
    if p.ocean.neuroticism.is_some_and(|v| v <= 3) {
        s.push(crate::i18n::tr("strategy_change_low_n", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_change_high_c", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_change_low_e", lang).into());
    }
    if p.ocean.openness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_change_high_o", lang).into());
    }
    if peoplemodeler_core::validation::discipline_lazy_gap(&p.ocean, &p.rep_scores) {
        s.push(crate::i18n::tr("strategy_change_discipline_rhetoric", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_change_fallback", lang).into());
    }
    s
}

fn feedback_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_feedback_high_n", lang).into());
    }
    if p.ocean.neuroticism.is_some_and(|v| v <= 3) {
        s.push(crate::i18n::tr("strategy_feedback_low_n", lang).into());
    }
    if p.ocean.agreeableness.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_feedback_low_a", lang).into());
    }
    if p.ocean.extraversion.is_some_and(|v| v <= 4) {
        s.push(crate::i18n::tr("strategy_feedback_low_e", lang).into());
    }
    if p.ocean.conscientiousness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_feedback_high_c", lang).into());
    }
    if peoplemodeler_core::validation::helping_selfish_gap(&p.motivations, &p.rep_scores) {
        s.push(crate::i18n::tr("strategy_feedback_helping_rhetoric", lang).into());
    }
    if peoplemodeler_core::validation::warmth_blunt_gap(&p.ocean, &p.rep_scores) {
        s.push(crate::i18n::tr("strategy_feedback_warmth_rhetoric", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_feedback_fallback", lang).into());
    }
    s
}

fn injustice_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.agreeableness.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_injustice_high_a", lang).into());
    }
    if p.ocean.neuroticism.is_some_and(|v| v >= 7) {
        s.push(crate::i18n::tr("strategy_injustice_high_n", lang).into());
    }
    if peoplemodeler_core::validation::fairness_rhetoric_gap(&p.motivations, &p.rep_scores) {
        s.push(crate::i18n::tr("strategy_injustice_fairness_rhetoric", lang).into());
    } else if let Some(m) = p.top_motivation()
        && m.r#type == peoplemodeler_core::models::MotivationType::Fairness
        && m.intensity >= 6
    {
        s.push(crate::i18n::tr("strategy_injustice_fairness", lang).into());
    }
    if let Some(m) = p.top_motivation()
        && m.r#type == peoplemodeler_core::models::MotivationType::Power
        && m.intensity >= 7
    {
        if peoplemodeler_core::validation::ambition_lazy_gap(&p.motivations, &p.rep_scores) {
            s.push(crate::i18n::tr("strategy_injustice_ambition_rhetoric", lang).into());
        } else {
            s.push(crate::i18n::tr("strategy_injustice_power", lang).into());
        }
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_injustice_fallback", lang).into());
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::*;

    fn p(name: &str) -> Person {
        Person {
            id: "id".into(),
            name: name.into(),
            role: "Engineer".into(),
            context: "Core".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        }
    }

    fn with_ocean(mut person: Person, o: u8, c: u8, e: u8, a: u8, n: u8) -> Person {
        person.ocean.openness = Some(o);
        person.ocean.conscientiousness = Some(c);
        person.ocean.extraversion = Some(e);
        person.ocean.agreeableness = Some(a);
        person.ocean.neuroticism = Some(n);
        person
    }

    fn with_mot(mut person: Person, mt: MotivationType, intensity: u8) -> Person {
        person.motivations.push(Motivation {
            r#type: mt,
            intensity,
            notes: String::new(),
        });
        person
    }

    #[test]
    fn trigger_label_all_variants() {
        let lang = Lang::En;
        assert!(!trigger_label(&BehaviorTrigger::Stress, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Conflict, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Success, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Uncertainty, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Recognition, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Threatened, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Change, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Feedback, lang).is_empty());
        assert!(!trigger_label(&BehaviorTrigger::Injustice, lang).is_empty());
    }

    #[test]
    fn stress_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
        assert!(s.iter().any(|x| !x.is_empty()));
    }

    #[test]
    fn stress_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn conflict_strategy_low_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn conflict_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn success_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn success_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn uncertainty_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn recognition_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn recognition_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn threatened_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn change_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = change_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn change_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn feedback_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = feedback_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn feedback_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn injustice_strategy_high_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 8, 5);
        let s = injustice_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn injustice_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn stress_strategy_french() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = stress_strategy(&p, Lang::Fr);
        assert!(!s.is_empty());
    }

    #[test]
    fn build_top_rec_empty_recs() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let result = build_top_rec(&p, &BehaviorTrigger::Stress, &[], Lang::En);
        assert!(!result.is_empty());
    }

    #[test]
    fn build_top_rec_with_recs() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let recs = vec!["Some advice".to_string()];
        let result = build_top_rec(&p, &BehaviorTrigger::Stress, &recs, Lang::En);
        assert!(!result.is_empty());
    }

    #[test]
    fn generate_insight_stress() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let out = generate_insight(&p, &BehaviorTrigger::Stress, Lang::En);
        assert!(!out.top.is_empty());
        assert!(!out.secondary.is_empty());
    }

    #[test]
    fn generate_insight_conflict() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let out = generate_insight(&p, &BehaviorTrigger::Conflict, Lang::En);
        assert!(!out.top.is_empty());
    }

    #[test]
    fn generate_insight_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let out = generate_insight(&p, &BehaviorTrigger::Stress, Lang::En);
        assert!(!out.top.is_empty());
    }

    #[test]
    fn success_strategy_recognition_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            8,
        );
        let s = success_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn success_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = success_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_security_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Security,
            8,
        );
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn recognition_strategy_recognition_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            9,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn recognition_strategy_mid_intensity() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            6,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn recognition_strategy_low_intensity() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            3,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn threatened_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = threatened_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn injustice_strategy_fairness_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Fairness,
            7,
        );
        let s = injustice_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn injustice_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = injustice_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_low_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 2);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_low_o() {
        let p = with_ocean(p("A"), 3, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn conflict_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn conflict_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn conflict_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn change_strategy_low_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 2);
        let s = change_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn change_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn change_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn change_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn feedback_strategy_low_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 2);
        let s = feedback_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn feedback_strategy_low_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn feedback_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn feedback_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_low_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_low_c() {
        let p = with_ocean(p("A"), 5, 3, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn stress_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn success_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn success_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn success_strategy_high_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 8, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_low_o_val() {
        let p = with_ocean(p("A"), 3, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn uncertainty_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn recognition_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn threatened_strategy_high_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 8, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn threatened_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = threatened_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }

    #[test]
    fn conflict_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = conflict_strategy(&p, Lang::En);
        assert!(!s.is_empty());
    }
}

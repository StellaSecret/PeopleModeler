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
    use crate::i18n::tr;
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

    fn has(s: &[String], key: &'static str, lang: Lang) -> bool {
        s.contains(&tr(key, lang).to_string())
    }

    #[test]
    fn stress_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_high_n", Lang::En));
    }

    #[test]
    fn stress_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_high_e", Lang::En));
    }

    #[test]
    fn stress_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_low_e", Lang::En));
    }

    #[test]
    fn stress_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_fallback", Lang::En));
    }

    #[test]
    fn conflict_strategy_low_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_low_a", Lang::En));
    }

    #[test]
    fn conflict_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_fallback", Lang::En));
    }

    #[test]
    fn success_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_high_o", Lang::En));
    }

    #[test]
    fn success_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_fallback", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_high_n", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_fallback", Lang::En));
    }

    #[test]
    fn recognition_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_recognition_fallback", Lang::En));
    }

    #[test]
    fn recognition_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_recognition_high_e", Lang::En));
    }

    #[test]
    fn threatened_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_threat_fallback", Lang::En));
    }

    #[test]
    fn change_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_high_n", Lang::En));
    }

    #[test]
    fn change_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_fallback", Lang::En));
    }

    #[test]
    fn feedback_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_high_n", Lang::En));
    }

    #[test]
    fn feedback_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_fallback", Lang::En));
    }

    #[test]
    fn injustice_strategy_high_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 8, 5);
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_injustice_high_a", Lang::En));
    }

    #[test]
    fn injustice_strategy_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_injustice_fallback", Lang::En));
    }

    #[test]
    fn stress_strategy_french() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = stress_strategy(&p, Lang::Fr);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_high_n", Lang::Fr));
    }

    #[test]
    fn build_top_rec_empty_recs() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let result = build_top_rec(&p, &BehaviorTrigger::Stress, &[], Lang::En);
        assert!(result.contains("A"));
        assert!(result.contains("stress"));
    }

    #[test]
    fn build_top_rec_with_recs() {
        let mut person = with_ocean(p("A"), 5, 5, 5, 5, 5);
        person.role = String::new();
        person.context = String::new();
        let recs = vec!["Some advice".to_string()];
        let result = build_top_rec(&person, &BehaviorTrigger::Stress, &recs, Lang::En);
        assert!(result.contains("Some advice"));
    }

    #[test]
    fn build_top_rec_with_role() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let recs = vec!["Do this".to_string()];
        let result = build_top_rec(&p, &BehaviorTrigger::Stress, &recs, Lang::En);
        assert!(result.contains("Do this"));
        assert!(result.contains("Engineer"));
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
        assert!(has(&out.secondary, "strategy_conflict_low_a", Lang::En));
    }

    #[test]
    fn generate_insight_fallback() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let out = generate_insight(&p, &BehaviorTrigger::Stress, Lang::En);
        assert!(out.top.contains("A"));
        assert_eq!(out.secondary.len(), 1);
        assert!(!out.has_secondary);
    }

    #[test]
    fn success_strategy_recognition_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            8,
        );
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_recognition", Lang::En));
    }

    #[test]
    fn success_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_power", Lang::En));
    }

    #[test]
    fn stress_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_power", Lang::En));
    }

    #[test]
    fn stress_strategy_security_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Security,
            8,
        );
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_security", Lang::En));
    }

    #[test]
    fn recognition_strategy_recognition_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            9,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 2);
        assert!(has(&s, "strategy_recognition_high", Lang::En));
        assert!(has(&s, "strategy_recognition_high_e", Lang::En));
    }

    #[test]
    fn recognition_strategy_mid_intensity() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            6,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_mid", Lang::En));
    }

    #[test]
    fn recognition_strategy_low_intensity() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            3,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_low", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_8_is_high() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            8,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_high", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_7_is_mid() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            7,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_mid", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_4_is_low() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            4,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_low", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_5_is_mid() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            5,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_mid", Lang::En));
    }

    #[test]
    fn threatened_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = threatened_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_threat_power", Lang::En));
    }

    #[test]
    fn threatened_strategy_power_mot_low_intensity() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert!(!has(&s, "strategy_threat_power", Lang::En));
    }

    #[test]
    fn injustice_strategy_fairness_mot() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Fairness,
            7,
        );
        let s = injustice_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_injustice_fairness", Lang::En));
    }

    #[test]
    fn injustice_strategy_fairness_mot_low_intensity() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Fairness,
            4,
        );
        let s = injustice_strategy(&p, Lang::En);
        assert!(!has(&s, "strategy_injustice_fairness", Lang::En));
    }

    #[test]
    fn injustice_strategy_power_mot() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 8);
        let s = injustice_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_injustice_power", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_low_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 2);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_low_n", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_low_o() {
        let p = with_ocean(p("A"), 3, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_low_o", Lang::En));
    }

    #[test]
    fn conflict_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_conflict_high_e", Lang::En));
    }

    #[test]
    fn conflict_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_conflict_high_c", Lang::En));
    }

    #[test]
    fn conflict_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_conflict_low_e", Lang::En));
    }

    #[test]
    fn change_strategy_low_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 2);
        let s = change_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_change_low_n", Lang::En));
    }

    #[test]
    fn change_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_change_high_c", Lang::En));
    }

    #[test]
    fn change_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_change_low_e", Lang::En));
    }

    #[test]
    fn change_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_change_high_o", Lang::En));
    }

    #[test]
    fn feedback_strategy_low_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 2);
        let s = feedback_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_feedback_low_n", Lang::En));
    }

    #[test]
    fn feedback_strategy_low_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_feedback_low_a", Lang::En));
    }

    #[test]
    fn feedback_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_feedback_low_e", Lang::En));
    }

    #[test]
    fn feedback_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_feedback_high_c", Lang::En));
    }

    #[test]
    fn stress_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_high_c", Lang::En));
    }

    #[test]
    fn stress_strategy_low_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 3, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_low_a", Lang::En));
    }

    #[test]
    fn stress_strategy_low_c() {
        let p = with_ocean(p("A"), 5, 3, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_low_c", Lang::En));
    }

    #[test]
    fn stress_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_high_o", Lang::En));
    }

    #[test]
    fn success_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_success_high_c", Lang::En));
    }

    #[test]
    fn success_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_success_low_e", Lang::En));
    }

    #[test]
    fn success_strategy_high_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 8, 5);
        let s = success_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_success_high_a", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_high_o() {
        let p = with_ocean(p("A"), 8, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_uncertainty_high_o", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_low_o_val() {
        let p = with_ocean(p("A"), 3, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_uncertainty_low_o", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_high_c() {
        let p = with_ocean(p("A"), 5, 8, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_uncertainty_high_c", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_high_e() {
        let p = with_ocean(p("A"), 5, 5, 8, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_uncertainty_high_e", Lang::En));
    }

    #[test]
    fn recognition_strategy_low_e() {
        let p = with_ocean(p("A"), 5, 5, 3, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_recognition_low_e", Lang::En));
    }

    #[test]
    fn threatened_strategy_high_a() {
        let p = with_ocean(p("A"), 5, 5, 5, 8, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_threat_high_a", Lang::En));
    }

    #[test]
    fn threatened_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = threatened_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_threat_high_n", Lang::En));
    }

    #[test]
    fn conflict_strategy_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let s = conflict_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_conflict_high_n", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_high_n", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_e7() {
        let p = with_ocean(p("A"), 5, 5, 7, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_high_e", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_e4() {
        let p = with_ocean(p("A"), 5, 5, 4, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_low_e", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_c7() {
        let p = with_ocean(p("A"), 5, 7, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_high_c", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_a4() {
        let p = with_ocean(p("A"), 5, 5, 5, 4, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_low_a", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_c4() {
        let p = with_ocean(p("A"), 5, 4, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_low_c", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_o7() {
        let p = with_ocean(p("A"), 7, 5, 5, 5, 5);
        let s = stress_strategy(&p, Lang::En);
        assert!(has(&s, "strategy_stress_high_o", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_power_i7() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 7);
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_power", Lang::En));
    }

    #[test]
    fn stress_strategy_boundary_security_i7() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Security,
            7,
        );
        let s = stress_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_stress_security", Lang::En));
    }

    #[test]
    fn conflict_strategy_boundary_a4() {
        let p = with_ocean(p("A"), 5, 5, 5, 4, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_low_a", Lang::En));
    }

    #[test]
    fn conflict_strategy_boundary_a7() {
        let p = with_ocean(p("A"), 5, 5, 5, 7, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_high_a", Lang::En));
    }

    #[test]
    fn conflict_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_high_n", Lang::En));
    }

    #[test]
    fn conflict_strategy_boundary_e7() {
        let p = with_ocean(p("A"), 5, 5, 7, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_high_e", Lang::En));
    }

    #[test]
    fn conflict_strategy_boundary_c7() {
        let p = with_ocean(p("A"), 5, 7, 5, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_high_c", Lang::En));
    }

    #[test]
    fn conflict_strategy_boundary_e4() {
        let p = with_ocean(p("A"), 5, 5, 4, 5, 5);
        let s = conflict_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_conflict_low_e", Lang::En));
    }

    #[test]
    fn success_strategy_boundary_o7() {
        let p = with_ocean(p("A"), 7, 5, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_high_o", Lang::En));
    }

    #[test]
    fn success_strategy_boundary_c7() {
        let p = with_ocean(p("A"), 5, 7, 5, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_high_c", Lang::En));
    }

    #[test]
    fn success_strategy_boundary_e4() {
        let p = with_ocean(p("A"), 5, 5, 4, 5, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_low_e", Lang::En));
    }

    #[test]
    fn success_strategy_boundary_a7() {
        let p = with_ocean(p("A"), 5, 5, 5, 7, 5);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_high_a", Lang::En));
    }

    #[test]
    fn success_strategy_boundary_recognition_i7() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Recognition,
            7,
        );
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_recognition", Lang::En));
    }

    #[test]
    fn success_strategy_boundary_power_i7() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 7);
        let s = success_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_success_power", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_high_n", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_boundary_n3() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 3);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_low_n", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_boundary_o7() {
        let p = with_ocean(p("A"), 7, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_high_o", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_boundary_o4() {
        let p = with_ocean(p("A"), 4, 5, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_low_o", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_boundary_c7() {
        let p = with_ocean(p("A"), 5, 7, 5, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_high_c", Lang::En));
    }

    #[test]
    fn uncertainty_strategy_boundary_e7() {
        let p = with_ocean(p("A"), 5, 5, 7, 5, 5);
        let s = uncertainty_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_uncertainty_high_e", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_i5() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            5,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 2);
        assert!(has(&s, "strategy_recognition_mid", Lang::En));
        assert!(has(&s, "strategy_recognition_high_e", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_i4() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 8, 5, 5),
            MotivationType::Recognition,
            4,
        );
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 2);
        assert!(has(&s, "strategy_recognition_low", Lang::En));
        assert!(has(&s, "strategy_recognition_high_e", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_e7() {
        let p = with_ocean(p("A"), 5, 5, 7, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_recognition_high_e", Lang::En));
    }

    #[test]
    fn recognition_strategy_boundary_e4() {
        let p = with_ocean(p("A"), 5, 5, 4, 5, 5);
        let s = recognition_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_recognition_low_e", Lang::En));
    }

    #[test]
    fn threatened_strategy_boundary_a4() {
        let p = with_ocean(p("A"), 5, 5, 5, 4, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_threat_low_a", Lang::En));
    }

    #[test]
    fn threatened_strategy_boundary_a7() {
        let p = with_ocean(p("A"), 5, 5, 5, 7, 5);
        let s = threatened_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_threat_high_a", Lang::En));
    }

    #[test]
    fn threatened_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = threatened_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_threat_high_n", Lang::En));
    }

    #[test]
    fn threatened_strategy_boundary_power_i7() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 7);
        let s = threatened_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_threat_power", Lang::En));
    }

    #[test]
    fn change_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_high_n", Lang::En));
    }

    #[test]
    fn change_strategy_boundary_n3() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 3);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_low_n", Lang::En));
    }

    #[test]
    fn change_strategy_boundary_c7() {
        let p = with_ocean(p("A"), 5, 7, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_high_c", Lang::En));
    }

    #[test]
    fn change_strategy_boundary_e4() {
        let p = with_ocean(p("A"), 5, 5, 4, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_low_e", Lang::En));
    }

    #[test]
    fn change_strategy_boundary_o7() {
        let p = with_ocean(p("A"), 7, 5, 5, 5, 5);
        let s = change_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_change_high_o", Lang::En));
    }

    #[test]
    fn feedback_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_high_n", Lang::En));
    }

    #[test]
    fn feedback_strategy_boundary_n3() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 3);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_low_n", Lang::En));
    }

    #[test]
    fn feedback_strategy_boundary_a4() {
        let p = with_ocean(p("A"), 5, 5, 5, 4, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_low_a", Lang::En));
    }

    #[test]
    fn feedback_strategy_boundary_e4() {
        let p = with_ocean(p("A"), 5, 5, 4, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_low_e", Lang::En));
    }

    #[test]
    fn feedback_strategy_boundary_c7() {
        let p = with_ocean(p("A"), 5, 7, 5, 5, 5);
        let s = feedback_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_feedback_high_c", Lang::En));
    }

    #[test]
    fn injustice_strategy_boundary_a7() {
        let p = with_ocean(p("A"), 5, 5, 5, 7, 5);
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_injustice_high_a", Lang::En));
    }

    #[test]
    fn injustice_strategy_boundary_n7() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 7);
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_injustice_high_n", Lang::En));
    }

    #[test]
    fn injustice_strategy_boundary_fairness_i6() {
        let p = with_mot(
            with_ocean(p("A"), 5, 5, 5, 5, 5),
            MotivationType::Fairness,
            6,
        );
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_injustice_fairness", Lang::En));
    }

    #[test]
    fn injustice_strategy_boundary_power_i7() {
        let p = with_mot(with_ocean(p("A"), 5, 5, 5, 5, 5), MotivationType::Power, 7);
        let s = injustice_strategy(&p, Lang::En);
        assert_eq!(s.len(), 1);
        assert!(has(&s, "strategy_injustice_power", Lang::En));
    }

    #[test]
    fn trigger_label_all_variants_en() {
        let labels: Vec<(&BehaviorTrigger, &str)> = BehaviorTrigger::ALL
            .iter()
            .map(|t| (t, trigger_label(t, Lang::En)))
            .collect();
        assert!(labels.iter().any(|(_, l)| *l == "Under stress"));
        assert!(labels.iter().any(|(_, l)| *l == "In conflict"));
        assert!(labels.iter().any(|(_, l)| *l == "In success"));
        assert!(labels.iter().any(|(_, l)| *l == "In uncertainty"));
        assert!(labels.iter().any(|(_, l)| *l == "Seeking recognition"));
        assert!(labels.iter().any(|(_, l)| *l == "Feeling threatened"));
        assert!(labels.iter().any(|(_, l)| *l == "Facing change"));
        assert!(labels.iter().any(|(_, l)| *l == "Receiving feedback"));
        assert!(labels.iter().any(|(_, l)| *l == "Facing injustice"));
        let distinct: std::collections::HashSet<&str> = labels.iter().map(|(_, l)| *l).collect();
        assert_eq!(distinct.len(), BehaviorTrigger::ALL.len());
    }

    #[test]
    fn build_top_rec_contains_advice() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 5);
        let result = build_top_rec(
            &p,
            &BehaviorTrigger::Stress,
            &["Drink tea".into()],
            Lang::En,
        );
        assert!(result.contains("Drink tea"));
    }

    #[test]
    fn build_top_rec_with_role_and_context() {
        let mut person = p("A");
        person.role = "Engineer".into();
        person.context = "Backend".into();
        let result = build_top_rec(
            &person,
            &BehaviorTrigger::Stress,
            &["advice".into()],
            Lang::En,
        );
        assert!(result.contains("Engineer"));
        assert!(result.contains("Backend"));
    }

    #[test]
    fn generate_insight_stress_high_n() {
        let p = with_ocean(p("A"), 5, 5, 5, 5, 8);
        let out = generate_insight(&p, &BehaviorTrigger::Stress, Lang::En);
        assert!(out.top.contains("A"));
        assert!(has(&out.secondary, "strategy_stress_high_n", Lang::En));
    }

    #[test]
    fn all_strategies_active_high_ocean() {
        let person = with_ocean(p("A"), 8, 8, 8, 8, 8);
        assert_eq!(stress_strategy(&person, Lang::En).len(), 4);
        assert_eq!(conflict_strategy(&person, Lang::En).len(), 4);
        assert_eq!(success_strategy(&person, Lang::En).len(), 3);
        assert_eq!(uncertainty_strategy(&person, Lang::En).len(), 4);
        assert_eq!(recognition_strategy(&person, Lang::En).len(), 1);
        assert_eq!(threatened_strategy(&person, Lang::En).len(), 2);
        assert_eq!(change_strategy(&person, Lang::En).len(), 3);
        assert_eq!(feedback_strategy(&person, Lang::En).len(), 2);
        assert_eq!(injustice_strategy(&person, Lang::En).len(), 2);
    }

    #[test]
    fn all_fallbacks_exact_values() {
        let person = with_ocean(p("A"), 5, 5, 5, 5, 5);
        assert_eq!(
            stress_strategy(&person, Lang::En),
            vec![tr("strategy_stress_fallback", Lang::En)]
        );
        assert_eq!(
            conflict_strategy(&person, Lang::En),
            vec![tr("strategy_conflict_fallback", Lang::En)]
        );
        assert_eq!(
            success_strategy(&person, Lang::En),
            vec![tr("strategy_success_fallback", Lang::En)]
        );
        assert_eq!(
            uncertainty_strategy(&person, Lang::En),
            vec![tr("strategy_uncertainty_fallback", Lang::En)]
        );
        assert_eq!(
            recognition_strategy(&person, Lang::En),
            vec![tr("strategy_recognition_fallback", Lang::En)]
        );
        assert_eq!(
            threatened_strategy(&person, Lang::En),
            vec![tr("strategy_threat_fallback", Lang::En)]
        );
        assert_eq!(
            change_strategy(&person, Lang::En),
            vec![tr("strategy_change_fallback", Lang::En)]
        );
        assert_eq!(
            feedback_strategy(&person, Lang::En),
            vec![tr("strategy_feedback_fallback", Lang::En)]
        );
        assert_eq!(
            injustice_strategy(&person, Lang::En),
            vec![tr("strategy_injustice_fallback", Lang::En)]
        );
    }

    // ── generate_insight: has_secondary = all_recs.len() > 1 (line 85) ──

    #[test]
    fn generate_insight_has_secondary_true() {
        let mut p = p("Alice");
        p.ocean.neuroticism = Some(8);
        p.ocean.extraversion = Some(8);
        let out = generate_insight(&p, &BehaviorTrigger::Stress, Lang::En);
        assert!(
            out.secondary.len() > 1,
            "expected multiple recs for high-N + high-E"
        );
        assert!(
            out.has_secondary,
            "has_secondary should be true when >1 recs"
        );
    }

    #[test]
    fn generate_insight_has_secondary_false() {
        let p = p("Bob");
        let out = generate_insight(&p, &BehaviorTrigger::Stress, Lang::En);
        assert_eq!(out.secondary.len(), 1, "expected single fallback rec");
        assert!(
            !out.has_secondary,
            "has_secondary should be false when 1 rec"
        );
    }

    // ── build_top_rec: role/context inclusion (line 96) ──

    #[test]
    fn build_top_rec_role_only_no_context() {
        let mut p = p("Dave");
        p.role = "PM".into();
        p.context = String::new();
        let top = build_top_rec(&p, &BehaviorTrigger::Stress, &["advice".into()], Lang::En);
        assert!(top.contains("PM"), "role should be included: {}", top);
        assert!(!top.contains(","), "no comma with only role: {}", top);
    }

    #[test]
    fn build_top_rec_empty_role_and_context() {
        let mut p = p("Eve");
        p.role = String::new();
        p.context = String::new();
        let top = build_top_rec(&p, &BehaviorTrigger::Stress, &["advice".into()], Lang::En);
        assert!(!top.contains("("), "no role/context parens: {}", top);
        assert!(
            top.contains("Eve"),
            "name should still be included: {}",
            top
        );
    }
}

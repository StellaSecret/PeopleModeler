use dioxus::prelude::*;
use peoplemodeler_core::models::{BehaviorTrigger, Person};

pub(crate) const ALL_TRIGGERS: [BehaviorTrigger; 8] = [
    BehaviorTrigger::Stress,
    BehaviorTrigger::Conflict,
    BehaviorTrigger::Success,
    BehaviorTrigger::Uncertainty,
    BehaviorTrigger::Recognition,
    BehaviorTrigger::Threatened,
    BehaviorTrigger::Change,
    BehaviorTrigger::Feedback,
];

use crate::Route;
use crate::db;
use crate::i18n::Lang;

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
    }
}

pub(crate) fn generate_insight(p: &Person, trigger: &BehaviorTrigger, lang: Lang) -> String {
    let top_mot = p.top_motivation();
    let top_bias = p.top_bias();

    let mut lines = Vec::new();
    let tl = trigger_label(trigger, lang);
    let pd = crate::i18n::tr("insights_primary_driver", lang);
    let kb = crate::i18n::tr("insights_key_bias", lang);
    let rec = crate::i18n::tr("insights_recommendations", lang);
    lines.push(crate::i18n::tr_fmt(
        "insights_context_analysis",
        lang,
        &[("trigger", tl), ("name", &p.name)],
    ));
    lines.push(String::new());

    if let Some(m) = top_mot {
        lines.push(format!("{}: {:?} ({}/10)", pd, m.r#type, m.intensity));
    }
    if let Some(b) = top_bias {
        lines.push(format!("{}: {:?} ({}/10)", kb, b.r#type, b.intensity));
    }

    let strategy = match trigger {
        BehaviorTrigger::Stress => stress_strategy(p, lang),
        BehaviorTrigger::Conflict => conflict_strategy(p, lang),
        BehaviorTrigger::Success => success_strategy(p, lang),
        BehaviorTrigger::Uncertainty => uncertainty_strategy(p, lang),
        BehaviorTrigger::Recognition => recognition_strategy(p, lang),
        BehaviorTrigger::Threatened => threatened_strategy(p, lang),
        BehaviorTrigger::Change => change_strategy(p, lang),
        BehaviorTrigger::Feedback => feedback_strategy(p, lang),
    };
    lines.push(String::new());
    lines.push(rec.into());
    for s in strategy {
        lines.push(format!("• {}", s));
    }

    lines.join("\n")
}

fn stress_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_n", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_e", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_stress_low_e", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_c", lang).into());
    }
    if p.ocean.agreeableness.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_stress_low_a", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_stress_low_c", lang).into());
    }
    if p.ocean.openness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_stress_high_o", lang).into());
    }
    if let Some(m) = p.top_motivation() {
        match m.r#type {
            peoplemodeler_core::models::MotivationType::Power => {
                s.push(crate::i18n::tr("strategy_stress_power", lang).into())
            }
            peoplemodeler_core::models::MotivationType::Security => {
                s.push(crate::i18n::tr("strategy_stress_security", lang).into())
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
    if p.ocean.agreeableness.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_conflict_low_a", lang).into());
    }
    if p.ocean.agreeableness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_a", lang).into());
    }
    if p.ocean.neuroticism.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_n", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_e", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_conflict_high_c", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_conflict_low_e", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_conflict_fallback", lang).into());
    }
    s
}

fn success_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.openness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_success_high_o", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_success_high_c", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_success_low_e", lang).into());
    }
    if p.ocean.agreeableness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_success_high_a", lang).into());
    }
    if let Some(m) = p.top_motivation() {
        if m.r#type == peoplemodeler_core::models::MotivationType::Recognition && m.intensity >= 7 {
            s.push(crate::i18n::tr("strategy_success_recognition", lang).into());
        }
        if m.r#type == peoplemodeler_core::models::MotivationType::Power && m.intensity >= 7 {
            s.push(crate::i18n::tr("strategy_success_power", lang).into());
        }
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_success_fallback", lang).into());
    }
    s
}

fn uncertainty_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_n", lang).into());
    }
    if p.ocean.neuroticism.map_or(false, |v| v <= 3) {
        s.push(crate::i18n::tr("strategy_uncertainty_low_n", lang).into());
    }
    if p.ocean.openness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_o", lang).into());
    }
    if p.ocean.openness.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_uncertainty_low_o", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_c", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_uncertainty_high_e", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_uncertainty_fallback", lang).into());
    }
    s
}

fn recognition_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if let Some(m) = p.top_motivation() {
        if m.r#type == peoplemodeler_core::models::MotivationType::Recognition {
            match m.intensity {
                8.. => s.push(crate::i18n::tr("strategy_recognition_high", lang).into()),
                5.. => s.push(crate::i18n::tr("strategy_recognition_mid", lang).into()),
                _ => s.push(crate::i18n::tr("strategy_recognition_low", lang).into()),
            }
        }
    }
    if p.ocean.extraversion.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_recognition_high_e", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_recognition_low_e", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_recognition_fallback", lang).into());
    }
    s
}

fn threatened_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.agreeableness.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_threat_low_a", lang).into());
    }
    if p.ocean.agreeableness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_threat_high_a", lang).into());
    }
    if p.ocean.neuroticism.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_threat_high_n", lang).into());
    }
    if let Some(m) = p.top_motivation() {
        if m.r#type == peoplemodeler_core::models::MotivationType::Power && m.intensity >= 7 {
            s.push(crate::i18n::tr("strategy_threat_power", lang).into());
        }
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_threat_fallback", lang).into());
    }
    s
}

fn change_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_change_high_n", lang).into());
    }
    if p.ocean.neuroticism.map_or(false, |v| v <= 3) {
        s.push(crate::i18n::tr("strategy_change_low_n", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_change_high_c", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_change_low_e", lang).into());
    }
    if p.ocean.openness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_change_high_o", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_change_fallback", lang).into());
    }
    s
}

fn feedback_strategy(p: &Person, lang: Lang) -> Vec<String> {
    let mut s = Vec::new();
    if p.ocean.neuroticism.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_feedback_high_n", lang).into());
    }
    if p.ocean.neuroticism.map_or(false, |v| v <= 3) {
        s.push(crate::i18n::tr("strategy_feedback_low_n", lang).into());
    }
    if p.ocean.agreeableness.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_feedback_low_a", lang).into());
    }
    if p.ocean.extraversion.map_or(false, |v| v <= 4) {
        s.push(crate::i18n::tr("strategy_feedback_low_e", lang).into());
    }
    if p.ocean.conscientiousness.map_or(false, |v| v >= 7) {
        s.push(crate::i18n::tr("strategy_feedback_high_c", lang).into());
    }
    if s.is_empty() {
        s.push(crate::i18n::tr("strategy_feedback_fallback", lang).into());
    }
    s
}

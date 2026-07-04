use dioxus::prelude::*;
use peoplemodeler_core::models::{
    BehavioralPattern, Bias, BiasType, Motivation, MotivationType, OceanScores, Person,
    BehaviorTrigger, AVATAR_EMOJIS,
};

use crate::db;
use crate::i18n::Lang;
use crate::Route;

#[component]
pub fn PersonNew() -> Element {
    rsx! { PersonEditForm {} }
}

#[component]
pub fn PersonEdit(id: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let existing = db::person(&id);
    let not_found = crate::i18n::tr("person_not_found", lang());
    match existing {
        None => rsx! { div { class: "page", h2 { "{not_found}" } } },
        Some(p) => rsx! { PersonEditForm { initial: p } },
    }
}

#[component]
fn PersonEditForm(initial: Option<Person>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let is_new = initial.is_none();
    let p = initial.unwrap_or_else(|| Person {
        id: uuid::Uuid::new_v4().to_string(),
        name: String::new(),
        role: String::new(),
        context: String::new(),
        avatar_emoji: "👤".into(),
        tags: Vec::new(),
        notes: String::new(),
        motivations: Vec::new(),
        biases: Vec::new(),
        behavioral_patterns: Vec::new(),
        ocean: OceanScores::default(),
        predictions: Vec::new(),
        created_at: chrono::Utc::now().timestamp_millis(),
        updated_at: chrono::Utc::now().timestamp_millis(),
    });

    let mut name = use_signal(|| p.name.clone());
    let mut role = use_signal(|| p.role.clone());
    let mut context = use_signal(|| p.context.clone());
    let mut emoji = use_signal(|| p.avatar_emoji.clone());
    let mut notes = use_signal(|| p.notes.clone());
    let mut tags_str = use_signal(|| p.tags.join(", "));
    let mut ocean = use_signal(|| p.ocean.clone());
    let motivations = use_signal(|| p.motivations.clone());
    let biases = use_signal(|| p.biases.clone());
    let patterns = use_signal(|| p.behavioral_patterns.clone());

    let pers_id = p.id.clone();

    let save = move || {
        let person = Person {
            id: pers_id.clone(),
            name: name(),
            role: role(),
            context: context(),
            avatar_emoji: emoji(),
            tags: tags_str().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            notes: notes(),
            motivations: motivations(),
            biases: biases(),
            behavioral_patterns: patterns(),
            ocean: ocean(),
            predictions: Vec::new(),
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: chrono::Utc::now().timestamp_millis(),
        };
        db::save_person(&person);
        dioxus::prelude::navigator().push(Route::PersonDetail { id: pers_id.clone() });
    };

    let form_new_title = crate::i18n::tr("form_new_title", lang());
    let form_edit_title = crate::i18n::tr("form_edit_title", lang());
    let form_name = crate::i18n::tr("form_name", lang());
    let form_role = crate::i18n::tr("form_role", lang());
    let form_context = crate::i18n::tr("form_context", lang());
    let form_avatar = crate::i18n::tr("form_avatar", lang());
    let form_tags = crate::i18n::tr("form_tags", lang());
    let form_notes = crate::i18n::tr("form_notes", lang());
    let form_ocean_title = crate::i18n::tr("form_ocean_title", lang());
    let form_save = crate::i18n::tr("form_save", lang());
    let form_cancel = crate::i18n::tr("form_cancel", lang());

    rsx! {
        div { class: "page",
            h2 { if is_new { "{form_new_title}" } else { "{form_edit_title}" } }
            div { class: "form",
                label { "{form_name}" }
                input { value: "{name}", oninput: move |e| name.set(e.value()) }

                label { "{form_role}" }
                input { value: "{role}", oninput: move |e| role.set(e.value()) }

                label { "{form_context}" }
                textarea { value: "{context}", oninput: move |e| context.set(e.value()) }

                label { "{form_avatar}" }
                div { class: "emoji-picker",
                    for e in AVATAR_EMOJIS {
                        button {
                            class: "emoji-btn",
                            class: if emoji() == *e { "selected" },
                            onclick: move |_| emoji.set(e.to_string()),
                            "{e}"
                        }
                    }
                }

                label { "{form_tags}" }
                input { value: "{tags_str}", oninput: move |e| tags_str.set(e.value()) }

                label { "{form_notes}" }
                textarea { value: "{notes}", rows: "4", oninput: move |e| notes.set(e.value()) }

                fieldset { class: "ocean-inputs",
                    legend { "{form_ocean_title}" }
                    OceanSlider { label: crate::i18n::tr("ocean_openness", lang()), val: ocean().openness, onchange: move |v| { let mut o = ocean.write(); o.openness = v; } }
                    OceanSlider { label: crate::i18n::tr("ocean_conscientiousness", lang()), val: ocean().conscientiousness, onchange: move |v| { let mut o = ocean.write(); o.conscientiousness = v; } }
                    OceanSlider { label: crate::i18n::tr("ocean_extraversion", lang()), val: ocean().extraversion, onchange: move |v| { let mut o = ocean.write(); o.extraversion = v; } }
                    OceanSlider { label: crate::i18n::tr("ocean_agreeableness", lang()), val: ocean().agreeableness, onchange: move |v| { let mut o = ocean.write(); o.agreeableness = v; } }
                    OceanSlider { label: crate::i18n::tr("ocean_neuroticism", lang()), val: ocean().neuroticism, onchange: move |v| { let mut o = ocean.write(); o.neuroticism = v; } }
                }

                MotEditPanel { motivations: motivations.clone() }
                BiasEditPanel { biases: biases.clone() }
                PatternEditPanel { patterns: patterns.clone() }

                div { class: "form-actions",
                    button { class: "btn btn-primary", onclick: move |_| save(), "{form_save}" }
                    Link { to: Route::PeopleList {}, class: "btn", "{form_cancel}" }
                }
            }
        }
    }
}

#[component]
fn MotEditPanel(motivations: Signal<Vec<Motivation>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut sel_type = use_signal(|| MotivationType::Achievement);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let edit_motivations = crate::i18n::tr("edit_motivations", lang());
    let notes_pl = crate::i18n::tr("edit_notes_placeholder", lang());
    let add_btn = crate::i18n::tr("add_btn", lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_motivations}" }
            div { class: "add-row",
                select { value: "{sel_type}",
                    onchange: move |e| { sel_type.set(parse_mot_type(&e.value())); },
                    for t in MotivationType::ALL {
                        option { value: "{t:?}", "{t.emoji()} {t:?}" }
                    }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_intensity}",
                    oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_intensity}" }
                input { placeholder: "{notes_pl}", value: "{sel_notes}",
                    oninput: move |e| { sel_notes.set(e.value()); }
                }
                button { class: "btn", onclick: move |_| {
                    motivations.write().push(Motivation { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() });
                    sel_notes.set(String::new());
                }, "{add_btn}" }
            }
            for (i, m) in motivations().iter().enumerate() {
                div { class: "list-item",
                    strong { "{m.r#type.emoji()} {m.r#type:?}" }
                    span { " {m.intensity}/10" }
                    span { " {m.notes}" }
                    button { class: "btn btn-small", onclick: move |_| { motivations.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

#[component]
fn BiasEditPanel(biases: Signal<Vec<Bias>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut sel_type = use_signal(|| BiasType::Confirmation);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_evidence = use_signal(String::new);
    let edit_biases = crate::i18n::tr("edit_biases", lang());
    let evidence_pl = crate::i18n::tr("edit_evidence_placeholder", lang());
    let add_btn = crate::i18n::tr("add_btn", lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_biases}" }
            div { class: "add-row",
                select { value: "{sel_type}",
                    onchange: move |e| { sel_type.set(parse_bias_type(&e.value())); },
                    for t in BiasType::ALL {
                        option { value: "{t:?}", "{t.emoji()} {t:?}" }
                    }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_intensity}",
                    oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_intensity}" }
                input { placeholder: "{evidence_pl}", value: "{sel_evidence}",
                    oninput: move |e| { sel_evidence.set(e.value()); }
                }
                button { class: "btn", onclick: move |_| {
                    biases.write().push(Bias { r#type: sel_type(), intensity: sel_intensity(), evidence: sel_evidence() });
                    sel_evidence.set(String::new());
                }, "{add_btn}" }
            }
            for (i, b) in biases().iter().enumerate() {
                div { class: "list-item",
                    strong { "{b.r#type.emoji()} {b.r#type:?}" }
                    span { " {b.intensity}/10" }
                    span { " {b.evidence}" }
                    button { class: "btn btn-small", onclick: move |_| { biases.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

#[component]
fn PatternEditPanel(patterns: Signal<Vec<BehavioralPattern>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut sel_trigger = use_signal(|| BehaviorTrigger::Stress);
    let mut sel_behavior = use_signal(String::new);
    let mut sel_conf = use_signal(|| 5u8);

    let edit_patterns = crate::i18n::tr("edit_patterns", lang());
    let ctx_stress = crate::i18n::tr("ctx_stress", lang());
    let ctx_conflict = crate::i18n::tr("ctx_conflict", lang());
    let ctx_success = crate::i18n::tr("ctx_success", lang());
    let ctx_uncertainty = crate::i18n::tr("ctx_uncertainty", lang());
    let ctx_recognition = crate::i18n::tr("ctx_recognition", lang());
    let ctx_threatened = crate::i18n::tr("ctx_threatened", lang());
    let outcome_pl = crate::i18n::tr("pred_outcome_placeholder", lang());
    let add_btn = crate::i18n::tr("add_btn", lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_patterns}" }
            div { class: "add-row",
                select { value: "{sel_trigger}",
                    onchange: move |e| { sel_trigger.set(parse_trigger(&e.value())); },
                    option { value: "Stress", "{ctx_stress}" }
                    option { value: "Conflict", "{ctx_conflict}" }
                    option { value: "Success", "{ctx_success}" }
                    option { value: "Uncertainty", "{ctx_uncertainty}" }
                    option { value: "Recognition", "{ctx_recognition}" }
                    option { value: "Threatened", "{ctx_threatened}" }
                }
                input { placeholder: "{outcome_pl}", value: "{sel_behavior}",
                    oninput: move |e| { sel_behavior.set(e.value()); }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_conf}",
                    oninput: move |e| { sel_conf.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_conf}" }
                button { class: "btn", onclick: move |_| {
                    patterns.write().push(BehavioralPattern { trigger: sel_trigger(), predicted_behavior: sel_behavior(), confidence: sel_conf() });
                    sel_behavior.set(String::new());
                }, "{add_btn}" }
            }
            for (i, bp) in patterns().iter().enumerate() {
                div { class: "list-item",
                    strong { "{bp.trigger:?}" }
                    span { " {bp.predicted_behavior}" }
                    span { " ({bp.confidence}/10)" }
                    button { class: "btn btn-small", onclick: move |_| { patterns.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

#[component]
fn OceanSlider(label: String, val: u8, onchange: EventHandler<u8>) -> Element {
    rsx! {
        div { class: "ocean-slider",
            label { "{label}" }
            input {
                r#type: "range",
                min: "1",
                max: "10",
                value: "{val}",
                oninput: move |e| onchange.call(e.value().parse::<u8>().unwrap_or(5)),
            }
            span { "{val}" }
        }
    }
}

fn parse_mot_type(s: &str) -> MotivationType {
    match s {
        "Power" => MotivationType::Power,
        "Achievement" => MotivationType::Achievement,
        "Affiliation" => MotivationType::Affiliation,
        "Security" => MotivationType::Security,
        "Autonomy" => MotivationType::Autonomy,
        "Recognition" => MotivationType::Recognition,
        "Learning" => MotivationType::Learning,
        "Helping" => MotivationType::Helping,
        _ => MotivationType::Achievement,
    }
}

fn parse_bias_type(s: &str) -> BiasType {
    match s {
        "Confirmation" => BiasType::Confirmation,
        "Anchoring" => BiasType::Anchoring,
        "Availability" => BiasType::Availability,
        "SunkCost" => BiasType::SunkCost,
        "DunningKruger" => BiasType::DunningKruger,
        "LossAversion" => BiasType::LossAversion,
        "SocialProof" => BiasType::SocialProof,
        "Authority" => BiasType::Authority,
        "Recency" => BiasType::Recency,
        "InGroup" => BiasType::InGroup,
        _ => BiasType::Confirmation,
    }
}

fn parse_trigger(s: &str) -> BehaviorTrigger {
    match s {
        "Stress" => BehaviorTrigger::Stress,
        "Conflict" => BehaviorTrigger::Conflict,
        "Success" => BehaviorTrigger::Success,
        "Uncertainty" => BehaviorTrigger::Uncertainty,
        "Recognition" => BehaviorTrigger::Recognition,
        "Threatened" => BehaviorTrigger::Threatened,
        _ => BehaviorTrigger::Stress,
    }
}

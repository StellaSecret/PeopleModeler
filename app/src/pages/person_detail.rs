use dioxus::prelude::*;
use peoplemodeler_core::models::{BehaviorTrigger, Person, Prediction};

use crate::db;
use crate::i18n::Lang;
use crate::pages::predictions::PredictionList;
use crate::Route;

fn core_lang(l: Lang) -> peoplemodeler_core::i18n::Lang {
    match l {
        Lang::Fr => peoplemodeler_core::i18n::Lang::Fr,
        Lang::En => peoplemodeler_core::i18n::Lang::En,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Motivations,
    Biases,
    Ocean,
    Predictions,
    Insights,
}

#[component]
pub fn PersonDetail(id: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let person = use_signal(|| db::person(&id));
    let mut tab = use_signal(|| Tab::Motivations);
    let not_found = crate::i18n::tr("person_not_found", lang());

    let p = person.read().clone();
    match p {
        None => rsx! { div { class: "page", h2 { "{not_found}" } } },
        Some(ref person) => {
            let edit_btn = crate::i18n::tr("edit_btn", lang());
            let delete_btn = crate::i18n::tr("delete_btn", lang());
            let mot_title = crate::i18n::tr("motivations_title", lang());
            let bias_title = crate::i18n::tr("biases_title", lang());
            let ocean_title = crate::i18n::tr("ocean_title", lang());
            let pred_title = crate::i18n::tr("pred_title", lang());
            let insights_title = crate::i18n::tr("insights_title", lang());
            let no_mot = crate::i18n::tr("no_motivations", lang());
            let no_bias = crate::i18n::tr("no_biases", lang());
            let pat_title = crate::i18n::tr("patterns_title", lang());
            let conf_label = crate::i18n::tr("confidence_label", lang());
            let no_pat = crate::i18n::tr("no_patterns", lang());

            let mut preds = use_signal(|| db::predictions_for_person(&id));
            let mut ctx = use_signal(String::new);
            let mut predicted = use_signal(String::new);
            let ctx_pl = crate::i18n::tr("pred_context_placeholder", lang());
            let outcome_pl = crate::i18n::tr("pred_outcome_placeholder", lang());
            let add_btn = crate::i18n::tr("pred_add_btn", lang());

            let mut trigger = use_signal(|| BehaviorTrigger::Stress);
            let observed_label = crate::i18n::tr("insights_observed", lang());
            let insight_text = crate::pages::insights::generate_insight(person, &trigger(), lang());

            let id_pred = id.clone();
            let mut add_pred = move || {
                let c = ctx();
                let p = predicted();
                if c.is_empty() || p.is_empty() {
                    return;
                }
                let pred = Prediction {
                    id: uuid::Uuid::new_v4().to_string(),
                    person_id: id_pred.clone(),
                    context: c,
                    predicted_outcome: p,
                    actual_outcome: None,
                    accuracy: None,
                    created_at: chrono::Utc::now().timestamp_millis(),
                    resolved_at: None,
                    resolved: false,
                };
                db::save_prediction(&pred);
                ctx.set(String::new());
                predicted.set(String::new());
                preds.set(db::predictions_for_person(&id_pred));
            };

            rsx! {
                div { class: "page",
                    div { class: "toolbar",
                        Link { to: Route::PersonEdit { id: id.clone() }, class: "btn", "{edit_btn}" }
                        button {
                            class: "btn btn-danger",
                            onclick: move |_| {
                                db::delete_person(&id);
                                navigator().push(Route::PeopleList {});
                            },
                            "{delete_btn}"
                        }
                    }

                    div { class: "person-header",
                        span { class: "avatar-lg", "{person.avatar_emoji}" }
                        h1 { "{person.name}" }
                        p { "{person.role}" }
                        p { class: "context", "{person.context}" }
                        if !person.tags.is_empty() {
                            div { class: "tags",
                                for tag in &person.tags {
                                    span { class: "tag", "{tag}" }
                                }
                            }
                        }
                    }

                    div { class: "tab-bar",
                        button { class: if tab() == Tab::Motivations { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Motivations), "💡 {mot_title}" }
                        button { class: if tab() == Tab::Biases { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Biases), "🧠 {bias_title}" }
                        button { class: if tab() == Tab::Ocean { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Ocean), "🌊 {ocean_title}" }
                        button { class: if tab() == Tab::Predictions { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Predictions), "🔮 {pred_title}" }
                        button { class: if tab() == Tab::Insights { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Insights), "✨ {insights_title}" }
                    }

                    if tab() == Tab::Motivations {
                        div { class: "section",
                            h2 { "{mot_title}" }
                            for m in &person.motivations {
                                div { class: "motivation-item",
                                    div { class: "item-icon", "{m.r#type.emoji()}" }
                                    div { class: "item-info",
                                        div { class: "item-name", "{m.r#type.i18n(core_lang(lang())).label}" }
                                        div { class: "item-bar-row",
                                            div { class: "item-bar",
                                                div { class: "item-bar-fill cyan", style: "width: {m.intensity * 10}%" }
                                            }
                                            div { class: "item-intensity", "{m.intensity}/10" }
                                        }
                                        if !m.notes.is_empty() { div { class: "item-notes", "{m.notes}" } }
                                    }
                                }
                            }
                            if person.motivations.is_empty() { p { "{no_mot}" } }
                        }
                        div { class: "section",
                            h2 { "{pat_title}" }
                            for bp in &person.behavioral_patterns {
                                div { class: "pattern-item",
                                    strong { { crate::pages::insights::trigger_label(&bp.trigger, lang()) } }
                                    p { "{bp.predicted_behavior}" }
                                    span { "{conf_label}: {bp.confidence}/10" }
                                }
                            }
                            if person.behavioral_patterns.is_empty() { p { "{no_pat}" } }
                        }
                    }

                    if tab() == Tab::Biases {
                        div { class: "section",
                            h2 { "{bias_title}" }
                            for b in &person.biases {
                                div { class: "bias-item",
                                    div { class: "item-icon", "{b.r#type.emoji()}" }
                                    div { class: "item-info",
                                        div { class: "item-name", "{b.r#type.i18n(core_lang(lang())).label}" }
                                        div { class: "item-bar-row",
                                            div { class: "item-bar",
                                                div { class: "item-bar-fill red", style: "width: {b.intensity * 10}%" }
                                            }
                                            div { class: "item-intensity", "{b.intensity}/10" }
                                        }
                                        if !b.evidence.is_empty() { div { class: "item-notes", "{b.evidence}" } }
                                    }
                                }
                            }
                            if person.biases.is_empty() { p { "{no_bias}" } }
                        }
                    }

                    if tab() == Tab::Ocean {
                        OceanChart { person: person.clone() }
                    }

                    if tab() == Tab::Predictions {
                        div { class: "card",
                            h2 { "{pred_title}" }
                            div { class: "form-row",
                                input { placeholder: "{ctx_pl}", value: "{ctx}", oninput: move |e| ctx.set(e.value()) }
                                input { placeholder: "{outcome_pl}", value: "{predicted}", oninput: move |e| predicted.set(e.value()) }
                                button { class: "btn btn-primary", onclick: move |_| add_pred(), "{add_btn}" }
                            }
                        }
                        PredictionList { predictions: preds(), person_filter: Some(id.clone()),
                            onresolve: {
                                let pid = id.clone();
                                move |_| preds.set(db::predictions_for_person(&pid))
                            },
                            ondelete: {
                                let pid = id.clone();
                                move |_| preds.set(db::predictions_for_person(&pid))
                            },
                        }
                    }

                    if tab() == Tab::Insights {
                        div { class: "card",
                            h2 { "{insights_title}" }
                            div { class: "trigger-selector",
                                for t in crate::pages::insights::ALL_TRIGGERS {
                                    button {
                                        class: if *trigger.read() == t { "btn active" } else { "btn" },
                                        onclick: { let t = t; move |_| trigger.set(t) },
                                        { crate::pages::insights::trigger_label(&t, lang()) }
                                    }
                                }
                            }
                            div { class: "insight-content",
                                pre { "{insight_text}" }
                            }
                        }
                        if !person.behavioral_patterns.is_empty() {
                            div { class: "card",
                                h3 { "{observed_label}" }
                                for bp in &person.behavioral_patterns {
                                    div { class: "pattern-item",
                                        strong { { crate::pages::insights::trigger_label(&bp.trigger, lang()) } }
                                        p { "{bp.predicted_behavior}" }
                                    }
                                }
                            }
                        }
                    }

                    Link { to: Route::PersonEdit { id: id.clone() }, class: "fab", "✏" }
                }
            }
        }
    }
}

#[component]
fn OceanChart(person: Person) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let ocean_title = crate::i18n::tr("ocean_title", lang());
    let labels = [
        crate::i18n::tr("ocean_o", lang()),
        crate::i18n::tr("ocean_c", lang()),
        crate::i18n::tr("ocean_e", lang()),
        crate::i18n::tr("ocean_a", lang()),
        crate::i18n::tr("ocean_n", lang()),
    ];
    let scores = [
        person.ocean.openness,
        person.ocean.conscientiousness,
        person.ocean.extraversion,
        person.ocean.agreeableness,
        person.ocean.neuroticism,
    ];
    rsx! {
        div { class: "section ocean-chart",
            h2 { "{ocean_title}" }
            for (i, label) in labels.iter().enumerate() {
                div { class: "ocean-bar",
                    span { "{label}" }
                    div { class: "bar-wrapper",
                        div {
                            class: "bar-fill",
                            style: "width: {scores[i] * 10}%",
                        }
                    }
                    span { "{scores[i]}/10" }
                }
            }
        }
    }
}

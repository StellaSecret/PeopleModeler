use dioxus::prelude::*;
use peoplemodeler_core::models::{BehaviorTrigger, Person, Prediction};

use crate::db;
use crate::i18n::Lang;
use crate::pages::predictions::{PredictionList, format_date};
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
    Log,
    Relationships,
}

#[component]
pub fn PersonDetail(id: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut person_sig = use_signal(|| db::person(&id));
    let mut tab = use_signal(|| Tab::Motivations);
    let mut toast_sig = use_context::<Signal<Option<String>>>();
    let not_found = crate::i18n::tr("person_not_found", lang());
    let tag_filter = use_context::<Signal<Option<String>>>();

    let p = person_sig.read().clone();
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
            let rep_title = crate::i18n::tr("reputation_title", lang());
            let no_rep = crate::i18n::tr("no_reputation", lang());
            let pat_title = crate::i18n::tr("patterns_title", lang());
            let conf_label = crate::i18n::tr("confidence_label", lang());
            let compare_btn = crate::i18n::tr("compare_btn", lang());
            let no_pat = crate::i18n::tr("no_patterns", lang());
            let log_title = crate::i18n::tr("log_title", lang());
            let log_placeholder = crate::i18n::tr("log_placeholder", lang());
            let log_add = crate::i18n::tr("log_add", lang());
            let log_empty = crate::i18n::tr("log_empty", lang());
            let rel_person_rel = crate::i18n::tr("rel_person_rel", lang());
            let rel_none = crate::i18n::tr("rel_none", lang());
            let rel_title = crate::i18n::tr("rel_title", lang());

            let mut preds = use_signal(|| db::predictions_for_person(&id));
            let mut ctx = use_signal(String::new);
            let mut predicted = use_signal(String::new);
            let ctx_pl = crate::i18n::tr("pred_context_placeholder", lang());
            let outcome_pl = crate::i18n::tr("pred_outcome_placeholder", lang());
            let add_btn = crate::i18n::tr("pred_add_btn", lang());

            let mut comparing = use_signal(|| false);
            let other_persons = use_signal(|| db::all_persons());
            let mut log_text = use_signal(String::new);
            let person_rels = use_signal(|| {
                let all = db::all_relationships();
                let pid = id.clone();
                all.into_iter().filter(|r| r.source_id == pid || r.target_id == pid).collect::<Vec<_>>()
            });

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
                        button { class: "btn", onclick: move |_| comparing.set(!comparing()), "{compare_btn}" }
                        button {
                            class: "btn btn-danger",
                            onclick: move |_| {
                                db::delete_person(&id);
                                toast_sig.set(Some(crate::i18n::tr("toast_deleted", lang()).into()));
                                navigator().push(Route::PeopleList {});
                            },
                            "{delete_btn}"
                        }
                    }
                    if comparing() {
                        div { class: "compare-picker",
                            for other in other_persons().iter().filter(|p| p.id != id) {
                                Link {
                                    to: Route::ComparePersons { id1: id.clone(), id2: other.id.clone() },
                                    class: "compare-option",
                                    span { "{other.avatar_emoji} {other.name}" }
                                }
                            }
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
                                    span {
                                        class: "tag tag-clickable",
                                        onclick: {
                                            let t = tag.clone();
                                            let mut tf = tag_filter.clone();
                                            move |_| tf.set(Some(t.clone()))
                                        },
                                        "{tag}"
                                    }
                                }
                            }
                        }
                        div { class: "confidence-badge",
                            span { "{conf_label}: {person.confidence}/10" }
                        }
                    }

                    div { class: "tab-bar", role: "tablist",
                        button { role: "tab", aria_label: "{mot_title}", class: if tab() == Tab::Motivations { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Motivations), "💡 {mot_title}" }
                        button { role: "tab", aria_label: "{bias_title}", class: if tab() == Tab::Biases { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Biases), "🧠 {bias_title}" }
                        button { role: "tab", aria_label: "{ocean_title}", class: if tab() == Tab::Ocean { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Ocean), "🌊 {ocean_title}" }
                        button { role: "tab", aria_label: "{pred_title}", class: if tab() == Tab::Predictions { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Predictions), "🔮 {pred_title}" }
                        button { role: "tab", aria_label: "{insights_title}", class: if tab() == Tab::Insights { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Insights), "✨ {insights_title}" }
                        button { role: "tab", aria_label: "Log", class: if tab() == Tab::Log { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Log), "{log_title}" }
                        button { role: "tab", aria_label: "Relationships", class: if tab() == Tab::Relationships { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Relationships), "{rel_person_rel}" }
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
                        div { class: "section",
                            h2 { "{rep_title}" }
                            for r in &person.reputation {
                                div { class: "bias-item",
                                    div { class: "item-icon", "{r.r#type.emoji()}" }
                                    div { class: "item-info",
                                        div { class: "item-name", "{r.r#type.i18n(core_lang(lang())).label}" }
                                        div { class: "item-bar-row",
                                            div { class: "item-bar",
                                                div { class: "item-bar-fill red", style: "width: {r.intensity * 10}%" }
                                            }
                                            div { class: "item-intensity", "{r.intensity}/10" }
                                        }
                                        if !r.evidence.is_empty() { div { class: "item-notes", "{r.evidence}" } }
                                    }
                                }
                            }
                            if person.reputation.is_empty() { p { "{no_rep}" } }
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
                            {
                                let all = preds();
                                let total = all.len();
                                let resolved: Vec<_> = all.into_iter().filter(|p| p.resolved).collect();
                                let n = resolved.len();
                                if n > 0 {
                                    let avg: u8 = resolved.iter().filter_map(|p| p.accuracy).sum::<u8>() / n as u8;
                                    rsx! {
                                        div { class: "pred-stats",
                                            span { "✓ {n}/{total} — Σ {avg}/10" }
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
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

                    if tab() == Tab::Log {
                        div { class: "card",
                            h2 { "{log_title}" }
                            div { class: "form-row",
                                input { placeholder: "{log_placeholder}", aria_label: "New log entry", value: "{log_text}",
                                    oninput: move |e| log_text.set(e.value()) }
                                button { class: "btn btn-primary", aria_label: "{log_add}", onclick: {
                                    let pid = id.clone();
                                    move |_| {
                                        let t = log_text();
                                        if t.is_empty() { return; }
                                        let mut p = person_sig.write().clone();
                                        if let Some(ref mut p) = p {
                                            p.log.push(peoplemodeler_core::models::InteractionEntry {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                timestamp: chrono::Utc::now().timestamp_millis(),
                                                text: t,
                                            });
                                            db::save_person(p);
                                            person_sig.set(db::person(&pid));
                                        }
                                        log_text.set(String::new());
                                    }
                                }, "{log_add}" }
                            }
                        }
                        if person.log.is_empty() {
                            p { "{log_empty}" }
                        } else {
                            div { class: "log-list",
                                for entry in person.log.iter().rev() {
                                    div { class: "log-entry",
                                        div { class: "log-entry-header",
                                            small { class: "log-date", "{format_date(entry.timestamp)}" }
                                            button {
                                                class: "btn-icon btn-danger",
                                                onclick: {
                                                    let eid = entry.id.clone();
                                                    let pid = id.clone();
                                                    move |_| {
                                                        let mut p = person_sig.write().clone();
                                                        if let Some(ref mut p) = p {
                                                            p.log.retain(|e| e.id != eid);
                                                            db::save_person(p);
                                                            person_sig.set(db::person(&pid));
                                                        }
                                                    }
                                                },
                                                "✕"
                                            }
                                        }
                                        p { "{entry.text}" }
                                    }
                                }
                            }
                        }
                    }

                    if tab() == Tab::Relationships {
                        RelationshipSection { person_id: id.clone(), rels: person_rels(), rel_person_rel, rel_none, rel_title }
                    }

                    Link { to: Route::PersonEdit { id: id.clone() }, class: "fab", aria_label: "Edit person", "✏" }
                }
            }
        }
    }
}

fn radar_poly(radius: f64, cx: f64, cy: f64, scale: f64) -> String {
    use std::f64::consts::PI;
    let r = radius * scale;
    let mut pts = Vec::new();
    for i in 0..5 {
        let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
        pts.push(format!("{:.1},{:.1}", cx + r * a.cos(), cy + r * a.sin()));
    }
    pts.push(pts[0].clone());
    pts.join(" ")
}

fn axis_label(cx: f64, cy: f64, r: f64, i: usize) -> (f64, f64) {
    use std::f64::consts::PI;
    let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
    let x = cx + (r + 16.0) * a.cos();
    let y = cy + (r + 16.0) * a.sin();
    (x, y)
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

    let cx = 110.0; let cy = 110.0; let r = 80.0;

    use std::f64::consts::PI;
    let pts: Vec<(f64, f64)> = scores.iter().enumerate().map(|(i, s)| {
        let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
        let pr = r * *s as f64 / 10.0;
        (cx + pr * a.cos(), cy + pr * a.sin())
    }).collect();
    let data_poly: String = pts.iter().map(|(x, y)| format!("{:.1},{:.1}", x, y)).collect::<Vec<_>>().join(" ");
    let data_poly = format!("{} {:.1},{:.1}", data_poly, pts[0].0, pts[0].1);
    let label_pos: Vec<(f64, f64)> = (0..5).map(|i| {
        axis_label(cx, cy, r, i)
    }).collect();

    rsx! {
        div { class: "section ocean-chart",
            h2 { "{ocean_title}" }
            div { class: "radar-wrapper",
                svg { view_box: "0 0 220 220",
                    // grid
                    for level in [2, 4, 6, 8, 10] {
                        polygon {
                            fill: "none",
                            stroke: "var(--border)",
                            stroke_width: "1",
                            points: "{radar_poly(r, cx, cy, level as f64 / 10.0)}"
                        }
                    }
                    // data polygon
                    polygon {
                        fill: "var(--cyan)",
                        fill_opacity: "0.2",
                        stroke: "var(--cyan)",
                        stroke_width: "2",
                        points: "{data_poly}"
                    }
                    // data points
                    for (x, y) in pts.iter() {
                        circle { cx: "{x:.1}", cy: "{y:.1}", r: "3.5", fill: "var(--cyan)" }
                    }
                    // labels
                    for (i, label) in labels.iter().enumerate() {
                        text {
                            x: "{label_pos[i].0:.1}", y: "{label_pos[i].1:.1}",
                            text_anchor: "middle",
                            dominant_baseline: "central",
                            font_size: "10",
                            fill: "var(--text-muted)",
                            font_family: "var(--font-display)",
                            "{label}"
                        }
                    }
                    // center score summary
                    text {
                        x: "{cx}", y: "{cy}",
                        text_anchor: "middle",
                        dominant_baseline: "central",
                        font_size: "14",
                        font_weight: "bold",
                        fill: "var(--text)",
                        font_family: "var(--font-display)",
                        "{scores.iter().sum::<u8>() / 5}/10"
                    }
                }
            }
        }
    }
}

use peoplemodeler_core::models::Relationship;

fn render_rel_item(rel: &Relationship, person_id: &str) -> Element {
    let other_id = if rel.source_id == person_id { &rel.target_id } else { &rel.source_id };
    let other = db::person(other_id);
    let dir = if rel.source_id == person_id { "→" } else { "←" };
    rsx! {
        div { class: "relationship-item",
            if let Some(ref o) = other {
                Link { to: Route::PersonDetail { id: other_id.to_string() }, "{o.avatar_emoji} {o.name}" }
            } else {
                span { "{other_id}" }
            }
            span { " {dir} " }
            span { class: "tag", "{rel.r#type}" }
            if !rel.notes.is_empty() {
                p { class: "note", "{rel.notes}" }
            }
        }
    }
}

#[component]
fn RelationshipSection(person_id: String, rels: Vec<Relationship>, rel_person_rel: String, rel_none: String, rel_title: String) -> Element {
    rsx! {
        div { class: "section",
            h2 { "{rel_person_rel}" }
            if rels.is_empty() {
                p { "{rel_none}" }
            } else {
                for rel in rels {
                    {render_rel_item(&rel, &person_id)}
                }
            }
            div { class: "section" }
            Link { to: Route::Relationships {}, class: "btn", "{rel_title}" }
        }
    }
}

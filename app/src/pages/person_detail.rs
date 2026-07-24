use dioxus::prelude::*;
use peoplemodeler_core::models::{
    BehaviorTrigger, Person, Prediction, RelationType, Relationship, RepDim,
};
use peoplemodeler_core::synergy::{compute_person_profile, synergy_bands};

use crate::Route;
use crate::db;
use crate::i18n::Lang;
use crate::pages::predictions::{PredictionList, format_date};

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
    let mut prev_id = use_signal(|| id.clone());
    if prev_id() != id {
        person_sig.set(db::person(&id));
        prev_id.set(id.clone());
    }
    let mut tab = use_signal(|| Tab::Motivations);
    let mut toast_sig = use_context::<Signal<Option<String>>>();
    let not_found = crate::i18n::tr("person_not_found", lang());
    let tag_filter = use_context::<Signal<Option<String>>>();

    let person_guard = person_sig.read();
    let p = &*person_guard;
    match p {
        None => rsx! { div { class: "page", h2 { "{not_found}" } } },
        Some(person) => {
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
            let res_label = crate::i18n::tr("resilience_label", lang());
            let risk_label = crate::i18n::tr("risk_appetite_label", lang());
            let comp_label = crate::i18n::tr("profile_completeness", lang());
            let compare_btn = crate::i18n::tr("compare_btn", lang());
            let no_pat = crate::i18n::tr("no_patterns", lang());
            let log_title = crate::i18n::tr("log_title", lang());
            let log_placeholder = crate::i18n::tr("log_placeholder", lang());
            let log_add = crate::i18n::tr("log_add", lang());
            let log_empty = crate::i18n::tr("log_empty", lang());
            let rel_person_rel = crate::i18n::tr("rel_person_rel", lang());
            let rel_none = crate::i18n::tr("rel_none", lang());
            let rel_title = crate::i18n::tr("rel_title", lang());
            let rel_notes = crate::i18n::tr("rel_notes", lang());
            let rel_confirm_delete = crate::i18n::tr("rel_confirm_delete", lang());
            let rel_open_add = crate::i18n::tr("rel_open_add", lang());
            let rel_close_add = crate::i18n::tr("rel_close_add", lang());
            let rel_search_placeholder = crate::i18n::tr("rel_search_placeholder", lang());
            let common_add = crate::i18n::tr("common_add", lang());
            let common_save = crate::i18n::tr("common_save", lang());
            let common_cancel = crate::i18n::tr("common_cancel", lang());
            let common_edit = crate::i18n::tr("common_edit", lang());
            let common_delete = crate::i18n::tr("common_delete", lang());

            let mut preds = use_signal(|| db::predictions_for_person(&id));
            let mut ctx = use_signal(String::new);
            let mut predicted = use_signal(String::new);
            let ctx_pl = crate::i18n::tr("pred_context_placeholder", lang());
            let outcome_pl = crate::i18n::tr("pred_outcome_placeholder", lang());
            let add_btn = crate::i18n::tr("pred_add_btn", lang());

            let mut comparing = use_signal(|| false);
            let other_persons = use_signal(db::all_persons);
            let mut log_text = use_signal(String::new);

            let mut trigger = use_signal(|| BehaviorTrigger::Stress);
            let observed_label = crate::i18n::tr("insights_observed", lang());
            let insight_output =
                crate::pages::insights::generate_insight(person, &trigger(), lang());

            let cl = core_lang(lang());
            let profile_score = compute_person_profile(person);
            let self_score_label = crate::i18n::tr("person_self_score", lang());
            let bands = synergy_bands();
            let active_band = bands
                .iter()
                .position(|&(lo, hi)| profile_score.total >= lo && profile_score.total <= hi)
                .unwrap_or(2);
            let band_keys = [
                "scale_tension",
                "scale_friction",
                "scale_moderate",
                "scale_good",
                "scale_strong",
            ];
            let band_cls = [
                "ps-tension",
                "ps-friction",
                "ps-moderate",
                "ps-good",
                "ps-strong",
            ];
            let band_label = crate::i18n::tr(band_keys[active_band], lang());
            let ps_breakdown: Vec<(&'static str, u8)> = vec![
                (
                    crate::i18n::tr("compare_cat_ocean", lang()),
                    (profile_score.ocean * 100.0).round() as u8,
                ),
                (
                    crate::i18n::tr("compare_cat_reputation", lang()),
                    (profile_score.reputation * 100.0).round() as u8,
                ),
                (
                    crate::i18n::tr("compare_cat_motivation", lang()),
                    (profile_score.motivation * 100.0).round() as u8,
                ),
                (
                    crate::i18n::tr("compare_cat_patterns", lang()),
                    (profile_score.patterns * 100.0).round() as u8,
                ),
                (
                    crate::i18n::tr("compare_cat_bias", lang()),
                    (profile_score.bias * 100.0).round() as u8,
                ),
            ];
            let rep_items: Vec<_> = RepDim::ALL
                .iter()
                .filter_map(|d| {
                    let score = person.rep_scores.score(*d)?;
                    Some((d, score))
                })
                .collect();

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
                if let Err(e) = db::save_prediction(&pred) {
                    toast_sig.set(Some(format!(
                        "{}: {e}",
                        crate::i18n::tr("toast_error", lang())
                    )));
                    return;
                }
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
                                match db::delete_person(&id) {
                                    Ok(()) => {
                                        toast_sig.set(Some(crate::i18n::tr("toast_deleted", lang()).into()));
                                        navigator().push(Route::PeopleList {});
                                    }
                                    Err(e) => toast_sig.set(Some(format!("{}: {e}", crate::i18n::tr("toast_error", lang())))),
                                }
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
                                        let t = tag.name.clone();
                                        let mut tf = tag_filter;
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
                        if let Some(r) = person.resilience {
                            div { class: "confidence-badge",
                                span { "{res_label}: {r}/10" }
                            }
                        }
                        if let Some(r) = person.risk_appetite {
                            div { class: "confidence-badge",
                                span { "{risk_label}: {r}/10" }
                            }
                        }
                        div { class: "completeness-badge",
                            span { "{comp_label} {profile_score.completeness}%" }
                        }
                        div { class: "profile-score",
                            div { class: "ps-band {band_cls[active_band]}", "{band_label}" }
                            div { class: "ps-score", "{profile_score.total}" }
                            div { class: "ps-range", "{self_score_label} /100" }
                            div { class: "ps-bar-wrap",
                                div { class: "ps-bar",
                                    for _ in 0..5 {
                                        div { class: "ps-seg" }
                                    }
                                    div { class: "ps-dot", style: "left: {profile_score.total}%" }
                                }
                            }
                        }
                        div { class: "ps-breakdown",
                            for (label, val) in &ps_breakdown {
                                div { class: "ps-bd-row",
                                    span { class: "ps-bd-label", "{label}" }
                                    span { class: "ps-bd-bar-wrap",
                                        span { class: "ps-bd-bar", style: "width: {val}%" }
                                    }
                                    span { class: "ps-bd-val", "{val}/100" }
                                }
                            }
                        }
                    }

                    div { class: "tab-bar", role: "tablist",
                        button { role: "tab", aria_label: "{mot_title}", aria_selected: tab() == Tab::Motivations, class: if tab() == Tab::Motivations { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Motivations), "💡 {mot_title}" }
                        button { role: "tab", aria_label: "{bias_title}", aria_selected: tab() == Tab::Biases, class: if tab() == Tab::Biases { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Biases), "🧠 {bias_title}" }
                        button { role: "tab", aria_label: "{ocean_title}", aria_selected: tab() == Tab::Ocean, class: if tab() == Tab::Ocean { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Ocean), "🌊 {ocean_title}" }
                        button { role: "tab", aria_label: "{pred_title}", aria_selected: tab() == Tab::Predictions, class: if tab() == Tab::Predictions { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Predictions), "🔮 {pred_title}" }
                        button { role: "tab", aria_label: "{insights_title}", aria_selected: tab() == Tab::Insights, class: if tab() == Tab::Insights { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Insights), "✨ {insights_title}" }
                        button { role: "tab", aria_label: "{log_title}", aria_selected: tab() == Tab::Log, class: if tab() == Tab::Log { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Log), "{log_title}" }
                        button { role: "tab", aria_label: "{rel_title}", aria_selected: tab() == Tab::Relationships, class: if tab() == Tab::Relationships { "tab active" } else { "tab" }, onclick: move |_| tab.set(Tab::Relationships), "{rel_person_rel}" }
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
                            if person.motivations.is_empty() { div { class: "empty-state", p { "{no_mot}" } } }
                        }
                        div { class: "section",
                            h2 { "{pat_title}" }
                            for bp in &person.behavioral_patterns {
                            div { class: "pattern-item",
                                strong { { crate::pages::insights::trigger_label(&bp.trigger, lang()) } }
                                p { "{bp.predicted_behavior.label(cl)}" }
                                if !bp.notes.is_empty() { p { class: "item-notes", "{bp.notes}" } }
                            }
                        }
                            if person.behavioral_patterns.is_empty() { div { class: "empty-state", p { "{no_pat}" } } }
                        }
                        div { class: "section",
                            h2 { "{crate::i18n::tr(\"style_panel_title\", lang())}" }
                            for s in &person.styles {
                                div { class: "motivation-item",
                                    div { class: "item-icon", "{s.r#type.emoji()}" }
                                    div { class: "item-info",
                                        div { class: "item-name", "{s.r#type.i18n_label(cl)}" }
                                        div { class: "item-bar-row",
                                            div { class: "item-bar",
                                                div { class: "item-bar-fill purple", style: "width: {s.intensity * 10}%" }
                                            }
                                            div { class: "item-intensity", "{s.intensity}/10" }
                                        }
                                        if !s.notes.is_empty() { div { class: "item-notes", "{s.notes}" } }
                                    }
                                }
                            }
                            if person.styles.is_empty() { div { class: "empty-state", p { "{crate::i18n::tr(\"style_no_styles\", lang())}" } } }
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
                            if person.biases.is_empty() { div { class: "empty-state", p { "{no_bias}" } } }
                        }
                        div { class: "section",
                            h2 { "{rep_title}" }
                            {rep_items.iter().map(|(dim, score)| {
                                let ri = dim.i18n(cl);
                                let label = if *score >= 5 {
                                    format!("{} {} {}/10", ri.label_a, dim.emoji(), score)
                                } else {
                                    format!("{} {} {}/10", ri.label_b, dim.emoji(), score)
                                };
                                rsx! {
                                    div { key: "{dim:?}", class: "bias-item",
                                        div { class: "item-info",
                                            div { class: "item-name", "{label}" }
                                            div { class: "item-bar-row",
                                                div { class: "item-bar",
                                                    div { class: "item-bar-fill red", style: "width: {score * 10}%" }
                                                }
                                                div { class: "item-intensity", "{score}/10" }
                                            }
                                        }
                                    }
                                }
                            })}
                            {if rep_items.is_empty() {
                                rsx! { p { "{no_rep}" } }
                            } else {
                                rsx! {}
                            }}
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
                                        onclick: move |_| trigger.set(t),
                                        { crate::pages::insights::trigger_label(&t, lang()) }
                                    }
                                }
                            }
                            div { class: "insight-content",
                                div { class: "top-rec",
                                    span { class: "rec-icon", "💡" }
                                    div { class: "rec-text",
                                        for line in insight_output.top.lines() {
                                            if line.is_empty() { br {} }
                                            else { "{line}" br {} }
                                        }
                                    }
                                }
                                if insight_output.has_secondary {
                                    details { class: "more-recs",
                                        summary {
                                            span { "{crate::i18n::tr(\"more_recs\", lang())} (" "{insight_output.secondary.len() - 1}" ")" }
                                        }
                                        ul {
                                            for s in &insight_output.secondary[1..] {
                                                li { "{s}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !person.behavioral_patterns.is_empty() {
                            div { class: "card",
                                h3 { "{observed_label}" }
                                for bp in &person.behavioral_patterns {
                                    div { class: "pattern-item",
                                        strong { { crate::pages::insights::trigger_label(&bp.trigger, lang()) } }
                                p { "{bp.predicted_behavior.label_bare(cl)}" }
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
                                            if let Err(e) = db::save_person(p) {
                                                toast_sig.set(Some(format!("{}: {e}", crate::i18n::tr("toast_error", lang()))));
                                                return;
                                            }
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
                                                            if let Err(e) = db::save_person(p) {
                                                                toast_sig.set(Some(format!("{}: {e}", crate::i18n::tr("toast_error", lang()))));
                                                                return;
                                                            }
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
                        RelationshipSection { person: person.clone(), person_id: id.clone(), rel_person_rel, rel_none, rel_title, rel_notes, rel_confirm_delete, rel_open_add, rel_close_add, rel_search_placeholder, common_add, common_save, common_cancel, common_edit, common_delete }
                    }

                    Link { to: Route::PersonEdit { id: id.clone() }, class: "fab", aria_label: "{common_edit}", "✏" }
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

    let cx = 110.0;
    let cy = 110.0;
    let r = 80.0;

    use std::f64::consts::PI;
    let pts: Vec<(f64, f64)> = scores
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
            let pr = r * s.unwrap_or(0) as f64 / 10.0;
            (cx + pr * a.cos(), cy + pr * a.sin())
        })
        .collect();
    let data_poly: String = pts
        .iter()
        .map(|(x, y)| format!("{:.1},{:.1}", x, y))
        .collect::<Vec<_>>()
        .join(" ");
    let data_poly = format!("{} {:.1},{:.1}", data_poly, pts[0].0, pts[0].1);
    let label_pos: Vec<(f64, f64)> = (0..5).map(|i| axis_label(cx, cy, r, i)).collect();
    let total: u8 = scores.iter().filter_map(|s| *s).sum();
    let count = scores.iter().filter(|s| s.is_some()).count().max(1);
    let avg_score = total / count as u8;

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
                        "{avg_score}/10"
                    }
                }
            }
        }
    }
}

type RelGroup = Vec<(RelationType, Vec<(String, String, bool)>)>;

fn group_relationships(rels: Vec<Relationship>, person_id: &str) -> RelGroup {
    let edges: Vec<(String, String, bool, RelationType)> = rels
        .into_iter()
        .map(|rel| {
            let is_outgoing = rel.source_id == person_id;
            let other_id = if is_outgoing {
                rel.target_id
            } else {
                rel.source_id
            };
            (rel.id, other_id, is_outgoing, rel.r#type)
        })
        .collect();
    let mut groups: RelGroup = Vec::new();
    for (rel_id, other_id, is_outgoing, rel_type) in edges {
        if let Some(pos) = groups.iter().position(|(t, _)| *t == rel_type) {
            groups[pos].1.push((rel_id, other_id, is_outgoing));
        } else {
            groups.push((rel_type, vec![(rel_id, other_id, is_outgoing)]));
        }
    }
    groups
}

const TYPE_COLORS: [&str; 8] = [
    "var(--cyan)",
    "var(--orange)",
    "var(--green)",
    "var(--pink)",
    "var(--purple)",
    "var(--gold)",
    "var(--teal)",
    "var(--blue)",
];

fn match_type(s: &str) -> RelationType {
    match s {
        "Manages" => RelationType::Manages,
        "ReportsTo" => RelationType::ReportsTo,
        "Friends" => RelationType::Friends,
        "Family" => RelationType::Family,
        "Partner" => RelationType::Partner,
        "Mentors" => RelationType::Mentors,
        "Collaborates" => RelationType::Collaborates,
        _ => RelationType::WorksWith,
    }
}

#[component]
fn RelationshipSection(
    person: Person,
    person_id: String,
    rel_person_rel: String,
    rel_none: String,
    rel_title: String,
    rel_notes: String,
    rel_confirm_delete: String,
    rel_open_add: String,
    rel_close_add: String,
    rel_search_placeholder: String,
    common_add: String,
    common_save: String,
    common_cancel: String,
    common_edit: String,
    common_delete: String,
) -> Element {
    let nav = use_navigator();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let mut toast_sig = use_context::<Signal<Option<String>>>();
    let persons = use_signal(db::all_persons);
    let mut all_rels = use_signal(db::all_relationships);

    let mut refresh = move || all_rels.set(db::all_relationships());

    let filtered = all_rels()
        .into_iter()
        .filter(|r| r.source_id == person_id || r.target_id == person_id)
        .collect::<Vec<_>>();
    let type_groups = group_relationships(filtered, &person_id);

    // Add form
    let mut adding = use_signal(|| false);
    let mut search_text = use_signal(String::new);
    let mut selected_ids = use_signal(std::collections::HashSet::<String>::new);
    let mut new_type = use_signal(|| RelationType::WorksWith);
    let mut new_notes = use_signal(String::new);

    // Edit
    let mut editing_id = use_signal(String::new);
    let mut edit_type = use_signal(|| RelationType::WorksWith);
    let mut edit_notes = use_signal(String::new);

    // Delete
    let mut confirm_del = use_signal(String::new);

    let pid = person_id.clone();
    let mut add_rel = move || {
        let ids: Vec<String> = selected_ids().into_iter().collect();
        if ids.is_empty() {
            return;
        }
        let rel_type = new_type();
        let notes = new_notes();
        for target in ids {
            if target == pid {
                continue;
            }
            let rel = Relationship {
                id: uuid::Uuid::new_v4().to_string(),
                source_id: pid.clone(),
                target_id: target,
                r#type: rel_type,
                notes: notes.clone(),
                created_at: chrono::Utc::now().timestamp_millis(),
            };
            db::save_relationship(&rel).unwrap_or_else(|e| {
                toast_sig.set(Some(format!(
                    "{}: {e}",
                    crate::i18n::tr("toast_error", lang())
                )))
            });
        }
        refresh();
        selected_ids.set(std::collections::HashSet::new());
        new_type.set(RelationType::WorksWith);
        new_notes.set(String::new());
        adding.set(false);
    };

    let mut start_edit = move |rel_id: String| {
        if let Some(rel) = all_rels().iter().find(|r| r.id == rel_id) {
            editing_id.set(rel_id);
            edit_type.set(rel.r#type);
            edit_notes.set(rel.notes.clone());
        }
    };

    let mut save_edit = move || {
        let id = editing_id();
        if id.is_empty() {
            return;
        }
        if let Some(rel) = all_rels().iter().find(|r| r.id == id) {
            let mut updated = rel.clone();
            updated.r#type = edit_type();
            updated.notes = edit_notes();
            db::save_relationship(&updated).unwrap_or_else(|e| {
                toast_sig.set(Some(format!(
                    "{}: {e}",
                    crate::i18n::tr("toast_error", lang())
                )))
            });
            refresh();
        }
        editing_id.set(String::new());
    };

    let mut cancel_edit = move || editing_id.set(String::new());
    let mut confirm_delete = move |rel_id: String| confirm_del.set(rel_id);
    let mut cancel_delete = move || confirm_del.set(String::new());

    let mut execute_delete = move || {
        let id = confirm_del();
        if !id.is_empty() {
            db::delete_relationship(&id).unwrap_or_else(|e| {
                toast_sig.set(Some(format!(
                    "{}: {e}",
                    crate::i18n::tr("toast_error", lang())
                )))
            });
            refresh();
            confirm_del.set(String::new());
        }
    };

    rsx! {
        div { class: "section",
            h2 { "{rel_person_rel}" }

            div { class: "rel-controls",
                button {
                    class: "btn",
                    onclick: move |_| adding.set(!adding()),
                    if adding() { "{rel_close_add}" } else { "{rel_open_add}" }
                }
            }

            if adding() {
                {
                let person_options: Vec<Person> = persons().into_iter()
                    .filter(|p| p.id != person_id)
                    .filter(|p| {
                        let q = search_text().to_lowercase();
                        q.is_empty() || p.name.to_lowercase().contains(&q)
                    })
                    .collect();
                let selected_count = selected_ids().len();
                rsx! {
                    div { class: "rel-add-form",
                        input {
                            class: "rel-autocomplete-input",
                            placeholder: "{rel_search_placeholder}",
                            value: "{search_text}",
                            oninput: move |e| search_text.set(e.value()),
                        }
                        div { class: "rel-person-check-list",
                            for p in &person_options {
                                {
                                let checked = selected_ids().contains(&p.id);
                                rsx! {
                                    div {
                                        key: "{p.id}",
                                        class: "rel-person-check-row",
                                        onclick: {
                                            let pid = p.id.clone();
                                            move |_| {
                                                let mut s = selected_ids();
                                                if s.contains(&pid) { s.remove(&pid); }
                                                else { s.insert(pid.clone()); }
                                                selected_ids.set(s);
                                            }
                                        },
                                        input {
                                            r#type: "checkbox",
                                            checked: checked,
                                        }
                                        span { "{p.avatar_emoji} {p.name}" }
                                    }
                                }
                                }
                            }
                        }
                        div { class: "rel-add-actions",
                            select {
                                class: "rel-type-select",
                                value: "{new_type():?}",
                                onchange: move |e| new_type.set(match_type(&e.value())),
                                for rt in RelationType::ALL {
                                                                option { value: "{rt:?}", "{rt.label(cl)}" }
                                }
                            }
                            input {
                                class: "rel-notes-input",
                                placeholder: "{rel_notes}",
                                value: "{new_notes}",
                                oninput: move |e| new_notes.set(e.value()),
                            }
                            button {
                                class: "btn btn-primary",
                                disabled: selected_count == 0,
                                onclick: move |_| add_rel(),
                                "{common_add}",
                                if selected_count > 0 {
                                    " ({selected_count})"
                                }
                            }
                        }
                    }
                }
                }
            }

            if type_groups.is_empty() {
                p { "{rel_none}" }
            } else {
                div { class: "rel-cards",
                    for (t_idx, (rel_type, people)) in type_groups.iter().enumerate() {
                        {
                        let color = TYPE_COLORS[t_idx % TYPE_COLORS.len()];
                        rsx! {
                            div {
                                key: "{rel_type}",
                                class: "rel-type-card",
                                style: "border-color: {color}",
                                h3 { class: "rel-type-title", style: "color: {color}", "{rel_type.label(cl)}" }
                                div { class: "rel-people",
                                    for (rel_id, other_id, is_outgoing) in people {
                                        {
                                        let rid = rel_id.clone();
                                        let oid = other_id.clone();
                                        let matched = persons().iter().find(|p| p.id == oid).cloned();
                                        rsx! {
                                            div {
                                                key: "{rel_id}",
                                                class: "rel-person-row",
                                                if editing_id() == rid {
                                                    div { class: "rel-edit-row",
                                                        select {
                                                            value: "{edit_type():?}",
                                                            onchange: move |e| edit_type.set(match_type(&e.value())),
                                                            for rt in RelationType::ALL {
                                    option { value: "{rt:?}", "{rt.label(cl)}" }
                                                            }
                                                        }
                                                        input {
                                                            value: "{edit_notes}",
                                                            oninput: move |e| edit_notes.set(e.value()),
                                                        }
                                                        button {
                                                            class: "btn btn-small btn-primary",
                                                            onclick: move |_| save_edit(),
                                                            "{common_save}"
                                                        }
                                                        button {
                                                            class: "btn btn-small",
                                                            onclick: move |_| cancel_edit(),
                                                            "{common_cancel}"
                                                        }
                                                    }
                                                } else {
                                                    if let Some(ref o) = matched {
                                                        span {
                                                            class: "rel-person",
                                                            onclick: move |_| { let _ = nav.push(Route::PersonDetail { id: oid.clone() }); },
                                                            "{o.avatar_emoji} {o.name}"
                                                        }
                                                    } else {
                                                        span { class: "rel-person", "{oid}" }
                                                    }

                                                    span { class: "rel-direction", if *is_outgoing { "→" } else { "←" } }

                                                    if confirm_del() == rid {
                                                        span { class: "rel-confirm-delete",
                                                            button {
                                                                class: "btn btn-small btn-danger",
                                                                onclick: move |_| execute_delete(),
                                                                "{rel_confirm_delete}"
                                                            }
                                                            button {
                                                                class: "btn btn-small",
                                                                onclick: move |_| cancel_delete(),
                                                                "{common_cancel}"
                                                            }
                                                        }
                                                    } else {
                                                        span { class: "rel-person-actions",
                                                            button {
                                                                class: "btn-icon",
                                                                title: "{common_edit}",
                                                                onclick: {
                                                                    let rid2 = rid.clone();
                                                                    move |_| start_edit(rid2.clone())
                                                                },
                                                                "✏"
                                                            }
                                                            button {
                                                                class: "btn-icon",
                                                                title: "{common_delete}",
                                                                onclick: {
                                                                    let rid2 = rid.clone();
                                                                    move |_| confirm_delete(rid2.clone())
                                                                },
                                                                "✕"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        }
                                    }
                                }
                            }
                        }
                        }
                    }
                }
            }

            Link { to: Route::Relationships {}, class: "btn", "{rel_title}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::RelationType;

    fn rel(source: &str, target: &str, r#type: RelationType) -> Relationship {
        Relationship {
            id: String::new(),
            source_id: source.to_string(),
            target_id: target.to_string(),
            r#type,
            notes: String::new(),
            created_at: 0,
        }
    }

    #[test]
    fn group_empty_rels() {
        let g = group_relationships(vec![], "alice");
        assert!(g.is_empty());
    }

    #[test]
    fn group_single_rel() {
        let g = group_relationships(vec![rel("alice", "bob", RelationType::WorksWith)], "alice");
        assert_eq!(g.len(), 1);
        assert_eq!(format!("{}", g[0].0), "WorksWith");
        assert_eq!(g[0].1.len(), 1);
        assert_eq!(g[0].1[0].1, "bob");
        assert!(g[0].1[0].2);
    }

    #[test]
    fn group_same_type_merged() {
        let g = group_relationships(
            vec![
                rel("alice", "bob", RelationType::Friends),
                rel("alice", "carol", RelationType::Friends),
            ],
            "alice",
        );
        assert_eq!(g.len(), 1);
        assert_eq!(format!("{}", g[0].0), "Friends");
        assert_eq!(g[0].1.len(), 2);
    }

    #[test]
    fn group_different_types_separate() {
        let g = group_relationships(
            vec![
                rel("alice", "bob", RelationType::WorksWith),
                rel("alice", "carol", RelationType::Friends),
            ],
            "alice",
        );
        assert_eq!(g.len(), 2);
        assert_eq!(format!("{}", g[0].0), "WorksWith");
        assert_eq!(format!("{}", g[1].0), "Friends");
    }

    #[test]
    fn group_incoming_rel() {
        let g = group_relationships(vec![rel("bob", "alice", RelationType::Manages)], "alice");
        assert_eq!(g.len(), 1);
        assert!(!g[0].1[0].2);
    }

    #[test]
    fn group_preserves_type_order() {
        let g = group_relationships(
            vec![
                rel("alice", "bob", RelationType::Family),
                rel("alice", "carol", RelationType::WorksWith),
                rel("alice", "dave", RelationType::Partner),
            ],
            "alice",
        );
        assert_eq!(format!("{}", g[0].0), "Family");
        assert_eq!(format!("{}", g[1].0), "WorksWith");
        assert_eq!(format!("{}", g[2].0), "Partner");
    }
}

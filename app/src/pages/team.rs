use std::collections::HashMap;

use dioxus::prelude::*;
use peoplemodeler_core::models::AVATAR_EMOJIS;
use peoplemodeler_core::synergy::{compute_team_synergy, synergy_bands};

use crate::Route;
use crate::db;
use crate::i18n::Lang;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Synergy,
    Members,
}

#[component]
pub fn TeamDetail(id: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let title = crate::i18n::tr("team_title", lang());
    let empty = crate::i18n::tr("team_empty", lang());
    let size_label = crate::i18n::tr("team_size", lang());
    let avg_label = crate::i18n::tr("team_avg_score", lang());
    let strongest_label = crate::i18n::tr("team_strongest", lang());
    let weakest_label = crate::i18n::tr("team_weakest", lang());
    let max_danger_label = crate::i18n::tr("team_max_danger", lang());
    let avg_danger_label = crate::i18n::tr("team_avg_danger", lang());
    let ctx_avg_title = crate::i18n::tr("team_ctx_avg", lang());
    let pairs_title = crate::i18n::tr("team_pairs", lang());
    let no_danger = crate::i18n::tr("team_no_danger", lang());
    let tab_synergy = crate::i18n::tr("team_tab_synergy", lang());
    let tab_members = crate::i18n::tr("team_tab_members", lang());
    let all_no_edit = crate::i18n::tr("team_all_no_edit", lang());
    let members_count_fmt = crate::i18n::tr("team_members_count", lang());
    let team_rename_label = crate::i18n::tr("team_rename", lang());
    let team_icon_label = crate::i18n::tr("team_icon", lang());
    let team_edit_label = crate::i18n::tr("team_edit", lang());
    let form_save = crate::i18n::tr("common_save", lang());
    let form_cancel = crate::i18n::tr("common_cancel", lang());

    let is_all = id == "all";
    let team_name = if is_all {
        crate::i18n::tr("teams_all", lang()).to_string()
    } else {
        db::team(&id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| title.to_string())
    };

    let mut tab = use_signal(|| Tab::Synergy);
    let mut team_sig = use_signal(|| db::team(&id));
    let all_persons = use_signal(db::all_persons);
    let mut editing = use_signal(|| false);
    let mut edit_name = use_signal(|| team_name.clone());
    let mut edit_icon = use_signal(|| {
        db::team(&id)
            .map(|t| {
                if t.icon.is_empty() {
                    "🎯".to_string()
                } else {
                    t.icon
                }
            })
            .unwrap_or_else(|| "🎯".to_string())
    });

    let team_guard = team_sig.read();
    let member_ids: Vec<String> = team_guard
        .as_ref()
        .map(|t| t.member_ids.clone())
        .unwrap_or_default();
    let member_count_str = members_count_fmt.replace("{0}", &member_ids.len().to_string());

    let persons = if is_all {
        db::all_persons()
    } else {
        let all = db::all_persons();
        all.into_iter()
            .filter(|p| member_ids.contains(&p.id))
            .collect()
    };
    let current_icon = team_guard
        .as_ref()
        .map(|t| t.icon.clone())
        .unwrap_or_default();
    let display_icon = if current_icon.is_empty() {
        "🎯".to_string()
    } else {
        current_icon
    };
    drop(team_guard);
    let rels = db::all_relationships();
    let all_preds = db::all_predictions();
    let preds_map: HashMap<String, Vec<_>> = {
        let mut m: HashMap<String, Vec<_>> = HashMap::new();
        for p in &all_preds {
            m.entry(p.person_id.clone()).or_default().push(p.clone());
        }
        m
    };
    let team = compute_team_synergy(&persons, &rels, &preds_map);

    rsx! {
        div { class: "page",
            div { class: "team-detail-header",
                if editing() {
                    div { class: "team-edit-row",
                        div { class: "emoji-picker", role: "radiogroup", aria_label: "{team_icon_label}",
                            for e in AVATAR_EMOJIS {
                                button {
                                    class: "emoji-btn",
                                    class: if edit_icon() == *e { "selected" },
                                    role: "radio",
                                    aria_label: "Icon {e}",
                                    aria_checked: if edit_icon() == *e { "true" } else { "false" },
                                    onclick: move |_| edit_icon.set(e.to_string()),
                                    "{e}"
                                }
                            }
                        }
                        input {
                            id: "team-rename-input",
                            aria_label: "{team_rename_label}",
                            value: "{edit_name()}",
                            oninput: move |e: Event<FormData>| edit_name.set(e.value()),
                            onkeydown: move |e: Event<KeyboardData>| {
                                if e.key() == Key::Enter {
                                    let name = edit_name.read().trim().to_string();
                                    if name.is_empty() { return; }
                                    if let Some(ref mut t) = *team_sig.write() {
                                        t.name = name;
                                        t.icon = edit_icon();
                                        let _ = db::save_team(t);
                                    }
                                    editing.set(false);
                                } else if e.key() == Key::Escape {
                                    editing.set(false);
                                }
                            }
                        }
                        button { class: "btn btn-primary btn-sm",
                            onclick: move |_| {
                                let name = edit_name.read().trim().to_string();
                                if name.is_empty() { return; }
                                if let Some(ref mut t) = *team_sig.write() {
                                    t.name = name;
                                    t.icon = edit_icon();
                                    let _ = db::save_team(t);
                                }
                                editing.set(false);
                            },
                            "{form_save}"
                        }
                        button { class: "btn btn-ghost btn-sm",
                            onclick: move |_| editing.set(false),
                            "{form_cancel}"
                        }
                    }
                } else {
                    div { class: "team-name-row",
                        span { class: "teams-row-emoji", "{display_icon}" }
                        h2 { "{team_name}" }
                        if !is_all {
                            button { class: "btn btn-ghost btn-sm team-edit-btn",
                                onclick: move |_| {
                                    edit_name.set(team_name.clone());
                                    edit_icon.set(display_icon.clone());
                                    editing.set(true);
                                },
                                "{team_edit_label}"
                            }
                        }
                    }
                }
            }

            div { class: "tab-bar", role: "tablist",
                button { role: "tab",
                    aria_label: "{tab_synergy}",
                    aria_selected: tab() == Tab::Synergy,
                    class: if tab() == Tab::Synergy { "tab active" } else { "tab" },
                    onclick: move |_| tab.set(Tab::Synergy),
                    "📊 {tab_synergy}"
                }
                button { role: "tab",
                    aria_label: "{tab_members}",
                    aria_selected: tab() == Tab::Members,
                    class: if tab() == Tab::Members { "tab active" } else { "tab" },
                    onclick: move |_| tab.set(Tab::Members),
                    "👥 {tab_members}"
                }
            }

            if tab() == Tab::Synergy {
                {
                    match team {
                        None => rsx! {
                            div { class: "empty-state",
                                div { class: "empty-icon", "👥" }
                                p { "{empty}" }
                            }
                        },
                        Some(ts) => {
                            let bands = synergy_bands();
                            let band_cls = ["ps-tension", "ps-friction", "ps-moderate", "ps-good", "ps-strong"];
                            let max_danger_str = if ts.max_danger > 0.0 {
                                format!("{:.0}%", ts.max_danger * 100.0)
                            } else {
                                no_danger.to_string()
                            };
                            let avg_danger_str = if ts.avg_danger > 0.0 {
                                format!("{:.0}%", ts.avg_danger * 100.0)
                            } else {
                                no_danger.to_string()
                            };
                            rsx! {
                                div { class: "team-summary",
                                    div { class: "summary-card",
                                        span { class: "summary-value", "{ts.team_size}" }
                                        span { class: "summary-label", "{size_label}" }
                                    }
                                    div { class: "summary-card",
                                        span { class: "summary-value",
                                            "{ts.avg_score}%"
                                        }
                                        span { class: "summary-label", "{avg_label}" }
                                    }
                                    if let Some((ref a, ref b, score)) = ts.strongest {
                                        div { class: "summary-card highlight-good",
                                            span { class: "summary-value", "{score}%" }
                                            span { class: "summary-label", "{strongest_label}" }
                                            span { class: "summary-detail", "{a} ↔ {b}" }
                                        }
                                    }
                                    if let Some((ref a, ref b, score)) = ts.weakest {
                                        div { class: "summary-card highlight-warn",
                                            span { class: "summary-value", "{score}%" }
                                            span { class: "summary-label", "{weakest_label}" }
                                            span { class: "summary-detail", "{a} ↔ {b}" }
                                        }
                                    }
                                    div { class: "summary-card",
                                        span { class: "summary-value danger-val", "{max_danger_str}" }
                                        span { class: "summary-label", "{max_danger_label}" }
                                    }
                                    div { class: "summary-card",
                                        span { class: "summary-value danger-val", "{avg_danger_str}" }
                                        span { class: "summary-label", "{avg_danger_label}" }
                                    }
                                }

                                div { class: "team-section",
                                    h3 { "{ctx_avg_title}" }
                                    div { class: "ctx-bars",
                                        for (ctx, score) in &ts.context_averages {
                                            {
                                                let pct = *score as f64 / 101.0 * 100.0;
                                                let cls = bands
                                                    .iter()
                                                    .position(|(lo, hi)| *score >= *lo && *score <= *hi)
                                                    .map(|i| match i {
                                                        0 => "scale-tension",
                                                        1 => "scale-friction",
                                                        2 => "scale-moderate",
                                                        3 => "scale-good",
                                                        _ => "scale-strong",
                                                    })
                                                    .unwrap_or("scale-moderate");
                                                let ctx_label = crate::i18n::tr(ctx_key(*ctx), lang());
                                                rsx! {
                                                    div { class: "ctx-row",
                                                        span { class: "ctx-label", "{ctx_label}" }
                                                        div { class: "ctx-bar",
                                                            div { class: "ctx-fill {cls}", width: "{pct:.0}%" }
                                                        }
                                                        span { class: "ctx-pct", "{score}%" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                div { class: "team-section",
                                    h3 { "{pairs_title}" }
                                    div { class: "team-pairs-grid",
                                        for pair in &ts.pairs {
                                            {
                                                let score = pair.breakdown.total;
                                                let band_idx = bands
                                                    .iter()
                                                    .position(|(lo, hi)| score >= *lo && score <= *hi)
                                                    .unwrap_or(2);
                                                let score_cls = band_cls[band_idx];
                                                let has_danger = pair.breakdown.danger > 0.0;
                                                let danger_txt = pair.breakdown.danger_details.clone();
                                                let pa = pair.person_a.clone();
                                                let pb = pair.person_b.clone();
                                                let id_a = pair.id_a.clone();
                                                let id_b = pair.id_b.clone();
                                                rsx! {
                                                    Link {
                                                        to: Route::ComparePersons { id1: id_a, id2: id_b },
                                                        class: "team-pair-card",
                                                        div { class: "pair-header",
                                                            span { class: "pair-names", "{pa} ↔ {pb}" }
                                                            span { class: "pair-score {score_cls}", "{score}%" }
                                                        }
                                                        if has_danger {
                                                            div { class: "pair-danger",
                                                                "⚠ {danger_txt}"
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

            if tab() == Tab::Members {
                if is_all {
                    div { class: "empty-state",
                        div { class: "empty-icon", "👥" }
                        p { "{all_no_edit}" }
                    }
                } else {
                    div { class: "team-members-count",
                        "{member_count_str}"
                    }
                    div { class: "team-members-list",
                        for person in all_persons().iter() {
                            {
                                let is_member = member_ids.contains(&person.id);
                                let emoji = person.avatar_emoji.clone();
                                let name = person.name.clone();
                                let pid = person.id.clone();
                                rsx! {
                                    div { class: "team-member-row",
                                        label { class: "team-member-label",
                                            input {
                                                r#type: "checkbox",
                                                checked: is_member,
                                                aria_label: "Toggle {name}",
                                                onchange: {
                                                    let pid = pid.clone();
                                                    move |e: Event<FormData>| {
                                                        let checked = e.checked();
                                                        let mut team_guard = team_sig.write();
                                                        if let Some(ref mut t) = *team_guard {
                                                            if checked {
                                                                if !t.member_ids.contains(&pid) {
                                                                    t.member_ids.push(pid.clone());
                                                                }
                                                            } else {
                                                                t.member_ids.retain(|m| m != &pid);
                                                            }
                                                            let _ = db::save_team(t);
                                                        }
                                                    }
                                                }
                                            }
                                            span { class: "team-member-emoji", "{emoji}" }
                                            span { class: "team-member-name", "{name}" }
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

fn ctx_key(c: peoplemodeler_core::insights::InsightContext) -> &'static str {
    match c {
        peoplemodeler_core::insights::InsightContext::Decision => "ctx_decision",
        peoplemodeler_core::insights::InsightContext::Team => "ctx_team",
        peoplemodeler_core::insights::InsightContext::Stress => "ctx_stress",
        peoplemodeler_core::insights::InsightContext::Communication => "ctx_communication",
        peoplemodeler_core::insights::InsightContext::Leadership => "ctx_leadership",
        peoplemodeler_core::insights::InsightContext::Growth => "ctx_growth",
    }
}

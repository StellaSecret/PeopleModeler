use std::collections::HashMap;

use dioxus::prelude::*;
use peoplemodeler_core::synergy::{compute_team_synergy, synergy_bands};

use crate::db;
use crate::i18n::Lang;

#[component]
pub fn Team() -> Element {
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

    let persons = db::all_persons();
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
            h2 { "{title}" }
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
                                            let o = (pair.breakdown.ocean * 100.0).round() as u8;
                                            let r = (pair.breakdown.reputation * 100.0).round() as u8;
                                            let m = (pair.breakdown.motivation * 100.0).round() as u8;
                                            let p = (pair.breakdown.patterns * 100.0).round() as u8;
                                            let b = (pair.breakdown.bias * 100.0).round() as u8;
                                            let s = (pair.breakdown.styles * 100.0).round() as u8;
                                            let v = (pair.breakdown.values * 100.0).round() as u8;
                                            let has_danger = pair.breakdown.danger > 0.0;
                                            let danger_txt = pair.breakdown.danger_details.clone();
                                            let pa = pair.person_a.clone();
                                            let pb = pair.person_b.clone();
                                            rsx! {
                                                div { class: "team-pair-card",
                                                    div { class: "pair-header",
                                                        span { class: "pair-names", "{pa} ↔ {pb}" }
                                                        span { class: "pair-score {score_cls}", "{score}%" }
                                                    }
                                                    if has_danger {
                                                        div { class: "pair-danger",
                                                            "⚠ {danger_txt}"
                                                        }
                                                    }
                                                    div { class: "pair-breakdown-mini",
                                                        span { "O:{o}" }
                                                        span { "R:{r}" }
                                                        span { "M:{m}" }
                                                        span { "P:{p}" }
                                                        span { "B:{b}" }
                                                        span { "S:{s}" }
                                                        span { "V:{v}" }
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

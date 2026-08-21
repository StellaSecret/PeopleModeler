use dioxus::prelude::*;
use peoplemodeler_core::insights::InsightContext;
use peoplemodeler_core::models::{BehaviorTrigger, Person, RelationType};

use peoplemodeler_core::synergy::{
    RelContext, Trend, compute_synergy_score_ctx, compute_synergy_score_with_preds, synergy_bands,
};

use crate::db;
use crate::i18n::Lang;

fn core_lang(l: Lang) -> peoplemodeler_core::i18n::Lang {
    match l {
        Lang::Fr => peoplemodeler_core::i18n::Lang::Fr,
        Lang::En => peoplemodeler_core::i18n::Lang::En,
    }
}

/// i18n key for a per-context compatibility score label.
fn ctx_key(c: InsightContext) -> &'static str {
    match c {
        InsightContext::Decision => "ctx_decision",
        InsightContext::Team => "ctx_team",
        InsightContext::Stress => "ctx_stress",
        InsightContext::Communication => "ctx_communication",
        InsightContext::Leadership => "ctx_leadership",
        InsightContext::Growth => "ctx_growth",
    }
}

/// Prefill the relationship selector from an existing Relationship row between
/// the two ids (either direction).
fn prefill_rel(id1: &str, id2: &str) -> (Option<RelationType>, u8) {
    prefill_rel_from(&db::all_relationships(), id1, id2)
}

fn prefill_rel_from(
    rels: &[peoplemodeler_core::models::Relationship],
    id1: &str,
    id2: &str,
) -> (Option<RelationType>, u8) {
    for r in rels {
        if (r.source_id == id1 && r.target_id == id2) || (r.source_id == id2 && r.target_id == id1)
        {
            return (Some(r.r#type), r.strength);
        }
    }
    (None, 5)
}

fn benefit_labels(
    a_score: u8,
    b_score: u8,
    a_name: &str,
    b_name: &str,
    more_label: &str,
    balanced_label: &str,
) -> (String, String) {
    if a_score > b_score {
        (
            format!("(+{}% — {} {})", a_score - b_score, a_name, more_label),
            String::new(),
        )
    } else if b_score > a_score {
        (
            String::new(),
            format!("(+{}% — {} {})", b_score - a_score, b_name, more_label),
        )
    } else {
        (
            format!("({})", balanced_label),
            format!("({})", balanced_label),
        )
    }
}

fn format_band_label(band: u8, hint_template: &str) -> String {
    if band > 0 {
        hint_template.replacen("{}", &band.to_string(), 1)
    } else {
        String::new()
    }
}

fn format_signed_delta(delta: i8) -> String {
    if delta > 0 {
        format!("+{}", delta)
    } else if delta < 0 {
        format!("{}", delta)
    } else {
        String::new()
    }
}

fn should_show_extra_strategies(n: usize) -> bool {
    n > 1
}

#[component]
pub fn ComparePersons(id1: String, id2: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let nav = use_navigator();
    let p1 = use_signal(|| db::person(&id1));
    let p2 = use_signal(|| db::person(&id2));
    let (prefill_type, prefill_strength) = prefill_rel(&id1, &id2);
    let mut rel_type: Signal<Option<RelationType>> = use_signal(|| prefill_type);
    let mut rel_strength: Signal<u8> = use_signal(|| prefill_strength);
    let cl = core_lang(lang());
    let not_found = crate::i18n::tr("person_not_found", lang());
    let compare_title = crate::i18n::tr("compare_title", lang());
    let back_btn = crate::i18n::tr("common_back", lang());

    match (p1(), p2()) {
        (Some(a), Some(b)) => {
            let a_flags = peoplemodeler_core::validation::all_person_flags(&a);
            let b_flags = peoplemodeler_core::validation::all_person_flags(&b);
            let a_preds = db::predictions_for_person(&a.id);
            let b_preds = db::predictions_for_person(&b.id);
            let ctx = rel_type().map(|rtype| RelContext {
                rtype,
                strength: rel_strength().clamp(1, 10),
            });
            let brk = match &ctx {
                Some(rc) => compute_synergy_score_ctx(&a, &b, Some(rc), &a_preds, &b_preds),
                None => compute_synergy_score_with_preds(&a, &b, &a_preds, &b_preds),
            };
            let score = brk.total;
            let na = a.name.clone();
            let nb = b.name.clone();
            let (synergies, frictions, (top_strategy, all_strategies)) =
                compare_analysis(&a, &b, lang());
            let compare_sub = crate::i18n::tr("compare_sub", lang());
            let compare_vs = crate::i18n::tr("compare_vs", lang());
            let compare_asymmetric = crate::i18n::tr("compare_asymmetric", lang());
            let compare_benefit_more = crate::i18n::tr("compare_benefit_more", lang());
            let compare_balanced = crate::i18n::tr("compare_balanced", lang());
            let a_score = brk.a_score;
            let b_score = brk.b_score;
            let (a_benefit_label, b_benefit_label) = benefit_labels(
                a_score,
                b_score,
                &na,
                &nb,
                compare_benefit_more,
                compare_balanced,
            );
            let compare_breakdown = crate::i18n::tr("compare_breakdown", lang());
            let compare_ctx_title = crate::i18n::tr("compare_ctx_title", lang());
            let ctx_rows: Vec<(String, u8)> = brk
                .per_context
                .iter()
                .map(|(c, s)| (crate::i18n::tr(ctx_key(*c), lang()).to_string(), *s))
                .collect();
            let cat_ocean = crate::i18n::tr("compare_cat_ocean", lang());
            let cat_rep = crate::i18n::tr("compare_cat_reputation", lang());
            let cat_mot = crate::i18n::tr("compare_cat_motivation", lang());
            let cat_pat = crate::i18n::tr("compare_cat_patterns", lang());
            let cat_bias = crate::i18n::tr("compare_cat_bias", lang());
            let cat_styles = crate::i18n::tr("compare_cat_styles", lang());
            let cat_values = crate::i18n::tr("compare_cat_values", lang());
            let top_mot_label = crate::i18n::tr("compare_top_mot", lang());
            let bias_label = crate::i18n::tr("compare_bias_main", lang());
            let ocean_label = crate::i18n::tr("compare_ocean", lang());
            let analysis_title = crate::i18n::tr("compare_analysis_title", lang());
            let synergies_title = crate::i18n::tr("compare_synergies", lang());
            let friction_title = crate::i18n::tr("compare_friction", lang());
            let strategy_title = crate::i18n::tr("compare_strategy", lang());
            let ethics = crate::i18n::tr("compare_ethics", lang());
            let has_extra_strategies = should_show_extra_strategies(all_strategies.len());
            let rel_title = crate::i18n::tr("compare_rel_title", lang());
            let rel_none = crate::i18n::tr("compare_rel_none", lang());
            let rel_strength_label = crate::i18n::tr("compare_rel_strength", lang());
            let band_hint = crate::i18n::tr("compare_band_hint", lang());
            let band_label = format_band_label(brk.band, band_hint);
            let rel_cl = core_lang(lang());

            let trend_label = match brk.trajectory_trend {
                Trend::Improving => crate::i18n::tr("trend_improving", lang()),
                Trend::Stable => crate::i18n::tr("trend_stable", lang()),
                Trend::Deteriorating => crate::i18n::tr("trend_deteriorating", lang()),
            };
            let trend_cls = match brk.trajectory_trend {
                Trend::Improving => "trend-up",
                Trend::Stable => "trend-flat",
                Trend::Deteriorating => "trend-down",
            };
            let trend_arrow = match brk.trajectory_trend {
                Trend::Improving => "↑",
                Trend::Stable => "→",
                Trend::Deteriorating => "↓",
            };
            let trend_delta = format_signed_delta(brk.trajectory_delta);
            let trend_hint = crate::i18n::tr("trend_hint", lang());

            // Scale ruler — thresholds dynamically derived from sim formula
            let band_ranges = synergy_bands();
            let band_meta: [(&str, &str); 5] = [
                ("scale_tension", "scale-tension"),
                ("scale_friction", "scale-friction"),
                ("scale_moderate", "scale-moderate"),
                ("scale_good", "scale-good"),
                ("scale_strong", "scale-strong"),
            ];
            let scale_bands: [(&str, u8, u8, &str); 5] = std::array::from_fn(|i| {
                let (lo, hi) = band_ranges[i];
                (band_meta[i].0, lo, hi, band_meta[i].1)
            });
            let active_band = peoplemodeler_core::ocean::active_band_index(
                score,
                &[
                    (band_ranges[0].0, band_ranges[0].1),
                    (band_ranges[1].0, band_ranges[1].1),
                    (band_ranges[2].0, band_ranges[2].1),
                    (band_ranges[3].0, band_ranges[3].1),
                    (band_ranges[4].0, band_ranges[4].1),
                ],
                2,
            );

            rsx! {
                div { class: "page",
                    button { class: "btn", onclick: move |_| nav.go_back(), "{back_btn}" }
                    h2 { "{compare_title}" }
                    p { class: "compare-sub", "{compare_sub}" }

                    div { class: "compare-grid",
                        div { class: "compare-card",
                            PersonCard { person: a.clone() }
                            div { class: "compare-section",
                                h4 { "{top_mot_label}" }
                                if let Some(m) = a.top_motivation() {
                                    div { class: "compare-chip green",
                                        "{m.r#type.emoji()} {m.r#type.i18n(cl).label}: {m.intensity}/10"
                                    }
                                }
                            }
                            div { class: "compare-section",
                                h4 { "{bias_label}" }
                                if let Some(bias) = a.top_bias() {
                                    div { class: "compare-chip red",
                                        "{bias.r#type.emoji()} {bias.r#type.i18n(cl).label}: {bias.intensity}/10"
                                    }
                                }
                            }
                            div { class: "compare-section",
                                h4 { "{ocean_label}" }
                                MiniBars { scores: [
                                    a.ocean.openness, a.ocean.conscientiousness,
                                    a.ocean.extraversion, a.ocean.agreeableness,
                                    a.ocean.neuroticism,
                                ] }
                            }
                            {if a_flags.is_empty() {
                                rsx! {}
                            } else {
                                rsx! {
                                    div { class: "flag-chips",
                                        {a_flags.iter().map(|k| {
                                            let txt = crate::i18n::tr(k, lang());
                                            rsx! { div { class: "danger-warning", title: "{txt}", "⚠ {txt}" } }
                                        })}
                                    }
                                }
                            }}
                        }

                        div { class: "vs-divider",
                            div { class: "vs-text", "{compare_vs}" }
                            div { class: "rel-context-box",
                                div { class: "rel-context-title", "{rel_title}" }
                                div { class: "rel-context-row",
                                    select {
                                        onchange: move |e| {
                                            let v = e.value();
                                            if v == "none" {
                                                rel_type.set(None);
                                            } else {
                                                rel_type.set(RelationType::ALL.iter().find(|t| {
                                                    format!("{t:?}") == v
                                                }).copied());
                                            }
                                        },
                                        option { value: "none", selected: rel_type().is_none(), "{rel_none}" }
                                        {RelationType::ALL.iter().map(|rt| {
                                            let is_sel = rel_type() == Some(*rt);
                                            rsx! { option { value: "{rt:?}", selected: is_sel, "{rt.label(rel_cl)}" } }
                                        })}
                                    }
                                    if rel_type().is_some() {
                                        span { class: "rel-strength",
                                            "{rel_strength_label}: {rel_strength}/10"
                                        }
                                        input {
                                            r#type: "range", min: "1", max: "10", step: "1",
                                            value: "{rel_strength}",
                                            oninput: move |e| {
                                                if let Ok(v) = e.value().parse::<u8>() {
                                                    rel_strength.set(v.clamp(1, 10));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                            div { class: "compatibility-score",
                                div { class: "scale-ruler-hero",
                                    div { class: "scale-band-hero {scale_bands[active_band].3}",
                                        "{crate::i18n::tr(scale_bands[active_band].0, lang())}"
                                    }
                                    div { class: "scale-score",
                                        "{score}%"
                                        div { class: "scale-range",
                                            "{scale_bands[active_band].1}–{scale_bands[active_band].2}%"
                                        }
                                        if !band_label.is_empty() {
                                            div { class: "scale-band-hint", "{band_label}" }
                                        }
                                        if brk.trajectory_sample > 0 {
                                            div { class: "trend-chip {trend_cls}", title: "{trend_hint}",
                                                span { class: "trend-arrow", "{trend_arrow}" }
                                                span { class: "trend-text", "{trend_label}"
                                                    if !trend_delta.is_empty() {
                                                        " {trend_delta}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                 if brk.danger > 0.0 {
                                     div { class: "danger-warning", "⚠ {crate::i18n::tr_danger_details(&brk.danger_details, lang())}" }
                                  }
                                 div { class: "scale-ruler",
                                    div { class: "scale-bar",
                                        {scale_bands.iter().enumerate().map(|(i, (_, lo, hi, color))| {
                                            let pct = (hi - lo + 1) as f64 / 101.0 * 100.0;
                                            let cls = if i == active_band { "scale-band active" } else { "scale-band" };
                                            rsx! { div { class: "{cls} {color}", width: "{pct:.0}%" } }
                                        })}
                                        div { class: "scale-dot", style: "left: {score}%" }
                                    }
                                    div { class: "scale-labels",
                                        {scale_bands.iter().enumerate().map(|(i, (key, lo, hi, _))| {
                                            let pct = (hi - lo + 1) as f64 / 101.0 * 100.0;
                                            let label = crate::i18n::tr(key, lang());
                                            let cls = if i == active_band { "scale-lbl-wrap active" } else { "scale-lbl-wrap" };
                                            rsx! {
                                                div { class: "{cls}", width: "{pct:.0}%",
                                                    span { class: "scale-lbl", "{label}" }
                                                }
                                            }
                                        })}
                                    }
                                }
                                p { class: "asymmetric-title", "{compare_asymmetric}" }
                                div { class: "asymmetric-scores",
                                    span { class: "asym-row",
                                        span { class: "asym-direction", "{na} → {nb}" }
                                        span { class: "asym-value", "{a_score}%" }
                                        if !a_benefit_label.is_empty() {
                                            span { class: "asym-benefit", "{a_benefit_label}" }
                                        }
                                    }
                                    span { class: "asym-row",
                                        span { class: "asym-direction", "{nb} → {na}" }
                                        span { class: "asym-value", "{b_score}%" }
                                        if !b_benefit_label.is_empty() {
                                            span { class: "asym-benefit", "{b_benefit_label}" }
                                        }
                                    }
                                }
                            }
                            div { class: "breakdown-section",
                                h4 { "{compare_breakdown}" }
                                BreakdownBars {
                                    cat_ocean, cat_rep, cat_mot, cat_pat, cat_bias, cat_styles, cat_values,
                                    s_ocean: brk.ocean, s_rep: brk.reputation,
                                    s_mot: brk.motivation, s_pat: brk.patterns, s_bias: brk.bias,
                                    s_styles: brk.styles, s_values: brk.values,
                                    bias_mod_active: brk.bias_mod_active,
                                }
                            }
                            div { class: "ctx-section",
                                h4 { "{compare_ctx_title}" }
                                ContextBars { rows: ctx_rows }
                            }
                        }

                        div { class: "compare-card",
                            PersonCard { person: b.clone() }
                            div { class: "compare-section",
                                h4 { "{top_mot_label}" }
                                if let Some(m) = b.top_motivation() {
                                    div { class: "compare-chip green",
                                        "{m.r#type.emoji()} {m.r#type.i18n(cl).label}: {m.intensity}/10"
                                    }
                                }
                            }
                            div { class: "compare-section",
                                h4 { "{bias_label}" }
                                if let Some(bias) = b.top_bias() {
                                    div { class: "compare-chip red",
                                        "{bias.r#type.emoji()} {bias.r#type.i18n(cl).label}: {bias.intensity}/10"
                                    }
                                }
                            }
                            div { class: "compare-section",
                                h4 { "{ocean_label}" }
                                MiniBars { scores: [
                                    b.ocean.openness, b.ocean.conscientiousness,
                                    b.ocean.extraversion, b.ocean.agreeableness,
                                    b.ocean.neuroticism,
                                ] }
                            }
                            {if b_flags.is_empty() {
                                rsx! {}
                            } else {
                                rsx! {
                                    div { class: "flag-chips",
                                        {b_flags.iter().map(|k| {
                                            let txt = crate::i18n::tr(k, lang());
                                            rsx! { div { class: "danger-warning", title: "{txt}", "⚠ {txt}" } }
                                        })}
                                    }
                                }
                            }}
                        }
                    }

                    div { class: "analysis-section",
                        h2 { "{analysis_title}" }
                        div { class: "analysis-grid",
                            div { class: "analysis-card synergy",
                                h3 { "{synergies_title}" }
                                ul {
                                    for s in &synergies {
                                        li { "{s}" }
                                    }
                                }
                            }
                            div { class: "analysis-card friction",
                                h3 { "{friction_title}" }
                                ul {
                                    for f in &frictions {
                                        li { "{f}" }
                                    }
                                }
                            }
                            div { class: "analysis-card strategy",
                                h3 { "{strategy_title}" }
                                div { class: "top-rec",
                                    span { class: "rec-icon", "💡" }
                                    div { class: "rec-text", "{top_strategy}" }
                                }
                                 if has_extra_strategies {
                                    details { class: "more-recs",
                                        summary {
                                            span { "More (" "{all_strategies.len() - 1}" ")" }
                                        }
                                        ul {
                                            for s in &all_strategies[1..] {
                                                li { "{s}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    {if !a_flags.is_empty() || !b_flags.is_empty() {
                        let risk_title = crate::i18n::tr("compare_risk_mitigation", lang());
                        let a_rm = peoplemodeler_core::advice::risk_mitigation_pair(&a);
                        let b_rm = peoplemodeler_core::advice::risk_mitigation_pair(&b);
                        rsx! {
                            div { class: "analysis-section",
                                h2 { "{risk_title}" }
                                div { class: "analysis-grid",
                                    if !a_rm.is_empty() {
                                        div { class: "analysis-card friction",
                                            h3 { "{na}" }
                                            ul {
                                                for (risk_key, mitigation) in &a_rm {
                                                    li { class: "risk-mit-row",
                                                        span { class: "risk-text", "⚠ {crate::i18n::tr(risk_key, lang())}" }
                                                        span { class: "mit-arrow", " → " }
                                                        span { class: "mit-text", "{mitigation}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if !b_rm.is_empty() {
                                        div { class: "analysis-card friction",
                                            h3 { "{nb}" }
                                            ul {
                                                for (risk_key, mitigation) in &b_rm {
                                                    li { class: "risk-mit-row",
                                                        span { class: "risk-text", "⚠ {crate::i18n::tr(risk_key, lang())}" }
                                                        span { class: "mit-arrow", " → " }
                                                        span { class: "mit-text", "{mitigation}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }}

                    div { class: "ethics-banner", "{ethics}" }
                }
            }
        }
        _ => rsx! { div { class: "page", h2 { "{not_found}" } } },
    }
}

#[component]
fn PersonCard(person: Person) -> Element {
    rsx! {
        div { class: "compare-avatar", "{person.avatar_emoji}" }
        h3 { "{person.name}" }
        p { "{person.role}" }
    }
}

#[component]
fn BreakdownBars(
    cat_ocean: String,
    cat_rep: String,
    cat_mot: String,
    cat_pat: String,
    cat_bias: String,
    cat_styles: String,
    cat_values: String,
    s_ocean: f64,
    s_rep: f64,
    s_mot: f64,
    s_pat: f64,
    s_bias: f64,
    s_styles: f64,
    s_values: f64,
    bias_mod_active: bool,
) -> Element {
    let cats = [
        (&cat_ocean, s_ocean, false),
        (&cat_rep, s_rep, false),
        (&cat_mot, s_mot, false),
        (&cat_pat, s_pat, false),
        (&cat_bias, s_bias, bias_mod_active),
        (&cat_styles, s_styles, false),
        (&cat_values, s_values, false),
    ];
    let pcts: Vec<u8> = peoplemodeler_core::ocean::scores_to_percentages(
        &cats.iter().map(|(_, v, _)| *v).collect::<Vec<_>>(),
    );

    rsx! {
        div { class: "breakdown-bars",
            for (i, (label, _, mod_flag)) in cats.iter().enumerate() {
                div { class: "bb-row",
                    span { class: "bb-label", "{label}" }
                    div { class: "bb-bar",
                        div { class: "bb-fill", width: "{pcts[i]}%" }
                    }
                    span { class: "bb-pct",
                        "{pcts[i]}%"
                        if *mod_flag && pcts[i] == 0 {
                            " ⚡"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextBars(rows: Vec<(String, u8)>) -> Element {
    let bands = synergy_bands();
    rsx! {
        div { class: "ctx-bars",
            {rows.into_iter().map(|(label, score)| {
                let pct = score as f64 / 101.0 * 100.0;
                let cls = bands
                    .iter()
                    .position(|(lo, hi)| score >= *lo && score <= *hi)
                    .map(|i| match i {
                        0 => "scale-tension",
                        1 => "scale-friction",
                        2 => "scale-moderate",
                        3 => "scale-good",
                        _ => "scale-strong",
                    })
                    .unwrap_or("scale-moderate");
                rsx! {
                    div { class: "ctx-row",
                        span { class: "ctx-label", "{label}" }
                        div { class: "ctx-bar",
                            div { class: "ctx-fill {cls}", width: "{pct:.0}%" }
                        }
                        span { class: "ctx-pct", "{score}%" }
                    }
                }
            })}
        }
    }
}

#[component]
fn MiniBars(scores: [Option<u8>; 5]) -> Element {
    let labels = ["O", "C", "E", "A", "N"];
    let vals: [String; 5] = scores.map(|s| s.map_or("-".to_string(), |v| v.to_string()));
    rsx! {
        div { class: "mini-bars",
            for (i, s) in scores.iter().enumerate() {
                div { class: "mb-row",
                    span { "{labels[i]}" }
                    div { class: "mb-bar",
                        div { class: "mb-fill", width: "{s.unwrap_or(0) * 10}%" }
                    }
                    span { "{vals[i]}" }
                }
            }
        }
    }
}

fn compare_analysis(
    a: &Person,
    b: &Person,
    lang: Lang,
) -> (Vec<String>, Vec<String>, (String, Vec<String>)) {
    let oa = &a.ocean;
    let ob = &b.ocean;
    let cl = core_lang(lang);

    let mut syn = Vec::new();
    let mut fri = Vec::new();
    let mut str = Vec::new();

    let na = a.name.clone();
    let nb = b.name.clone();

    // --- Synergies ---

    // O-C complementarity
    if oa
        .openness
        .zip(ob.conscientiousness)
        .is_some_and(|(o, c)| o >= 7 && c >= 7)
    {
        syn.push(if lang == Lang::Fr {
            format!("{na} apporte la vision créative, {nb} assure l'exécution rigoureuse")
        } else {
            format!("{na} brings creative vision, {nb} ensures rigorous execution")
        });
    } else if ob
        .openness
        .zip(oa.conscientiousness)
        .is_some_and(|(o, c)| o >= 7 && c >= 7)
    {
        syn.push(if lang == Lang::Fr {
            format!("{nb} apporte la vision créative, {na} assure l'exécution rigoureuse")
        } else {
            format!("{nb} brings creative vision, {na} ensures rigorous execution")
        });
    } else if oa
        .openness
        .zip(ob.openness)
        .is_some_and(|(a, b)| a.abs_diff(b) <= 2)
        && oa
            .conscientiousness
            .zip(ob.conscientiousness)
            .is_some_and(|(a, b)| a.abs_diff(b) <= 2)
    {
        syn.push(if lang == Lang::Fr {
            "Profils OCEAN très proches — communication fluide et attentes alignées".into()
        } else {
            "Very similar OCEAN profiles — smooth communication and aligned expectations".into()
        });
    }

    // E-A complementarity
    if oa
        .extraversion
        .zip(ob.agreeableness)
        .is_some_and(|(e, a)| e >= 7 && a >= 7)
        || ob
            .extraversion
            .zip(oa.agreeableness)
            .is_some_and(|(e, a)| e >= 7 && a >= 7)
    {
        syn.push(if lang == Lang::Fr {
            "Extraversion et agréabilité se compensent : l'un conduit, l'autre harmonise".into()
        } else {
            "Extraversion and agreeableness complement: one drives, one harmonizes".into()
        });
    }

    // Motivation synergy (uses core synergy value, not just equality)
    if let (Some(m1), Some(m2)) = (a.top_motivation(), b.top_motivation()) {
        let msyn = peoplemodeler_core::synergy::motivation_synergy(m1.r#type, m2.r#type);
        if msyn > 0.0 {
            syn.push(if lang == Lang::Fr {
                format!(
                    "Motivations complémentaires — {} et {} se renforcent mutuellement",
                    m1.r#type.i18n(cl).label,
                    m2.r#type.i18n(cl).label,
                )
            } else {
                format!(
                    "Complementary motivations — {} and {} reinforce each other",
                    m1.r#type.i18n(cl).label,
                    m2.r#type.i18n(cl).label,
                )
            });
        } else if msyn < 0.0 {
            fri.push(if lang == Lang::Fr {
                format!(
                    "Motivations concurrentes — {} et {} peuvent créer des tensions",
                    m1.r#type.i18n(cl).label,
                    m2.r#type.i18n(cl).label,
                )
            } else {
                format!(
                    "Competing motivations — {} and {} may create tension",
                    m1.r#type.i18n(cl).label,
                    m2.r#type.i18n(cl).label,
                )
            });
        }
        if m1.r#type == m2.r#type {
            syn.push(if lang == Lang::Fr {
                format!(
                    "Motivation {} partagée — même langage, mêmes priorités",
                    m1.r#type.i18n(cl).label
                )
            } else {
                format!(
                    "Shared {} motivation — same language, same priorities",
                    m1.r#type.i18n(cl).label
                )
            });
        }
    }

    // --- Frictions ---

    // Agreeableness gap
    if oa.agreeableness.is_some_and(|v| v >= 7) && ob.agreeableness.is_some_and(|v| v <= 4) {
        fri.push(if lang == Lang::Fr {
            format!("{nb} (faible A) peut sembler agressif pour {na} (haute A)")
        } else {
            format!("{nb} (low A) may seem aggressive to {na} (high A)")
        });
    } else if ob.agreeableness.is_some_and(|v| v >= 7) && oa.agreeableness.is_some_and(|v| v <= 4) {
        fri.push(if lang == Lang::Fr {
            format!("{na} (faible A) peut sembler agressif pour {nb} (haute A)")
        } else {
            format!("{na} (low A) may seem aggressive to {nb} (high A)")
        });
    }

    // Neuroticism gap
    if let (Some(na_n), Some(nb_n)) = (oa.neuroticism, ob.neuroticism) {
        let nd = na_n.abs_diff(nb_n);
        if nd >= 3 {
            let (stable, reactive) = if na_n <= nb_n { (&na, &nb) } else { (&nb, &na) };
            fri.push(if lang == Lang::Fr {
                format!("{reactive} plus réactif au stress que {stable} — risque d'incompréhension")
            } else {
                format!(
                    "{reactive} more reactive to stress than {stable} — risk of misunderstanding"
                )
            });
        }
    }

    // Reputation synergy (distance-based, per shared dimension)
    {
        use peoplemodeler_core::models::RepDim;
        let cl = core_lang(lang);
        for dim in RepDim::ALL {
            if let (Some(va), Some(vb)) = (a.rep_scores.score(dim), b.rep_scores.score(dim)) {
                let ri = dim.i18n(cl);
                let close = va.abs_diff(vb) <= 2;
                let both_high = va >= 7 && vb >= 7;
                let both_low = va <= 3 && vb <= 3;
                if close && both_high {
                    syn.push(if lang == Lang::Fr {
                        format!("Tous deux {} — affinité naturelle", ri.label_a)
                    } else {
                        format!("Both {} — natural affinity", ri.label_a)
                    });
                } else if close && both_low {
                    syn.push(if lang == Lang::Fr {
                        format!("Tous deux {} — complicité inattendue", ri.label_b)
                    } else {
                        format!("Both {} — unexpected camaraderie", ri.label_b)
                    });
                } else if !close && va > vb {
                    fri.push(if lang == Lang::Fr {
                        format!(
                            "{na} plus {} que {nb} — déséquilibre sur ce trait",
                            ri.label_a
                        )
                    } else {
                        format!(
                            "{na} more {} than {nb} — imbalance on this trait",
                            ri.label_a
                        )
                    });
                } else if !close {
                    fri.push(if lang == Lang::Fr {
                        format!(
                            "{na} plus {} que {nb} — déséquilibre sur ce trait",
                            ri.label_b
                        )
                    } else {
                        format!(
                            "{na} more {} than {nb} — imbalance on this trait",
                            ri.label_b
                        )
                    });
                }
            }
        }
    }

    // Bias conflict
    match (a.top_bias(), b.top_bias()) {
        (Some(b1), Some(b2)) if b1.r#type != b2.r#type => {
            fri.push(if lang == Lang::Fr {
                format!(
                    "Biais {} vs {} : désaccords sur les références de décision",
                    b1.r#type.i18n(cl).label,
                    b2.r#type.i18n(cl).label,
                )
            } else {
                format!(
                    "Bias {} vs {} : disagreements on decision references",
                    b1.r#type.i18n(cl).label,
                    b2.r#type.i18n(cl).label,
                )
            });
        }
        (Some(b1), Some(_b2)) => {
            fri.push(if lang == Lang::Fr {
                format!(
                    "Même biais {} des deux côtés — angles morts renforcés",
                    b1.r#type.i18n(cl).label
                )
            } else {
                format!(
                    "Same {} bias on both sides — reinforced blind spots",
                    b1.r#type.i18n(cl).label
                )
            });
        }
        _ => {}
    }

    // --- Behavioral Patterns ---
    if let (Some(pa), Some(pb)) = (a.behavioral_patterns.first(), b.behavioral_patterns.first()) {
        let cl = core_lang(lang);
        syn.push(if lang == Lang::Fr {
            format!(
                "A réagit par «{}» | B réagit par «{}»",
                pa.predicted_behavior.label_bare(cl),
                pb.predicted_behavior.label_bare(cl)
            )
        } else {
            format!(
                "A responds with «{}» | B responds with «{}»",
                pa.predicted_behavior.label_bare(cl),
                pb.predicted_behavior.label_bare(cl)
            )
        });

        match (&pa.trigger, &pb.trigger) {
            (BehaviorTrigger::Change, BehaviorTrigger::Change) => {
                syn.push(if lang == Lang::Fr {
                    "Tous deux s'adaptent au changement — organisation fluide".into()
                } else {
                    "Both adapt well to change — smooth transitions".into()
                });
            }
            (BehaviorTrigger::Feedback, BehaviorTrigger::Feedback) => {
                syn.push(if lang == Lang::Fr {
                    "Tous deux réceptifs aux retours — culture d'amélioration continue".into()
                } else {
                    "Both receptive to feedback — culture of continuous improvement".into()
                });
            }
            (BehaviorTrigger::Conflict, BehaviorTrigger::Conflict) => {
                fri.push(if lang == Lang::Fr {
                    "Tous deux conflictuels en situation tendue — risque d'escalade".into()
                } else {
                    "Both combative under tension — risk of escalation".into()
                });
            }
            (BehaviorTrigger::Stress, BehaviorTrigger::Stress) => {
                fri.push(if lang == Lang::Fr {
                    "Tous deux stressés sous pression — anxiété contagieuse".into()
                } else {
                    "Both stressed under pressure — contagious anxiety".into()
                });
            }
            (BehaviorTrigger::Change, BehaviorTrigger::Feedback)
            | (BehaviorTrigger::Feedback, BehaviorTrigger::Change) => {
                syn.push(if lang == Lang::Fr {
                    "L'un s'adapte, l'autre apprend — duo qui évolue ensemble".into()
                } else {
                    "One adapts, one learns — a duo that grows together".into()
                });
            }
            _ => {}
        }
    }

    // Extraversion gap friction
    if let (Some(ea_e), Some(eb_e)) = (oa.extraversion, ob.extraversion) {
        let ed = ea_e.abs_diff(eb_e);
        if ed >= 4 {
            fri.push(if lang == Lang::Fr {
                "Écart d'extraversion important — rythme social et besoin de stimulation différents"
                    .into()
            } else {
                "Large extraversion gap — different social pace and stimulation needs".into()
            });
        }
    }

    // --- Strategies ---

    // Based on top motivations
    if let (Some(m1), Some(m2)) = (a.top_motivation(), b.top_motivation()) {
        if m1.r#type == peoplemodeler_core::models::MotivationType::Power
            || m2.r#type == peoplemodeler_core::models::MotivationType::Power
        {
            str.push(if lang == Lang::Fr {
                "Donner des espaces d'initiative à la personne motivée par le pouvoir".into()
            } else {
                "Give the power-driven person opportunities to take initiative".into()
            });
        }
        if m1.r#type == peoplemodeler_core::models::MotivationType::Recognition
            || m2.r#type == peoplemodeler_core::models::MotivationType::Recognition
        {
            str.push(if lang == Lang::Fr {
                "Reconnaître publiquement les contributions de chacun".into()
            } else {
                "Publicly acknowledge each person's contributions".into()
            });
        }
        if m1.r#type == peoplemodeler_core::models::MotivationType::Security
            || m2.r#type == peoplemodeler_core::models::MotivationType::Security
        {
            str.push(if lang == Lang::Fr {
                "Fournir des cadres stables et des procédures claires".into()
            } else {
                "Provide stable frameworks and clear procedures".into()
            });
        }
    }

    // Conscientiousness-based strategy
    if oa.conscientiousness.is_some_and(|v| v >= 7) || ob.conscientiousness.is_some_and(|v| v >= 7)
    {
        str.push(if lang == Lang::Fr {
            "Présenter les informations de manière structurée avec des données tangibles".into()
        } else {
            "Present information in a structured way with tangible data".into()
        });
    }

    // Conflict resolution
    if oa.agreeableness.is_some_and(|v| v >= 7) && ob.agreeableness.is_some_and(|v| v >= 7) {
        str.push(if lang == Lang::Fr {
            "En cas de conflit, privilégier la médiation — les deux parties chercheront l'harmonie"
                .into()
        } else {
            "In conflict, prioritize mediation — both parties will seek harmony".into()
        });
    } else if oa.agreeableness.is_some_and(|v| v <= 4) && ob.agreeableness.is_some_and(|v| v <= 4) {
        str.push(if lang == Lang::Fr {
            "En cas de désaccord, aller droit au fait — les deux préfèrent la franchise".into()
        } else {
            "When disagreeing, get straight to the point — both prefer directness".into()
        });
    }

    // --- OCEAN-gap strategies ---
    if let (Some(ea), Some(eb)) = (oa.extraversion, ob.extraversion)
        && ea.abs_diff(eb) >= 3
    {
        let (more, quieter) = if ea >= eb { (&na, &nb) } else { (&nb, &na) };
        str.push(if lang == Lang::Fr {
            format!("Rythme social très différent — {more} préfère plus d'échanges, {quieter} plus de calme")
        } else {
            format!("Very different social pace — {more} prefers more interaction, {quieter} more quiet time")
        });
    }
    if let (Some(aa), Some(ab)) = (oa.agreeableness, ob.agreeableness)
        && aa.abs_diff(ab) >= 3
    {
        str.push(if lang == Lang::Fr {
            "Styles de conflit différents — l'un cherche l'harmonie, l'autre la franchise".into()
        } else {
            "Different conflict styles — one seeks harmony, the other directness".into()
        });
    }
    if let (Some(ca), Some(cb)) = (oa.conscientiousness, ob.conscientiousness)
        && ca.abs_diff(cb) >= 3
    {
        str.push(if lang == Lang::Fr {
            "Niveaux d'organisation différents — adapter le niveau de détail et de structure".into()
        } else {
            "Different organization levels — adjust detail and structure expectations".into()
        });
    }

    // --- Trigger-pair clash strategies ---
    if let (Some(pa), Some(pb)) = (a.behavioral_patterns.first(), b.behavioral_patterns.first()) {
        let t_syn = peoplemodeler_core::synergy::trigger_synergy(pa.trigger, pb.trigger);
        let cl = core_lang(lang);
        let a_resp = pa.predicted_behavior.label_bare(cl);
        let b_resp = pb.predicted_behavior.label_bare(cl);
        let resp_info = format!(" (A: {a_resp} | B: {b_resp})");
        if t_syn < -0.1 {
            str.push(if lang == Lang::Fr {
                format!(
                    "Risque de déclenchement mutuel — leurs réactions au stress s'amplifient{}",
                    resp_info
                )
            } else {
                format!(
                    "Risk of mutual triggering — their stress responses amplify each other{}",
                    resp_info
                )
            });
        } else if t_syn > 0.1 {
            str.push(if lang == Lang::Fr {
                format!(
                    "Complémentarité comportementale — leurs réactions s'équilibrent{}",
                    resp_info
                )
            } else {
                format!(
                    "Natural behavioral complementarity — their responses balance each other{}",
                    resp_info
                )
            });
        }
    }

    if syn.is_empty() {
        syn.push(if lang == Lang::Fr {
            "Aucune synergie évidente détectée".into()
        } else {
            "No obvious synergy detected".into()
        });
    }
    if fri.is_empty() {
        fri.push(if lang == Lang::Fr {
            "Aucun point de friction majeur identifié".into()
        } else {
            "No major friction points identified".into()
        });
    }

    // Pick #1 strategy (motivation-based > OCEAN-gap > conflict > C-structure > fallback)
    let top = if str.is_empty() {
        str.push(if lang == Lang::Fr {
            "Communiquer ouvertement et observer les réactions".into()
        } else {
            "Communicate openly and observe reactions".into()
        });
        str[0].clone()
    } else {
        // First is always the strongest signal (strategies added in priority order)
        str[0].clone()
    };

    (syn, fri, (top, str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::*;

    fn p(name: &str) -> Person {
        Person {
            id: name.into(),
            name: name.into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "X".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            ocean: OceanScores {
                openness: None,
                conscientiousness: None,
                extraversion: None,
                agreeableness: None,
                neuroticism: None,
            },
            resilience: None,
            risk_appetite: None,
            confidence: 5,
            log: vec![],
            created_at: 0,
            updated_at: 0,
        }
    }

    // ── ctx_key ──

    #[test]
    fn ctx_key_all_variants() {
        assert_eq!(ctx_key(InsightContext::Decision), "ctx_decision");
        assert_eq!(ctx_key(InsightContext::Team), "ctx_team");
        assert_eq!(ctx_key(InsightContext::Stress), "ctx_stress");
        assert_eq!(ctx_key(InsightContext::Communication), "ctx_communication");
        assert_eq!(ctx_key(InsightContext::Leadership), "ctx_leadership");
        assert_eq!(ctx_key(InsightContext::Growth), "ctx_growth");
    }

    // ── O-C complementarity ──

    #[test]
    fn oc_complementarity_a_open_b_consc() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(8);
        b.ocean.conscientiousness = Some(9);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("creative vision")));
    }

    #[test]
    fn oc_complementarity_reverse() {
        let mut a = p("A");
        let mut b = p("B");
        b.ocean.openness = Some(8);
        a.ocean.conscientiousness = Some(9);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("creative vision")));
    }

    #[test]
    fn similar_ocean_profiles() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(8);
        a.ocean.conscientiousness = Some(3);
        b.ocean.openness = Some(9);
        b.ocean.conscientiousness = Some(2);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("similar OCEAN")));
    }

    #[test]
    fn oc_none_match() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(3);
        b.ocean.conscientiousness = Some(2);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!syn.iter().any(|s| s.contains("creative vision")));
        assert!(!syn.iter().any(|s| s.contains("similar OCEAN")));
    }

    // ── E-A complementarity ──

    #[test]
    fn ea_complementarity_a_e_high_b_a_high() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(8);
        b.ocean.agreeableness = Some(9);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("one drives, one harmonizes")));
    }

    #[test]
    fn ea_complementarity_reverse() {
        let mut a = p("A");
        let mut b = p("B");
        b.ocean.extraversion = Some(8);
        a.ocean.agreeableness = Some(9);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("one drives, one harmonizes")));
    }

    #[test]
    fn ea_none_match() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(3);
        b.ocean.agreeableness = Some(3);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!syn.iter().any(|s| s.contains("one drives")));
    }

    // ── Motivation synergy ──

    #[test]
    fn motivation_synergy_positive() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 7,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("Complementary motivations")));
    }

    #[test]
    fn motivation_synergy_negative() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Affiliation,
            intensity: 7,
            notes: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("Competing motivations")));
    }

    #[test]
    fn motivation_same_type() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 7,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("Shared Security")));
    }

    #[test]
    fn motivation_neutral() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Autonomy,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Autonomy,
            intensity: 7,
            notes: String::new(),
        }];
        let (syn, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!syn.iter().any(|s| s.contains("Complementary")));
        assert!(!fri.iter().any(|s| s.contains("Competing")));
        assert!(syn.iter().any(|s| s.contains("Shared")));
    }

    // ── Agreeableness gap ──

    #[test]
    fn agreeableness_gap_a_high_b_low() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A"))
        );
    }

    #[test]
    fn agreeableness_gap_reverse() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(3);
        b.ocean.agreeableness = Some(8);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A"))
        );
    }

    // ── Neuroticism gap ──

    #[test]
    fn neuroticism_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(9);
        b.ocean.neuroticism = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("more reactive to stress")));
    }

    #[test]
    fn neuroticism_close() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(5);
        b.ocean.neuroticism = Some(6);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!fri.iter().any(|s| s.contains("reactive to stress")));
    }

    // ── Reputation synergy ──

    #[test]
    fn rep_close_both_high() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("natural affinity")));
    }

    #[test]
    fn rep_close_both_low() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(2),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("unexpected camaraderie")));
    }

    #[test]
    fn rep_far_a_higher() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("imbalance")));
    }

    #[test]
    fn rep_far_b_higher() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("imbalance")));
    }

    // ── Bias conflict ──

    #[test]
    fn bias_different_types() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 7,
            evidence: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("Bias") && s.contains("vs")));
    }

    #[test]
    fn bias_same_type() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 7,
            evidence: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("Same") && s.contains("bias")));
    }

    #[test]
    fn bias_none() {
        let a = p("A");
        let b = p("B");
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!fri.iter().any(|s| s.contains("Bias") || s.contains("bias")));
    }

    // ── Behavioral patterns ──

    #[test]
    fn pattern_change_change() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("adapt well to change")));
    }

    #[test]
    fn pattern_feedback_feedback() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("continuous improvement")));
    }

    #[test]
    fn pattern_conflict_conflict() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("escalation")));
    }

    #[test]
    fn pattern_stress_stress() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("contagious anxiety")));
    }

    #[test]
    fn pattern_change_feedback() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("One adapts, one learns")));
    }

    #[test]
    fn pattern_feedback_change() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("One adapts, one learns")));
    }

    #[test]
    fn pattern_no_patterns() {
        let a = p("A");
        let b = p("B");
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.is_empty() || syn.iter().all(|s| !s.contains("responds with")));
    }

    // ── Extraversion gap ──

    #[test]
    fn extraversion_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(9);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("extraversion")));
    }

    #[test]
    fn extraversion_close() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(5);
        b.ocean.extraversion = Some(7);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!fri.iter().any(|s| s.contains("extraversion")));
    }

    // ── Strategies: motivation-based ──

    #[test]
    fn strategy_power() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("take initiative")));
    }

    #[test]
    fn strategy_recognition() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("acknowledge")));
    }

    #[test]
    fn strategy_security() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("stable frameworks")));
    }

    // ── Strategies: conscientiousness ──

    #[test]
    fn strategy_high_conscientiousness() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(8);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("structured")));
    }

    // ── Strategies: conflict resolution ──

    #[test]
    fn strategy_both_high_agreeableness() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("mediation")));
    }

    #[test]
    fn strategy_both_low_agreeableness() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(2);
        b.ocean.agreeableness = Some(3);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("straight to the point")));
    }

    // ── Strategies: OCEAN-gap ──

    #[test]
    fn strategy_extraversion_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("social pace")));
    }

    #[test]
    fn strategy_agreeableness_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(2);
        b.ocean.agreeableness = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("conflict styles")));
    }

    #[test]
    fn strategy_conscientiousness_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(2);
        b.ocean.conscientiousness = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("organization levels")));
    }

    // ── Strategies: trigger-pair clash ──

    #[test]
    fn strategy_trigger_negative() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("mutual triggering")));
    }

    #[test]
    fn strategy_trigger_positive() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("complementarity")));
    }

    #[test]
    fn strategy_trigger_neutral() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Uncertainty,
            predicted_behavior: BehaviorResponse::SeeksData,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Success,
            predicted_behavior: BehaviorResponse::CelebratesWithOthers,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(!str.iter().any(|s| s.contains("mutual triggering")));
        assert!(!str.iter().any(|s| s.contains("complementarity")));
    }

    // ── Empty fallbacks ──

    #[test]
    fn empty_syn_fallback() {
        let a = p("A");
        let b = p("B");
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("No obvious synergy")));
    }

    #[test]
    fn empty_fri_fallback() {
        let a = p("A");
        let b = p("B");
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("No major friction")));
    }

    #[test]
    fn empty_str_fallback() {
        let a = p("A");
        let b = p("B");
        let (_, _, (top, _)) = compare_analysis(&a, &b, Lang::En);
        assert!(top.contains("Communicate openly"));
    }

    #[test]
    fn nonempty_str_no_fallback() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(9);
        let (_, _, (top, _)) = compare_analysis(&a, &b, Lang::En);
        assert!(!top.contains("Communicate openly"));
    }

    // ── French language ──

    #[test]
    fn french_oc_complementarity() {
        let mut a = p("Alice");
        let mut b = p("Bob");
        a.ocean.openness = Some(8);
        b.ocean.conscientiousness = Some(9);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("vision créative")));
    }

    #[test]
    fn french_ea_complementarity() {
        let mut a = p("Alice");
        let mut b = p("Bob");
        a.ocean.extraversion = Some(8);
        b.ocean.agreeableness = Some(9);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("l'un conduit")));
    }

    #[test]
    fn french_empty_syn() {
        let a = p("A");
        let b = p("B");
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("Aucune synergie")));
    }

    #[test]
    fn french_empty_fri() {
        let a = p("A");
        let b = p("B");
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("Aucun point de friction")));
    }

    #[test]
    fn french_empty_str() {
        let a = p("A");
        let b = p("B");
        let (_, _, (top, _)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(top.contains("Communiquer ouvertement"));
    }

    #[test]
    fn french_bias_different() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 7,
            evidence: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("désaccords")));
    }

    #[test]
    fn french_bias_same() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 7,
            evidence: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("Même biais")));
    }

    #[test]
    fn french_motivation_positive() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 7,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("complémentaires")));
    }

    #[test]
    fn french_motivation_negative() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Affiliation,
            intensity: 7,
            notes: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("concurrentes")));
    }

    #[test]
    fn french_same_motivation() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 7,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("partagée")));
    }

    #[test]
    fn french_agreeableness_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("faible A")));
    }

    #[test]
    fn french_agreeableness_gap_reverse() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(3);
        b.ocean.agreeableness = Some(8);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("faible A")));
    }

    #[test]
    fn french_neuroticism_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(9);
        b.ocean.neuroticism = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("réactif au stress")));
    }

    #[test]
    fn french_rep_close_high() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("affinité naturelle")));
    }

    #[test]
    fn french_rep_close_low() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(2),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("complicité inattendue")));
    }

    #[test]
    fn french_rep_far_a_higher() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("déséquilibre")));
    }

    #[test]
    fn french_rep_far_b_higher() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("déséquilibre")));
    }

    #[test]
    fn french_pattern_change_change() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("s'adaptent au changement")));
    }

    #[test]
    fn french_pattern_feedback_feedback() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("amélioration continue")));
    }

    #[test]
    fn french_pattern_conflict_conflict() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("escalade")));
    }

    #[test]
    fn french_pattern_stress_stress() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("anxiété contagieuse")));
    }

    #[test]
    fn french_pattern_change_feedback() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksFeedback,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(syn.iter().any(|s| s.contains("évolue ensemble")));
    }

    #[test]
    fn french_extraversion_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(9);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(fri.iter().any(|s| s.contains("Écart d'extraversion")));
    }

    #[test]
    fn french_strategy_power() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("initiative")));
    }

    #[test]
    fn french_strategy_recognition() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("Reconnaître publiquement")));
    }

    #[test]
    fn french_strategy_security() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("cadres stables")));
    }

    #[test]
    fn french_strategy_high_c() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(8);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("structurée")));
    }

    #[test]
    fn french_strategy_both_high_a() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("médiation")));
    }

    #[test]
    fn french_strategy_both_low_a() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(2);
        b.ocean.agreeableness = Some(3);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("droit au fait")));
    }

    #[test]
    fn french_strategy_e_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("Rythme social")));
    }

    #[test]
    fn french_strategy_a_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(2);
        b.ocean.agreeableness = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("Styles de conflit")));
    }

    #[test]
    fn french_strategy_c_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(2);
        b.ocean.conscientiousness = Some(9);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("organisation différents")));
    }

    #[test]
    fn french_trigger_negative() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::Panics,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(str.iter().any(|s| s.contains("déclenchement mutuel")));
    }

    #[test]
    fn french_trigger_positive() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::EmbracesChange,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(
            str.iter()
                .any(|s| s.contains("Complémentarité comportementale"))
        );
    }

    #[test]
    fn french_trigger_neutral() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Uncertainty,
            predicted_behavior: BehaviorResponse::SeeksData,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Success,
            predicted_behavior: BehaviorResponse::CelebratesWithOthers,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::Fr);
        assert!(!str.iter().any(|s| s.contains("déclenchement mutuel")));
        assert!(
            !str.iter()
                .any(|s| s.contains("Complémentarité comportementale"))
        );
    }

    #[test]
    fn core_lang_en() {
        assert!(matches!(
            core_lang(Lang::En),
            peoplemodeler_core::i18n::Lang::En
        ));
    }

    #[test]
    fn core_lang_fr() {
        assert!(matches!(
            core_lang(Lang::Fr),
            peoplemodeler_core::i18n::Lang::Fr
        ));
    }

    #[test]
    fn prefill_rel_no_match() {
        crate::db::init();
        let (t, s) = prefill_rel("nonexistent_a", "nonexistent_b");
        assert!(t.is_none());
        assert_eq!(s, 5);
    }

    #[test]
    fn compare_boundary_oc_close_at_2() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(7);
        a.ocean.conscientiousness = Some(5);
        b.ocean.openness = Some(9);
        b.ocean.conscientiousness = Some(5);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("similar OCEAN")));
    }

    #[test]
    fn compare_boundary_oc_far_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(7);
        b.ocean.openness = Some(3);
        a.ocean.conscientiousness = Some(5);
        b.ocean.conscientiousness = Some(5);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!syn.iter().any(|s| s.contains("similar OCEAN")));
    }

    #[test]
    fn compare_boundary_oc_complement_at_7() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(7);
        b.ocean.conscientiousness = Some(7);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("creative vision")));
    }

    #[test]
    fn compare_boundary_oc_complement_below_7() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(6);
        b.ocean.conscientiousness = Some(6);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!syn.iter().any(|s| s.contains("creative vision")));
    }

    #[test]
    fn compare_boundary_ea_at_7() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(7);
        b.ocean.agreeableness = Some(7);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("one drives, one harmonizes")));
    }

    #[test]
    fn compare_boundary_ea_below_7() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(6);
        b.ocean.agreeableness = Some(6);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!syn.iter().any(|s| s.contains("one drives")));
    }

    #[test]
    fn compare_boundary_agreeableness_high_at_7() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(7);
        b.ocean.agreeableness = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A"))
        );
    }

    #[test]
    fn compare_boundary_agreeableness_low_at_4() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(4);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A"))
        );
    }

    #[test]
    fn compare_boundary_agreeableness_low_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A"))
        );
    }

    #[test]
    fn compare_boundary_neuroticism_gap_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(8);
        b.ocean.neuroticism = Some(5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("more reactive to stress")));
    }

    #[test]
    fn compare_boundary_neuroticism_gap_below_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(7);
        b.ocean.neuroticism = Some(5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!fri.iter().any(|s| s.contains("more reactive to stress")));
    }

    #[test]
    fn compare_boundary_extraversion_gap_at_4() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(6);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(fri.iter().any(|s| s.contains("extraversion")));
    }

    #[test]
    fn compare_boundary_extraversion_gap_below_4() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(3);
        b.ocean.extraversion = Some(6);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(!fri.iter().any(|s| s.contains("extraversion")));
    }

    #[test]
    fn compare_boundary_oc_gap_strat_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("social pace")));
    }

    #[test]
    fn compare_boundary_oc_gap_strat_below_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(4);
        b.ocean.extraversion = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(!str.iter().any(|s| s.contains("social pace")));
    }

    #[test]
    fn compare_boundary_agreeableness_strat_gap_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(2);
        b.ocean.agreeableness = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("conflict styles")));
    }

    #[test]
    fn compare_boundary_conscientiousness_strat_at_7() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(7);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("structured")));
    }

    #[test]
    fn compare_boundary_conscientiousness_strat_below_7() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(6);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(!str.iter().any(|s| s.contains("structured")));
    }

    #[test]
    fn compare_boundary_conflict_res_high_at_7() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(7);
        b.ocean.agreeableness = Some(7);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("mediation")));
    }

    #[test]
    fn compare_boundary_conflict_res_low_at_4() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(4);
        b.ocean.agreeableness = Some(4);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("straight to the point")));
    }

    #[test]
    fn compare_boundary_conflict_res_low_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(3);
        b.ocean.agreeableness = Some(3);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("straight to the point")));
    }

    #[test]
    fn compare_boundary_rep_close_at_2() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("natural affinity")));
    }

    #[test]
    fn compare_boundary_rep_far_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(4),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter()
                .any(|s| s.contains("natural affinity") || s.contains("unexpected camaraderie"))
        );
    }

    #[test]
    fn compare_boundary_c_conscientiousness_gap_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(2);
        b.ocean.conscientiousness = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("organization levels")));
    }

    #[test]
    fn compare_boundary_c_conscientiousness_gap_below_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(4);
        b.ocean.conscientiousness = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(!str.iter().any(|s| s.contains("organization levels")));
    }

    #[test]
    fn compare_boundary_conscientiousness_strategy_high_at_7() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(7);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("structured")));
    }

    #[test]
    fn compare_boundary_extraversion_gap_strat_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(7);
        b.ocean.extraversion = Some(4);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("social pace")));
    }

    #[test]
    fn compare_boundary_conflict_strat_gap_at_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(7);
        b.ocean.agreeableness = Some(4);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(str.iter().any(|s| s.contains("conflict styles")));
    }

    // ══════════════════════════════════════════════════════════════════
    // Mutation-killing tests — targeted boundary & directional checks
    // ══════════════════════════════════════════════════════════════════

    // ── E-A complement: && vs || (lines 655, 659) ──

    #[test]
    fn mut_ea_first_arm_high_e_low_a_no_synergy() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(8);
        b.ocean.agreeableness = Some(3);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("one drives")),
            "E-A synergy must NOT fire when only E is high and A is low"
        );
    }

    #[test]
    fn mut_ea_second_arm_high_e_low_a_no_synergy() {
        let mut a = p("A");
        let mut b = p("B");
        b.ocean.extraversion = Some(8);
        a.ocean.agreeableness = Some(3);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("one drives")),
            "E-A synergy must NOT fire when only b.E is high and a.A is low"
        );
    }

    #[test]
    fn mut_ea_both_arms_fires_when_both_high() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(8);
        b.ocean.agreeableness = Some(8);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("one drives")));
    }

    // ── Agreeableness gap boundary (line 718: >=7 && <=4) ──

    #[test]
    fn mut_agreeableness_gap_a7_b4_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(7);
        b.ocean.agreeableness = Some(4);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("low A")),
            "Agreeableness gap must trigger at exactly A=7 vs A=4"
        );
    }

    #[test]
    fn mut_agreeableness_gap_a8_b5_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A")),
            "Agreeableness gap must NOT trigger when b.A=5 (>4)"
        );
    }

    #[test]
    fn mut_agreeableness_gap_reverse_b7_a4_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(4);
        b.ocean.agreeableness = Some(7);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("low A")),
            "Agreeableness gap must trigger in reverse at A=4 vs A=7"
        );
    }

    #[test]
    fn mut_agreeableness_gap_reverse_a5_b8_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(5);
        b.ocean.agreeableness = Some(8);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter()
                .any(|s| s.contains("low A") && s.contains("high A")),
            "Agreeableness gap reverse must NOT trigger when a.A=5 (>4)"
        );
    }

    // ── Neuroticism gap boundary (line 735: >= 3) ──

    #[test]
    fn mut_neuroticism_gap_exactly_3_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(8);
        b.ocean.neuroticism = Some(5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("reactive to stress")),
            "Neuroticism gap must trigger at diff=3 (>=3)"
        );
    }

    #[test]
    fn mut_neuroticism_gap_exactly_2_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(7);
        b.ocean.neuroticism = Some(5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter().any(|s| s.contains("reactive to stress")),
            "Neuroticism gap must NOT trigger at diff=2 (<3)"
        );
    }

    #[test]
    fn mut_neuroticism_gap_higher_is_reactive() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(3);
        b.ocean.neuroticism = Some(9);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("B more reactive to stress than A")),
            "Higher-N person must be named as the reactive one"
        );
    }

    // ── Rep dimension: close boundary (line 754: <=2) ──

    #[test]
    fn mut_rep_close_boundary_diff_2() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(10),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            syn.iter().any(|s| s.contains("natural affinity")),
            "Rep diff=2 must be 'close'"
        );
    }

    #[test]
    fn mut_rep_not_close_boundary_diff_3() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(8),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter()
                .any(|s| s.contains("natural affinity") || s.contains("unexpected camaraderie")),
            "Rep diff=3 must NOT be 'close'"
        );
    }

    // ── Rep dimension: both_high / both_low guards (lines 755-756) ──

    #[test]
    fn mut_rep_close_mixed_not_both_high() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(6),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(7),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("natural affinity")),
            "Close but A=6 (<7) must NOT trigger both_high"
        );
    }

    #[test]
    fn mut_rep_close_mixed_not_both_low() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("unexpected camaraderie")),
            "Close but B=5 (>3) must NOT trigger both_low"
        );
    }

    // ── Rep dimension: far, direction (lines 769, 781) ──

    #[test]
    fn mut_rep_far_a_higher_than_b() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("A more") && s.contains("imbalance")),
            "Far rep where A > B must say 'A more ... than B'"
        );
        assert!(
            !fri.iter()
                .any(|s| s.contains("A more") && s.contains("B more")),
            "Only one direction of imbalance should fire"
        );
    }

    #[test]
    fn mut_rep_far_b_higher_than_a() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("A more") && s.contains("B") && s.contains("imbalance")),
            "Far rep where a < b must say 'A more ... than B'"
        );
    }

    #[test]
    fn mut_rep_far_equal_values_no_far_friction() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(5),
            ..RepScores::default()
        };
        let (syn, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter().any(|s| s.contains("imbalance")),
            "Equal rep values (diff=0) must NOT trigger far friction"
        );
        assert!(
            !syn.iter()
                .any(|s| s.contains("natural affinity") || s.contains("unexpected camaraderie")),
            "Equal mid-range values (5) are neither both_high nor both_low"
        );
    }

    // ── Bias conflict: != and == guards (lines 800, 815) ──

    #[test]
    fn mut_bias_different_produces_vs_message() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Anchoring,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::LossAversion,
            intensity: 7,
            evidence: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("Bias") && s.contains("vs")),
            "Different biases must produce 'Bias X vs Y' friction"
        );
        assert!(
            !fri.iter().any(|s| s.contains("Same") && s.contains("bias")),
            "Different biases must NOT produce 'Same bias' message"
        );
    }

    #[test]
    fn mut_bias_same_produces_same_message() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: String::new(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 7,
            evidence: String::new(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("Same") && s.contains("bias")),
            "Same biases must produce 'Same X bias' message"
        );
        assert!(
            !fri.iter().any(|s| s.contains("Bias") && s.contains("vs")),
            "Same biases must NOT produce 'Bias X vs Y' message"
        );
    }

    // ── Strategy: motivation == checks (lines 907, 916, 925) ──

    #[test]
    fn mut_strategy_power_b_has_power() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 7,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("take initiative")),
            "Strategy must trigger when b has Power motivation (not just a)"
        );
    }

    #[test]
    fn mut_strategy_recognition_b_has_recognition() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Recognition,
            intensity: 7,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("acknowledge")),
            "Strategy must trigger when b has Recognition motivation"
        );
    }

    #[test]
    fn mut_strategy_security_b_has_security() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Security,
            intensity: 7,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("stable frameworks")),
            "Strategy must trigger when b has Security motivation"
        );
    }

    #[test]
    fn mut_strategy_neither_has_power_no_initiative() {
        let mut a = p("A");
        let mut b = p("B");
        a.motivations = vec![Motivation {
            r#type: MotivationType::Learning,
            intensity: 8,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Affiliation,
            intensity: 7,
            notes: String::new(),
        }];
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("take initiative")),
            "Strategy must NOT trigger when neither has Power"
        );
    }

    // ── Conscientiousness strategy threshold (line 936: >= 7) ──

    #[test]
    fn mut_strategy_c_threshold_at_7() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(7);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("structured")),
            "C>=7 must trigger structured strategy"
        );
    }

    #[test]
    fn mut_strategy_c_threshold_below_7() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.conscientiousness = Some(6);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("structured")),
            "C=6 (<7) must NOT trigger structured strategy"
        );
    }

    #[test]
    fn mut_strategy_c_threshold_b_high() {
        let a = p("A");
        let mut b = p("B");
        b.ocean.conscientiousness = Some(8);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("structured")),
            "Strategy must trigger when b has high C (not just a)"
        );
    }

    // ── Conflict resolution: && vs || (line 946, 953) ──

    #[test]
    fn mut_conflict_mediation_only_one_high_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(3);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("mediation")),
            "Mediation strategy must NOT trigger when only one A is high (catches &&→||)"
        );
    }

    #[test]
    fn mut_conflict_direct_only_one_low_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(3);
        b.ocean.agreeableness = Some(7);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("straight to the point")),
            "Direct strategy must NOT trigger when only one A is low (catches &&→||)"
        );
    }

    #[test]
    fn mut_conflict_mediation_both_high() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(7);
        b.ocean.agreeableness = Some(8);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("mediation")),
            "Mediation must trigger when both A >= 7"
        );
    }

    #[test]
    fn mut_conflict_direct_both_low() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(4);
        b.ocean.agreeableness = Some(3);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("straight to the point")),
            "Direct must trigger when both A <= 4"
        );
    }

    // ── OCEAN-gap strategies: boundary at diff >= 3 ──

    #[test]
    fn mut_ocean_gap_e_diff_3_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(8);
        b.ocean.extraversion = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("social pace")),
            "E gap diff=3 must trigger social pace strategy"
        );
    }

    #[test]
    fn mut_ocean_gap_e_diff_2_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(7);
        b.ocean.extraversion = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("social pace")),
            "E gap diff=2 must NOT trigger social pace strategy"
        );
    }

    #[test]
    fn mut_ocean_gap_a_diff_3_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(8);
        b.ocean.agreeableness = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("conflict styles")),
            "A gap diff=3 must trigger conflict styles strategy"
        );
    }

    #[test]
    fn mut_ocean_gap_a_diff_2_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.agreeableness = Some(6);
        b.ocean.agreeableness = Some(4);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("conflict styles")),
            "A gap diff=2 must NOT trigger conflict styles strategy"
        );
    }

    #[test]
    fn mut_ocean_gap_c_diff_3_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(9);
        b.ocean.conscientiousness = Some(6);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            str.iter().any(|s| s.contains("organization levels")),
            "C gap diff=3 must trigger organization levels strategy"
        );
    }

    #[test]
    fn mut_ocean_gap_c_diff_2_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(7);
        b.ocean.conscientiousness = Some(5);
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("organization levels")),
            "C gap diff=2 must NOT trigger organization levels strategy"
        );
    }

    // ── Extraversion friction: boundary at >= 4 ──

    #[test]
    fn mut_extraversion_friction_diff_4_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(2);
        b.ocean.extraversion = Some(6);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("extraversion") || s.contains("social pace")),
            "E friction diff=4 must trigger"
        );
    }

    #[test]
    fn mut_extraversion_friction_diff_3_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(3);
        b.ocean.extraversion = Some(6);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter().any(|s| s.contains("Large extraversion gap")),
            "E friction diff=3 must NOT trigger (threshold is >=4)"
        );
    }

    // ── Rep with different dimensions (catches per-dim iteration) ──

    #[test]
    fn mut_rep_different_dims_not_hardworker() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            honest_deceitful: Some(9),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            honest_deceitful: Some(3),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("imbalance")),
            "Non-hardworker dim far apart must still produce friction"
        );
    }

    #[test]
    fn mut_rep_multiple_dims_produce_multiple_entries() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores = RepScores {
            hardworker_lazy: Some(9),
            honest_deceitful: Some(8),
            ..RepScores::default()
        };
        b.rep_scores = RepScores {
            hardworker_lazy: Some(3),
            honest_deceitful: Some(3),
            ..RepScores::default()
        };
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        let imbalance_count = fri.iter().filter(|s| s.contains("imbalance")).count();
        assert!(
            imbalance_count >= 2,
            "Multiple far-apart rep dims must produce multiple friction entries"
        );
    }

    // ── No OCEAN values → fallback assertions ──

    #[test]
    fn mut_no_ocean_all_none() {
        let a = p("A");
        let b = p("B");
        let (syn, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("No obvious synergy")));
        assert!(fri.iter().any(|s| s.contains("No major friction")));
    }

    // ── Combined: E-A trigger + no agreeableness gap (catches independent conditions) ──

    #[test]
    fn mut_ea_triggers_but_no_a_gap() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(9);
        b.ocean.agreeableness = Some(8);
        a.ocean.agreeableness = Some(6);
        let (syn, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(syn.iter().any(|s| s.contains("one drives")));
        assert!(
            !fri.iter().any(|s| s.contains("low A")),
            "E-A synergy and agreeableness gap are independent conditions"
        );
    }

    // ── Neuroticism: reversed who-is-reactive ──

    #[test]
    fn mut_neuroticism_reactive_person_correctly_identified() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(2);
        b.ocean.neuroticism = Some(8);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("B more reactive to stress than A")),
            "When B has higher N, B must be called more reactive"
        );
    }

    #[test]
    fn mut_neuroticism_swap_reactive_identity() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.neuroticism = Some(8);
        b.ocean.neuroticism = Some(2);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("A more reactive to stress than B")),
            "When A has higher N, A must be called more reactive"
        );
    }

    // ── Strategy with NO motivation types → no motivation-based strategies ──

    #[test]
    fn mut_no_motivations_no_power_strategy() {
        let a = p("A");
        let b = p("B");
        let (_, _, (_, str)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !str.iter().any(|s| s.contains("take initiative")),
            "No motivations → no power strategy"
        );
        assert!(
            !str.iter().any(|s| s.contains("acknowledge")),
            "No motivations → no recognition strategy"
        );
        assert!(
            !str.iter().any(|s| s.contains("stable frameworks")),
            "No motivations → no security strategy"
        );
    }

    // ── E-A boundary: exactly at 7 ──

    #[test]
    fn mut_ea_boundary_exactly_7_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(7);
        b.ocean.agreeableness = Some(7);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            syn.iter().any(|s| s.contains("one drives")),
            "E-A must trigger at exactly E=7, A=7 (>=7 boundary)"
        );
    }

    #[test]
    fn mut_ea_boundary_e7_a6_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(7);
        b.ocean.agreeableness = Some(6);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("one drives")),
            "E-A must NOT trigger when A=6 (<7)"
        );
    }

    // ── O-C boundary: exactly at 7 ──

    #[test]
    fn mut_oc_boundary_exactly_7_triggers() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(7);
        b.ocean.conscientiousness = Some(7);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            syn.iter().any(|s| s.contains("creative vision")),
            "O-C must trigger at exactly O=7, C=7"
        );
    }

    #[test]
    fn mut_oc_boundary_o7_c6_no_trigger() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(7);
        b.ocean.conscientiousness = Some(6);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("creative vision")),
            "O-C must NOT trigger when C=6 (<7)"
        );
    }

    // ── Both neuroticism None → no friction ──

    #[test]
    fn mut_neuroticism_both_none_no_friction() {
        let a = p("A");
        let b = p("B");
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter().any(|s| s.contains("reactive to stress")),
            "Both N=None must not produce neuroticism friction"
        );
    }

    // ── One neuroticism None → no friction ──

    #[test]
    fn mut_neuroticism_one_none_no_friction() {
        let mut a = p("A");
        let b = p("B");
        a.ocean.neuroticism = Some(9);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter().any(|s| s.contains("reactive to stress")),
            "One N=None must not produce neuroticism friction"
        );
    }

    // ── Similar profiles: O diff within 2 but C diff >2 → no similarity ──

    #[test]
    fn mut_similar_profiles_c_too_far() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(7);
        a.ocean.conscientiousness = Some(2);
        b.ocean.openness = Some(8);
        b.ocean.conscientiousness = Some(6);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("similar OCEAN")),
            "Must NOT be 'similar' when C diff=4 (>2)"
        );
    }

    #[test]
    fn mut_similar_profiles_o_too_far() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.openness = Some(2);
        a.ocean.conscientiousness = Some(5);
        b.ocean.openness = Some(6);
        b.ocean.conscientiousness = Some(7);
        let (syn, _, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !syn.iter().any(|s| s.contains("similar OCEAN")),
            "Must NOT be 'similar' when O diff=4 (>2)"
        );
    }

    // ── Top strategy selection: first strategy wins ──

    #[test]
    fn mut_top_strategy_is_first() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.conscientiousness = Some(8);
        a.motivations = vec![Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: String::new(),
        }];
        b.motivations = vec![Motivation {
            r#type: MotivationType::Achievement,
            intensity: 5,
            notes: String::new(),
        }];
        let (_, _, (top, str)) = compare_analysis(&a, &b, Lang::En);
        assert_eq!(top, str[0], "top must equal the first element of str");
        assert!(
            top.contains("take initiative"),
            "Power motivation strategy should be top (first added)"
        );
    }

    // ── Rep imbalance (line 769): !close && va > vb ──

    #[test]
    fn rep_imbalance_a_higher() {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores.hardworker_lazy = Some(9);
        b.rep_scores.hardworker_lazy = Some(3);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("more") && s.contains("imbalance")),
            "expected friction for rep imbalance A>B, got: {:?}",
            fri
        );
    }

    // ── Same bias type (line 815): b1.type == b2.type ──

    #[test]
    fn same_bias_type_adds_friction() {
        let mut a = p("A");
        let mut b = p("B");
        a.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: "e1".into(),
        }];
        b.biases = vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 6,
            evidence: "e2".into(),
        }];
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter().any(|s| s.contains("Same") && s.contains("bias")),
            "expected same-bias friction, got: {:?}",
            fri
        );
    }

    // ── Behavioral patterns FR (line 834): lang == Lang::Fr ──

    #[test]
    fn behavioral_patterns_fr() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksSupport,
            notes: String::new(),
        }];
        let (syn, _, _) = compare_analysis(&a, &b, Lang::Fr);
        assert!(
            syn.iter().any(|s| s.contains("réagit")),
            "expected FR behavioral pattern synergy, got: {:?}",
            syn
        );
    }

    // ── Trigger synergy negative (line 998): t_syn < -0.1 ──

    #[test]
    fn trigger_synergy_negative() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::Escalates,
            notes: String::new(),
        }];
        let (_, _, (_, strats)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            strats.iter().any(|s| s.contains("mutual triggering")),
            "expected trigger clash strategy, got: {:?}",
            strats
        );
    }

    // ── Trigger synergy positive (line 1010): t_syn > 0.1 ──

    #[test]
    fn trigger_synergy_positive() {
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Change,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Feedback,
            predicted_behavior: BehaviorResponse::SeeksSupport,
            notes: String::new(),
        }];
        let (_, _, (_, strats)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            strats
                .iter()
                .any(|s| s.contains("complementarity") || s.contains("balance")),
            "expected trigger complementarity strategy, got: {:?}",
            strats
        );
    }

    // ── prefill_rel_from ──

    #[test]
    fn prefill_rel_from_found_forward() {
        let rels = vec![peoplemodeler_core::models::Relationship {
            id: "r1".into(),
            source_id: "a".into(),
            target_id: "b".into(),
            r#type: RelationType::Friends,
            strength: 8,
            notes: String::new(),
            created_at: 0,
        }];
        let (typ, str) = prefill_rel_from(&rels, "a", "b");
        assert_eq!(typ, Some(RelationType::Friends));
        assert_eq!(str, 8);
    }

    #[test]
    fn prefill_rel_from_found_reverse() {
        let rels = vec![peoplemodeler_core::models::Relationship {
            id: "r1".into(),
            source_id: "b".into(),
            target_id: "a".into(),
            r#type: RelationType::Mentors,
            strength: 3,
            notes: String::new(),
            created_at: 0,
        }];
        let (typ, str) = prefill_rel_from(&rels, "a", "b");
        assert_eq!(typ, Some(RelationType::Mentors));
        assert_eq!(str, 3);
    }

    #[test]
    fn prefill_rel_from_not_found() {
        let rels = vec![];
        let (typ, str) = prefill_rel_from(&rels, "a", "b");
        assert_eq!(typ, None);
        assert_eq!(str, 5);
    }

    // ── benefit_labels ──

    #[test]
    fn benefit_labels_a_higher() {
        let (a, b) = benefit_labels(8, 5, "Alice", "Bob", "more", "balanced");
        assert!(a.contains("+3%"));
        assert!(a.contains("Alice"));
        assert!(b.is_empty());
    }

    #[test]
    fn benefit_labels_b_higher() {
        let (a, b) = benefit_labels(3, 7, "Alice", "Bob", "more", "balanced");
        assert!(a.is_empty());
        assert!(b.contains("+4%"));
        assert!(b.contains("Bob"));
    }

    #[test]
    fn benefit_labels_equal() {
        let (a, b) = benefit_labels(5, 5, "Alice", "Bob", "more", "balanced");
        assert!(a.contains("balanced"));
        assert!(b.contains("balanced"));
    }

    // ── format_band_label ──

    #[test]
    fn format_band_label_positive() {
        let result = format_band_label(3, "Band {} of 10");
        assert_eq!(result, "Band 3 of 10");
    }

    #[test]
    fn format_band_label_zero() {
        let result = format_band_label(0, "Band {} of 10");
        assert_eq!(result, "");
    }

    // ── format_signed_delta ──

    #[test]
    fn format_signed_delta_positive() {
        assert_eq!(format_signed_delta(5), "+5");
    }

    #[test]
    fn format_signed_delta_negative() {
        assert_eq!(format_signed_delta(-3), "-3");
    }

    #[test]
    fn format_signed_delta_zero() {
        assert_eq!(format_signed_delta(0), "");
    }

    // ── prefill_rel_from (line 43): half-matches must not count ──

    #[test]
    fn prefill_rel_from_requires_exact_twin_match() {
        let rel = |s: &str, t: &str| Relationship {
            id: format!("{s}-{t}"),
            source_id: s.into(),
            target_id: t.into(),
            r#type: RelationType::WorksWith,
            strength: 7,
            notes: String::new(),
            created_at: 0,
        };
        assert_eq!(
            prefill_rel_from(&[rel("a", "b")], "a", "b"),
            (Some(RelationType::WorksWith), 7)
        );
        assert_eq!(
            prefill_rel_from(&[rel("b", "a")], "a", "b"),
            (Some(RelationType::WorksWith), 7)
        );
        assert_eq!(prefill_rel_from(&[rel("a", "x")], "a", "b"), (None, 5));
        assert_eq!(prefill_rel_from(&[rel("x", "b")], "a", "b"), (None, 5));
        assert_eq!(prefill_rel_from(&[rel("x", "a")], "a", "b"), (None, 5));
        assert_eq!(prefill_rel_from(&[rel("b", "x")], "a", "b"), (None, 5));
    }

    // ── should_show_extra_strategies (line 165): gate is `len > 1` ──

    #[test]
    fn should_show_extra_strategies_gates() {
        assert!(!should_show_extra_strategies(0));
        assert!(!should_show_extra_strategies(1));
        assert!(should_show_extra_strategies(2));
    }

    // ── Rep reputation imbalance (lines 802-826) ──

    fn rep_pair(va: u8, vb: u8) -> (Person, Person) {
        let mut a = p("A");
        let mut b = p("B");
        a.rep_scores.hardworker_lazy = Some(va);
        b.rep_scores.hardworker_lazy = Some(vb);
        (a, b)
    }

    #[test]
    fn reputation_imbalance_label_a_when_va_gt_vb() {
        let (a, b) = rep_pair(8, 5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("Hardworker") && !s.contains("Lazy")),
            "va>vb should name the high-end label, got: {:?}",
            fri
        );
    }

    #[test]
    fn reputation_close_no_imbalance() {
        let (a, b) = rep_pair(6, 5);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !fri.iter().any(|s| s.contains("imbalance")),
            "close scores must not report imbalance, got: {:?}",
            fri
        );
    }

    #[test]
    fn reputation_imbalance_label_b_when_va_lt_vb() {
        let (a, b) = rep_pair(5, 8);
        let (_, fri, _) = compare_analysis(&a, &b, Lang::En);
        assert!(
            fri.iter()
                .any(|s| s.contains("Lazy") && !s.contains("Hardworker")),
            "vb>va should name the low-end label, got: {:?}",
            fri
        );
    }

    // ── OCEAN-gap social pace (line 998): higher-extraversion named first ──

    #[test]
    fn extraversion_gap_names_higher_pace_person_first() {
        let mut a = p("A");
        let mut b = p("B");
        a.ocean.extraversion = Some(8);
        b.ocean.extraversion = Some(5);
        let (_, _, (_, strats)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            strats
                .iter()
                .any(|s| s.contains("A prefers more interaction")),
            "expected A (higher extraversion) first, got: {:?}",
            strats
        );
    }

    // ── Trigger synergy boundary (line 1031): t_syn == -0.1 is NOT a clash ──

    #[test]
    fn trigger_synergy_exact_minus_point_one_no_strategy() {
        // Conflict×Injustice resolves to exactly -0.1 (model_config
        // trigger_synergy matrix), so the `< -0.1` guard must not fire.
        let mut a = p("A");
        let mut b = p("B");
        a.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Conflict,
            predicted_behavior: BehaviorResponse::RemainsCalm,
            notes: String::new(),
        }];
        b.behavioral_patterns = vec![BehavioralPattern {
            trigger: BehaviorTrigger::Injustice,
            predicted_behavior: BehaviorResponse::SeeksRestoration,
            notes: String::new(),
        }];
        let (_, _, (_, strats)) = compare_analysis(&a, &b, Lang::En);
        assert!(
            !strats.iter().any(|s| s.contains("mutual triggering")),
            "exact -0.1 must not count as a clash, got: {:?}",
            strats
        );
    }
}

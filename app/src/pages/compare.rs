use dioxus::prelude::*;
use peoplemodeler_core::models::{BehaviorTrigger, Person, RelationType};

use peoplemodeler_core::synergy::{
    compute_synergy_score_ctx, compute_synergy_score_with_preds, RelContext, synergy_bands,
};

use crate::db;
use crate::i18n::Lang;

fn core_lang(l: Lang) -> peoplemodeler_core::i18n::Lang {
    match l {
        Lang::Fr => peoplemodeler_core::i18n::Lang::Fr,
        Lang::En => peoplemodeler_core::i18n::Lang::En,
    }
}

/// Prefill the relationship selector from an existing Relationship row between
/// the two ids (either direction).
fn prefill_rel(id1: &str, id2: &str) -> (Option<RelationType>, u8) {
    for r in db::all_relationships() {
        if (r.source_id == id1 && r.target_id == id2)
            || (r.source_id == id2 && r.target_id == id1)
        {
            return (Some(r.r#type), r.strength);
        }
    }
    (None, 5)
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
            let (a_benefit_label, b_benefit_label) = if a_score > b_score {
                (
                    format!(
                        "(+{}% — {} {})",
                        a_score - b_score,
                        na,
                        compare_benefit_more
                    ),
                    String::new(),
                )
            } else if b_score > a_score {
                (
                    String::new(),
                    format!(
                        "(+{}% — {} {})",
                        b_score - a_score,
                        nb,
                        compare_benefit_more
                    ),
                )
            } else {
                (
                    format!("({})", compare_balanced),
                    format!("({})", compare_balanced),
                )
            };
            let compare_breakdown = crate::i18n::tr("compare_breakdown", lang());
            let cat_ocean = crate::i18n::tr("compare_cat_ocean", lang());
            let cat_rep = crate::i18n::tr("compare_cat_reputation", lang());
            let cat_mot = crate::i18n::tr("compare_cat_motivation", lang());
            let cat_pat = crate::i18n::tr("compare_cat_patterns", lang());
            let cat_bias = crate::i18n::tr("compare_cat_bias", lang());
            let cat_styles = crate::i18n::tr("compare_cat_styles", lang());
            let top_mot_label = crate::i18n::tr("compare_top_mot", lang());
            let bias_label = crate::i18n::tr("compare_bias_main", lang());
            let ocean_label = crate::i18n::tr("compare_ocean", lang());
            let analysis_title = crate::i18n::tr("compare_analysis_title", lang());
            let synergies_title = crate::i18n::tr("compare_synergies", lang());
            let friction_title = crate::i18n::tr("compare_friction", lang());
            let strategy_title = crate::i18n::tr("compare_strategy", lang());
            let ethics = crate::i18n::tr("compare_ethics", lang());
            let has_extra_strategies = all_strategies.len() > 1;
            let rel_title = crate::i18n::tr("compare_rel_title", lang());
            let rel_none = crate::i18n::tr("compare_rel_none", lang());
            let rel_strength_label = crate::i18n::tr("compare_rel_strength", lang());
            let band_hint = crate::i18n::tr("compare_band_hint", lang());
            let band_label = if brk.band > 0 {
                band_hint.replacen("{}", &brk.band.to_string(), 1)
            } else {
                String::new()
            };
            let rel_cl = core_lang(lang());

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
            let active_band = scale_bands
                .iter()
                .position(|(_, lo, hi, _)| score >= *lo && score <= *hi)
                .unwrap_or(2);

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
                                    cat_ocean, cat_rep, cat_mot, cat_pat, cat_bias, cat_styles,
                                    s_ocean: brk.ocean, s_rep: brk.reputation,
                                    s_mot: brk.motivation, s_pat: brk.patterns, s_bias: brk.bias,
                                    s_styles: brk.styles,
                                    bias_mod_active: brk.bias_mod_active,
                                }
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
    s_ocean: f64,
    s_rep: f64,
    s_mot: f64,
    s_pat: f64,
    s_bias: f64,
    s_styles: f64,
    bias_mod_active: bool,
) -> Element {
    let cats = [
        (&cat_ocean, s_ocean, false),
        (&cat_rep, s_rep, false),
        (&cat_mot, s_mot, false),
        (&cat_pat, s_pat, false),
        (&cat_bias, s_bias, bias_mod_active),
        (&cat_styles, s_styles, false),
    ];
    let pcts: Vec<u8> = cats.iter().map(|(_, v, _)| (*v * 100.0) as u8).collect();

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
        (Some(b1), Some(b2)) if b1.r#type == b2.r#type => {
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

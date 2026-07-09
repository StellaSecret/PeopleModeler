use dioxus::prelude::*;
use peoplemodeler_core::models::{BehaviorTrigger, BiasType, Person, RepDim, RepScores};

use crate::Route;
use crate::db;
use crate::i18n::Lang;

fn core_lang(l: Lang) -> peoplemodeler_core::i18n::Lang {
    match l {
        Lang::Fr => peoplemodeler_core::i18n::Lang::Fr,
        Lang::En => peoplemodeler_core::i18n::Lang::En,
    }
}

#[component]
pub fn ComparePersons(id1: String, id2: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let p1 = use_signal(|| db::person(&id1));
    let p2 = use_signal(|| db::person(&id2));
    let cl = core_lang(lang());
    let not_found = crate::i18n::tr("person_not_found", lang());
    let compare_title = crate::i18n::tr("compare_title", lang());
    let back_btn = crate::i18n::tr("common_back", lang());

    match (p1(), p2()) {
        (Some(a), Some(b)) => {
            let brk = compute_synergy_score(&a, &b);
            let score = brk.total;
            let (synergies, frictions, strategies) = compare_analysis(&a, &b, lang());
            let compare_sub = crate::i18n::tr("compare_sub", lang());
            let compare_vs = crate::i18n::tr("compare_vs", lang());
            let compare_synergy = crate::i18n::tr("compare_synergy", lang());
            let compare_breakdown = crate::i18n::tr("compare_breakdown", lang());
            let cat_ocean = crate::i18n::tr("compare_cat_ocean", lang());
            let cat_rep = crate::i18n::tr("compare_cat_reputation", lang());
            let cat_mot = crate::i18n::tr("compare_cat_motivation", lang());
            let cat_pat = crate::i18n::tr("compare_cat_patterns", lang());
            let cat_bias = crate::i18n::tr("compare_cat_bias", lang());
            let top_mot_label = crate::i18n::tr("compare_top_mot", lang());
            let bias_label = crate::i18n::tr("compare_bias_main", lang());
            let ocean_label = crate::i18n::tr("compare_ocean", lang());
            let analysis_title = crate::i18n::tr("compare_analysis_title", lang());
            let synergies_title = crate::i18n::tr("compare_synergies", lang());
            let friction_title = crate::i18n::tr("compare_friction", lang());
            let strategy_title = crate::i18n::tr("compare_strategy", lang());
            let ethics = crate::i18n::tr("compare_ethics", lang());

            rsx! {
                div { class: "page",
                    Link { to: Route::PersonDetail { id: id1.clone() }, class: "btn", "{back_btn}" }
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
                        }

                        div { class: "vs-divider",
                            div { class: "vs-text", "{compare_vs}" }
                            div { class: "compatibility-score",
                                CompatRing { score }
                                div { class: "compat-label", "{score}%", br {} "{compare_synergy}" }
                            }
                            div { class: "breakdown-section",
                                h4 { "{compare_breakdown}" }
                                BreakdownBars {
                                    cat_ocean, cat_rep, cat_mot, cat_pat, cat_bias,
                                    s_ocean: brk.ocean, s_rep: brk.reputation,
                                    s_mot: brk.motivation, s_pat: brk.patterns, s_bias: brk.bias,
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
                                ul {
                                    for s in &strategies {
                                        li { "{s}" }
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
    cat_ocean: String, cat_rep: String, cat_mot: String, cat_pat: String, cat_bias: String,
    s_ocean: f64, s_rep: f64, s_mot: f64, s_pat: f64, s_bias: f64,
) -> Element {
    let cats = [
        (&cat_ocean, s_ocean),
        (&cat_rep, s_rep),
        (&cat_mot, s_mot),
        (&cat_pat, s_pat),
        (&cat_bias, s_bias),
    ];
    let pcts: Vec<u8> = cats.iter().map(|(_, v)| (*v * 100.0) as u8).collect();

    rsx! {
        div { class: "breakdown-bars",
            for (i, (label, _)) in cats.iter().enumerate() {
                div { class: "bb-row",
                    span { class: "bb-label", "{label}" }
                    div { class: "bb-bar",
                        div { class: "bb-fill", width: "{pcts[i]}%" }
                    }
                    span { class: "bb-pct", "{pcts[i]}%" }
                }
            }
        }
    }
}

#[component]
fn MiniBars(scores: [u8; 5]) -> Element {
    let labels = ["O", "C", "E", "A", "N"];
    rsx! {
        div { class: "mini-bars",
            for (i, &s) in scores.iter().enumerate() {
                div { class: "mb-row",
                    span { "{labels[i]}" }
                    div { class: "mb-bar",
                        div { class: "mb-fill", width: "{s * 10}%" }
                    }
                }
            }
        }
    }
}

#[component]
fn CompatRing(score: u8) -> Element {
    let r: f64 = 34.0;
    let circ = 2.0 * std::f64::consts::PI * r;
    let offset = circ * (1.0 - score as f64 / 100.0);
    let score_str = format!("{}", score);
    rsx! {
        svg { view_box: "0 0 80 80", width: "80", height: "80",
            circle { cx: "40", cy: "40", r: "{r}", fill: "none",
                stroke: "var(--border)", stroke_width: "8" }
            circle { cx: "40", cy: "40", r: "{r}", fill: "none",
                stroke: "var(--green)", stroke_width: "8",
                stroke_dasharray: "{circ}",
                stroke_dashoffset: "{offset}",
                stroke_linecap: "round",
                transform: "rotate(-90 40 40)" }
            text { x: "40", y: "40", text_anchor: "middle",
                dominant_baseline: "central",
                font_size: "18", font_weight: "700",
                fill: "var(--text)", "{score_str}%" }
        }
    }
}

fn rep_scores_synergy(a: &RepScores, b: &RepScores) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;
    for dim in RepDim::ALL {
        if let (Some(va), Some(vb)) = (a.score(dim), b.score(dim)) {
            let dist = if va >= vb { va - vb } else { vb - va };
            sum += 1.0 - dist as f64 / 10.0;
            count += 1;
        }
    }
    if count == 0 { 0.5 } else { sum / count as f64 }
}

fn bias_synergy(a_type: Option<&BiasType>, b_type: Option<&BiasType>) -> f64 {
    match (a_type, b_type) {
        (Some(a), Some(b)) if a != b => 0.3,
        (Some(_), Some(_)) => 0.0,
        _ => 0.0,
    }
}

fn pattern_synergy(
    a_trigger: Option<&BehaviorTrigger>,
    b_trigger: Option<&BehaviorTrigger>,
) -> f64 {
    match (a_trigger, b_trigger) {
        (Some(a), Some(b)) => match (a, b) {
            (BehaviorTrigger::Change, BehaviorTrigger::Change) => 0.3,
            (BehaviorTrigger::Feedback, BehaviorTrigger::Feedback) => 0.3,
            (BehaviorTrigger::Feedback, BehaviorTrigger::Change)
            | (BehaviorTrigger::Change, BehaviorTrigger::Feedback) => 0.3,
            (BehaviorTrigger::Success, BehaviorTrigger::Success) => 0.3,
            (BehaviorTrigger::Conflict, BehaviorTrigger::Conflict) => -0.3,
            (BehaviorTrigger::Stress, BehaviorTrigger::Stress) => -0.2,
            (BehaviorTrigger::Stress, BehaviorTrigger::Conflict)
            | (BehaviorTrigger::Conflict, BehaviorTrigger::Stress) => -0.3,
            (BehaviorTrigger::Change, BehaviorTrigger::Stress)
            | (BehaviorTrigger::Stress, BehaviorTrigger::Change) => -0.2,
            (BehaviorTrigger::Conflict, BehaviorTrigger::Uncertainty)
            | (BehaviorTrigger::Uncertainty, BehaviorTrigger::Conflict) => -0.2,
            (BehaviorTrigger::Feedback, BehaviorTrigger::Recognition)
            | (BehaviorTrigger::Recognition, BehaviorTrigger::Feedback) => 0.2,
            _ => 0.0,
        },
        _ => 0.0,
    }
}

struct SynergyBreakdown {
    pub total: u8,
    pub ocean: f64,
    pub reputation: f64,
    pub motivation: f64,
    pub patterns: f64,
    pub bias: f64,
}

fn compute_synergy_score(a: &Person, b: &Person) -> SynergyBreakdown {
    let oa = &a.ocean;
    let ob = &b.ocean;

    let oc = if (oa.openness >= 7 && ob.conscientiousness >= 7)
        || (ob.openness >= 7 && oa.conscientiousness >= 7)
    {
        1.0
    } else if oa.openness.abs_diff(ob.openness) <= 3
        && oa.conscientiousness.abs_diff(ob.conscientiousness) <= 3
    {
        0.7
    } else {
        0.4
    };

    let ea = if (oa.extraversion >= 7 && ob.agreeableness >= 7)
        || (ob.extraversion >= 7 && oa.agreeableness >= 7)
    {
        1.0
    } else if oa.extraversion.abs_diff(ob.extraversion) <= 3
        && oa.agreeableness.abs_diff(ob.agreeableness) <= 3
    {
        0.7
    } else {
        0.4
    };

    let nd = oa.neuroticism.abs_diff(ob.neuroticism);
    let n = if nd <= 2 {
        0.8
    } else if nd <= 4 {
        0.5
    } else {
        0.3
    };

    let ocean = (oc + ea + n) / 3.0;

    // Motivation: weighted by min intensity / 10, different types get bonus
    let mot_raw = match (a.top_motivation(), b.top_motivation()) {
        (Some(m1), Some(m2)) => {
            let w = (m1.intensity.min(m2.intensity) as f64) / 10.0;
            let base = if m1.r#type != m2.r#type { 0.6 } else { 0.3 };
            base + 0.4 * w
        }
        _ => 0.5,
    };

    // Reputation: distance-based synergy, average of shared dimensions
    let reputation = rep_scores_synergy(&a.rep_scores, &b.rep_scores);

    // Patterns: weighted by min confidence / 10
    let pat_raw = {
        let pa = a.behavioral_patterns.iter().max_by_key(|p| p.confidence);
        let pb = b.behavioral_patterns.iter().max_by_key(|p| p.confidence);
        let r = pattern_synergy(pa.map(|p| &p.trigger), pb.map(|p| &p.trigger));
        let w = match (pa, pb) {
            (Some(p1), Some(p2)) => (p1.confidence.min(p2.confidence) as f64) / 10.0,
            _ => 1.0,
        };
        r * w
    };
    let patterns = (pat_raw + 0.5).clamp(0.0, 1.0);

    // Bias: different types = bonus
    let bias_raw = {
        let ba = a.top_bias();
        let bb = b.top_bias();
        let r = bias_synergy(ba.map(|b| &b.r#type), bb.map(|b| &b.r#type));
        let w = match (ba, bb) {
            (Some(b1), Some(b2)) => (b1.intensity.min(b2.intensity) as f64) / 10.0,
            _ => 1.0,
        };
        r * w
    };
    let bias = (0.5 + bias_raw).clamp(0.0, 1.0);

    let motivation = mot_raw;
    let raw = ocean * 0.30 + reputation * 0.30 + motivation * 0.20 + patterns * 0.12 + bias * 0.08;
    let score = ((raw * 100.0).round() as u8).max(25).min(98);

    SynergyBreakdown { total: score, ocean, reputation, motivation, patterns, bias }
}

fn compare_analysis(a: &Person, b: &Person, lang: Lang) -> (Vec<String>, Vec<String>, Vec<String>) {
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
    if oa.openness >= 7 && ob.conscientiousness >= 7 {
        syn.push(if lang == Lang::Fr {
            format!("{na} apporte la vision créative, {nb} assure l'exécution rigoureuse")
        } else {
            format!("{na} brings creative vision, {nb} ensures rigorous execution")
        });
    } else if ob.openness >= 7 && oa.conscientiousness >= 7 {
        syn.push(if lang == Lang::Fr {
            format!("{nb} apporte la vision créative, {na} assure l'exécution rigoureuse")
        } else {
            format!("{nb} brings creative vision, {na} ensures rigorous execution")
        });
    } else if oa.openness.abs_diff(ob.openness) <= 2
        && oa.conscientiousness.abs_diff(ob.conscientiousness) <= 2
    {
        syn.push(if lang == Lang::Fr {
            "Profils OCEAN très proches — communication fluide et attentes alignées".into()
        } else {
            "Very similar OCEAN profiles — smooth communication and aligned expectations".into()
        });
    }

    // E-A complementarity
    if (oa.extraversion >= 7 && ob.agreeableness >= 7)
        || (ob.extraversion >= 7 && oa.agreeableness >= 7)
    {
        syn.push(if lang == Lang::Fr {
            "Extraversion et agréabilité se compensent : l'un conduit, l'autre harmonise".into()
        } else {
            "Extraversion and agreeableness complement: one drives, one harmonizes".into()
        });
    }

    // Motivation non-conflict
    match (a.top_motivation(), b.top_motivation()) {
        (Some(m1), Some(m2)) if m1.r#type != m2.r#type => {
            syn.push(if lang == Lang::Fr {
                format!(
                    "Motivations non-concurrentes — {} et {} ne se marchent pas sur les pieds",
                    m1.r#type.i18n(cl).label,
                    m2.r#type.i18n(cl).label,
                )
            } else {
                format!(
                    "Non-competing motivations — {} and {} don't step on each other",
                    m1.r#type.i18n(cl).label,
                    m2.r#type.i18n(cl).label,
                )
            });
        }
        (Some(m1), Some(_)) => {
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
        _ => {}
    }

    // --- Frictions ---

    // Agreeableness gap
    if oa.agreeableness >= 7 && ob.agreeableness <= 4 {
        fri.push(if lang == Lang::Fr {
            format!("{nb} (faible A) peut sembler agressif pour {na} (haute A)")
        } else {
            format!("{nb} (low A) may seem aggressive to {na} (high A)")
        });
    } else if ob.agreeableness >= 7 && oa.agreeableness <= 4 {
        fri.push(if lang == Lang::Fr {
            format!("{na} (faible A) peut sembler agressif pour {nb} (haute A)")
        } else {
            format!("{na} (low A) may seem aggressive to {nb} (high A)")
        });
    }

    // Neuroticism gap
    let nd = oa.neuroticism.abs_diff(ob.neuroticism);
    if nd >= 3 {
        let (stable, reactive) = if oa.neuroticism <= ob.neuroticism {
            (&na, &nb)
        } else {
            (&nb, &na)
        };
        fri.push(if lang == Lang::Fr {
            format!("{reactive} plus réactif au stress que {stable} — risque d'incompréhension")
        } else {
            format!("{reactive} more reactive to stress than {stable} — risk of misunderstanding")
        });
    }

    // Reputation synergy (distance-based, per shared dimension)
    {
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
    match (
        a.behavioral_patterns.iter().max_by_key(|p| p.confidence),
        b.behavioral_patterns.iter().max_by_key(|p| p.confidence),
    ) {
        (Some(pa), Some(pb)) => match (&pa.trigger, &pb.trigger) {
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
        },
        _ => {}
    }

    // Extraversion gap friction
    let ed = oa.extraversion.abs_diff(ob.extraversion);
    if ed >= 4 {
        fri.push(if lang == Lang::Fr {
            "Écart d'extraversion important — rythme social et besoin de stimulation différents"
                .into()
        } else {
            "Large extraversion gap — different social pace and stimulation needs".into()
        });
    }

    // --- Strategies ---

    // Based on top motivations
    match (a.top_motivation(), b.top_motivation()) {
        (Some(m1), Some(m2)) => {
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
        _ => {}
    }

    // Conscientiousness-based strategy
    if oa.conscientiousness >= 7 || ob.conscientiousness >= 7 {
        str.push(if lang == Lang::Fr {
            "Présenter les informations de manière structurée avec des données tangibles".into()
        } else {
            "Present information in a structured way with tangible data".into()
        });
    }

    // Conflict resolution
    if oa.agreeableness >= 7 && ob.agreeableness >= 7 {
        str.push(if lang == Lang::Fr {
            "En cas de conflit, privilégier la médiation — les deux parties chercheront l'harmonie"
                .into()
        } else {
            "In conflict, prioritize mediation — both parties will seek harmony".into()
        });
    } else if oa.agreeableness <= 4 && ob.agreeableness <= 4 {
        str.push(if lang == Lang::Fr {
            "En cas de désaccord, aller droit au fait — les deux préfèrent la franchise".into()
        } else {
            "When disagreeing, get straight to the point — both prefer directness".into()
        });
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
    if str.is_empty() {
        str.push(if lang == Lang::Fr {
            "Communiquer ouvertement et observer les réactions".into()
        } else {
            "Communicate openly and observe reactions".into()
        });
    }

    (syn, fri, str)
}

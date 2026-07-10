use dioxus::prelude::*;
use peoplemodeler_core::models::{
    AVATAR_EMOJIS, BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation, MotivationType,
    OceanScores, Person, RepDim, RepScores,
};

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
pub fn PersonNew() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut selected = use_signal(|| None::<usize>);
    let templates = crate::templates::all();
    let new_person_title = crate::i18n::tr("form_new_title", lang());
    let template_title = crate::i18n::tr("template_title", lang());
    let template_blank = crate::i18n::tr("template_blank", lang());

    match selected() {
        Some(idx) => {
            let blank = Person {
                id: uuid::Uuid::new_v4().to_string(),
                name: String::new(),
                role: String::new(),
                context: String::new(),
                avatar_emoji: "👤".into(),
                tags: Vec::new(),
                notes: String::new(),
                motivations: Vec::new(),
                biases: Vec::new(),
                rep_scores: RepScores::default(),
                behavioral_patterns: Vec::new(),
                ocean: OceanScores::default(),
                confidence: 5,
                predictions: Vec::new(),
                log: Vec::new(),
                created_at: chrono::Utc::now().timestamp_millis(),
                updated_at: chrono::Utc::now().timestamp_millis(),
            };
            let person = if idx < templates.len() {
                let t = &templates[idx];
                Person {
                    ocean: t.ocean.clone(),
                    motivations: t.motivations.clone(),
                    biases: t.biases.clone(),
                    rep_scores: t.rep_scores.clone(),
                    ..blank
                }
            } else {
                blank
            };
            rsx! { PersonEditForm { initial: person } }
        }
        None => rsx! {
            div { class: "page",
                h2 { "{new_person_title}" }
                h3 { "{template_title}" }
                div { class: "template-grid",
                    for (i, tpl) in templates.iter().enumerate() {
                        button { class: "template-card", onclick: move |_| selected.set(Some(i)),
                            span { class: "template-emoji", "{tpl.emoji}" }
                            strong { "{tpl.name}" }
                        }
                    }
                }
                p { class: "template-skip",
                    button { class: "btn", onclick: move |_| selected.set(Some(999)), "{template_blank}" }
                }
            }
        },
    }
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
    let mut toast_sig = use_context::<Signal<Option<String>>>();
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
        rep_scores: RepScores::default(),
        behavioral_patterns: Vec::new(),
        ocean: OceanScores::default(),
        confidence: 5,
        predictions: Vec::new(),
        log: Vec::new(),
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
    let mut confidence = use_signal(|| p.confidence);
    let motivations = use_signal(|| p.motivations.clone());
    let biases = use_signal(|| p.biases.clone());
    let rep_scores = use_signal(|| p.rep_scores.clone());
    let patterns = use_signal(|| p.behavioral_patterns.clone());

    let pers_id = p.id.clone();

    let mut save = move || {
        let person = Person {
            id: pers_id.clone(),
            name: name(),
            role: role(),
            context: context(),
            avatar_emoji: emoji(),
            tags: tags_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            notes: notes(),
            motivations: motivations(),
            biases: biases(),
            rep_scores: rep_scores(),
            behavioral_patterns: patterns(),
            ocean: ocean(),
            confidence: confidence(),
            predictions: Vec::new(),
            log: Vec::new(),
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: chrono::Utc::now().timestamp_millis(),
        };
        db::save_person(&person);
        toast_sig.set(Some(crate::i18n::tr("toast_saved", lang()).into()));
        dioxus::prelude::navigator().push(Route::PersonDetail {
            id: pers_id.clone(),
        });
    };

    let form_new_title = crate::i18n::tr("form_new_title", lang());
    let form_edit_title = crate::i18n::tr("form_edit_title", lang());
    let form_name = crate::i18n::tr("form_name", lang());
    let form_role = crate::i18n::tr("form_role", lang());
    let form_context = crate::i18n::tr("form_context", lang());
    let form_avatar = crate::i18n::tr("form_avatar", lang());
    let form_tags = crate::i18n::tr("form_tags", lang());
    let form_notes = crate::i18n::tr("form_notes", lang());
    let form_confidence = crate::i18n::tr("form_confidence", lang());
    let form_ocean_title = crate::i18n::tr("form_ocean_title", lang());
    let form_save = crate::i18n::tr("form_save", lang());
    let form_cancel = crate::i18n::tr("form_cancel", lang());
    let cl = core_lang(lang());

    rsx! {
        div { class: "page",
            h2 { if is_new { "{form_new_title}" } else { "{form_edit_title}" } }
            div { class: "form",
                label { "{form_name}" }
                input { aria_label: "{form_name}", value: "{name}", oninput: move |e| name.set(e.value()) }

                label { "{form_role}" }
                input { aria_label: "{form_role}", value: "{role}", oninput: move |e| role.set(e.value()) }

                label { "{form_context}" }
                textarea { aria_label: "{form_context}", value: "{context}", oninput: move |e| context.set(e.value()) }

                label { "{form_avatar}" }
                div { class: "emoji-picker", role: "radiogroup", aria_label: "{form_avatar}",
                    for e in AVATAR_EMOJIS {
                        button {
                            class: "emoji-btn",
                            class: if emoji() == *e { "selected" },
                            role: "radio",
                            aria_label: "Avatar {e}",
                            aria_checked: if emoji() == *e { "true" } else { "false" },
                            onclick: move |_| emoji.set(e.to_string()),
                            "{e}"
                        }
                    }
                }

                label { "{form_tags}" }
                input { aria_label: "{form_tags}", value: "{tags_str}", oninput: move |e| tags_str.set(e.value()) }

                label { "{form_notes}" }
                textarea { aria_label: "{form_notes}", value: "{notes}", rows: "4", oninput: move |e| notes.set(e.value()) }

                label { "{form_confidence}" }
                div { class: "ocean-slider",
                    span { "{confidence}/10" }
                    input { r#type: "range", min: "1", max: "10", value: "{confidence}",
                        oninput: move |e| confidence.set(e.value().parse().unwrap_or(5)),
                    }
                }

                fieldset { class: "ocean-inputs",
                    legend { "{form_ocean_title}" }
                    OceanSlider {
                        label: crate::i18n::tr("ocean_openness", lang()),
                        val: ocean().openness,
                        onchange: move |v| { let mut o = ocean.write(); o.openness = v; },
                        low_hint: Some(crate::i18n::tr("ocean_o_low", lang()).into()),
                        high_hint: Some(crate::i18n::tr("ocean_o_high", lang()).into()),
                    }
                    OceanSlider {
                        label: crate::i18n::tr("ocean_conscientiousness", lang()),
                        val: ocean().conscientiousness,
                        onchange: move |v| { let mut o = ocean.write(); o.conscientiousness = v; },
                        low_hint: Some(crate::i18n::tr("ocean_c_low", lang()).into()),
                        high_hint: Some(crate::i18n::tr("ocean_c_high", lang()).into()),
                    }
                    OceanSlider {
                        label: crate::i18n::tr("ocean_extraversion", lang()),
                        val: ocean().extraversion,
                        onchange: move |v| { let mut o = ocean.write(); o.extraversion = v; },
                        low_hint: Some(crate::i18n::tr("ocean_e_low", lang()).into()),
                        high_hint: Some(crate::i18n::tr("ocean_e_high", lang()).into()),
                    }
                    OceanSlider {
                        label: crate::i18n::tr("ocean_agreeableness", lang()),
                        val: ocean().agreeableness,
                        onchange: move |v| { let mut o = ocean.write(); o.agreeableness = v; },
                        low_hint: Some(crate::i18n::tr("ocean_a_low", lang()).into()),
                        high_hint: Some(crate::i18n::tr("ocean_a_high", lang()).into()),
                    }
                    OceanSlider {
                        label: crate::i18n::tr("ocean_neuroticism", lang()),
                        val: ocean().neuroticism,
                        onchange: move |v| { let mut o = ocean.write(); o.neuroticism = v; },
                        low_hint: Some(crate::i18n::tr("ocean_n_low", lang()).into()),
                        high_hint: Some(crate::i18n::tr("ocean_n_high", lang()).into()),
                    }
                }

                MotEditPanel { motivations: motivations.clone(), lang: cl }
                BiasEditPanel { biases: biases.clone(), lang: cl }
                RepEditPanel { rep_scores: rep_scores.clone(), lang: cl }
                PatternEditPanel { patterns: patterns.clone(), lang: lang() }

                div { class: "form-actions",
                    button { class: "btn btn-primary", aria_label: "{form_save}", onclick: move |_| save(), "{form_save}" }
                    Link { to: Route::PeopleList {}, class: "btn", aria_label: "{form_cancel}", "{form_cancel}" }
                }
            }
        }
    }
}

#[component]
fn MotEditPanel(
    motivations: Signal<Vec<Motivation>>,
    lang: peoplemodeler_core::i18n::Lang,
) -> Element {
    let app_lang = use_context::<Signal<Lang>>();
    let mut sel_type = use_signal(|| MotivationType::Achievement);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let mut edit_idx = use_signal(|| None::<usize>);
    let edit_motivations = crate::i18n::tr("edit_motivations", app_lang());
    let notes_pl = crate::i18n::tr("edit_notes_placeholder", app_lang());
    let add_btn = crate::i18n::tr("add_btn", app_lang());
    let update_btn = crate::i18n::tr("edit_update_btn", app_lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_motivations}" }
            div { class: "add-row",
                select { value: "{sel_type}",
                    onchange: move |e| { sel_type.set(parse_mot_type(&e.value())); },
                    for t in MotivationType::ALL {
                        option { value: "{t:?}", "{t.emoji()} {t.i18n(lang).label}" }
                    }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_intensity}",
                    oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_intensity}" }
                input { placeholder: "{notes_pl}", value: "{sel_notes}",
                    oninput: move |e| { sel_notes.set(e.value()); }
                }
                button { class: "btn", aria_label: if edit_idx().is_some() { "Update motivation" } else { "Add motivation" }, onclick: move |_| {
                    if let Some(idx) = edit_idx() {
                        let mut items = motivations.write();
                        if idx < items.len() {
                            items[idx] = Motivation { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() };
                        }
                        edit_idx.set(None);
                    } else {
                        motivations.write().push(Motivation { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() });
                    }
                    sel_notes.set(String::new());
                    sel_intensity.set(5);
                }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
            }
            div { class: "helper-text", "{mot_helper(&sel_type(), app_lang())}" }
            for (i, m) in motivations().iter().enumerate() {
                div { class: "list-item",
                    button { class: "reorder-btn", aria_label: "Move motivation up", onclick: move |_| { mot_move(motivations, i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move motivation down", onclick: move |_| { mot_move(motivations, i, false); }, "▼" }
                    button { class: "btn btn-small", aria_label: "Edit motivation", onclick: {
                        let m = m.clone();
                        move |_| {
                            sel_type.set(m.r#type);
                            sel_intensity.set(m.intensity);
                            sel_notes.set(m.notes.clone());
                            edit_idx.set(Some(i));
                        }
                    }, "✏" }
                    strong { "{m.r#type.emoji()} {m.r#type.i18n(lang).label}" }
                    span { " {m.intensity}/10" }
                    span { " {m.notes}" }
                    button { class: "btn btn-small", aria_label: "Delete motivation", onclick: move |_| { motivations.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

fn mot_move(mut motivations: Signal<Vec<Motivation>>, i: usize, up: bool) {
    let len = motivations.read().len();
    if up && i > 0 {
        motivations.write().swap(i, i - 1);
    } else if !up && i + 1 < len {
        motivations.write().swap(i, i + 1);
    }
}

#[component]
fn BiasEditPanel(biases: Signal<Vec<Bias>>, lang: peoplemodeler_core::i18n::Lang) -> Element {
    let app_lang = use_context::<Signal<Lang>>();
    let mut sel_type = use_signal(|| BiasType::Confirmation);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_evidence = use_signal(String::new);
    let mut edit_idx = use_signal(|| None::<usize>);
    let edit_biases = crate::i18n::tr("edit_biases", app_lang());
    let evidence_pl = crate::i18n::tr("edit_evidence_placeholder", app_lang());
    let add_btn = crate::i18n::tr("add_btn", app_lang());
    let update_btn = crate::i18n::tr("edit_update_btn", app_lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_biases}" }
            div { class: "add-row",
                select { value: "{sel_type}",
                    onchange: move |e| { sel_type.set(parse_bias_type(&e.value())); },
                    for t in BiasType::ALL {
                        option { value: "{t:?}", "{t.emoji()} {t.i18n(lang).label}" }
                    }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_intensity}",
                    oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_intensity}" }
                input { placeholder: "{evidence_pl}", value: "{sel_evidence}",
                    oninput: move |e| { sel_evidence.set(e.value()); }
                }
                button { class: "btn", aria_label: if edit_idx().is_some() { "Update bias" } else { "Add bias" }, onclick: move |_| {
                    if let Some(idx) = edit_idx() {
                        let mut items = biases.write();
                        if idx < items.len() {
                            items[idx] = Bias { r#type: sel_type(), intensity: sel_intensity(), evidence: sel_evidence() };
                        }
                        edit_idx.set(None);
                    } else {
                        biases.write().push(Bias { r#type: sel_type(), intensity: sel_intensity(), evidence: sel_evidence() });
                    }
                    sel_evidence.set(String::new());
                    sel_intensity.set(5);
                }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
            }
            div { class: "helper-text", "{bias_helper(&sel_type(), app_lang())}" }
            for (i, b) in biases().iter().enumerate() {
                div { class: "list-item",
                    button { class: "reorder-btn", aria_label: "Move bias up", onclick: move |_| { bias_move(biases, i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move bias down", onclick: move |_| { bias_move(biases, i, false); }, "▼" }
                    button { class: "btn btn-small", aria_label: "Edit bias", onclick: {
                        let b = b.clone();
                        move |_| {
                            sel_type.set(b.r#type);
                            sel_intensity.set(b.intensity);
                            sel_evidence.set(b.evidence.clone());
                            edit_idx.set(Some(i));
                        }
                    }, "✏" }
                    strong { "{b.r#type.emoji()} {b.r#type.i18n(lang).label}" }
                    span { " {b.intensity}/10" }
                    span { " {b.evidence}" }
                    button { class: "btn btn-small", aria_label: "Delete bias", onclick: move |_| { biases.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

fn bias_move(mut biases: Signal<Vec<Bias>>, i: usize, up: bool) {
    let len = biases.read().len();
    if up && i > 0 {
        biases.write().swap(i, i - 1);
    } else if !up && i + 1 < len {
        biases.write().swap(i, i + 1);
    }
}

#[component]
fn RepEditPanel(rep_scores: Signal<RepScores>, lang: peoplemodeler_core::i18n::Lang) -> Element {
    let app_lang = use_context::<Signal<Lang>>();
    let edit_rep = crate::i18n::tr("edit_reputation", app_lang());
    let cl = core_lang(app_lang());

    let rep_data: Vec<_> = RepDim::ALL
        .iter()
        .map(|dim| {
            let ri = dim.i18n(cl);
            let cur = rep_scores.read().score(*dim);
            (*dim, ri, cur)
        })
        .collect();

    rsx! {
        fieldset { class: "ocean-inputs",
            legend { "{edit_rep}" }
            {rep_data.into_iter().map(|(dim, ri, cur)| {
                let start_val = cur.unwrap_or(5);
                let start_on = cur.is_some();
                rsx! {
                    RepDimSlider {
                        dim,
                        label_a: ri.label_a,
                        label_b: ri.label_b,
                        desc: ri.desc,
                        start_val,
                        start_on,
                        onchange: move |(on, val): (bool, u8)| {
                            let mut s = rep_scores.write();
                            let new = if on { Some(val.clamp(0, 10)) } else { None };
                            match dim {
                                RepDim::HardworkerLazy => s.hardworker_lazy = new,
                                RepDim::AuthoritativeSubmissive => s.authoritative_submissive = new,
                                RepDim::HonestDeceitful => s.honest_deceitful = new,
                                RepDim::ReliableFlaky => s.reliable_flaky = new,
                                RepDim::HumbleArrogant => s.humble_arrogant = new,
                                RepDim::CalmReactive => s.calm_reactive = new,
                                RepDim::DiplomaticBlunt => s.diplomatic_blunt = new,
                                RepDim::GenerousSelfish => s.generous_selfish = new,
                            }
                        }
                    }
                }
            })}
        }
    }
}

#[component]
fn RepDimSlider(
    dim: RepDim,
    label_a: &'static str,
    label_b: &'static str,
    desc: &'static str,
    start_val: u8,
    start_on: bool,
    onchange: EventHandler<(bool, u8)>,
) -> Element {
    let mut on = use_signal(|| start_on);
    let mut val = use_signal(|| start_val);

    rsx! {
        div { class: "ocean-slider",
            div { class: "ocean-header",
                span { class: "ocean-label",
                    "{dim.emoji()} {label_b} ← → {label_a}"
                }
                label { class: "dim-toggle",
                    input { r#type: "checkbox",
                        checked: on(),
                        oninput: move |e| {
                            let new = e.value() == "true";
                            on.set(new);
                            onchange.call((new, val()));
                        }
                    }
                    if on() { "✓" } else { "✗" }
                }
            }
            if on() {
                div { class: "rep-slider-bar",
                    span { class: "rep-pole-b", "{label_b}" }
                    input { r#type: "range", min: "0", max: "10", value: "{val}",
                        oninput: move |e| {
                            let v = e.value().parse().unwrap_or(5);
                            val.set(v);
                            onchange.call((true, v));
                        }
                    }
                    span { class: "rep-pole-a", "{label_a}" }
                }
                div { class: "rep-dim-value",
                    strong { "{val}/10" }
                    span { " — {desc}" }
                }
            }
        }
    }
}

#[component]
fn PatternEditPanel(patterns: Signal<Vec<BehavioralPattern>>, lang: Lang) -> Element {
    let ctx_stress = crate::i18n::tr("ctx_stress", lang);
    let ctx_conflict = crate::i18n::tr("ctx_conflict", lang);
    let ctx_success = crate::i18n::tr("ctx_success", lang);
    let ctx_uncertainty = crate::i18n::tr("ctx_uncertainty", lang);
    let ctx_recognition = crate::i18n::tr("ctx_recognition", lang);
    let ctx_threatened = crate::i18n::tr("ctx_threatened", lang);
    let ctx_change = crate::i18n::tr("ctx_change", lang);
    let ctx_feedback = crate::i18n::tr("ctx_feedback", lang);
    let mut sel_trigger = use_signal(|| BehaviorTrigger::Stress);
    let mut sel_behavior = use_signal(String::new);
    let mut sel_conf = use_signal(|| 5u8);

    let edit_patterns = crate::i18n::tr("edit_patterns", lang);
    let outcome_pl = crate::i18n::tr("pred_outcome_placeholder", lang);
    let add_btn = crate::i18n::tr("add_btn", lang);
    let pattern_hint = crate::i18n::tr("pattern_hint", lang);
    let trigger_label = |t: BehaviorTrigger| -> &'static str {
        match t {
            BehaviorTrigger::Stress => ctx_stress,
            BehaviorTrigger::Conflict => ctx_conflict,
            BehaviorTrigger::Success => ctx_success,
            BehaviorTrigger::Uncertainty => ctx_uncertainty,
            BehaviorTrigger::Recognition => ctx_recognition,
            BehaviorTrigger::Threatened => ctx_threatened,
            BehaviorTrigger::Change => ctx_change,
            BehaviorTrigger::Feedback => ctx_feedback,
        }
    };

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
                    option { value: "Change", "{ctx_change}" }
                    option { value: "Feedback", "{ctx_feedback}" }
                }
                input { placeholder: "{outcome_pl}", value: "{sel_behavior}",
                    oninput: move |e| { sel_behavior.set(e.value()); }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_conf}",
                    oninput: move |e| { sel_conf.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_conf}" }
                small { " {pattern_hint}" }
                button { class: "btn", aria_label: "Add pattern", onclick: move |_| {
                    patterns.write().push(BehavioralPattern { trigger: sel_trigger(), predicted_behavior: sel_behavior(), confidence: sel_conf() });
                    sel_behavior.set(String::new());
                }, "{add_btn}" }
            }
            div { class: "helper-text", "{pattern_helper(&sel_trigger(), lang)}" }
            for (i, bp) in patterns().iter().enumerate() {
                div { class: "list-item",
                    strong { "{trigger_label(bp.trigger)}" }
                    span { " {bp.predicted_behavior}" }
                    span { " ({bp.confidence}/10)" }
                    button { class: "btn btn-small", aria_label: "Delete pattern", onclick: move |_| { patterns.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

#[component]
fn OceanSlider(
    label: String,
    val: u8,
    onchange: EventHandler<u8>,
    low_hint: Option<String>,
    high_hint: Option<String>,
) -> Element {
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
            if let (Some(l), Some(h)) = (low_hint.as_ref(), high_hint.as_ref()) {
                div { class: "ocean-hint",
                    span { class: "hint-low", "↓ {l}" }
                    span { class: "hint-sep", "|" }
                    span { class: "hint-high", "↑ {h}" }
                }
            }
        }
    }
}

fn mot_helper(t: &MotivationType, lang: Lang) -> &'static str {
    match t {
        MotivationType::Achievement => crate::i18n::tr("mot_helper_achievement", lang),
        MotivationType::Power => crate::i18n::tr("mot_helper_power", lang),
        MotivationType::Affiliation => crate::i18n::tr("mot_helper_affiliation", lang),
        MotivationType::Security => crate::i18n::tr("mot_helper_security", lang),
        MotivationType::Autonomy => crate::i18n::tr("mot_helper_autonomy", lang),
        MotivationType::Recognition => crate::i18n::tr("mot_helper_recognition", lang),
        MotivationType::Learning => crate::i18n::tr("mot_helper_learning", lang),
        MotivationType::Helping => crate::i18n::tr("mot_helper_helping", lang),
    }
}

fn bias_helper(t: &BiasType, lang: Lang) -> &'static str {
    match t {
        BiasType::Confirmation => crate::i18n::tr("bias_helper_confirmation", lang),
        BiasType::Anchoring => crate::i18n::tr("bias_helper_anchoring", lang),
        BiasType::Availability => crate::i18n::tr("bias_helper_availability", lang),
        BiasType::SunkCost => crate::i18n::tr("bias_helper_sunk_cost", lang),
        BiasType::DunningKruger => crate::i18n::tr("bias_helper_dunning_kruger", lang),
        BiasType::LossAversion => crate::i18n::tr("bias_helper_loss_aversion", lang),
        BiasType::SocialProof => crate::i18n::tr("bias_helper_social_proof", lang),
        BiasType::Authority => crate::i18n::tr("bias_helper_authority", lang),
        BiasType::Recency => crate::i18n::tr("bias_helper_recency", lang),
        BiasType::InGroup => crate::i18n::tr("bias_helper_in_group", lang),
    }
}

fn pattern_helper(t: &BehaviorTrigger, lang: Lang) -> &'static str {
    match t {
        BehaviorTrigger::Stress => crate::i18n::tr("pattern_helper_stress", lang),
        BehaviorTrigger::Conflict => crate::i18n::tr("pattern_helper_conflict", lang),
        BehaviorTrigger::Success => crate::i18n::tr("pattern_helper_success", lang),
        BehaviorTrigger::Uncertainty => crate::i18n::tr("pattern_helper_uncertainty", lang),
        BehaviorTrigger::Recognition => crate::i18n::tr("pattern_helper_recognition", lang),
        BehaviorTrigger::Threatened => crate::i18n::tr("pattern_helper_threat", lang),
        BehaviorTrigger::Change => crate::i18n::tr("pattern_helper_change", lang),
        BehaviorTrigger::Feedback => crate::i18n::tr("pattern_helper_feedback", lang),
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
        "Change" => BehaviorTrigger::Change,
        "Feedback" => BehaviorTrigger::Feedback,
        _ => BehaviorTrigger::Stress,
    }
}

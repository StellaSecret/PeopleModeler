use dioxus::prelude::*;
use peoplemodeler_core::models::{
    AVATAR_EMOJIS, BehaviorResponse, BehaviorTrigger, BehavioralPattern, Bias, BiasType,
    Motivation, MotivationType, OceanScores, Person, PersonalStyle, RepDim, RepScores,
    StyleCategory, StyleType, Tag, Value, ValueType,
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
                styles: Vec::new(),
                values: Vec::new(),
                ocean: OceanScores::default(),
                resilience: None,
                risk_appetite: None,
                confidence: 5,
                log: Vec::new(),
                created_at: chrono::Utc::now().timestamp_millis(),
                updated_at: chrono::Utc::now().timestamp_millis(),
            };
            let person = if idx < templates.len() {
                let t = &templates[idx];
                let mut person = blank;
                person.ocean = t.ocean.clone();
                person.motivations = t.motivations.clone();
                person.biases = t.biases.clone();
                person.rep_scores = t.rep_scores.clone();
                person
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
        styles: Vec::new(),
        values: Vec::new(),
        ocean: OceanScores::default(),
        resilience: None,
        risk_appetite: None,
        confidence: 5,
        log: Vec::new(),
        created_at: chrono::Utc::now().timestamp_millis(),
        updated_at: chrono::Utc::now().timestamp_millis(),
    });

    let mut name = use_signal(|| p.name.clone());
    let mut role = use_signal(|| p.role.clone());
    let mut context = use_signal(|| p.context.clone());
    let mut emoji = use_signal(|| p.avatar_emoji.clone());
    let mut notes = use_signal(|| p.notes.clone());
    let mut tags_str = use_signal(|| {
        p.tags
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    });
    let mut ocean = use_signal(|| p.ocean.clone());
    let mut confidence = use_signal(|| p.confidence);
    let mut resilience = use_signal(|| p.resilience.unwrap_or(5));
    let mut risk_appetite = use_signal(|| p.risk_appetite.unwrap_or(5));
    let motivations = use_signal(|| p.motivations.clone());
    let biases = use_signal(|| p.biases.clone());
    let rep_scores = use_signal(|| p.rep_scores.clone());
    let patterns = use_signal(|| p.behavioral_patterns.clone());
    let styles = use_signal(|| p.styles.clone());
    let values = use_signal(|| p.values.clone());

    let ocean_rep_flags = use_memo(move || {
        let mut flags = peoplemodeler_core::validation::ocean_rep_flags(&ocean(), &rep_scores());
        flags.extend(peoplemodeler_core::validation::rhetoric_gap_flags(
            &ocean(),
            &rep_scores(),
            &motivations(),
        ));
        if peoplemodeler_core::validation::pattern_calm_volatile_gap(&patterns(), &rep_scores()) {
            flags.push("flag_pattern_calm_volatile");
        }
        if peoplemodeler_core::validation::pattern_honest_exploiter_gap(&patterns(), &rep_scores())
        {
            flags.push("flag_pattern_honest_exploiter");
        }
        if peoplemodeler_core::validation::bias_confirmation_open_gap(&biases(), &ocean()) {
            flags.push("flag_bias_confirmation_open");
        }
        if peoplemodeler_core::validation::bias_favoritism_fairness_gap(&biases(), &motivations()) {
            flags.push("flag_bias_favoritism_fairness");
        }
        if peoplemodeler_core::validation::authority_dominant_gap(&biases(), &rep_scores()) {
            flags.push("flag_authority_dominant");
        }
        if peoplemodeler_core::validation::social_proof_open_gap(&biases(), &ocean()) {
            flags.push("flag_social_proof_open");
        }
        if peoplemodeler_core::validation::sunk_cost_flexible_gap(&biases(), &rep_scores()) {
            flags.push("flag_sunk_cost_flexible");
        }
        if peoplemodeler_core::validation::loss_aversion_risky_gap(&biases(), Some(risk_appetite()))
        {
            flags.push("flag_loss_aversion_risky");
        }
        if peoplemodeler_core::validation::dunning_kruger_humble_gap(&biases(), &rep_scores()) {
            flags.push("flag_dunning_kruger_humble");
        }
        if peoplemodeler_core::validation::impostor_arrogant_gap(&biases(), &rep_scores()) {
            flags.push("flag_impostor_arrogant");
        }
        if peoplemodeler_core::validation::recency_reliable_gap(&biases(), &rep_scores()) {
            flags.push("flag_recency_reliable");
        }
        if peoplemodeler_core::validation::pattern_diplomat_escalator_gap(
            &patterns(),
            &rep_scores(),
        ) {
            flags.push("flag_pattern_diplomat_escalator");
        }
        if peoplemodeler_core::validation::pattern_fair_exploiter_gap(&patterns(), &rep_scores()) {
            flags.push("flag_pattern_fair_exploiter");
        }
        if peoplemodeler_core::validation::pattern_humble_dismissive_gap(&patterns(), &rep_scores())
        {
            flags.push("flag_pattern_humble_dismissive");
        }
        if peoplemodeler_core::validation::pattern_trusting_paranoid_gap(&patterns(), &rep_scores())
        {
            flags.push("flag_pattern_trusting_paranoid");
        }
        if peoplemodeler_core::validation::pattern_reliable_shirker_gap(&patterns(), &rep_scores())
        {
            flags.push("flag_pattern_reliable_shirker");
        }
        if peoplemodeler_core::validation::pattern_hardworker_complacent_gap(
            &patterns(),
            &rep_scores(),
        ) {
            flags.push("flag_pattern_hardworker_complacent");
        }
        if peoplemodeler_core::validation::pattern_passive_blowup_gap(&patterns(), &rep_scores()) {
            flags.push("flag_pattern_passive_blowup");
        }
        if peoplemodeler_core::validation::pattern_assertive_quiet_gap(&patterns(), &rep_scores()) {
            flags.push("flag_pattern_assertive_quiet");
        }
        if peoplemodeler_core::validation::security_risky_gap(&motivations(), Some(risk_appetite()))
        {
            flags.push("flag_security_risky");
        }
        if peoplemodeler_core::validation::resilient_reactive_gap(Some(resilience()), &rep_scores())
        {
            flags.push("flag_resilient_reactive");
        }
        if peoplemodeler_core::validation::risk_appetite_ambition_gap(
            &motivations(),
            Some(risk_appetite()),
        ) {
            flags.push("flag_risk_appetite_ambition");
        }
        if peoplemodeler_core::validation::resilient_hides_gap(Some(resilience()), &rep_scores()) {
            flags.push("flag_resilient_hides");
        }
        if peoplemodeler_core::validation::pattern_generous_exploiter_gap(
            &patterns(),
            &rep_scores(),
        ) {
            flags.push("flag_pattern_generous_exploiter");
        }
        if peoplemodeler_core::validation::pattern_empath_dismissive_gap(&patterns(), &rep_scores())
        {
            flags.push("flag_pattern_empath_dismissive");
        }
        if peoplemodeler_core::validation::pattern_flexible_resister_gap(&patterns(), &rep_scores())
        {
            flags.push("flag_pattern_flexible_resister");
        }
        if peoplemodeler_core::validation::anchoring_open_gap(&biases(), &ocean()) {
            flags.push("flag_anchoring_open");
        }
        if peoplemodeler_core::validation::pattern_helping_exploiter_gap(
            &patterns(),
            &motivations(),
        ) {
            flags.push("flag_pattern_helping_exploiter");
        }
        if peoplemodeler_core::validation::pattern_warmth_dismissive_gap(&patterns(), &ocean()) {
            flags.push("flag_pattern_warmth_dismissive");
        }
        if peoplemodeler_core::validation::pattern_discipline_shirker_gap(&patterns(), &ocean()) {
            flags.push("flag_pattern_discipline_shirker");
        }
        if peoplemodeler_core::validation::pattern_claimed_calm_volatile_gap(&patterns(), &ocean())
        {
            flags.push("flag_pattern_claimed_calm_volatile");
        }
        if peoplemodeler_core::validation::pattern_fairness_exploiter_gap(
            &patterns(),
            &motivations(),
        ) {
            flags.push("flag_pattern_fairness_exploiter");
        }
        if peoplemodeler_core::validation::pattern_achievement_complacent_gap(
            &patterns(),
            &motivations(),
        ) {
            flags.push("flag_pattern_achievement_complacent");
        }
        if peoplemodeler_core::validation::pattern_learning_resister_gap(
            &patterns(),
            &motivations(),
        ) {
            flags.push("flag_pattern_learning_resister");
        }
        if peoplemodeler_core::validation::pattern_extravert_quiet_gap(&patterns(), &ocean()) {
            flags.push("flag_pattern_extravert_quiet");
        }
        if peoplemodeler_core::validation::pattern_open_resister_gap(&patterns(), &ocean()) {
            flags.push("flag_pattern_open_resister");
        }
        if peoplemodeler_core::validation::pattern_recognition_dismissive_gap(
            &patterns(),
            &motivations(),
        ) {
            flags.push("flag_pattern_recognition_dismissive");
        }
        if peoplemodeler_core::validation::availability_calm_gap(&biases(), &rep_scores()) {
            flags.push("flag_availability_calm");
        }
        flags.extend(peoplemodeler_core::validation::style_gap_flags(
            &styles(),
            &rep_scores(),
        ));
        flags
    });

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
                .map(|name| Tag { name, color: None })
                .collect(),
            notes: notes(),
            motivations: motivations(),
            biases: biases(),
            rep_scores: rep_scores(),
            behavioral_patterns: patterns(),
            styles: styles(),
            values: values(),
            ocean: ocean(),
            resilience: Some(resilience()),
            risk_appetite: Some(risk_appetite()),
            confidence: confidence(),
            log: p.log.clone(),
            created_at: if is_new {
                chrono::Utc::now().timestamp_millis()
            } else {
                p.created_at
            },
            updated_at: chrono::Utc::now().timestamp_millis(),
        };
        if let Err(e) = db::save_person(&person) {
            toast_sig.set(Some(format!(
                "{}: {e}",
                crate::i18n::tr("toast_error", lang())
            )));
            return;
        }
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
    let confidence_hint = crate::i18n::tr("confidence_hint", lang());
    let reliability_title = crate::i18n::tr("reliability_title", lang());
    let form_resilience = crate::i18n::tr("form_resilience", lang());
    let form_risk_appetite = crate::i18n::tr("form_risk_appetite", lang());
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

                fieldset { class: "reliability",
                    legend { "{reliability_title}" }
                    div { class: "reliability-hint", "{confidence_hint}" }
                    label { "{form_confidence}" }
                    div { class: "ocean-slider",
                        span { "{confidence}/10" }
                        input { r#type: "range", min: "1", max: "10", value: "{confidence}",
                            oninput: move |e| confidence.set(e.value().parse().unwrap_or(5)),
                        }
                    }
                }

                label { "{form_resilience}" }
                div { class: "ocean-slider",
                    span { "{resilience}/10" }
                    input { r#type: "range", min: "1", max: "10", value: "{resilience}",
                        oninput: move |e| resilience.set(e.value().parse().unwrap_or(5)),
                    }
                }

                label { "{form_risk_appetite}" }
                div { class: "ocean-slider",
                    span { "{risk_appetite}/10" }
                    input { r#type: "range", min: "1", max: "10", value: "{risk_appetite}",
                        oninput: move |e| risk_appetite.set(e.value().parse().unwrap_or(5)),
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
                for key in ocean_rep_flags() {
                    div { class: "danger-warning", "⚠ {crate::i18n::tr(key, lang())}" }
                }

                MotEditPanel { motivations, lang: cl }
                BiasEditPanel { biases, lang: cl }
                RepEditPanel { rep_scores, lang: cl }
                PatternEditPanel { patterns, lang: lang() }
                StyleEditPanel { styles, lang: cl }
                ValEditPanel { values, lang: cl }

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
    let mot_undefined_warning = crate::i18n::tr("mot_undefined_warning", app_lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_motivations}" }
            div { class: "helper-text", "{mot_undefined_warning}" }
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
                    button { class: "reorder-btn", aria_label: "Move motivation up", onclick: move |_| { swap_item_in_list(&mut motivations.write(), i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move motivation down", onclick: move |_| { swap_item_in_list(&mut motivations.write(), i, false); }, "▼" }
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

fn swap_item_in_list<T>(list: &mut [T], i: usize, up: bool) {
    let len = list.len();
    if up && i > 0 {
        list.swap(i, i - 1);
    } else if !up && i + 1 < len {
        list.swap(i, i + 1);
    }
}

#[component]
fn ValEditPanel(values: Signal<Vec<Value>>, lang: peoplemodeler_core::i18n::Lang) -> Element {
    let app_lang = use_context::<Signal<Lang>>();
    let mut sel_type = use_signal(|| ValueType::Career);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_priority = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let mut edit_idx = use_signal(|| None::<usize>);
    let edit_values = crate::i18n::tr("edit_values", app_lang());
    let notes_pl = crate::i18n::tr("edit_notes_placeholder", app_lang());
    let priority_label = crate::i18n::tr("edit_priority", app_lang());
    let value_intensity_helper = crate::i18n::tr("value_intensity_helper", app_lang());
    let value_priority_helper = crate::i18n::tr("value_priority_helper", app_lang());
    let add_btn = crate::i18n::tr("add_btn", app_lang());
    let update_btn = crate::i18n::tr("edit_update_btn", app_lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_values}" }
            div { class: "add-row",
                select { value: "{sel_type}",
                    onchange: move |e| { sel_type.set(parse_val_type(&e.value())); },
                    for t in ValueType::ALL {
                        option { value: "{t:?}", "{t.emoji()} {t.i18n(lang).label}" }
                    }
                }
                div { class: "dual-range",
                    span { "{sel_intensity()}" }
                    input { r#type: "range", min: "1", max: "10", value: "{sel_intensity}",
                        oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                    }
                    span { class: "range-label", "I" }
                    span { "{sel_priority()}" }
                    input { r#type: "range", min: "1", max: "10", value: "{sel_priority}",
                        oninput: move |e| { sel_priority.set(e.value().parse().unwrap_or(5)); }
                    }
                    span { class: "range-label", "{priority_label}" }
                }
                input { placeholder: "{notes_pl}", value: "{sel_notes}",
                    oninput: move |e| { sel_notes.set(e.value()); }
                }
                button { class: "btn", aria_label: if edit_idx().is_some() { "Update value" } else { "Add value" }, onclick: move |_| {
                    if let Some(idx) = edit_idx() {
                        let mut items = values.write();
                        if idx < items.len() {
                            items[idx] = Value { r#type: sel_type(), intensity: sel_intensity(), priority: sel_priority(), notes: sel_notes() };
                        }
                        edit_idx.set(None);
                    } else {
                        values.write().push(Value { r#type: sel_type(), intensity: sel_intensity(), priority: sel_priority(), notes: sel_notes() });
                    }
                    sel_notes.set(String::new());
                    sel_intensity.set(5);
                    sel_priority.set(5);
                }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
            }
            div { class: "helper-text",
                div { "{value_helper(&sel_type(), app_lang())}" }
                div { "{value_intensity_helper}" }
                div { "{value_priority_helper}" }
            }
            for (i, v) in values().iter().enumerate() {
                div { class: "list-item",
                    button { class: "reorder-btn", aria_label: "Move value up", onclick: move |_| { swap_item_in_list(&mut values.write(), i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move value down", onclick: move |_| { swap_item_in_list(&mut values.write(), i, false); }, "▼" }
                    button { class: "btn btn-small", aria_label: "Edit value", onclick: {
                        let v = v.clone();
                        move |_| {
                            sel_type.set(v.r#type);
                            sel_intensity.set(v.intensity);
                            sel_priority.set(v.priority);
                            sel_notes.set(v.notes.clone());
                            edit_idx.set(Some(i));
                        }
                    }, "✏" }
                    strong { "{v.r#type.emoji()} {v.r#type.i18n(lang).label}" }
                    span { " I{v.intensity}/10 P{v.priority}/10" }
                    span { " {v.notes}" }
                    button { class: "btn btn-small", aria_label: "Delete value", onclick: move |_| { values.write().remove(i); }, "✕" }
                }
            }
        }
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
    let bias_undefined_warning = crate::i18n::tr("bias_undefined_warning", app_lang());
    let evidence_pl = crate::i18n::tr("edit_evidence_placeholder", app_lang());
    let add_btn = crate::i18n::tr("add_btn", app_lang());
    let update_btn = crate::i18n::tr("edit_update_btn", app_lang());

    rsx! {
        fieldset { class: "section",
            legend { "{edit_biases}" }
            div { class: "helper-text", "{bias_undefined_warning}" }
            div { class: "add-row",
                select { value: "{sel_type}",
                    onchange: move |e| { sel_type.set(parse_bias_type(&e.value())); },
                    for t in BiasType::ALL {
                        option { value: "{t:?}", "{t.emoji()} {t.i18n(lang).label}" }
                    }
                }
                input { r#type: "range", min: "0", max: "10", value: "{sel_intensity}",
                    oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_intensity}/10" }
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
                    button { class: "reorder-btn", aria_label: "Move bias up", onclick: move |_| { swap_item_in_list(&mut biases.write(), i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move bias down", onclick: move |_| { swap_item_in_list(&mut biases.write(), i, false); }, "▼" }
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

#[component]
fn RepEditPanel(rep_scores: Signal<RepScores>, lang: peoplemodeler_core::i18n::Lang) -> Element {
    let app_lang = use_context::<Signal<Lang>>();
    let edit_rep = crate::i18n::tr("edit_reputation", app_lang());
    let rep_undefined_warning = crate::i18n::tr("rep_undefined_warning", app_lang());
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
            div { class: "helper-text", "{rep_undefined_warning}" }
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
                                RepDim::FairFavoritism => s.fair_favoritism = new,
                                RepDim::TrustingSuspicious => s.trusting_suspicious = new,
                                RepDim::AssertivePassive => s.assertive_passive = new,
                                RepDim::EmpatheticDetached => s.empathetic_detached = new,
                                RepDim::AdaptableRigid => s.adaptable_rigid = new,
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
    let ctx_injustice = crate::i18n::tr("ctx_injustice", lang);
    let mut sel_trigger = use_signal(|| BehaviorTrigger::Stress);
    let mut sel_behavior = use_signal(|| BehaviorResponse::SeeksSupport);
    let mut sel_notes = use_signal(String::new);
    let mut edit_idx = use_signal(|| None::<usize>);

    let cl = core_lang(lang);

    let edit_patterns = crate::i18n::tr("edit_patterns", lang);
    let notes_pl = crate::i18n::tr("edit_notes_placeholder", lang);
    let add_btn = crate::i18n::tr("add_btn", lang);
    let update_btn = crate::i18n::tr("edit_update_btn", lang);
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
            BehaviorTrigger::Injustice => ctx_injustice,
        }
    };

    rsx! {
        fieldset { class: "section",
            legend { "{edit_patterns}" }
            div { class: "add-row",
                select { value: "{sel_trigger}",
                    onchange: move |e| { sel_trigger.set(parse_trigger(&e.value())); sel_behavior.set(BehaviorResponse::options_for(sel_trigger())[0]); },
                    option { value: "Stress", "{ctx_stress}" }
                    option { value: "Conflict", "{ctx_conflict}" }
                    option { value: "Success", "{ctx_success}" }
                    option { value: "Uncertainty", "{ctx_uncertainty}" }
                    option { value: "Recognition", "{ctx_recognition}" }
                    option { value: "Threatened", "{ctx_threatened}" }
                    option { value: "Change", "{ctx_change}" }
                    option { value: "Feedback", "{ctx_feedback}" }
                    option { value: "Injustice", "{ctx_injustice}" }
                }
                select { value: "{sel_behavior().serde_name()}",
                    onchange: move |e| { let _ = parse_response(&e.value()).map(|v| sel_behavior.set(v)); },
                    for opt in BehaviorResponse::options_for(sel_trigger()) {
                        option { value: "{opt.serde_name()}", "{opt.label(cl)}" }
                    }
                }
                input {
                    r#type: "text",
                    placeholder: "{notes_pl}",
                    value: "{sel_notes()}",
                    oninput: move |e| sel_notes.set(e.value()),
                }
                button { class: "btn", aria_label: if edit_idx().is_some() { "Update pattern" } else { "Add pattern" }, onclick: move |_| {
                    if let Some(idx) = edit_idx() {
                        let mut items = patterns.write();
                        if idx < items.len() {
                            items[idx] = BehavioralPattern { trigger: sel_trigger(), predicted_behavior: sel_behavior(), notes: sel_notes() };
                        }
                        edit_idx.set(None);
                    } else {
                        patterns.write().push(BehavioralPattern { trigger: sel_trigger(), predicted_behavior: sel_behavior(), notes: sel_notes() });
                    }
                    sel_behavior.set(BehaviorResponse::options_for(sel_trigger())[0]);
                    sel_notes.set(String::new());
                }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
            }
            div { class: "helper-text", "{pattern_helper(&sel_trigger(), lang)}" }
            for (i, bp) in patterns().iter().enumerate() {
                div { class: "list-item",
                    button { class: "reorder-btn", aria_label: "Move pattern up", onclick: move |_| { swap_item_in_list(&mut patterns.write(), i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move pattern down", onclick: move |_| { swap_item_in_list(&mut patterns.write(), i, false); }, "▼" }
                    button { class: "btn btn-small", aria_label: "Edit pattern", onclick: {
                        let bp = bp.clone();
                        move |_| {
                            sel_trigger.set(bp.trigger);
                            sel_behavior.set(bp.predicted_behavior);
                            sel_notes.set(bp.notes.clone());
                            edit_idx.set(Some(i));
                        }
                    }, "✏" }
                    strong { "{trigger_label(bp.trigger)}" }
                    span { " {bp.predicted_behavior.label(cl)}" }
                    if !bp.notes.is_empty() {
                        span { class: "item-notes", " — {bp.notes}" }
                    }
                    button { class: "btn btn-small", aria_label: "Delete pattern", onclick: move |_| { patterns.write().remove(i); sel_notes.set(String::new()); }, "✕" }
                }
            }
        }
    }
}

#[component]
fn OceanSlider(
    label: String,
    val: Option<u8>,
    onchange: EventHandler<Option<u8>>,
    low_hint: Option<String>,
    high_hint: Option<String>,
) -> Element {
    let current = val.unwrap_or(5);
    rsx! {
        div { class: "ocean-slider",
            label { "{label}" }
            input {
                r#type: "range",
                min: "1",
                max: "10",
                value: "{current}",
                oninput: move |e| onchange.call(Some(e.value().parse::<u8>().unwrap_or(5))),
            }
            span { if val.is_some() { "{current}" } else { "—" } }
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
        MotivationType::Creativity => crate::i18n::tr("mot_helper_creativity", lang),
        MotivationType::Fairness => crate::i18n::tr("mot_helper_fairness", lang),
    }
}

fn bias_helper(t: &BiasType, lang: Lang) -> &'static str {
    match t {
        BiasType::Confirmation => crate::i18n::tr("bias_helper_confirmation", lang),
        BiasType::Anchoring => crate::i18n::tr("bias_helper_anchoring", lang),
        BiasType::Availability => crate::i18n::tr("bias_helper_availability", lang),
        BiasType::SunkCost => crate::i18n::tr("bias_helper_sunk_cost", lang),
        BiasType::DunningKruger => crate::i18n::tr("bias_helper_dunning_kruger", lang),
        BiasType::Impostor => crate::i18n::tr("bias_helper_impostor", lang),
        BiasType::LossAversion => crate::i18n::tr("bias_helper_loss_aversion", lang),
        BiasType::SocialProof => crate::i18n::tr("bias_helper_social_proof", lang),
        BiasType::Authority => crate::i18n::tr("bias_helper_authority", lang),
        BiasType::Recency => crate::i18n::tr("bias_helper_recency", lang),
        BiasType::InGroup => crate::i18n::tr("bias_helper_in_group", lang),
        BiasType::Favoritism => crate::i18n::tr("bias_helper_favoritism", lang),
    }
}

fn style_helper(t: &StyleType, lang: Lang) -> &'static str {
    t.i18n_desc(core_lang(lang))
}

fn value_helper(t: &ValueType, lang: Lang) -> &'static str {
    t.i18n(core_lang(lang)).desc
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
        BehaviorTrigger::Injustice => crate::i18n::tr("pattern_helper_injustice", lang),
    }
}

fn parse_mot_type(s: &str) -> MotivationType {
    match s {
        "Power" => MotivationType::Power,
        "Affiliation" => MotivationType::Affiliation,
        "Security" => MotivationType::Security,
        "Autonomy" => MotivationType::Autonomy,
        "Recognition" => MotivationType::Recognition,
        "Learning" => MotivationType::Learning,
        "Helping" => MotivationType::Helping,
        "Creativity" => MotivationType::Creativity,
        "Fairness" => MotivationType::Fairness,
        _ => MotivationType::Achievement,
    }
}

fn parse_val_type(s: &str) -> ValueType {
    match s {
        "Family" => ValueType::Family,
        "Health" => ValueType::Health,
        "Wealth" => ValueType::Wealth,
        "Stability" => ValueType::Stability,
        "Adventure" => ValueType::Adventure,
        "Community" => ValueType::Community,
        "Knowledge" => ValueType::Knowledge,
        "Faith" => ValueType::Faith,
        "Loyalty" => ValueType::Loyalty,
        _ => ValueType::Career,
    }
}

fn parse_bias_type(s: &str) -> BiasType {
    match s {
        "Anchoring" => BiasType::Anchoring,
        "Availability" => BiasType::Availability,
        "SunkCost" => BiasType::SunkCost,
        "DunningKruger" => BiasType::DunningKruger,
        "Impostor" => BiasType::Impostor,
        "LossAversion" => BiasType::LossAversion,
        "SocialProof" => BiasType::SocialProof,
        "Authority" => BiasType::Authority,
        "Recency" => BiasType::Recency,
        "InGroup" => BiasType::InGroup,
        "Favoritism" => BiasType::Favoritism,
        _ => BiasType::Confirmation,
    }
}

fn parse_trigger(s: &str) -> BehaviorTrigger {
    match s {
        "Conflict" => BehaviorTrigger::Conflict,
        "Success" => BehaviorTrigger::Success,
        "Uncertainty" => BehaviorTrigger::Uncertainty,
        "Recognition" => BehaviorTrigger::Recognition,
        "Threatened" => BehaviorTrigger::Threatened,
        "Change" => BehaviorTrigger::Change,
        "Feedback" => BehaviorTrigger::Feedback,
        "Injustice" => BehaviorTrigger::Injustice,
        _ => BehaviorTrigger::Stress,
    }
}

fn parse_response(s: &str) -> Option<BehaviorResponse> {
    serde_json::from_str(&format!("\"{}\"", s)).ok()
}

fn parse_style_type(s: &str) -> StyleType {
    serde_json::from_str(&format!("\"{}\"", s)).unwrap_or(StyleType::DirectCommunicator)
}

fn parse_style_category(s: &str) -> peoplemodeler_core::models::StyleCategory {
    match s {
        "ConflictResolution" => peoplemodeler_core::models::StyleCategory::ConflictResolution,
        "DecisionMaking" => peoplemodeler_core::models::StyleCategory::DecisionMaking,
        "Leadership" => peoplemodeler_core::models::StyleCategory::Leadership,
        "TimeOrientation" => peoplemodeler_core::models::StyleCategory::TimeOrientation,
        "MoralFramework" => peoplemodeler_core::models::StyleCategory::MoralFramework,
        "InterpersonalConduct" => peoplemodeler_core::models::StyleCategory::InterpersonalConduct,
        "TrustStyle" => peoplemodeler_core::models::StyleCategory::TrustStyle,
        _ => peoplemodeler_core::models::StyleCategory::Communication,
    }
}

fn coerce_style_to_category(cat: StyleCategory, sel: StyleType) -> StyleType {
    let opts = StyleType::options_for(cat);
    if opts.contains(&sel) { sel } else { opts[0] }
}

#[component]
fn StyleEditPanel(
    styles: Signal<Vec<PersonalStyle>>,
    lang: peoplemodeler_core::i18n::Lang,
) -> Element {
    use peoplemodeler_core::models::StyleCategory;

    let app_lang = use_context::<Signal<Lang>>();
    let mut sel_category = use_signal(|| StyleCategory::Communication);
    let mut sel_type = use_signal(|| StyleType::DirectCommunicator);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let mut edit_idx = use_signal(|| None::<usize>);
    let panel_title = crate::i18n::tr("edit_styles", app_lang());
    let notes_pl = crate::i18n::tr("edit_notes_placeholder", app_lang());
    let add_btn = crate::i18n::tr("add_btn", app_lang());
    let update_btn = crate::i18n::tr("edit_update_btn", app_lang());

    let cl = core_lang(app_lang());

    use_effect(move || {
        let cat = sel_category();
        sel_type.set(coerce_style_to_category(cat, sel_type()));
    });

    rsx! {
        fieldset { class: "section",
            legend { "{panel_title}" }
            div { class: "add-row",
                select {
                    value: "{sel_category():?}",
                    onchange: move |e| {
                        let cat = parse_style_category(&e.value());
                        sel_category.set(cat);
                    },
                    for cat in StyleCategory::ALL {
                        option { value: "{cat:?}", "{cat.i18n_label(cl)}" }
                    }
                }
                select { value: "{sel_type()}",
                    onchange: move |e| { sel_type.set(parse_style_type(&e.value())); },
                    for t in StyleType::options_for(sel_category()) {
                        option { value: "{t:?}", "{t.emoji()} {t.i18n_label(cl)}" }
                    }
                }
                input { r#type: "range", min: "1", max: "10", value: "{sel_intensity}",
                    oninput: move |e| { sel_intensity.set(e.value().parse().unwrap_or(5)); }
                }
                span { "{sel_intensity}" }
                input { placeholder: "{notes_pl}", value: "{sel_notes}",
                    oninput: move |e| { sel_notes.set(e.value()); }
                }
                button { class: "btn", aria_label: if edit_idx().is_some() { "Update style" } else { "Add style" }, onclick: move |_| {
                    if let Some(idx) = edit_idx() {
                        let mut items = styles.write();
                        if idx < items.len() {
                            items[idx] = PersonalStyle { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() };
                        }
                        edit_idx.set(None);
                    } else {
                        styles.write().push(PersonalStyle { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() });
                    }
                    sel_notes.set(String::new());
                    sel_intensity.set(5);
                }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
            }
            div { class: "helper-text", "{style_helper(&sel_type(), app_lang())}" }
            for (i, s) in styles().iter().enumerate() {
                div { class: "list-item",
                    button { class: "reorder-btn", aria_label: "Move style up", onclick: move |_| { swap_item_in_list(&mut styles.write(), i, true); }, "▲" }
                    button { class: "reorder-btn", aria_label: "Move style down", onclick: move |_| { swap_item_in_list(&mut styles.write(), i, false); }, "▼" }
                    button { class: "btn btn-small", aria_label: "Edit style", onclick: {
                        let s = s.clone();
                        move |_| {
                            sel_category.set(s.r#type.category());
                            sel_type.set(s.r#type);
                            sel_intensity.set(s.intensity);
                            sel_notes.set(s.notes.clone());
                            edit_idx.set(Some(i));
                        }
                    }, "✏" }
                    span { class: "style-cat-badge", "{s.r#type.category().i18n_label(cl)}" }
                    strong { "{s.r#type.emoji()} {s.r#type.i18n_label(cl)}" }
                    span { " {s.intensity}/10" }
                    span { " {s.notes}" }
                    button { class: "btn btn-small", aria_label: "Delete style", onclick: move |_| { styles.write().remove(i); }, "✕" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mot_helper_all_variants() {
        let lang = Lang::En;
        assert!(!mot_helper(&MotivationType::Achievement, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Power, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Affiliation, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Security, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Autonomy, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Recognition, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Learning, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Helping, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Creativity, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Fairness, lang).is_empty());
    }

    #[test]
    fn bias_helper_all_variants() {
        let lang = Lang::En;
        assert!(!bias_helper(&BiasType::Confirmation, lang).is_empty());
        assert!(!bias_helper(&BiasType::Anchoring, lang).is_empty());
        assert!(!bias_helper(&BiasType::Availability, lang).is_empty());
        assert!(!bias_helper(&BiasType::SunkCost, lang).is_empty());
        assert!(!bias_helper(&BiasType::DunningKruger, lang).is_empty());
        assert!(!bias_helper(&BiasType::Impostor, lang).is_empty());
        assert!(!bias_helper(&BiasType::LossAversion, lang).is_empty());
        assert!(!bias_helper(&BiasType::SocialProof, lang).is_empty());
        assert!(!bias_helper(&BiasType::Authority, lang).is_empty());
        assert!(!bias_helper(&BiasType::Recency, lang).is_empty());
        assert!(!bias_helper(&BiasType::InGroup, lang).is_empty());
        assert!(!bias_helper(&BiasType::Favoritism, lang).is_empty());
    }

    #[test]
    fn style_helper_not_empty() {
        let lang = Lang::En;
        for t in StyleType::ALL {
            let h = style_helper(&t, lang);
            assert!(!h.is_empty(), "style_helper empty for {t:?}");
        }
    }

    #[test]
    fn pattern_helper_all_variants() {
        let lang = Lang::En;
        assert!(!pattern_helper(&BehaviorTrigger::Stress, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Conflict, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Success, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Uncertainty, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Recognition, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Threatened, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Change, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Feedback, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Injustice, lang).is_empty());
    }

    #[test]
    fn parse_mot_type_all_variants() {
        assert_eq!(parse_mot_type("Power"), MotivationType::Power);
        assert_eq!(parse_mot_type("Achievement"), MotivationType::Achievement);
        assert_eq!(parse_mot_type("Affiliation"), MotivationType::Affiliation);
        assert_eq!(parse_mot_type("Security"), MotivationType::Security);
        assert_eq!(parse_mot_type("Autonomy"), MotivationType::Autonomy);
        assert_eq!(parse_mot_type("Recognition"), MotivationType::Recognition);
        assert_eq!(parse_mot_type("Learning"), MotivationType::Learning);
        assert_eq!(parse_mot_type("Helping"), MotivationType::Helping);
        assert_eq!(parse_mot_type("Creativity"), MotivationType::Creativity);
        assert_eq!(parse_mot_type("Fairness"), MotivationType::Fairness);
    }

    #[test]
    fn parse_mot_type_unknown() {
        assert_eq!(parse_mot_type("bogus"), MotivationType::Achievement);
    }

    #[test]
    fn parse_val_type_all_variants() {
        assert_eq!(parse_val_type("Career"), ValueType::Career);
        assert_eq!(parse_val_type("Family"), ValueType::Family);
        assert_eq!(parse_val_type("Health"), ValueType::Health);
        assert_eq!(parse_val_type("Wealth"), ValueType::Wealth);
        assert_eq!(parse_val_type("Stability"), ValueType::Stability);
        assert_eq!(parse_val_type("Adventure"), ValueType::Adventure);
        assert_eq!(parse_val_type("Community"), ValueType::Community);
        assert_eq!(parse_val_type("Knowledge"), ValueType::Knowledge);
        assert_eq!(parse_val_type("Faith"), ValueType::Faith);
        assert_eq!(parse_val_type("Loyalty"), ValueType::Loyalty);
    }

    #[test]
    fn parse_val_type_unknown() {
        assert_eq!(parse_val_type("bogus"), ValueType::Career);
    }

    #[test]
    fn parse_bias_type_all_variants() {
        assert_eq!(parse_bias_type("Confirmation"), BiasType::Confirmation);
        assert_eq!(parse_bias_type("Anchoring"), BiasType::Anchoring);
        assert_eq!(parse_bias_type("Availability"), BiasType::Availability);
        assert_eq!(parse_bias_type("SunkCost"), BiasType::SunkCost);
        assert_eq!(parse_bias_type("DunningKruger"), BiasType::DunningKruger);
        assert_eq!(parse_bias_type("Impostor"), BiasType::Impostor);
        assert_eq!(parse_bias_type("LossAversion"), BiasType::LossAversion);
        assert_eq!(parse_bias_type("SocialProof"), BiasType::SocialProof);
        assert_eq!(parse_bias_type("Authority"), BiasType::Authority);
        assert_eq!(parse_bias_type("Recency"), BiasType::Recency);
        assert_eq!(parse_bias_type("InGroup"), BiasType::InGroup);
        assert_eq!(parse_bias_type("Favoritism"), BiasType::Favoritism);
    }

    #[test]
    fn parse_bias_type_unknown() {
        assert_eq!(parse_bias_type("bogus"), BiasType::Confirmation);
    }

    #[test]
    fn parse_trigger_all_variants() {
        assert_eq!(parse_trigger("Stress"), BehaviorTrigger::Stress);
        assert_eq!(parse_trigger("Conflict"), BehaviorTrigger::Conflict);
        assert_eq!(parse_trigger("Success"), BehaviorTrigger::Success);
        assert_eq!(parse_trigger("Uncertainty"), BehaviorTrigger::Uncertainty);
        assert_eq!(parse_trigger("Recognition"), BehaviorTrigger::Recognition);
        assert_eq!(parse_trigger("Threatened"), BehaviorTrigger::Threatened);
        assert_eq!(parse_trigger("Change"), BehaviorTrigger::Change);
        assert_eq!(parse_trigger("Feedback"), BehaviorTrigger::Feedback);
        assert_eq!(parse_trigger("Injustice"), BehaviorTrigger::Injustice);
    }

    #[test]
    fn parse_trigger_unknown() {
        assert_eq!(parse_trigger("bogus"), BehaviorTrigger::Stress);
    }

    #[test]
    fn parse_response_valid() {
        assert!(parse_response("remains_calm").is_some());
        assert!(parse_response("facilitates_resolution").is_some());
        assert!(parse_response("seeks_support").is_some());
    }

    #[test]
    fn parse_response_invalid() {
        assert!(parse_response("bogus").is_none());
    }

    #[test]
    fn parse_style_category_all_variants() {
        use peoplemodeler_core::models::StyleCategory;
        assert_eq!(
            parse_style_category("Communication"),
            StyleCategory::Communication
        );
        assert_eq!(
            parse_style_category("ConflictResolution"),
            StyleCategory::ConflictResolution
        );
        assert_eq!(
            parse_style_category("DecisionMaking"),
            StyleCategory::DecisionMaking
        );
        assert_eq!(
            parse_style_category("Leadership"),
            StyleCategory::Leadership
        );
        assert_eq!(
            parse_style_category("TimeOrientation"),
            StyleCategory::TimeOrientation
        );
        assert_eq!(
            parse_style_category("MoralFramework"),
            StyleCategory::MoralFramework
        );
        assert_eq!(
            parse_style_category("InterpersonalConduct"),
            StyleCategory::InterpersonalConduct
        );
        assert_eq!(
            parse_style_category("TrustStyle"),
            StyleCategory::TrustStyle
        );
    }

    #[test]
    fn parse_style_category_unknown() {
        use peoplemodeler_core::models::StyleCategory;
        assert_eq!(parse_style_category("bogus"), StyleCategory::Communication);
    }

    #[test]
    fn parse_style_type_valid() {
        let st = parse_style_type("DirectCommunicator");
        assert_eq!(st, StyleType::DirectCommunicator);
    }

    #[test]
    fn parse_style_type_invalid() {
        let st = parse_style_type("bogus");
        assert_eq!(st, StyleType::DirectCommunicator);
    }

    #[test]
    fn swap_item_in_list_up() {
        let mut v = vec!["a", "b", "c"];
        swap_item_in_list(&mut v, 1, true);
        assert_eq!(v, vec!["b", "a", "c"]);
    }

    #[test]
    fn swap_item_in_list_down() {
        let mut v = vec!["a", "b", "c"];
        swap_item_in_list(&mut v, 1, false);
        assert_eq!(v, vec!["a", "c", "b"]);
    }

    #[test]
    fn swap_item_in_list_first_up_noop() {
        let mut v = vec!["a", "b", "c"];
        swap_item_in_list(&mut v, 0, true);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn swap_item_in_list_last_down_noop() {
        let mut v = vec!["a", "b", "c"];
        swap_item_in_list(&mut v, 2, false);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn swap_item_in_list_single_element() {
        let mut v = vec!["a"];
        swap_item_in_list(&mut v, 0, true);
        assert_eq!(v, vec!["a"]);
        swap_item_in_list(&mut v, 0, false);
        assert_eq!(v, vec!["a"]);
    }

    #[test]
    fn swap_item_in_list_two_elements_up() {
        let mut v = vec!["a", "b"];
        swap_item_in_list(&mut v, 1, true);
        assert_eq!(v, vec!["b", "a"]);
    }

    #[test]
    fn swap_item_in_list_two_elements_down() {
        let mut v = vec!["a", "b"];
        swap_item_in_list(&mut v, 0, false);
        assert_eq!(v, vec!["b", "a"]);
    }

    #[test]
    fn swap_item_in_list_integers() {
        let mut v = vec![1, 2, 3, 4];
        swap_item_in_list(&mut v, 0, true);
        assert_eq!(v, vec![1, 2, 3, 4]);
        swap_item_in_list(&mut v, 2, true);
        assert_eq!(v, vec![1, 3, 2, 4]);
        swap_item_in_list(&mut v, 2, false);
        assert_eq!(v, vec![1, 3, 4, 2]);
    }

    #[test]
    fn mot_helper_returns_known_string() {
        let lang = Lang::En;
        assert_ne!(mot_helper(&MotivationType::Achievement, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Power, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Affiliation, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Security, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Autonomy, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Recognition, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Learning, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Helping, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Creativity, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Fairness, lang), "xyzzy");
    }

    #[test]
    fn bias_helper_returns_known_string() {
        let lang = Lang::En;
        assert_ne!(bias_helper(&BiasType::Confirmation, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Anchoring, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Availability, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::SunkCost, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::DunningKruger, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Impostor, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::LossAversion, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::SocialProof, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Authority, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Recency, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::InGroup, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Favoritism, lang), "xyzzy");
    }

    #[test]
    fn style_helper_returns_known_string() {
        let lang = Lang::En;
        for t in StyleType::ALL {
            let h = style_helper(&t, lang);
            assert_ne!(h, "xyzzy", "style_helper returned xyzzy for {t:?}");
        }
    }

    #[test]
    fn pattern_helper_returns_known_string() {
        let lang = Lang::En;
        assert_ne!(pattern_helper(&BehaviorTrigger::Stress, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Conflict, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Success, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Uncertainty, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Recognition, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Threatened, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Change, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Feedback, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Injustice, lang), "xyzzy");
    }

    #[test]
    fn mot_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = MotivationType::ALL
            .iter()
            .map(|t| mot_helper(t, lang))
            .collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), MotivationType::ALL.len());
    }

    #[test]
    fn bias_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = BiasType::ALL.iter().map(|t| bias_helper(t, lang)).collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), BiasType::ALL.len());
    }

    #[test]
    fn style_helper_exact_en_value() {
        let desc = style_helper(&StyleType::DirectCommunicator, Lang::En);
        assert_eq!(desc, "Speaks frankly, gets straight to the point");
    }

    #[test]
    fn style_helper_exact_fr_value() {
        let desc = style_helper(&StyleType::DirectCommunicator, Lang::Fr);
        assert_eq!(desc, "Parle franchement et va droit au but");
    }

    #[test]
    fn style_helper_en_differs_from_fr() {
        let en = style_helper(&StyleType::DirectCommunicator, Lang::En);
        let fr = style_helper(&StyleType::DirectCommunicator, Lang::Fr);
        assert_ne!(en, fr);
    }

    #[test]
    fn style_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = StyleType::ALL
            .iter()
            .map(|t| style_helper(t, lang))
            .collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), StyleType::ALL.len());
    }

    #[test]
    fn pattern_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = BehaviorTrigger::ALL
            .iter()
            .map(|t| pattern_helper(t, lang))
            .collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), BehaviorTrigger::ALL.len());
    }

    #[test]
    fn core_lang_maps_both_branches() {
        assert!(matches!(
            core_lang(Lang::Fr),
            peoplemodeler_core::i18n::Lang::Fr
        ));
        assert!(matches!(
            core_lang(Lang::En),
            peoplemodeler_core::i18n::Lang::En
        ));
    }

    #[test]
    fn value_helper_bilingual_not_empty() {
        for &v in &ValueType::ALL {
            let en = value_helper(&v, Lang::En);
            let fr = value_helper(&v, Lang::Fr);
            assert!(!en.is_empty(), "{v:?} en empty");
            assert!(!fr.is_empty(), "{v:?} fr empty");
            assert_ne!(en, "xyzzy");
            assert_ne!(fr, "xyzzy");
            assert_ne!(en, fr, "{v:?} langs identical");
        }
    }

    #[test]
    fn coerce_style_to_category_keeps_valid_choice() {
        for cat in StyleCategory::ALL {
            let opts = StyleType::options_for(cat);
            let valid = opts[1];
            assert_eq!(coerce_style_to_category(cat, valid), valid);
        }
    }

    #[test]
    fn coerce_style_to_category_falls_back_to_first() {
        let cat = peoplemodeler_core::models::StyleCategory::TrustStyle;
        let invalid = StyleType::DirectCommunicator;
        assert_eq!(
            coerce_style_to_category(cat, invalid),
            StyleType::options_for(cat)[0]
        );
    }
}

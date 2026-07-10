use dioxus::prelude::*;
use peoplemodeler_core::models::Relationship;

use crate::db;
use crate::i18n::Lang;
use crate::Route;

#[component]
pub fn Relationships() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut rels = use_signal(|| db::all_relationships());
    let persons = use_signal(|| db::all_persons());

    let title = crate::i18n::tr("rel_title", lang());
    let add_label = crate::i18n::tr("rel_add", lang());
    let none = crate::i18n::tr("rel_none", lang());
    let type_pl = crate::i18n::tr("rel_type_placeholder", lang());
    let notes_pl = crate::i18n::tr("rel_notes", lang());
    let add_btn = crate::i18n::tr("common_add", lang());
    let del_btn = crate::i18n::tr("common_delete", lang());
    let rel_from = crate::i18n::tr("rel_from", lang());

    let mut adding = use_signal(|| false);
    let mut r#type = use_signal(String::new);
    let mut notes = use_signal(String::new);
    let mut checked = use_signal(|| std::collections::HashSet::<String>::new());
    let mut source_id = use_signal(String::new);

    let persons_list = persons();

    let mut add_rel = move || {
        let src = source_id();
        let t = r#type();
        if src.is_empty() || t.is_empty() {
            return;
        }
        for cid in checked().iter() {
            if *cid == src {
                continue;
            }
            let rel = Relationship {
                id: uuid::Uuid::new_v4().to_string(),
                source_id: src.clone(),
                target_id: cid.clone(),
                r#type: t.clone(),
                notes: notes(),
                created_at: chrono::Utc::now().timestamp_millis(),
            };
            db::save_relationship(&rel);
        }
        rels.set(db::all_relationships());
        r#type.set(String::new());
        notes.set(String::new());
        checked.set(std::collections::HashSet::new());
        source_id.set(String::new());
        adding.set(false);
    };

    let person_name = |id: &str| -> String {
        persons_list
            .iter()
            .find(|p| p.id == id)
            .map(|p| format!("{} {}", p.avatar_emoji, p.name))
            .unwrap_or_else(|| id.to_string())
    };

    rsx! {
        div { class: "page",
            h2 { "{title}" }

            button { class: "btn", aria_label: "{add_label}", onclick: move |_| adding.set(!adding()), "{add_label}" }

            if adding() {
                div { class: "section rel-form",
                    p { class: "rel-hint", "{rel_from}" }
                    div { class: "rel-person-list",
                        for p in &persons_list {
                            div { class: "rel-person-row",
                                input {
                                    r#type: "radio",
                                    name: "rel-source",
                                    value: "{p.id}",
                                    checked: source_id() == p.id,
                                    onchange: {
                                        let pid = p.id.clone();
                                        move |_| source_id.set(pid.clone())
                                    },
                                }
                                input {
                                    r#type: "checkbox",
                                    checked: checked().contains(&p.id),
                                    onchange: {
                                        let pid = p.id.clone();
                                        move |_| {
                                            let mut c = checked();
                                            if c.contains(&pid) { c.remove(&pid); }
                                            else { c.insert(pid.clone()); }
                                            checked.set(c);
                                        }
                                    },
                                }
                                span { class: "rel-avatar", "{p.avatar_emoji}" }
                                span { "{p.name}" }
                            }
                        }
                    }
                    div { class: "form-row",
                        input {
                            placeholder: "{type_pl}",
                            value: "{r#type}",
                            oninput: move |e| r#type.set(e.value()),
                        }
                        input {
                            placeholder: "{notes_pl}",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                        button { class: "btn btn-primary", onclick: move |_| add_rel(), "{add_btn}" }
                    }
                }
            }

            if rels().is_empty() {
                p { "{none}" }
            } else {
                div { class: "relationship-list",
                    for rel in rels() {
                        div { class: "relationship-card",
                            div { class: "relationship-card-row",
                                Link { to: Route::PersonDetail { id: rel.source_id.clone() }, class: "person-link", "{person_name(&rel.source_id)}" }
                                span { class: "arrow", "→" }
                                Link { to: Route::PersonDetail { id: rel.target_id.clone() }, class: "person-link", "{person_name(&rel.target_id)}" }
                                span { class: "tag", "{rel.r#type}" }
                            }
                            if !rel.notes.is_empty() {
                                p { class: "note", "{rel.notes}" }
                            }
                            div { class: "card-actions",
                                button {
                                    class: "btn btn-small btn-danger",
                                    onclick: {
                                        let rid = rel.id.clone();
                                        move |_| {
                                            db::delete_relationship(&rid);
                                            rels.set(db::all_relationships());
                                        }
                                    },
                                    "{del_btn}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

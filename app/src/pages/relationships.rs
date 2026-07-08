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
    let rel_from = crate::i18n::tr("rel_from", lang());
    let rel_to = crate::i18n::tr("rel_to", lang());
    let type_pl = crate::i18n::tr("rel_type_placeholder", lang());
    let notes_pl = crate::i18n::tr("rel_notes", lang());
    let add_btn = crate::i18n::tr("common_add", lang());
    let del_btn = crate::i18n::tr("common_delete", lang());

    let mut source_id = use_signal(String::new);
    let mut target_id = use_signal(String::new);
    let mut r#type = use_signal(String::new);
    let mut notes = use_signal(String::new);
    let mut adding = use_signal(|| false);

    let persons_list = persons();

    let mut add_rel = move || {
        let sid = source_id();
        let tid = target_id();
        let t = r#type();
        if sid.is_empty() || tid.is_empty() || sid == tid || t.is_empty() {
            return;
        }
        let rel = Relationship {
            id: uuid::Uuid::new_v4().to_string(),
            source_id: sid,
            target_id: tid,
            r#type: t,
            notes: notes(),
            created_at: chrono::Utc::now().timestamp_millis(),
        };
        db::save_relationship(&rel);
        rels.set(db::all_relationships());
        source_id.set(String::new());
        target_id.set(String::new());
        r#type.set(String::new());
        notes.set(String::new());
        adding.set(false);
    };

    let person_name = |id: &str| -> String {
        persons_list.iter().find(|p| p.id == id).map(|p| format!("{} {}", p.avatar_emoji, p.name)).unwrap_or_else(|| id.to_string())
    };

    rsx! {
        div { class: "page",
            h2 { "{title}" }

            button { class: "btn", aria_label: "{add_label}", onclick: move |_| adding.set(!adding()), "{add_label}" }

            if adding() {
                div { class: "section",
                    div { class: "form-row",
                        select {
                            value: "{source_id}",
                            onchange: move |e| source_id.set(e.value()),
                            option { value: "", "— {rel_from} —" }
                            for p in &persons_list {
                                option { value: "{p.id}", "{p.avatar_emoji} {p.name}" }
                            }
                        }
                        select {
                            value: "{target_id}",
                            onchange: move |e| target_id.set(e.value()),
                            option { value: "", "— {rel_to} —" }
                            for p in &persons_list {
                                option { value: "{p.id}", "{p.avatar_emoji} {p.name}" }
                            }
                        }
                        input {
                            placeholder: "{type_pl}",
                            value: "{r#type}",
                            oninput: move |e| r#type.set(e.value()),
                        }
                    }
                    div { class: "form-row",
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
                                Link {
                                    to: Route::PersonDetail { id: rel.source_id.clone() },
                                    class: "person-link",
                                    "{person_name(&rel.source_id)}"
                                }
                                span { class: "arrow", "→" }
                                Link {
                                    to: Route::PersonDetail { id: rel.target_id.clone() },
                                    class: "person-link",
                                    "{person_name(&rel.target_id)}"
                                }
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

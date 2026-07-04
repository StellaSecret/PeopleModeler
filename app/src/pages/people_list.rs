use dioxus::prelude::*;
use peoplemodeler_core::models::Person;

use crate::db;
use crate::i18n::Lang;
use crate::Route;

#[component]
pub fn PeopleList() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let persons = use_signal(|| db::all_persons());
    let mut search = use_signal(String::new);

    let filtered = use_memo(move || {
        let q = search().to_lowercase();
        if q.is_empty() {
            persons()
        } else {
            persons()
                .into_iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&q)
                        || p.role.to_lowercase().contains(&q)
                        || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
                })
                .collect::<Vec<_>>()
        }
    });

    let has_items = !filtered().is_empty();

    let search_placeholder = crate::i18n::tr("search_placeholder", lang());
    let no_people = crate::i18n::tr("no_people_yet", lang());
    rsx! {
        div { class: "page",
            div { class: "toolbar",
                input {
                    class: "search-input",
                    placeholder: "{search_placeholder}",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }
            }
            if has_items {
                div { class: "person-list",
                    for person in filtered() {
                        PersonCard { person }
                    }
                }
            } else {
                div { class: "empty-state",
                    span { class: "empty-icon", "🧩" }
                    p { "{no_people}" }
                }
            }
            Link { to: Route::PersonNew {}, class: "fab", "＋" }
        }
    }
}

#[component]
fn PersonCard(person: Person) -> Element {
    let o = &person.ocean;
    rsx! {
        Link {
            to: Route::PersonDetail { id: person.id.clone() },
            class: "person-card",
            div { class: "card-header",
                span { class: "avatar", "{person.avatar_emoji}" }
                div { class: "card-info",
                    strong { "{person.name}" }
                    small { "{person.role}" }
                }
            }
            div { class: "ocean-mini",
                span { "O:{o.openness}" }
                span { "C:{o.conscientiousness}" }
                span { "E:{o.extraversion}" }
                span { "A:{o.agreeableness}" }
                span { "N:{o.neuroticism}" }
            }
            if !person.tags.is_empty() {
                div { class: "tags",
                    for tag in &person.tags {
                        span { class: "tag", "{tag}" }
                    }
                }
            }
        }
    }
}

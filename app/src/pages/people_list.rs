use dioxus::prelude::*;
use peoplemodeler_core::models::Person;

use crate::db;
use crate::i18n::Lang;
use crate::Route;

#[derive(Clone, Copy, Debug, PartialEq)]
enum SortBy { Name, Recent, Ocean }

#[component]
pub fn PeopleList() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let persons = use_signal(|| db::all_persons());
    let mut search = use_signal(String::new);
    let mut sort = use_signal(|| SortBy::Recent);

    let filtered = use_memo(move || {
        let q = search().to_lowercase();
        let mut items = if q.is_empty() {
            persons()
        } else {
            persons().into_iter().filter(|p| {
                p.name.to_lowercase().contains(&q)
                    || p.role.to_lowercase().contains(&q)
                    || p.context.to_lowercase().contains(&q)
                    || p.notes.to_lowercase().contains(&q)
                    || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
            }).collect::<Vec<_>>()
        };
        match sort() {
            SortBy::Name => items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortBy::Recent => items.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            SortBy::Ocean => items.sort_by(|a, b| {
                let avg = |p: &Person| (p.ocean.openness + p.ocean.conscientiousness + p.ocean.extraversion + p.ocean.agreeableness + p.ocean.neuroticism) as f64 / 5.0;
                avg(b).partial_cmp(&avg(a)).unwrap_or(std::cmp::Ordering::Equal)
            }),
        }
        items
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
                select {
                    class: "sort-select",
                    value: "{sort():?}",
                    onchange: move |e| {
                        sort.set(match e.value().as_str() {
                            "Name" => SortBy::Name,
                            "Ocean" => SortBy::Ocean,
                            _ => SortBy::Recent,
                        });
                    },
                    option { value: "Recent", {crate::i18n::tr("sort_recent", lang())} }
                    option { value: "Name", {crate::i18n::tr("sort_name", lang())} }
                    option { value: "Ocean", {crate::i18n::tr("sort_ocean", lang())} }
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

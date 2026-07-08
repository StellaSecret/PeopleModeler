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
    let tag_filter = use_context::<Signal<Option<String>>>();
    let mut tag_filter_w = tag_filter.clone();
    let tag_clear = crate::i18n::tr("tag_clear", lang());
    let filter_label = use_memo(move || {
        tag_filter().as_ref().map_or(String::new(), |tag| crate::i18n::tr_fmt("tag_filter", lang(), &[("tag", tag)]))
    });

    let filtered = use_memo(move || {
        let q = search().to_lowercase();
        let mut items: Vec<Person> = persons();
        if let Some(ref tag) = tag_filter() {
            items.retain(|p| p.tags.iter().any(|t| t == tag));
        }
        if !q.is_empty() {
            items.retain(|p| {
                p.name.to_lowercase().contains(&q)
                    || p.role.to_lowercase().contains(&q)
                    || p.context.to_lowercase().contains(&q)
                    || p.notes.to_lowercase().contains(&q)
                    || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
            });
        }
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
                    aria_label: "Search people",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }
                select {
                    class: "sort-select",
                    aria_label: "Sort by",
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
            if let Some(ref _tag) = tag_filter() {
                div { class: "tag-filter-banner",
                    span { "{filter_label()}" }
                    button { class: "btn btn-small", onclick: move |_| tag_filter_w.set(None), "{tag_clear}" }
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
            Link { to: Route::PersonNew {}, class: "fab", aria_label: "Add new person", "＋" }
        }
    }
}

#[component]
fn PersonCard(person: Person) -> Element {
    let o = &person.ocean;
    let tag_filter = use_context::<Signal<Option<String>>>();
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
                        span {
                            class: "tag tag-clickable",
                            onclick: {
                                let t = tag.clone();
                                let mut tf = tag_filter.clone();
                                move |_| tf.set(Some(t.clone()))
                            },
                            "{tag}"
                        }
                    }
                }
            }
        }
    }
}

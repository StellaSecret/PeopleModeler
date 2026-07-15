use dioxus::prelude::*;

use crate::Route;
use crate::db;
use crate::i18n::Lang;
use crate::pages::predictions::format_date;

#[component]
pub fn Timeline() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let persons = use_signal(db::all_persons);
    let title = crate::i18n::tr("tl_title", lang());
    let empty = crate::i18n::tr("tl_empty", lang());

    let mut entries: Vec<_> = persons()
        .into_iter()
        .flat_map(|p| {
            let pid = p.id.clone();
            let name = p.name.clone();
            let emoji = p.avatar_emoji.clone();
            p.log
                .into_iter()
                .map(move |e| (pid.clone(), name.clone(), emoji.clone(), e))
        })
        .collect();
    entries.sort_by_key(|b| std::cmp::Reverse(b.3.timestamp));

    if entries.is_empty() {
        return rsx! {
            div { class: "page",
                h2 { "{title}" }
                p { "{empty}" }
            }
        };
    }

    rsx! {
        div { class: "page",
            h2 { "{title}" }
            div { class: "timeline-list",
                for (pid, name, emoji, entry) in entries {
                    div { class: "timeline-entry",
                        div { class: "timeline-header",
                            Link {
                                to: Route::PersonDetail { id: pid },
                                "{emoji} {name}"
                            }
                            span { class: "date", "{format_date(entry.timestamp)}" }
                        }
                        p { "{entry.text}" }
                    }
                }
            }
        }
    }
}

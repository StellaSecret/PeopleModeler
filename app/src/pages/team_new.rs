use dioxus::prelude::*;
use peoplemodeler_core::models::AVATAR_EMOJIS;

use crate::Route;
use crate::db;
use crate::i18n::Lang;

#[component]
pub fn TeamNew() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let form_title = crate::i18n::tr("teams_create", lang());
    let form_save = crate::i18n::tr("common_save", lang());
    let form_cancel = crate::i18n::tr("common_cancel", lang());
    let icon_label = crate::i18n::tr("team_icon", lang());

    let mut icon = use_signal(|| "🎯".to_string());

    rsx! {
        div { class: "page",
            h2 { "{form_title}" }
            div { class: "form",
                label { "Name" }
                input {
                    id: "team-name-input",
                    aria_label: "Team name",
                    oninput: move |_| {},
                    placeholder: "Team name",
                }
                label { "{icon_label}" }
                div { class: "emoji-picker", role: "radiogroup", aria_label: "{icon_label}",
                    for e in AVATAR_EMOJIS {
                        button {
                            class: "emoji-btn",
                            class: if icon() == *e { "selected" },
                            role: "radio",
                            aria_label: "Icon {e}",
                            aria_checked: if icon() == *e { "true" } else { "false" },
                            onclick: move |_| icon.set(e.to_string()),
                            "{e}"
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn btn-primary",
                        aria_label: "{form_save}",
                        onclick: move |_| {
                            let name = read_input_value().trim().to_string();
                            if name.is_empty() { return; }
                            let team = peoplemodeler_core::models::Team {
                                id: uuid(),
                                name,
                                icon: icon(),
                                member_ids: vec![],
                                created_at: now_ts(),
                            };
                            let _ = db::save_team(&team);
                            dioxus::prelude::navigator().push(Route::TeamsList {});
                        },
                        "{form_save}"
                    }
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| {
                            dioxus::prelude::navigator().push(Route::TeamsList {});
                        },
                        "{form_cancel}"
                    }
                }
            }
        }
    }
}

fn read_input_value() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("team-name-input"))
            .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|input| input.value())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        String::new()
    }
}

#[cfg(target_arch = "wasm32")]
fn uuid() -> String {
    let ts = js_sys::Date::now() as u64;
    format!("team-{ts}")
}

#[cfg(target_arch = "wasm32")]
fn now_ts() -> i64 {
    js_sys::Date::now() as i64 / 1000
}

#[cfg(not(target_arch = "wasm32"))]
fn uuid() -> String {
    String::from("team-placeholder")
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ts() -> i64 {
    0
}

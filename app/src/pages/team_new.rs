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

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn read_input_value_nonwasm_returns_empty() {
        assert_eq!(read_input_value(), String::new());
    }

    #[test]
    fn uuid_nonwasm_returns_placeholder() {
        assert_eq!(uuid(), "team-placeholder");
    }

    #[test]
    fn now_ts_nonwasm_returns_zero() {
        assert_eq!(now_ts(), 0);
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn uuid_wasm_starts_with_team() {
        let id = super::uuid();
        assert!(
            id.starts_with("team-"),
            "uuid should start with 'team-', got '{id}'"
        );
    }

    #[wasm_bindgen_test]
    fn now_ts_wasm_positive() {
        let ts = super::now_ts();
        assert!(
            (1_000_000_000..2_000_000_000).contains(&ts),
            "now_ts should be a recent epoch second, got {ts}"
        );
    }

    #[wasm_bindgen_test]
    fn read_input_value_wasm_reads_dom() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let input = doc.create_element("input").unwrap();
        input.set_attribute("id", "team-name-input").unwrap();
        input.set_attribute("value", "Test Team").unwrap();
        doc.body().unwrap().append_child(&input).unwrap();
        assert_eq!(super::read_input_value(), "Test Team");
        let _ = doc.body().unwrap().remove_child(&input);
    }
}

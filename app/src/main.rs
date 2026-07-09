use dioxus::prelude::*;
use dioxus_router::Outlet;

use crate::i18n::Lang;
use crate::theme::Theme;
use crate::pages::insights::Insights;
use crate::pages::people_list::PeopleList;
use crate::pages::person_detail::PersonDetail;
use crate::pages::person_edit::PersonEdit;
use crate::pages::person_edit::PersonNew;
use crate::pages::predictions::Predictions;
use crate::pages::compare::ComparePersons;
use crate::pages::sync::SyncPage;
use crate::pages::relationships::Relationships;
use crate::pages::timeline::Timeline;

mod auth;
#[cfg(target_os = "android")]
mod android_auth;
#[cfg(target_os = "android")]
mod android_share;
#[cfg(target_arch = "wasm32")]
mod crypto;
mod db;
mod drive;
mod i18n;
mod templates;
mod theme;
mod pages;
mod toast;
mod undo;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(NavLayout)]
    #[route("/")]
    PeopleList {},
    #[route("/person/:id")]
    PersonDetail { id: String },
    #[route("/person/new")]
    PersonNew {},
    #[route("/person/:id/edit")]
    PersonEdit { id: String },
    #[route("/predictions")]
    Predictions {},
    #[route("/insights")]
    Insights {},
    #[route("/sync")]
    SyncPage {},
    #[route("/compare/:id1/:id2")]
    ComparePersons { id1: String, id2: String },
    #[route("/relationships")]
    Relationships {},
    #[route("/timeline")]
    Timeline {},
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        let _guard = _rt.enter();
        #[cfg(target_os = "android")]
        {
            let data_dir = std::path::PathBuf::from("/data/data/com.stellasecret.peoplemodeler/files");
            let _ = std::env::set_current_dir(&data_dir);
        }
        db::init();
        dioxus::launch(App);
    }
    #[cfg(target_arch = "wasm32")]
    {
        db::init();
        dioxus::launch(App);
    }
}

#[allow(non_snake_case)]
fn App() -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        auth::init();
        init_pwa();
    }

    let lang = use_signal(|| Lang::detect());
    let theme = use_signal(|| Theme::detect());
    let tag_filter: Signal<Option<String>> = use_signal(|| None);
    use_context_provider(|| lang);
    use_context_provider(|| theme);
    use_context_provider(|| tag_filter);
    let _toast = toast::provide_toast();
    // Ctrl+Z undo handler (one-time setup)
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;
        use_hook(move || {
            let mut toast = _toast.clone();
            let cb: Closure<dyn FnMut(web_sys::KeyboardEvent)> = Closure::new(move |e: web_sys::KeyboardEvent| {
                if e.ctrl_key() && e.key() == "z" {
                    if crate::undo::undo() {
                        toast.set(Some("↩ Undo".into()));
                    }
                }
            });
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ = doc.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            }
            cb.forget();
        });
    }
    // Persist theme + sync to html element (web needs data-theme on <html> for body bg etc)
    use_effect(move || {
        let t = theme();
        t.persist();
        #[cfg(target_arch = "wasm32")]
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.document_element().map(|el| {
                let _ = el.set_attribute("data-theme", t.as_str());
            });
        }
    });
    rsx! {
        style { {include_str!("../assets/styles.css")} }
        Router::<Route> {}
        div { class: "noise" }
    }
}

#[cfg(target_arch = "wasm32")]
fn init_pwa() {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    let head = match doc.head() {
        Some(h) => h,
        None => return,
    };
    if let Ok(link) = doc.create_element("link") {
        let _ = link.set_attribute("rel", "manifest");
        let _ = link.set_attribute("href", "/PeopleModeler/manifest.json");
        let _ = head.append_child(&link);
    }
    if let Some(w) = web_sys::window() {
        let _ = w.navigator().service_worker().register("/PeopleModeler/sw.js");
    }
}

#[component]
fn NavLayout() -> Element {
    let mut lang = use_context::<Signal<Lang>>();
    let mut theme = use_context::<Signal<Theme>>();
    let toast = use_context::<Signal<Option<String>>>();
    let nav_people = crate::i18n::tr("nav_people", lang());
    let nav_relationships = crate::i18n::tr("nav_relationships", lang());
    let nav_timeline = crate::i18n::tr("nav_timeline", lang());
    let nav_sync = crate::i18n::tr("nav_sync", lang());
    let toggle_lang = move |_| {
        let mut l = lang();
        l = match l {
            Lang::Fr => Lang::En,
            Lang::En => Lang::Fr,
        };
        l.persist();
        lang.set(l);
    };
    let toggle_theme = move |_| {
        let t = theme();
        theme.set(t.toggle());
    };
    auto_clear_toast(toast);
    let can_undo = crate::undo::can_undo();
    rsx! {
        div { class: "app", "data-theme": theme().as_str(),
            header { class: "top-bar",
                Link { to: Route::PeopleList {}, class: "logo",
                    "People"
                    span { "Modeler" }
                }
                div { class: "nav-links",
                    Link { to: Route::PeopleList {}, "{nav_people}" }
                    Link { to: Route::Relationships {}, "{nav_relationships}" }
                    Link { to: Route::Timeline {}, "{nav_timeline}" }
                    Link { to: Route::SyncPage {}, "{nav_sync}" }
                }
                div { class: "toggle-group",
                    if can_undo {
                        button { class: "undo-btn", aria_label: "Undo (Ctrl+Z)", onclick: move |_| {
                            let mut t = toast.clone();
                            if crate::undo::undo() {
                                t.set(Some("↩ Undo".into()));
                            }
                        }, "↩" }
                    }
                    button { class: "theme-toggle", aria_label: "Toggle theme", onclick: toggle_theme,
                        { theme().label() }
                    }
                    button { class: "lang-toggle", aria_label: "Toggle language", onclick: toggle_lang,
                        if lang() == Lang::Fr { "EN" } else { "FR" }
                    }
                }
            }
            main { class: "content",
                Outlet::<Route> {}
            }
            div { class: "toast-container",
                if let Some(msg) = toast() {
                    div { class: "toast", key: "{msg}", "{msg}" }
                }
            }
        }
    }
}

fn auto_clear_toast(toast: Signal<Option<String>>) {
    use_effect(move || {
        if toast().is_some() {
            let mut t = toast.clone();
            spawn(async move {
                sleep_ms(2000).await;
                t.set(None);
            });
        }
    });
}

#[cfg(target_arch = "wasm32")]
async fn sleep_ms(ms: u64) {
    let promise = js_sys::Promise::new(&mut move |resolve, _| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                ms as i32,
            );
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.ok();
}

#[cfg(not(target_arch = "wasm32"))]
async fn sleep_ms(ms: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
}

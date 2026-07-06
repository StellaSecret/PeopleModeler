use dioxus::prelude::*;
use dioxus::document::Stylesheet;
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

mod auth;
#[cfg(target_os = "android")]
mod android_auth;
mod db;
mod drive;
mod i18n;
mod templates;
mod theme;
mod pages;

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
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _rt = tokio::runtime::Runtime::new().unwrap();
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
    // Read OAuth token from URL fragment before any component reads get_token()
    #[cfg(target_arch = "wasm32")]
    auth::init();

    let lang = use_signal(|| Lang::detect());
    let theme = use_signal(|| Theme::detect());
    use_context_provider(|| lang);
    use_context_provider(|| theme);
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

#[component]
fn NavLayout() -> Element {
    let mut lang = use_context::<Signal<Lang>>();
    let mut theme = use_context::<Signal<Theme>>();
    let nav_people = crate::i18n::tr("nav_people", lang());
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
    rsx! {
        div { class: "app", "data-theme": theme().as_str(),
            header { class: "top-bar",
                Link { to: Route::PeopleList {}, class: "logo",
                    "People"
                    span { "Modeler" }
                }
                div { class: "nav-links",
                    Link { to: Route::PeopleList {}, "{nav_people}" }
                    Link { to: Route::SyncPage {}, "{nav_sync}" }
                }
                div { class: "toggle-group",
                    button { class: "theme-toggle", onclick: toggle_theme,
                        { theme().label() }
                    }
                    button { class: "lang-toggle", onclick: toggle_lang,
                        if lang() == Lang::Fr { "EN" } else { "FR" }
                    }
                }
            }
            main { class: "content",
                Outlet::<Route> {}
            }
        }
    }
}

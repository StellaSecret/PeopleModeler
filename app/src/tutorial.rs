use dioxus::prelude::*;

use crate::i18n::{tr, Lang};
use crate::Route;

#[derive(Clone, PartialEq)]
pub enum TutorialStatus {
    InProgress(usize),
    Done,
}

#[derive(Clone)]
struct StepDef {
    title_key: &'static str,
    body_key: &'static str,
    nav: Option<Route>,
}

const STEPS: &[StepDef] = &[
    StepDef {
        title_key: "tut_welcome_title",
        body_key: "tut_welcome_body",
        nav: None,
    },
    StepDef {
        title_key: "tut_people_title",
        body_key: "tut_people_body",
        nav: Some(Route::PeopleList {}),
    },
    StepDef {
        title_key: "tut_create_title",
        body_key: "tut_create_body",
        nav: Some(Route::PersonNew {}),
    },
    StepDef {
        title_key: "tut_ocean_title",
        body_key: "tut_ocean_body",
        nav: None,
    },
    StepDef {
        title_key: "tut_mot_bias_title",
        body_key: "tut_mot_bias_body",
        nav: None,
    },
    StepDef {
        title_key: "tut_rep_pattern_title",
        body_key: "tut_rep_pattern_body",
        nav: None,
    },
    StepDef {
        title_key: "tut_compare_title",
        body_key: "tut_compare_body",
        nav: None,
    },
    StepDef {
        title_key: "tut_done_title",
        body_key: "tut_done_body",
        nav: None,
    },
];

#[cfg(not(target_arch = "wasm32"))]
static TUTORIAL_ACTIVE: std::sync::Mutex<Option<usize>> = std::sync::Mutex::new(None);

pub fn is_done() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        let val: Option<String> = gloo_storage::LocalStorage::get("pm_tutorial_done").ok();
        val.as_deref() == Some("1")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let g = TUTORIAL_ACTIVE.lock().unwrap();
        g.is_none()
    }
}

fn mark_done() {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        let _ = gloo_storage::LocalStorage::set("pm_tutorial_done", "1");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut g = TUTORIAL_ACTIVE.lock().unwrap();
        *g = None;
    }
}

pub fn clear_mark() {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        gloo_storage::LocalStorage::delete("pm_tutorial_done");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut g = TUTORIAL_ACTIVE.lock().unwrap();
        *g = Some(0);
    }
}

#[component]
pub fn TutorialModal(
    status: Signal<TutorialStatus>,
) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut s = status;

    let TutorialStatus::InProgress(step_idx) = *s.read() else {
        return VNode::empty();
    };

    let Some(step) = STEPS.get(step_idx) else {
        return VNode::empty();
    };

    let total = STEPS.len();
    let title = tr(step.title_key, lang());
    let body = tr(step.body_key, lang());
    let is_last = step_idx + 1 >= total;
    let back_text = tr("common_back", lang());
    let skip_text = tr("common_skip", lang());
    let finish_text = tr("common_finish", lang());
    let next_text = tr("common_next", lang());

    let go_next = move |_| {
        if is_last {
            mark_done();
            s.set(TutorialStatus::Done);
        } else {
            let next = step_idx + 1;
            if let Some(nav) = STEPS.get(next).and_then(|st| st.nav.clone()) {
                navigator().push(nav);
            }
            s.set(TutorialStatus::InProgress(next));
        }
    };

    let go_back = move |_| {
        if step_idx == 0 {
            return;
        }
        let prev = step_idx - 1;
        if let Some(nav) = STEPS.get(prev).and_then(|st| st.nav.clone()) {
            navigator().push(nav);
        }
        s.set(TutorialStatus::InProgress(prev));
    };

    let mut skip = move |_| {
        mark_done();
        s.set(TutorialStatus::Done);
    };

    rsx! {
        div {
            class: "tut-overlay",
            role: "dialog",
            aria_label: "{title}",
            aria_modal: "true",
            tabindex: 0,
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    skip(());
                }
            },
            div { class: "tut-modal",
                div { class: "tut-header",
                    span { class: "tut-step-indicator",
                        "Step {step_idx + 1}/{total}"
                    }
                }
                h2 { class: "tut-title", "{title}" }
                p { class: "tut-body", "{body}" }
                div { class: "tut-actions",
                    if step_idx > 0 {
                        button { class: "btn", onclick: go_back, "{back_text}" }
                    }
                    button { class: "btn btn-ghost", onclick: move |_| { mark_done(); s.set(TutorialStatus::Done); }, "{skip_text}" }
                    if is_last {
                        button { class: "btn btn-primary", onclick: go_next, "{finish_text}" }
                    } else {
                        button { class: "btn btn-primary", onclick: go_next, "{next_text}" }
                    }
                }
            }
        }
    }
}

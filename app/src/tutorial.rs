use dioxus::prelude::*;

use crate::Route;
use crate::i18n::{Lang, tr};

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

fn is_last_step(step_idx: usize) -> bool {
    step_idx + 1 >= STEPS.len()
}

fn next_step(step_idx: usize) -> Option<usize> {
    let next = step_idx + 1;
    if next < STEPS.len() { Some(next) } else { None }
}

fn prev_step(step_idx: usize) -> Option<usize> {
    if step_idx == 0 {
        None
    } else {
        Some(step_idx - 1)
    }
}

#[cfg(not(target_arch = "wasm32"))]
static TUTORIAL_ACTIVE: std::sync::Mutex<Option<usize>> = std::sync::Mutex::new(None);

pub fn is_done() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        let val: Option<String> = gloo_storage::LocalStorage::get("pm_tutorial_done").ok();
        is_done_from_str(val.as_deref())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let g = TUTORIAL_ACTIVE.lock().unwrap();
        g.is_none()
    }
}

#[allow(dead_code)]
pub fn is_done_from_str(val: Option<&str>) -> bool {
    val == Some("1")
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
pub fn TutorialModal(status: Signal<TutorialStatus>) -> Element {
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
    let is_last = is_last_step(step_idx);
    let back_text = tr("common_back", lang());
    let skip_text = tr("common_skip", lang());
    let finish_text = tr("common_finish", lang());
    let next_text = tr("common_next", lang());
    let step_text = tr("tut_step", lang());

    let go_next = move |_| {
        if is_last {
            mark_done();
            s.set(TutorialStatus::Done);
        } else if let Some(next) = next_step(step_idx) {
            if let Some(nav) = STEPS.get(next).and_then(|st| st.nav.clone()) {
                navigator().push(nav);
            }
            s.set(TutorialStatus::InProgress(next));
        }
    };

    let go_back = move |_| {
        let Some(prev) = prev_step(step_idx) else {
            return;
        };
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
                        "{step_text} {step_idx + 1}/{total}"
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

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    static TUTORIAL_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn tutorial_mark_done_implies_is_done() {
        let _lock = TUTORIAL_TEST_LOCK.lock().unwrap();
        mark_done();
        assert!(is_done());
    }

    #[test]
    fn tutorial_clear_mark_implies_not_done() {
        let _lock = TUTORIAL_TEST_LOCK.lock().unwrap();
        mark_done();
        clear_mark();
        assert!(!is_done());
    }

    #[test]
    fn tutorial_is_done_false_after_clear() {
        let _lock = TUTORIAL_TEST_LOCK.lock().unwrap();
        clear_mark();
        assert!(!is_done());
    }

    #[test]
    fn is_done_from_str_one() {
        assert!(is_done_from_str(Some("1")));
    }

    #[test]
    fn is_done_from_str_none() {
        assert!(!is_done_from_str(None));
    }

    #[test]
    fn is_done_from_str_other_value() {
        assert!(!is_done_from_str(Some("0")));
        assert!(!is_done_from_str(Some("true")));
        assert!(!is_done_from_str(Some("")));
    }
}

#[cfg(test)]
mod step_tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn tutorial_is_last_step_boundary() {
        assert!(!is_last_step(0));
        assert!(!is_last_step(STEPS.len() - 2));
        assert!(is_last_step(STEPS.len() - 1));
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn tutorial_next_step_boundary() {
        assert_eq!(next_step(0), Some(1));
        assert_eq!(next_step(STEPS.len() - 2), Some(STEPS.len() - 1));
        assert_eq!(next_step(STEPS.len() - 1), None);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn tutorial_prev_step_boundary() {
        assert_eq!(prev_step(0), None);
        assert_eq!(prev_step(1), Some(0));
        assert_eq!(prev_step(STEPS.len() - 1), Some(STEPS.len() - 2));
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    fn clear_tutorial() {
        use gloo_storage::Storage;
        let _ = gloo_storage::LocalStorage::delete("pm_tutorial_done");
    }

    #[wasm_bindgen_test]
    fn is_done_wasm_false_initially() {
        clear_tutorial();
        assert!(!super::is_done());
    }

    #[wasm_bindgen_test]
    fn is_done_wasm_true_after_mark() {
        clear_tutorial();
        super::mark_done();
        assert!(super::is_done());
        clear_tutorial();
    }

    #[wasm_bindgen_test]
    fn clear_mark_wasm_resets() {
        super::mark_done();
        super::clear_mark();
        assert!(!super::is_done());
    }
}

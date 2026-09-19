use dioxus::prelude::*;
use peoplemodeler_core::models::FacetKind;

/// Three-state Base/Work/Online facet radiogroup shared by the edit form, the
/// people list and the person detail header. Previously each page hand-copied
/// the same `.facet-toggle` markup (a third copy in team.rs adds an
/// "Automatic" state over `Option<FacetKind>`, so it deliberately keeps its
/// own).
#[component]
pub fn FacetToggle(
    facet: Signal<FacetKind>,
    base_label: &'static str,
    work_label: &'static str,
    online_label: &'static str,
    #[props(default)] group_label: Option<String>,
) -> Element {
    rsx! {
        div { class: "facet-toggle", role: "radiogroup", aria_label: group_label,
            button {
                class: if facet() == FacetKind::Base { "facet-btn active" } else { "facet-btn" },
                role: "radio",
                aria_checked: if facet() == FacetKind::Base { "true" } else { "false" },
                onclick: move |_| facet.set(FacetKind::Base),
                "{base_label}"
            }
            button {
                class: if facet() == FacetKind::Work { "facet-btn active" } else { "facet-btn" },
                role: "radio",
                aria_checked: if facet() == FacetKind::Work { "true" } else { "false" },
                onclick: move |_| facet.set(FacetKind::Work),
                "{work_label}"
            }
            button {
                class: if facet() == FacetKind::Online { "facet-btn active" } else { "facet-btn" },
                role: "radio",
                aria_checked: if facet() == FacetKind::Online { "true" } else { "false" },
                onclick: move |_| facet.set(FacetKind::Online),
                "{online_label}"
            }
        }
    }
}

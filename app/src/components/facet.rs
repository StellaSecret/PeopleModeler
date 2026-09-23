use dioxus::prelude::*;
use peoplemodeler_core::models::FacetKind;

/// Three-state Base/Work/Online facet radiogroup shared by the edit form, the
/// people list and the person detail header. Previously each page hand-copied
/// the same `.facet-toggle` markup (a third copy in team.rs adds an
/// "Automatic" state over `Option<FacetKind>`, so it deliberately keeps its
/// own).
///
/// When `primary` is set, the primary context's button comes first and its
/// own label applies (edit/detail); without it (people list) the buttons keep
/// the fixed canonical Base/Work/Online order.
#[component]
pub fn FacetToggle(
    facet: Signal<FacetKind>,
    #[props(into)] base_label: String,
    #[props(into)] work_label: String,
    #[props(into)] online_label: String,
    #[props(default)] primary: Option<FacetKind>,
    #[props(default)] group_label: Option<String>,
) -> Element {
    let label = |kind: FacetKind| match kind {
        FacetKind::Base => base_label.clone(),
        FacetKind::Work => work_label.clone(),
        FacetKind::Online => online_label.clone(),
    };
    let order = match primary {
        Some(p) => {
            let rest: Vec<FacetKind> = FacetKind::ALL.into_iter().filter(|k| *k != p).collect();
            let mut all = vec![p];
            all.extend(rest);
            all
        }
        None => FacetKind::ALL.to_vec(),
    };
    rsx! {
        div { class: "facet-toggle", role: "radiogroup", aria_label: group_label,
            for kind in order {
                button {
                    class: if facet() == kind { "facet-btn active" } else { "facet-btn" },
                    role: "radio",
                    aria_checked: if facet() == kind { "true" } else { "false" },
                    onclick: move |_| facet.set(kind),
                    "{label(kind)}"
                }
            }
        }
    }
}

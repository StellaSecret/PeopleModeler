use dioxus::prelude::*;
use peoplemodeler_core::synergy::{compute_person_profile, synergy_bands};

use crate::Route;
use crate::db;
use crate::i18n::Lang;

#[component]
pub fn PeopleList() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let nav = use_navigator();
    let persons = use_signal(db::all_persons);
    let mut search = use_signal(String::new);

    let profiles = use_memo(move || {
        let all = persons();
        all.iter()
            .map(|p| {
                (
                    p.id.clone(),
                    p.name.clone(),
                    p.avatar_emoji.clone(),
                    compute_person_profile(p),
                )
            })
            .collect::<Vec<_>>()
    });

    let search_placeholder = crate::i18n::tr("search_placeholder", lang());
    let no_people = crate::i18n::tr("no_people_yet", lang());
    let name_hdr = crate::i18n::tr("pl_name", lang());
    let ps_hdr = crate::i18n::tr("person_self_score", lang());
    let ocean_hdr = crate::i18n::tr("compare_cat_ocean", lang());
    let rep_hdr = crate::i18n::tr("compare_cat_reputation", lang());
    let mot_hdr = crate::i18n::tr("compare_cat_motivation", lang());
    let pat_hdr = crate::i18n::tr("compare_cat_patterns", lang());
    let bias_hdr = crate::i18n::tr("compare_cat_bias", lang());

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
            }
            {
            let q = search().to_lowercase();
            let all_profiles = profiles();

            let mut rows: Vec<_> = all_profiles
                .into_iter()
                .filter(|(_, name, _, _)| q.is_empty() || name.to_lowercase().contains(&q))
                .collect();
            rows.sort_by_key(|(_, _, _, b)| std::cmp::Reverse(b.total));

            if rows.is_empty() {
                rsx! {
                    div { class: "empty-state",
                        span { class: "empty-icon", "🧩" }
                        p { "{no_people}" }
                    }
                }
            } else {
                let bands = synergy_bands();
                let band_cls = ["ps-tension", "ps-friction", "ps-moderate", "ps-good", "ps-strong"];

                rsx! {
                    table { class: "people-table",
                        thead {
                            tr {
                                th { "{name_hdr}" }
                                th { class: "pt-col-score", "{ps_hdr}" }
                                th { class: "pt-col-sub", "{ocean_hdr}" }
                                th { class: "pt-col-sub", "{rep_hdr}" }
                                th { class: "pt-col-sub", "{mot_hdr}" }
                                th { class: "pt-col-sub", "{pat_hdr}" }
                                th { class: "pt-col-sub", "{bias_hdr}" }
                            }
                        }
                        tbody {
                            for (pid, name, avatar, profile) in &rows {
                                {
                                let band_idx = bands.iter()
                                    .position(|&(lo, hi)| profile.total >= lo && profile.total <= hi)
                                    .unwrap_or(2);
                                let score_cls = band_cls[band_idx];
                                rsx! {
                                    tr {
                                        key: "{pid}",
                                        onclick: { let p = pid.clone(); move |_| { let _ = nav.push(Route::PersonDetail { id: p.clone() }); } },
                                        td { class: "pt-name-cell",
                                            span { class: "pt-avatar", "{avatar}" }
                                            span { "{name}" }
                                        }
                                        td { class: "pt-score {score_cls}", "{profile.total}" }
                                        td { class: "pt-sub", "{(profile.ocean * 100.0).round() as u8}" }
                                        td { class: "pt-sub", "{(profile.reputation * 100.0).round() as u8}" }
                                        td { class: "pt-sub", "{(profile.motivation * 100.0).round() as u8}" }
                                        td { class: "pt-sub", "{(profile.patterns * 100.0).round() as u8}" }
                                        td { class: "pt-sub", "{(profile.bias * 100.0).round() as u8}" }
                                    }
                                }
                                }
                            }
                        }
                    }
                }
            }
            }
            Link { to: Route::PersonNew {}, class: "fab", aria_label: "Add new person", "＋" }
        }
    }
}

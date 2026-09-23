use dioxus::prelude::*;
use peoplemodeler_core::models::{FacetKind, Person};
use peoplemodeler_core::synergy::{compute_person_profile, synergy_bands};

use crate::Route;
use crate::components::facet::FacetToggle;
use crate::db;
use crate::i18n::Lang;

fn list_profile_src(p: &Person, facet: FacetKind) -> Person {
    match facet {
        FacetKind::Work => p.facet_person(FacetKind::Work),
        FacetKind::Online => p.facet_person(FacetKind::Online),
        FacetKind::Base => p.clone(),
    }
}

#[component]
pub fn PeopleList() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let nav = use_navigator();
    let persons = use_signal(db::all_persons);
    let mut search = use_signal(String::new);
    let facet = use_signal(|| FacetKind::Base);

    let profiles = use_memo(move || {
        let all = persons();
        all.iter()
            .map(|p| {
                (
                    p.id.clone(),
                    p.name.clone(),
                    p.avatar_emoji.clone(),
                    compute_person_profile(&list_profile_src(p, facet())),
                )
            })
            .collect::<Vec<_>>()
    });

    let search_placeholder = crate::tr!("search_placeholder", lang());
    let no_people = crate::tr!("no_people_yet", lang());
    let no_search_results = crate::tr!("no_search_results", lang());
    let facet_base = crate::tr!("facet_main", lang());
    let facet_work = crate::tr!("facet_work", lang());
    let facet_online = crate::tr!("facet_online", lang());
    let name_hdr = crate::tr!("pl_name", lang());
    let ps_hdr = crate::tr!("person_self_score", lang());
    let ocean_hdr = crate::tr!("compare_cat_ocean", lang());
    let rep_hdr = crate::tr!("compare_cat_reputation", lang());
    let mot_hdr = crate::tr!("compare_cat_motivation", lang());
    let pat_hdr = crate::tr!("compare_cat_patterns", lang());
    let bias_hdr = crate::tr!("compare_cat_bias", lang());
    let comp_hdr = crate::tr!("profile_completeness", lang());

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
                FacetToggle { facet, base_label: facet_base, work_label: facet_work, online_label: facet_online, group_label: Some(format!("{facet_base} / {facet_work} / {facet_online}")) }
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
                let empty_msg = if q.is_empty() {
                    no_people.to_string()
                } else {
                    no_search_results.replace("{0}", &search())
                };
                rsx! {
                    div { class: "empty-state",
                        span { class: "empty-icon", "🧩" }
                        p { "{empty_msg}" }
                    }
                }
            } else {
                let bands = synergy_bands();
                let band_cls = ["ps-tension", "ps-friction", "ps-moderate", "ps-good", "ps-strong"];

                rsx! {
                    div { class: "table-wrap",
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
                                th { class: "pt-col-sub", "{comp_hdr}" }
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
                                        tabindex: "0",
                                        role: "button",
                                        onclick: { let p = pid.clone(); move |_| { let _ = nav.push(Route::PersonDetail { id: p.clone() }); } },
                                        onkeydown: { let p = pid.clone(); move |e| { if e.key() == Key::Enter { let _ = nav.push(Route::PersonDetail { id: p.clone() }); } } },
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
                                        td { class: "pt-sub", "{profile.completeness}%" }
                                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::{OceanScores, PersonaMask, RepScores};

    fn fixture_person() -> Person {
        Person {
            id: "i".into(),
            primary_facet: FacetKind::Base,
            name: "A".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "🧩".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    extraversion: Some(9),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            online_persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    extraversion: Some(7),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            ocean: OceanScores {
                extraversion: Some(2),
                ..OceanScores::default()
            },
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn list_profile_src_switches_to_merged_persona_on_work() {
        let p = fixture_person();
        let base_src = list_profile_src(&p, FacetKind::Base);
        assert!(base_src.persona.is_some(), "base facet keeps the persona");
        assert_eq!(base_src.ocean.extraversion, Some(2));
        let work_src = list_profile_src(&p, FacetKind::Work);
        assert!(
            work_src.persona.is_none(),
            "merged work facet drops the mask"
        );
        assert_eq!(work_src.ocean.extraversion, Some(9));
    }

    #[test]
    fn list_profile_src_switches_to_merged_persona_on_online() {
        let p = fixture_person();
        let online_src = list_profile_src(&p, FacetKind::Online);
        assert!(
            online_src.persona.is_none(),
            "merged online facet drops the mask"
        );
        assert_eq!(online_src.ocean.extraversion, Some(7));
        let base_src = list_profile_src(&p, FacetKind::Base);
        assert_eq!(base_src.ocean.extraversion, Some(2));
    }
}

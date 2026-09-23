use dioxus::prelude::*;
use peoplemodeler_core::models::{FacetKind, Person};
use peoplemodeler_core::synergy::{compute_person_profile, synergy_bands};

use crate::Route;
use crate::db;
use crate::i18n::Lang;

/// A ranking view: `None` anchors on each person's own primary context (always
/// defined), `Some(facet)` on that facet's materialized view. `None` when the
/// person does not define the ranked arena: they only exist in their primary
/// context, so their row is dropped instead of fabricating an anchor score.
fn list_profile_src(p: &Person, rank: Option<FacetKind>) -> Option<Person> {
    let kind = rank.unwrap_or(p.primary_facet);
    p.facet_person(kind)
}

#[component]
pub fn PeopleList() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let nav = use_navigator();
    let persons = use_signal(db::all_persons);
    let mut search = use_signal(String::new);
    let mut rank = use_signal(|| None::<FacetKind>);

    let profiles = use_memo(move || {
        let all = persons();
        all.iter()
            .filter_map(|p| {
                list_profile_src(p, rank()).map(|src| {
                    (
                        p.id.clone(),
                        p.name.clone(),
                        p.avatar_emoji.clone(),
                        compute_person_profile(&src),
                    )
                })
            })
            .collect::<Vec<_>>()
    });

    let search_placeholder = crate::tr!("search_placeholder", lang());
    let no_people = crate::tr!("no_people_yet", lang());
    let no_search_results = crate::tr!("no_search_results", lang());
    let facet_main = crate::tr!("facet_main", lang());
    let facet_base = crate::tr!("facet_base", lang());
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
                div { class: "facet-toggle", role: "radiogroup", aria_label: "{facet_main} / {facet_base} / {facet_work} / {facet_online}",
                    button {
                        class: if rank().is_none() { "facet-btn active" } else { "facet-btn" },
                        role: "radio",
                        aria_checked: if rank().is_none() { "true" } else { "false" },
                        onclick: move |_| rank.set(None),
                        "{facet_main}"
                    }
                    button {
                        class: if rank() == Some(FacetKind::Base) { "facet-btn active" } else { "facet-btn" },
                        role: "radio",
                        aria_checked: if rank() == Some(FacetKind::Base) { "true" } else { "false" },
                        onclick: move |_| rank.set(Some(FacetKind::Base)),
                        "{facet_base}"
                    }
                    button {
                        class: if rank() == Some(FacetKind::Work) { "facet-btn active" } else { "facet-btn" },
                        role: "radio",
                        aria_checked: if rank() == Some(FacetKind::Work) { "true" } else { "false" },
                        onclick: move |_| rank.set(Some(FacetKind::Work)),
                        "{facet_work}"
                    }
                    button {
                        class: if rank() == Some(FacetKind::Online) { "facet-btn active" } else { "facet-btn" },
                        role: "radio",
                        aria_checked: if rank() == Some(FacetKind::Online) { "true" } else { "false" },
                        onclick: move |_| rank.set(Some(FacetKind::Online)),
                        "{facet_online}"
                    }
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
            private_persona: None,
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
        let base_src = list_profile_src(&p, Some(FacetKind::Base)).unwrap();
        assert!(base_src.persona.is_some(), "base facet keeps the persona");
        assert_eq!(base_src.ocean.extraversion, Some(2));
        let work_src = list_profile_src(&p, Some(FacetKind::Work)).unwrap();
        assert!(
            work_src.persona.is_none(),
            "merged work facet drops the mask"
        );
        assert_eq!(work_src.ocean.extraversion, Some(9));
    }

    #[test]
    fn list_profile_src_switches_to_merged_persona_on_online() {
        let p = fixture_person();
        let online_src = list_profile_src(&p, Some(FacetKind::Online)).unwrap();
        assert!(
            online_src.persona.is_none(),
            "merged online facet drops the mask"
        );
        assert_eq!(online_src.ocean.extraversion, Some(7));
        let base_src = list_profile_src(&p, Some(FacetKind::Base)).unwrap();
        assert_eq!(base_src.ocean.extraversion, Some(2));
    }

    #[test]
    fn list_profile_src_base_tab_reads_private_mask_for_work_primary() {
        let mut p = fixture_person();
        p.primary_facet = FacetKind::Work;
        p.private_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                extraversion: Some(11),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        let base_src = list_profile_src(&p, Some(FacetKind::Base)).unwrap();
        assert_eq!(
            base_src.ocean.extraversion,
            Some(11),
            "the Personal-life tab must blend the private persona onto the work anchor"
        );
        assert!(
            base_src.private_persona.is_none(),
            "merged view strips masks"
        );
    }

    #[test]
    fn list_profile_src_principal_anchors_on_each_persons_primary() {
        // Base-primary: "Principal" ranking is the plain base view.
        let base_p = fixture_person();
        let base_src = list_profile_src(&base_p, None).unwrap();
        assert!(base_src.persona.is_some(), "principal keeps the masks");
        assert_eq!(base_src.ocean.extraversion, Some(2));

        // Work-primary with a private mask: "Principal" ranks the work anchor,
        // never the personal-life blend (that one is the Base tab's job).
        let mut work_p = fixture_person();
        work_p.primary_facet = FacetKind::Work;
        work_p.private_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                extraversion: Some(11),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        let work_src = list_profile_src(&work_p, None).unwrap();
        assert_eq!(
            work_src.ocean.extraversion,
            Some(2),
            "principal = the person's own anchor, not personal life"
        );
        assert!(
            work_src.persona.is_some(),
            "work anchor keeps the work mask"
        );
    }

    #[test]
    fn list_profile_src_drops_undefined_arena() {
        // A base-primary person with work/online masks ranks in all three.
        let p = fixture_person();
        assert!(list_profile_src(&p, Some(FacetKind::Base)).is_some());
        assert!(list_profile_src(&p, Some(FacetKind::Work)).is_some());
        assert!(list_profile_src(&p, Some(FacetKind::Online)).is_some());

        // No work persona → the person does not exist in the work arena.
        let mut bare = fixture_person();
        bare.persona = None;
        assert_eq!(list_profile_src(&bare, Some(FacetKind::Work)), None);
        // No private persona → no Personal-life arena for a work-primary person.
        let mut work_p = fixture_person();
        work_p.primary_facet = FacetKind::Work;
        work_p.private_persona = None;
        assert_eq!(list_profile_src(&work_p, Some(FacetKind::Base)), None);
        // "Principal" always resolves to the person's own anchor.
        assert!(list_profile_src(&bare, None).is_some());
        assert!(list_profile_src(&work_p, None).is_some());
    }
}

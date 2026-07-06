use dioxus::prelude::*;
use peoplemodeler_core::models::Person;
use peoplemodeler_core::i18n::Lang as CoreLang;

use crate::db;
use crate::i18n::Lang;
use crate::Route;

fn core_lang(l: Lang) -> CoreLang {
    match l {
        Lang::Fr => CoreLang::Fr,
        Lang::En => CoreLang::En,
    }
}

#[component]
pub fn ComparePersons(id1: String, id2: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let p1 = use_signal(|| db::person(&id1));
    let p2 = use_signal(|| db::person(&id2));
    let cl = core_lang(lang());
    let not_found = crate::i18n::tr("person_not_found", lang());

    match (p1(), p2()) {
        (Some(a), Some(b)) => {
            let compare_title = crate::i18n::tr("compare_title", lang());
            let mot_title = crate::i18n::tr("motivations_title", lang());
            let bias_title = crate::i18n::tr("biases_title", lang());
            let ocean_title = crate::i18n::tr("ocean_title", lang());
            let back_btn = crate::i18n::tr("common_back", lang());

            rsx! {
                div { class: "page",
                    Link { to: Route::PersonDetail { id: id1.clone() }, class: "btn", "{back_btn}" }
                    h2 { "{compare_title}" }
                    div { class: "compare-grid",
                        div { class: "compare-col",
                            PersonCard { person: a.clone() }
                        }
                        div { class: "compare-col",
                            PersonCard { person: b.clone() }
                        }
                    }
                    h3 { "{ocean_title}" }
                    div { class: "compare-grid",
                        div { class: "compare-col",
                            MiniRadar { scores: [
                                a.ocean.openness, a.ocean.conscientiousness,
                                a.ocean.extraversion, a.ocean.agreeableness,
                                a.ocean.neuroticism,
                            ] }
                        }
                        div { class: "compare-col",
                            MiniRadar { scores: [
                                b.ocean.openness, b.ocean.conscientiousness,
                                b.ocean.extraversion, b.ocean.agreeableness,
                                b.ocean.neuroticism,
                            ] }
                        }
                    }
                    h3 { "{mot_title}" }
                    div { class: "compare-grid",
                        div { class: "compare-col",
                            for m in &a.motivations {
                                div { class: "compare-item",
                                    span { "{m.r#type.emoji()} {m.r#type.i18n(cl).label}: {m.intensity}/10" }
                                }
                            }
                        }
                        div { class: "compare-col",
                            for m in &b.motivations {
                                div { class: "compare-item",
                                    span { "{m.r#type.emoji()} {m.r#type.i18n(cl).label}: {m.intensity}/10" }
                                }
                            }
                        }
                    }
                    h3 { "{bias_title}" }
                    div { class: "compare-grid",
                        div { class: "compare-col",
                            for b in &a.biases {
                                div { class: "compare-item",
                                    span { "{b.r#type.emoji()} {b.r#type.i18n(cl).label}: {b.intensity}/10" }
                                }
                            }
                        }
                        div { class: "compare-col",
                            for b in &b.biases {
                                div { class: "compare-item",
                                    span { "{b.r#type.emoji()} {b.r#type.i18n(cl).label}: {b.intensity}/10" }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => rsx! { div { class: "page", h2 { "{not_found}" } } },
    }
}

#[component]
fn PersonCard(person: Person) -> Element {
    rsx! {
        div { class: "person-card-static",
            span { class: "avatar-lg", "{person.avatar_emoji}" }
            h3 { "{person.name}" }
            p { "{person.role}" }
        }
    }
}

#[component]
fn MiniRadar(scores: [u8; 5]) -> Element {
    use std::f64::consts::PI;
    let cx = 60.0; let cy = 60.0; let r = 45.0;
    let pts: Vec<String> = scores.iter().enumerate().map(|(i, s)| {
        let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
        let pr = r * *s as f64 / 10.0;
        format!("{:.1},{:.1}", cx + pr * a.cos(), cy + pr * a.sin())
    }).collect();
    let data = pts.join(" ");
    let labels = ["O", "C", "E", "A", "N"];
    let lpos: Vec<(f64, f64)> = (0..5).map(|i| {
        let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
        (cx + (r + 10.0) * a.cos(), cy + (r + 10.0) * a.sin())
    }).collect();

    let grids: Vec<String> = [2, 4, 6, 8, 10].iter().map(|level| {
        let lr = r * *level as f64 / 10.0;
        let lp: Vec<String> = (0..5).map(|i| {
            let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
            format!("{:.1},{:.1}", cx + lr * a.cos(), cy + lr * a.sin())
        }).collect();
        format!("{} {}", lp.join(" "), lp[0])
    }).collect();

    let dots: Vec<(f64, f64)> = scores.iter().enumerate().map(|(i, s)| {
        let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
        let pr = r * *s as f64 / 10.0;
        (cx + pr * a.cos(), cy + pr * a.sin())
    }).collect();

    rsx! {
        svg { view_box: "0 0 120 120", width: "100%", height: "auto",
            for g in &grids {
                polygon { fill: "none", stroke: "var(--border)", stroke_width: "0.5", points: "{g}" }
            }
            polygon { fill: "var(--cyan)", fill_opacity: "0.2", stroke: "var(--cyan)",
                stroke_width: "1.5", points: "{data}" }
            for (x, y) in &dots {
                circle { cx: "{x:.1}", cy: "{y:.1}", r: "2.5", fill: "var(--cyan)" }
            }
            for (i, l) in labels.iter().enumerate() {
                text { x: "{lpos[i].0:.1}", y: "{lpos[i].1:.1}",
                    text_anchor: "middle", dominant_baseline: "central",
                    font_size: "8", fill: "var(--text-muted)", "{l}" }
            }
        }
    }
}

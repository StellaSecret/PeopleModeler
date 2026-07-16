use dioxus::prelude::*;
use peoplemodeler_core::models::RelationType;
use std::collections::{HashMap, HashSet};

use crate::db;
use crate::i18n::Lang;
use crate::Route;


const NODE_R: f64 = 14.0;
const CHORD_R: f64 = 260.0;
const SVG_S: f64 = 800.0;
const CX: f64 = SVG_S / 2.0;
const CY: f64 = SVG_S / 2.0;

const TYPE_COLORS: [&str; 8] = [
    "var(--cyan)", "var(--orange)", "var(--green)", "var(--pink)",
    "var(--purple)", "var(--gold)", "var(--teal)", "var(--blue)",
];

fn type_color(rt: &RelationType) -> &'static str {
    let idx = RelationType::ALL.iter().position(|t| t == rt).unwrap_or(0);
    TYPE_COLORS[idx % TYPE_COLORS.len()]
}

fn type_idx(rt: &RelationType) -> usize {
    RelationType::ALL.iter().position(|t| t == rt).unwrap_or(0)
}

struct NodePos {
    id: String,
    x: f64,
    y: f64,
    tx: f64,
    anchor: String,
    label: String,
}

struct ChordArc {
    key: String,
    path_d: String,
    color: &'static str,
}

#[component]
pub fn Relationships() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let nav = use_navigator();
    let rels = use_signal(db::all_relationships);
    let persons = use_signal(db::all_persons);

    let title = crate::i18n::tr("rel_title", lang());
    let none = crate::i18n::tr("rel_none", lang());

    let mut search = use_signal(String::new);
    let mut active_types = use_signal(|| RelationType::ALL.to_vec());

    let all_persons = persons();
    let all_rels = rels();
    let search_text = search();
    let active = active_types();

    let matched_persons: Vec<(String, String, String)> = all_persons
        .into_iter()
        .filter(|p| {
            search_text.is_empty()
                || p.name.to_lowercase().contains(&search_text.to_lowercase())
        })
        .map(|p| (p.id, p.name, p.avatar_emoji))
        .collect();

    let person_ids: HashSet<&str> = matched_persons.iter().map(|(id, _, _)| id.as_str()).collect();

    let mut pair_map: HashMap<(&str, &str), Vec<RelationType>> = HashMap::new();
    for rel in &all_rels {
        if !active.contains(&rel.r#type) {
            continue;
        }
        if !person_ids.contains(rel.source_id.as_str())
            || !person_ids.contains(rel.target_id.as_str())
        {
            continue;
        }
        let key = if rel.source_id < rel.target_id {
            (rel.source_id.as_str(), rel.target_id.as_str())
        } else {
            (rel.target_id.as_str(), rel.source_id.as_str())
        };
        let types = pair_map.entry(key).or_default();
        if !types.contains(&rel.r#type) {
            types.push(rel.r#type);
        }
    }

    let n = matched_persons.len() as f64;
    let n_usize = matched_persons.len();

    let mut nodes: Vec<NodePos> = Vec::new();
    for (i, (id, name, emoji)) in matched_persons.iter().enumerate() {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / n - std::f64::consts::PI / 2.0;
        let x = CX + CHORD_R * angle.cos();
        let y = CY + CHORD_R * angle.sin();
        let (anchor, tx) = if angle.cos() >= 0.0 {
            ("start".to_string(), x + NODE_R + 8.0)
        } else {
            ("end".to_string(), x - NODE_R - 8.0)
        };
        nodes.push(NodePos {
            id: id.clone(),
            x,
            y,
            tx,
            anchor,
            label: format!("{} {}", emoji, name),
        });
    }

    let mut chords: Vec<ChordArc> = Vec::new();
    for ((id1, id2), types) in &pair_map {
        let Some(p1) = nodes.iter().find(|n| n.id == *id1) else { continue };
        let Some(p2) = nodes.iter().find(|n| n.id == *id2) else { continue };
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 1.0 {
            continue;
        }
        let sx = x1 + dx / dist * NODE_R;
        let sy = y1 + dy / dist * NODE_R;
        let ex = x2 - dx / dist * NODE_R;
        let ey = y2 - dy / dist * NODE_R;

        let pdx = -dy / dist;
        let pdy = dx / dist;

        let mut sorted: Vec<&RelationType> = types.iter().collect();
        sorted.sort_by_key(|t| type_idx(t));

        for (i, rt) in sorted.iter().enumerate() {
            let offset = i as f64 * 7.0 - (sorted.len() as f64 - 1.0) * 3.5;
            let mx = (sx + ex) / 2.0 + pdx * offset;
            let my = (sy + ey) / 2.0 + pdy * offset;

            let cdx = CX - mx;
            let cdy = CY - my;
            let cd = (cdx * cdx + cdy * cdy).sqrt();
            let curvature = cd * 0.28;
            let cpx = mx + cdx / cd * curvature;
            let cpy = my + cdy / cd * curvature;

            let path_d = format!(
                "M {:.1},{:.1} Q {:.1},{:.1} {:.1},{:.1}",
                sx, sy, cpx, cpy, ex, ey
            );
            let color = type_color(rt);
            chords.push(ChordArc {
                key: format!("{}-{}-{:?}", id1, id2, rt),
                path_d,
                color,
            });
        }
    }

    rsx! {
        div { class: "page",
            div { class: "page-header",
                h2 { "{title}" }
            }

            div { class: "chord-controls",
                input {
                    class: "chord-search",
                    placeholder: "Search person…",
                    value: search,
                    oninput: move |e| search.set(e.value()),
                }
                div { class: "type-chips",
                    for rt in RelationType::ALL {
                        {
                        let on = active.contains(&rt);
                        let chip_color = type_color(&rt);
                        let chip_style = if on {
                            format!("border-color: {}; background: {};", chip_color, chip_color)
                        } else {
                            format!("border-color: {};", chip_color)
                        };
                        rsx! {
                            button {
                                key: "chip-{rt:?}",
                                class: if on { "type-chip On" } else { "type-chip" },
                                style: "{chip_style}",
                                onclick: {
                                    let rt2 = rt;
                                    move |_| {
                                        let mut v = active_types();
                                        if v.contains(&rt2) {
                                            v.retain(|t| *t != rt2);
                                        } else {
                                            v.push(rt2);
                                        }
                                        active_types.set(v);
                                    }
                                },
                                "{rt:?}"
                            }
                        }
                        }
                    }
                }
            }

            if n_usize < 2 {
                p { "{none}" }
            } else {
                svg {
                    view_box: "0 0 800 800",
                    class: "chord-diagram",
                    for c in &chords {
                        path {
                            key: "{c.key}",
                            d: "{c.path_d}",
                            fill: "none",
                            stroke: "{c.color}",
                            stroke_width: "2.5",
                            opacity: "0.65",
                        }
                    }
                    for pos in &nodes {
                        g {
                            key: "{pos.id}",
                            class: "chord-node",
                            onmousedown: {
                                let pid = pos.id.clone();
                                move |_| { let _ = nav.push(Route::PersonDetail { id: pid.clone() }); }
                            },
                            circle {
                                cx: "{pos.x:.1}",
                                cy: "{pos.y:.1}",
                                r: "{NODE_R}",
                                class: "chord-node-circle",
                            }
                            circle {
                                cx: "{pos.x:.1}",
                                cy: "{pos.y:.1}",
                                r: "{NODE_R}",
                                fill: "none",
                                stroke: "var(--border)",
                                stroke_width: "1.5",
                            }
                            text {
                                x: "{pos.tx:.1}",
                                y: "{pos.y:.1}",
                                text_anchor: "{pos.anchor}",
                                alignment_baseline: "middle",
                                class: "chord-node-text",
                                "{pos.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

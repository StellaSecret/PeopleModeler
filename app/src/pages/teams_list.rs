use dioxus::prelude::*;

use crate::Route;
use crate::db;
use crate::i18n::Lang;

#[component]
pub fn TeamsList() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let title = crate::i18n::tr("teams_title", lang());
    let all_label = crate::i18n::tr("teams_all", lang());
    let create_label = crate::i18n::tr("teams_create", lang());
    let confirm_delete = crate::i18n::tr("confirm_delete_team", lang());
    let members_fmt = crate::i18n::tr("teams_members", lang());
    let common_delete = crate::i18n::tr("common_delete", lang());
    let common_cancel = crate::i18n::tr("common_cancel", lang());

    let all_count = db::all_persons().len();
    let mut teams = use_signal(db::all_teams);
    let mut confirming_del: Signal<Option<String>> = use_signal(|| None);

    let members_str = |count: usize| -> String { members_fmt.replace("{0}", &count.to_string()) };

    let team_rows: Vec<(String, String, String, usize)> = teams()
        .iter()
        .map(|t| {
            (
                t.id.clone(),
                team_icon(t),
                t.name.clone(),
                t.member_ids.len(),
            )
        })
        .collect();

    rsx! {
        div { class: "page",
            div { class: "teams-header",
                h2 { "{title}" }
                Link { to: Route::TeamNew {}, class: "btn btn-primary",
                    "+ {create_label}"
                }
            }

            div { class: "teams-list",
                div { class: "teams-row teams-row-all",
                    Link { to: Route::TeamDetail { id: "all".to_string() }, class: "teams-row-link",
                        div { class: "teams-row-info",
                            span { class: "teams-row-emoji", "👥" }
                            span { class: "teams-row-name", "{all_label}" }
                        }
                        span { class: "teams-row-count", "{all_count}" }
                    }
                }
                for (tid, icon, name, count) in team_rows.into_iter() {
                    div { class: "teams-row",
                        Link { to: Route::TeamDetail { id: tid.clone() }, class: "teams-row-link",
                            div { class: "teams-row-info",
                                span { class: "teams-row-emoji", "{icon}" }
                                span { class: "teams-row-name", "{name}" }
                            }
                            span { class: "teams-row-count", "{members_str(count)}" }
                        }
                        if confirming_del() == Some(tid.clone()) {
                            div { class: "teams-confirm-delete",
                                span { "{confirm_delete}" }
                                button { class: "btn-danger",
                                    onclick: {
                                        let tid2 = tid.clone();
                                        move |_| {
                                            let _ = db::delete_team(&tid2);
                                            confirming_del.set(None);
                                            teams.set(db::all_teams());
                                        }
                                    },
                                    "{common_delete}"
                                }
                                button { class: "btn-ghost",
                                    onclick: move |_| confirming_del.set(None),
                                    "{common_cancel}"
                                }
                            }
                        } else {
                            button { class: "btn-ghost btn-sm teams-del-btn",
                                onclick: {
                                    let tid2 = tid.clone();
                                    move |_| confirming_del.set(Some(tid2.clone()))
                                },
                                "×"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn team_icon(t: &peoplemodeler_core::models::Team) -> String {
    if t.icon.is_empty() {
        "🎯".to_string()
    } else {
        t.icon.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use peoplemodeler_core::models::Team;

    #[test]
    fn team_icon_empty() {
        let t = Team {
            id: "1".into(),
            name: "Test".into(),
            icon: String::new(),
            member_ids: vec![],
            created_at: 0,
        };
        assert_eq!(team_icon(&t), "🎯");
    }

    #[test]
    fn team_icon_custom() {
        let t = Team {
            id: "1".into(),
            name: "Test".into(),
            icon: "🚀".into(),
            member_ids: vec![],
            created_at: 0,
        };
        assert_eq!(team_icon(&t), "🚀");
    }
}

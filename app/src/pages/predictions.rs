use dioxus::prelude::*;
use peoplemodeler_core::models::Prediction;

use crate::Route;
use crate::db;
use crate::i18n::Lang;

#[allow(dead_code)]
pub fn should_skip_add(ctx: &str, predicted: &str) -> bool {
    ctx.is_empty() || predicted.is_empty()
}

pub fn matches_person(pred: &Prediction, pid: &str) -> bool {
    pred.person_id == pid
}

#[component]
pub fn Predictions() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut preds = use_signal(db::all_predictions);
    let title = crate::i18n::tr("pred_all_title", lang());
    rsx! {
        div { class: "page",
            h2 { "{title}" }
            PredictionList { predictions: preds(), person_filter: None,
                onresolve: move |_| preds.set(db::all_predictions()),
                ondelete: move |_| preds.set(db::all_predictions()) }
        }
    }
}

#[component]
pub fn PersonPredictions(person_id: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let person = db::person(&person_id);
    let pid = person_id.clone();
    let mut preds = use_signal(|| db::predictions_for_person(&person_id));
    let mut context = use_signal(String::new);
    let mut predicted = use_signal(String::new);
    let mut toast_sig = use_context::<Signal<Option<String>>>();
    let pred_for = crate::i18n::tr("pred_for", lang());
    let pred_title = crate::i18n::tr("pred_title", lang());
    let ctx_pl = crate::i18n::tr("pred_context_placeholder", lang());
    let outcome_pl = crate::i18n::tr("pred_outcome_placeholder", lang());
    let add_btn = crate::i18n::tr("pred_add_btn", lang());
    let back_btn = crate::i18n::tr("common_back", lang());

    let mut add_pred = move || {
        let ctx = context();
        let pred = predicted();
        if should_skip_add(&ctx, &pred) {
            return;
        }
        let p = Prediction {
            id: uuid::Uuid::new_v4().to_string(),
            person_id: pid.clone(),
            context: ctx,
            predicted_outcome: pred,
            actual_outcome: None,
            accuracy: None,
            created_at: chrono::Utc::now().timestamp_millis(),
            resolved_at: None,
            resolved: false,
        };
        db::save_prediction(&p).unwrap_or_else(|e| {
            toast_sig.set(Some(format!(
                "{}: {e}",
                crate::i18n::tr("toast_error", lang())
            )))
        });
        context.set(String::new());
        predicted.set(String::new());
        preds.set(db::predictions_for_person(&pid));
    };

    rsx! {
        div { class: "page",
            h2 {
                if let Some(ref p) = person {
                    "{pred_for} {p.name}"
                } else {
                    "{pred_title}"
                }
            }
            div { class: "section",
                Link { to: Route::PersonDetail { id: person_id.clone() }, class: "btn", "{back_btn}" }
            }

            div { class: "form-row",
                input { placeholder: "{ctx_pl}", value: "{context}", oninput: move |e| context.set(e.value()) }
                input { placeholder: "{outcome_pl}", value: "{predicted}", oninput: move |e| predicted.set(e.value()) }
                button { class: "btn btn-primary", onclick: move |_| add_pred(), "{add_btn}" }
            }

            PredictionList { predictions: preds(), person_filter: Some(person_id.clone()),
                onresolve: {
                    let pid = person_id.clone();
                    move |_| preds.set(db::predictions_for_person(&pid))
                },
                ondelete: {
                    let pid = person_id.clone();
                    move |_| preds.set(db::predictions_for_person(&pid))
                } }
        }
    }
}

#[component]
pub fn PredictionList(
    predictions: Vec<Prediction>,
    person_filter: Option<String>,
    onresolve: EventHandler<()>,
    ondelete: EventHandler<()>,
) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let filtered = if let Some(ref pid) = person_filter {
        predictions
            .into_iter()
            .filter(|p| matches_person(p, pid))
            .collect::<Vec<_>>()
    } else {
        predictions
    };

    if filtered.is_empty() {
        let no_preds = crate::i18n::tr("pred_none", lang());
        return rsx! { p { "{no_preds}" } };
    }

    rsx! {
        div { class: "prediction-list",
            for pred in filtered {
                PredictionCard { prediction: pred, onresolve, ondelete }
            }
        }
    }
}

#[component]
fn PredictionCard(
    prediction: Prediction,
    onresolve: EventHandler<()>,
    ondelete: EventHandler<()>,
) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut toast_sig = use_context::<Signal<Option<String>>>();
    let resolved = prediction.resolved;
    let pred_label = crate::i18n::tr("pred_predicted_label", lang());
    let actual_label = crate::i18n::tr("pred_actual_label", lang());
    let acc_label = crate::i18n::tr("pred_accuracy_label", lang());
    let resolve_btn = crate::i18n::tr("pred_resolve_btn", lang());
    let delete_btn = crate::i18n::tr("pred_delete_btn", lang());
    let actual_pl = crate::i18n::tr("pred_actual_placeholder", lang());
    let resolve_submit = crate::i18n::tr("pred_resolve_submit", lang());
    let cancel_btn = crate::i18n::tr("pred_cancel_btn", lang());

    let mut show_form = use_signal(|| false);
    let mut actual = use_signal(String::new);
    let mut accuracy = use_signal(|| 5u8);
    let mut confirming_delete = use_signal(|| false);
    let confirm_delete_pred = crate::i18n::tr("confirm_delete_pred", lang());

    let outcome_str = prediction
        .actual_outcome
        .clone()
        .unwrap_or_else(|| "N/A".into());
    let acc_str = prediction
        .accuracy
        .map(|a| format!("{acc_label}: {a}/10"))
        .unwrap_or_default();
    let del_id = prediction.id.clone();
    let mut delete = move || {
        let e = match db::delete_prediction(&del_id) {
            Ok(()) => {
                ondelete.call(());
                return;
            }
            Err(e) => e,
        };
        toast_sig.set(Some(format!(
            "{}: {e}",
            crate::i18n::tr("toast_error", lang())
        )));
    };
    let resolve_pred = prediction.clone();
    let mut resolve = move || {
        let a = actual();
        if a.is_empty() {
            return;
        }
        let mut p = resolve_pred.clone();
        p.actual_outcome = Some(a);
        p.accuracy = Some(accuracy());
        p.resolved = true;
        p.resolved_at = Some(chrono::Utc::now().timestamp_millis());
        if let Err(e) = db::save_prediction(&p) {
            toast_sig.set(Some(format!(
                "{}: {e}",
                crate::i18n::tr("toast_error", lang())
            )));
            return;
        }
        show_form.set(false);
        onresolve.call(());
    };

    rsx! {
        div { class: "prediction-card",
            div { class: "pred-header",
                strong { "{prediction.context}" }
                span { class: "date", "{format_date(prediction.created_at)}" }
            }
            p { "{pred_label}: {prediction.predicted_outcome}" }
            if resolved {
                p { "{actual_label}: {outcome_str}" }
                p { "{acc_str}" }
            }
            if !resolved && !show_form() {
                button { class: "btn btn-small", onclick: move |_| show_form.set(true), "{resolve_btn}" }
            }
            if confirming_delete() {
                div { class: "pred-confirm-row",
                    button { class: "btn btn-small btn-danger", onclick: move |_| delete(), "{confirm_delete_pred}" }
                    button { class: "btn btn-small", onclick: move |_| confirming_delete.set(false), "{cancel_btn}" }
                }
            } else {
                button { class: "btn btn-small btn-danger", onclick: move |_| confirming_delete.set(true), "{delete_btn}" }
            }
            if !resolved && show_form() {
                div { class: "resolve-form",
                    input { placeholder: "{actual_pl}", value: "{actual}",
                        oninput: move |e| actual.set(e.value())
                    }
                    div { class: "ocean-slider",
                        label { "{acc_label}: {accuracy}/10" }
                        input { r#type: "range", min: "1", max: "10", value: "{accuracy}",
                            oninput: move |e| accuracy.set(e.value().parse().unwrap_or(5))
                        }
                    }
                    button { class: "btn btn-primary", onclick: move |_| resolve(), "{resolve_submit}" }
                    button { class: "btn", onclick: move |_| show_form.set(false), "{cancel_btn}" }
                }
            }
        }
    }
}

pub(crate) fn format_date(ts: i64) -> String {
    chrono::DateTime::from_timestamp_millis(ts)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "unknown".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_date_valid_timestamp() {
        let ts = 1_700_000_000_000;
        let result = format_date(ts);
        assert!(result.contains("2023"), "got {}", result);
        assert!(result.contains("-"), "expected date format, got {}", result);
    }

    #[test]
    fn format_date_invalid_returns_unknown() {
        // Very large timestamp that overflows DateTime representation
        let result = format_date(i64::MAX);
        assert_eq!(result, "unknown");
    }

    #[test]
    fn should_skip_add_both_empty() {
        assert!(should_skip_add("", ""));
    }

    #[test]
    fn should_skip_add_context_empty() {
        assert!(should_skip_add("", "rain tomorrow"));
    }

    #[test]
    fn should_skip_add_predicted_empty() {
        assert!(should_skip_add("will it rain?", ""));
    }

    #[test]
    fn should_skip_add_neither_empty() {
        assert!(!should_skip_add("will it rain?", "rain tomorrow"));
    }

    #[test]
    fn matches_person_same_id() {
        let p = Prediction {
            id: "p1".into(),
            person_id: "alice".into(),
            context: "ctx".into(),
            predicted_outcome: "out".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 0,
            resolved_at: None,
            resolved: false,
        };
        assert!(matches_person(&p, "alice"));
    }

    #[test]
    fn matches_person_different_id() {
        let p = Prediction {
            id: "p1".into(),
            person_id: "alice".into(),
            context: "ctx".into(),
            predicted_outcome: "out".into(),
            actual_outcome: None,
            accuracy: None,
            created_at: 0,
            resolved_at: None,
            resolved: false,
        };
        assert!(!matches_person(&p, "bob"));
    }
}

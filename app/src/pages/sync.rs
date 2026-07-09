use crate::auth;
use crate::db;
use crate::drive;
use crate::i18n::Lang;
use dioxus::prelude::*;

const ENV_CLIENT_ID: Option<&str> = option_env!("GOOGLE_CLIENT_ID");
const HAS_CLIENT: bool = match ENV_CLIENT_ID {
    Some(c) => !c.is_empty(),
    None => false,
};

fn drive_client_id() -> &'static str {
    match ENV_CLIENT_ID {
        Some(c) if !c.is_empty() => c,
        _ => "",
    }
}

fn mask_token(t: &str) -> String {
    if t.len() > 8 {
        format!("{}…{}", &t[..4], &t[t.len() - 4..])
    } else {
        "****".to_string()
    }
}

#[cfg(target_arch = "wasm32")]
fn export_file(json: &str) {
    use js_sys::Array;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;
    let arr = Array::new();
    arr.push(&JsValue::from(json));
    let blob = web_sys::Blob::new_with_str_sequence(&arr).expect("Blob creation failed");
    let url = web_sys::Url::create_object_url_with_blob(&blob).expect("Object URL creation failed");
    let window = web_sys::window().expect("no window in WASM");
    let doc = window.document().expect("no document in WASM");
    let a = doc.create_element("a").expect("createElement('a') failed");
    let _ = a.set_attribute("href", &url);
    let _ = a.set_attribute("download", "peoplemodeler_backup.json");
    if let Some(body) = doc.body() {
        let _ = body.append_child(&a);
    }
    if let Some(el) = a.dyn_ref::<web_sys::HtmlElement>() {
        el.click();
    }
    if let Some(body) = doc.body() {
        let _ = body.remove_child(&a);
    }
    web_sys::Url::revoke_object_url(&url).ok();
}

#[cfg(not(target_arch = "wasm32"))]
fn export_file(json: &str) {
    #[cfg(target_os = "android")]
    {
        crate::android_share::share_export_file(json);
    }
    #[cfg(not(target_os = "android"))]
    {
        let path = std::path::PathBuf::from(
            "/data/data/com.stellasecret.peoplemodeler/files/peoplemodeler_backup.json",
        );
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Err(e) = std::fs::write(&path, json) {
            eprintln!("Export failed: {e}");
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn import_button(lang: Lang, status: Signal<String>) -> Element {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::prelude::Closure;
    let s = status.clone();
    let import_btn = crate::i18n::tr("sync_import_btn", lang);
    rsx! {
        div { class: "form-row",
            button { class: "btn", aria_label: "{import_btn}", onclick: move |_| {
                let s2 = s.clone();
                let window = web_sys::window().expect("no window in WASM");
                let doc = window.document().expect("no document in WASM");
                let input = doc.create_element("input").expect("createElement('input') failed");
                let _ = input.set_attribute("type", "file");
                let _ = input.set_attribute("accept", ".json");
                let _ = input.set_attribute("style", "display:none");
                let input2 = input.clone();
                let cb = Closure::<dyn FnMut()>::new(move || {
                    let files = js_sys::Reflect::get(&input2, &"files".into())
                        .ok()
                        .and_then(|f| f.dyn_into::<web_sys::FileList>().ok());
                    if let Some(files) = files {
                        if files.length() > 0 {
                            let file = files.get(0).expect("file exists after length check");
                            let reader = web_sys::FileReader::new().expect("FileReader creation failed");
                            let r2 = reader.clone();
                            let mut s3 = s2.clone();
                            let onload = Closure::<dyn FnMut()>::new(move || {
                                let result = r2.result().ok()
                                    .and_then(|r| r.as_string());
                                if let Some(json) = result {
                                    match drive::restore_from_json(&json) {
                                        Ok(n) => s3.set(format!("{} {n} persons", crate::i18n::tr("sync_restored", lang))),
                                        Err(e) => s3.set(format!("❌ {e}")),
                                    }
                                }
                            });
                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                            onload.forget();
                            let blob: &web_sys::Blob = file.unchecked_ref();
                            let _ = reader.read_as_text(blob);
                        }
                    }
                });
                if let Some(body) = doc.body() {
                    let _ = body.append_child(&input);
                }
                let target: &web_sys::EventTarget = input.unchecked_ref();
                target.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref()).ok();
                if let Some(el) = input.dyn_ref::<web_sys::HtmlElement>() {
                    el.click();
                }
                cb.forget();
            }, "{import_btn}" }
        }
    }
}

#[cfg(target_os = "android")]
fn android_import_button(lang: Lang, status: Signal<String>) -> Element {
    let import_btn = crate::i18n::tr("sync_import_btn", lang);
    let restore_ok = crate::i18n::tr("sync_restored", lang);
    rsx! {
        div { class: "form-row",
            button { class: "btn", aria_label: "{import_btn}", onclick: move |_| {
                let mut s = status.clone();
                crate::android_share::start_import();
                let (tx, rx) = tokio::sync::oneshot::channel::<String>();
                crate::android_share::set_import_callback(tx);
                dioxus::prelude::spawn(async move {
                    let content = rx.await.unwrap_or_default();
                    if !content.is_empty() {
                        match crate::drive::restore_from_json(&content) {
                            Ok(n) => s.set(format!("{} {n} persons", restore_ok)),
                            Err(e) => s.set(format!("❌ {e}")),
                        }
                    }
                });
            }, "{import_btn}" }
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
fn import_ui(lang: Lang, status: Signal<String>) -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        return import_button(lang, status);
    }
    #[cfg(target_os = "android")]
    {
        return android_import_button(lang, status);
    }
    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    rsx! {}
}

#[cfg_attr(
    any(target_arch = "wasm32", target_os = "android"),
    allow(unused_mut, unused_variables)
)]
fn token_paste_ui(
    lang: Lang,
    has_token: bool,
    mut paste_buf: Signal<String>,
    mut token: Signal<String>,
    mut status: Signal<String>,
) -> Element {
    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    if !has_token {
        let ts1 = crate::i18n::tr("sync_token_instruction_1", lang);
        let ts2 = crate::i18n::tr("sync_token_instruction_2", lang);
        let ts3 = crate::i18n::tr("sync_token_instruction_3", lang);
        let ts4 = crate::i18n::tr("sync_token_instruction_4", lang);
        let paste_pl = crate::i18n::tr("sync_paste_placeholder", lang);
        let save_btn = crate::i18n::tr("sync_save_token_btn", lang);
        return rsx! {
            div { class: "instruction-box",
                p { class: "instruction-step", "{ts1}" }
                p { class: "instruction-step", "{ts2}" }
                p { class: "instruction-step", "{ts3}" }
                p { class: "instruction-step", "{ts4}" }
            }
            div { class: "form-row",
                input {
                    placeholder: "{paste_pl}",
                    value: "{paste_buf}",
                    oninput: move |e| paste_buf.set(e.value()),
                }
                button { class: "btn btn-small", aria_label: "{save_btn}", onclick: move |_| {
                    let raw = paste_buf();
                    let t = parse_token_from_url(&raw).unwrap_or(raw);
                    if !t.is_empty() {
                        auth::set_token(&t);
                        token.set(t.clone());
                        paste_buf.set(String::new());
                        status.set(crate::i18n::tr("sync_token_saved", lang).into());
                    }
                }, "{save_btn}" }
            }
        };
    }
    rsx! {}
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn parse_token_from_url(url: &str) -> Option<String> {
    let fragment = url.split('#').nth(1)?;
    for pair in fragment.split('&') {
        let mut parts = pair.splitn(2, '=');
        if parts.next()? == "access_token" {
            return parts.next().map(String::from);
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn spawn_async<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_async<F: std::future::Future<Output = ()> + 'static>(f: F) {
    dioxus::prelude::spawn(f);
}

#[component]
pub fn SyncPage() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut status = use_signal(|| String::new());
    let mut token = use_signal(|| auth::get_token().unwrap_or_default());
    let paste_buf = use_signal(String::new);
    let mut passphrase = use_signal(String::new);
    let mut show_pp = use_signal(|| false);

    let has_token = !token().is_empty();

    // On Android: wait for JNI push instead of polling
    #[cfg(target_os = "android")]
    {
        let mut t = token.clone();
        use_future(move || async move {
            // Already have token? Done.
            if let Some(new_t) = auth::get_token() {
                if !new_t.is_empty() {
                    t.set(new_t);
                    return;
                }
            }
            // Register oneshot, re-check token in case JNI callback already fired
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            crate::android_auth::set_token_callback(tx);
            if let Some(new_t) = auth::get_token() {
                if !new_t.is_empty() {
                    t.set(new_t);
                    return;
                }
            }
            let _ = rx.await;
            if let Some(new_t) = auth::get_token() {
                if !new_t.is_empty() {
                    t.set(new_t);
                }
            }
        });
    }

    let sync_title = crate::i18n::tr("sync_title", lang());
    let gdrive_title = crate::i18n::tr("sync_gdrive_title", lang());
    let not_configured = crate::i18n::tr("sync_not_configured", lang());
    let local_title = crate::i18n::tr("sync_local_title", lang());
    let local_desc = crate::i18n::tr("sync_local_desc", lang());
    let pp_label = crate::i18n::tr("sync_passphrase_label", lang());
    let pp_placeholder = crate::i18n::tr("sync_passphrase_placeholder", lang());
    let pp_show = crate::i18n::tr("sync_passphrase_show", lang());
    let pp_hide = crate::i18n::tr("sync_passphrase_hide", lang());
    let export_btn = crate::i18n::tr("sync_export_btn", lang());
    let token_loaded = crate::i18n::tr("sync_token_loaded", lang());
    let clear_btn = crate::i18n::tr("sync_clear_btn", lang());
    let sign_in = crate::i18n::tr("sync_sign_in", lang());
    let backup_btn = crate::i18n::tr("sync_backup_btn", lang());
    let restore_btn = crate::i18n::tr("sync_restore_btn", lang());
    let no_data_warn = crate::i18n::tr("sync_no_data_warn", lang());

    rsx! {
        div { class: "page",
            h2 { "{sync_title}" }

            fieldset { class: "section",
                legend { "{gdrive_title}" }

                if HAS_CLIENT {
                    div { class: "form-row",
                        if has_token {
                            p { class: "token-ok", "{token_loaded} ({mask_token(&token())})" }
                    button { class: "btn btn-small", aria_label: "{clear_btn}", onclick: move |_| {
                        auth::clear_token();
                        token.set(String::new());
                        status.set(crate::i18n::tr("sync_token_cleared", lang()).into());
                    }, "{clear_btn}" }
                        }
                    }

                    {token_paste_ui(lang(), has_token, paste_buf, token, status)}

                    div { class: "form-row passphrase-row",
                        label { "{pp_label}" }
                        div { class: "passphrase-input-group",
                            input {
                                r#type: if show_pp() { "text" } else { "password" },
                                placeholder: "{pp_placeholder}",
                                value: "{passphrase}",
                                oninput: move |e| passphrase.set(e.value()),
                            }
                            button { class: "btn btn-small", aria_label: "Toggle passphrase visibility", onclick: move |_| show_pp.set(!show_pp()),
                                if show_pp() { "{pp_hide}" } else { "{pp_show}" }
                            }
                        }
                    }

                    div { class: "sync-actions",
                        button { class: "btn", aria_label: "{sign_in}", onclick: move |_| {
                            let cid = drive_client_id();
                            #[cfg(target_arch = "wasm32")]
                            {
                                let mut t = token.clone();
                                auth::on_token_received(Box::new(move |new_token: &str| {
                                    t.set(new_token.to_string());
                                }));
                            }
                            auth::start_oauth(cid, "https://stellasecret.github.io/PeopleModeler/spa.html");
                        }, "{sign_in}" }

                        button { class: "btn", aria_label: "{backup_btn}", onclick: move |_| {
                            if db::all_persons().is_empty() {
                                status.set(no_data_warn.into());
                                #[cfg(target_arch = "wasm32")]
                                web_sys::window().map(|w| w.alert_with_message(&no_data_warn).ok());
                                return;
                            }
                            let t = token();
                            if t.is_empty() { status.set(crate::i18n::tr("sync_no_token", lang()).into()); return; }
                            let pp = passphrase();
                            let ll = lang();
                            status.set(crate::i18n::tr("sync_backing_up", ll).into());
                            let mut s = status.clone();
                            spawn_async(async move {
                                let pp_ref: Option<&str> = if pp.is_empty() { None } else { Some(&pp) };
                                match drive::drive_backup(&t, pp_ref).await {
                                    Ok(id) => s.set(format!("{} (file id: {id})", crate::i18n::tr("sync_backed_up", ll))),
                                    Err(e) => s.set(format!("❌ {e}")),
                                }
                            });
                        }, "{backup_btn}" }

                        button { class: "btn", aria_label: "{restore_btn}", onclick: move |_| {
                            let t = token();
                            if t.is_empty() { status.set(crate::i18n::tr("sync_no_token", lang()).into()); return; }
                            let pp = passphrase();
                            let ll = lang();
                            status.set(crate::i18n::tr("sync_restoring", ll).into());
                            let mut s = status.clone();
                            spawn_async(async move {
                                let pp_ref: Option<&str> = if pp.is_empty() { None } else { Some(&pp) };
                                match drive::drive_restore(&t, pp_ref).await {
                                    Ok(n) => s.set(format!("{} {n} persons from Drive", crate::i18n::tr("sync_restored", ll))),
                                    Err(e) => s.set(format!("❌ {e}")),
                                }
                            });
                        }, "{restore_btn}" }
                    }
                } else {
                    p { "{not_configured}" }
                }
            }

            fieldset { class: "section",
                legend { "{local_title}" }
                p { "{local_desc}" }

                div { class: "sync-actions",
                    button { class: "btn btn-primary", aria_label: "{export_btn}", onclick: move |_| {
                        if db::all_persons().is_empty() {
                            status.set(no_data_warn.into());
                            #[cfg(target_arch = "wasm32")]
                            web_sys::window().map(|w| w.alert_with_message(&no_data_warn).ok());
                            return;
                        }
                        let json = drive::build_backup();
                        export_file(&json);
                        status.set(crate::i18n::tr("sync_exported", lang()).into());
                    }, "{export_btn}" }
                }

                {import_ui(lang(), status)}
            }

            div { class: "sync-status",
                if !status().is_empty() {
                    p { "{status}" }
                }
            }
        }
    }
}

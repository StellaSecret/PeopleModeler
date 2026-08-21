#[cfg(target_arch = "wasm32")]
const TOKEN_KEY: &str = "pm_drive_token";

#[cfg(target_arch = "wasm32")]
use std::sync::OnceLock;

#[cfg(target_arch = "wasm32")]
static TOKEN_CLIENT: OnceLock<wasm_bindgen::JsValue> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static TOKEN_LISTENERS: RefCell<Vec<Box<dyn FnMut(&str)>>> = const { RefCell::new(Vec::new()) };
}

#[cfg(target_arch = "wasm32")]
pub fn on_token_received(cb: Box<dyn FnMut(&str)>) {
    TOKEN_LISTENERS.with(|listeners| {
        listeners.borrow_mut().push(cb);
    });
}

#[cfg(target_arch = "wasm32")]
pub fn start_oauth(client_id: &str, _redirect_uri: &str) {
    use wasm_bindgen::JsCast;

    if let Some(tc) = TOKEN_CLIENT.get() {
        if let Some(f) = js_sys::Reflect::get(tc, &"requestAccessToken".into())
            .ok()
            .and_then(|v| v.dyn_into::<js_sys::Function>().ok())
        {
            let _ = f.call0(tc);
        }
        return;
    }

    if get_gis_oauth2().is_err() {
        return;
    }

    use wasm_bindgen::prelude::Closure;

    let cb = Closure::wrap(Box::new(move |resp: wasm_bindgen::JsValue| {
        if let Some(token) = js_sys::Reflect::get(&resp, &"access_token".into())
            .ok()
            .and_then(|t| t.as_string())
        {
            set_token(&token);
            TOKEN_LISTENERS.with(|listeners| {
                for cb in listeners.borrow_mut().iter_mut() {
                    cb(&token);
                }
            });
        }
    }) as Box<dyn FnMut(wasm_bindgen::JsValue)>);

    let config = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&config, &"client_id".into(), &client_id.into());
    let _ = js_sys::Reflect::set(&config, &"scope".into(), &crate::drive::DRIVE_SCOPE.into());
    let _ = js_sys::Reflect::set(&config, &"callback".into(), cb.as_ref());

    if let Ok(oauth2) = get_gis_oauth2() {
        if let Some(f) = js_sys::Reflect::get(&oauth2, &"initTokenClient".into())
            .ok()
            .and_then(|v| v.dyn_into::<js_sys::Function>().ok())
        {
            if let Ok(tc) = f.call1(&oauth2, &config) {
                cb.forget();
                TOKEN_CLIENT.set(tc.clone()).ok();
                if let Some(f) = js_sys::Reflect::get(&tc, &"requestAccessToken".into())
                    .ok()
                    .and_then(|v| v.dyn_into::<js_sys::Function>().ok())
                {
                    let _ = f.call0(&tc);
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_gis_oauth2() -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let google = js_sys::Reflect::get(&js_sys::global(), &"google".into())?;
    let accounts = js_sys::Reflect::get(&google, &"accounts".into())?;
    js_sys::Reflect::get(&accounts, &"oauth2".into())
}

#[cfg(target_os = "android")]
pub fn start_oauth(_client_id: &str, _redirect_uri: &str) {
    crate::android_auth::start_sign_in();
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
pub fn start_oauth(client_id: &str, redirect_uri: &str) {
    let scope = crate::drive::DRIVE_SCOPE;
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
         client_id={client_id}&\
         redirect_uri={redirect_uri}&\
         response_type=token&\
         scope={scope}&state=peoplemodeler"
    );
    let _ = webbrowser::open(&auth_url);
}

#[cfg(target_arch = "wasm32")]
pub fn init() {
    use wasm_bindgen::JsCast;
    let doc = web_sys::window()
        .expect("no window in WASM")
        .document()
        .expect("no document in WASM");

    if get_gis_oauth2().is_ok() {
        return;
    }

    if let Ok(script) = doc.create_element("script") {
        if let Ok(s) = script.dyn_into::<web_sys::HtmlScriptElement>() {
            s.set_src("https://accounts.google.com/gsi/client");
            s.set_defer(true);
            if let Some(body) = doc.body() {
                let _ = body.append_child(&s);
            }
        }
    }
}

pub fn get_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        return gloo_storage::LocalStorage::get(TOKEN_KEY).ok();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = token_path()?;
        std::fs::read_to_string(&path)
            .ok()
            .map(|s| s.trim().to_string())
    }
}

pub fn set_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        let _ = gloo_storage::LocalStorage::set(TOKEN_KEY, token);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = token_path() {
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = std::fs::write(&path, token);
        }
    }
}

pub fn clear_token() {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        gloo_storage::LocalStorage::delete(TOKEN_KEY);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = token_path() {
            let _ = std::fs::remove_file(&path);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn token_path() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "android")]
    {
        let dir = crate::android_auth::get_files_dir()?;
        return Some(dir.join(".pm_drive_token"));
    }

    #[cfg(not(target_os = "android"))]
    std::env::current_dir()
        .ok()
        .map(|p| p.join(".pm_drive_token"))
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn token_path_returns_pm_drive_token() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        let path = token_path().unwrap();
        assert!(path.to_string_lossy().ends_with(".pm_drive_token"));
        let _ = std::env::set_current_dir(&orig);
    }

    #[test]
    fn get_token_none_when_no_file() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        assert_eq!(get_token(), None);
        let _ = std::env::set_current_dir(&orig);
    }

    #[test]
    fn set_token_then_get_roundtrip() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        set_token("abc123");
        assert_eq!(get_token(), Some("abc123".to_string()));
        let _ = std::env::set_current_dir(&orig);
    }

    #[test]
    fn clear_token_removes_file() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        set_token("to_be_cleared");
        clear_token();
        assert_eq!(get_token(), None);
        let _ = std::env::set_current_dir(&orig);
    }

    #[test]
    fn get_token_trims_whitespace() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        let path = token_path().unwrap();
        std::fs::write(&path, "  spaced_token  \n").unwrap();
        assert_eq!(get_token(), Some("spaced_token".to_string()));
        let _ = std::env::set_current_dir(&orig);
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    fn clear_all() {
        use gloo_storage::Storage;
        let _ = gloo_storage::LocalStorage::delete("pm_drive_token");
    }

    #[wasm_bindgen_test]
    fn on_token_received_stores_callback() {
        let mut called = false;
        let ptr = &mut called as *mut bool;
        let cb = Box::new(move |_: &str| unsafe {
            *ptr = true;
        }) as Box<dyn FnMut(&str)>;
        super::on_token_received(cb);
        super::TOKEN_LISTENERS.with(|l| {
            assert!(!l.borrow().is_empty(), "listener was not stored");
        });
    }

    #[wasm_bindgen_test]
    fn get_token_wasm_none_initially() {
        clear_all();
        assert_eq!(super::get_token(), None);
    }

    #[wasm_bindgen_test]
    fn get_token_wasm_roundtrip() {
        clear_all();
        super::set_token("wasm_abc123");
        assert_eq!(super::get_token(), Some("wasm_abc123".to_string()));
        clear_all();
    }

    #[wasm_bindgen_test]
    fn clear_token_wasm_removes() {
        clear_all();
        super::set_token("to_be_cleared");
        super::clear_token();
        assert_eq!(super::get_token(), None);
    }

    #[wasm_bindgen_test]
    fn init_injects_script_element() {
        let doc = web_sys::window().unwrap().document().unwrap();
        let before = doc.get_elements_by_tag_name("script").length();
        super::init();
        let after = doc.get_elements_by_tag_name("script").length();
        assert!(after > before, "init should add a script element");
    }
}

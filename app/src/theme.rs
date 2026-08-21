#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn detect() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let stored = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item("theme").ok())
                .flatten();
            if let Some(t) = stored {
                return if t == "light" {
                    Theme::Light
                } else {
                    Theme::Dark
                };
            }
            if let Some(w) = web_sys::window() {
                if let Ok(Some(mq)) = w.match_media("(prefers-color-scheme: light)") {
                    if mq.matches() {
                        return Theme::Light;
                    }
                }
            }
            Theme::Dark
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(t) = std::fs::read_to_string(
                std::env::current_dir()
                    .unwrap_or_else(|_| ".".into())
                    .join(".pm_theme"),
            ) && t.trim() == "light"
            {
                return Theme::Light;
            }
            Theme::Dark
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Theme::Dark => "☀️",
            Theme::Light => "🌙",
        }
    }

    pub fn persist(self) {
        let s = self.as_str();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item("theme", s);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::write(
                std::env::current_dir()
                    .unwrap_or_else(|_| ".".into())
                    .join(".pm_theme"),
                s,
            );
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn as_str_dark() {
        assert_eq!(Theme::Dark.as_str(), "dark");
    }

    #[test]
    fn as_str_light() {
        assert_eq!(Theme::Light.as_str(), "light");
    }

    #[test]
    fn label_dark() {
        assert_eq!(Theme::Dark.label(), "☀️");
    }

    #[test]
    fn label_light() {
        assert_eq!(Theme::Light.label(), "🌙");
    }

    #[test]
    fn toggle_dark_to_light() {
        assert_eq!(Theme::Dark.toggle(), Theme::Light);
    }

    #[test]
    fn toggle_light_to_dark() {
        assert_eq!(Theme::Light.toggle(), Theme::Dark);
    }

    #[test]
    fn persist_as_str_roundtrip() {
        assert_eq!(Theme::Dark.as_str(), "dark");
        assert_eq!(Theme::Light.as_str(), "light");
    }

    #[test]
    fn detect_reads_light_from_file() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let pm_theme = dir.path().join(".pm_theme");
        std::fs::write(&pm_theme, "light\n").unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        assert_eq!(Theme::detect(), Theme::Light);
        let _ = std::env::set_current_dir(&orig);
    }

    #[test]
    fn detect_defaults_to_dark_when_file_missing() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        assert_eq!(Theme::detect(), Theme::Dark);
        let _ = std::env::set_current_dir(&orig);
    }

    #[test]
    fn persist_writes_file() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let pm_theme = dir.path().join(".pm_theme");
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        Theme::Light.persist();
        assert_eq!(std::fs::read_to_string(&pm_theme).unwrap().trim(), "light");
        Theme::Dark.persist();
        assert_eq!(std::fs::read_to_string(&pm_theme).unwrap().trim(), "dark");
        let _ = std::env::set_current_dir(&orig);
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    fn raw_storage() -> Option<web_sys::Storage> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
    }

    fn set_theme(value: &str) {
        if let Some(s) = raw_storage() {
            let _ = s.set_item("theme", value);
        }
    }

    fn clear_theme() {
        if let Some(s) = raw_storage() {
            let _ = s.remove_item("theme");
        }
    }

    fn get_theme() -> Option<String> {
        raw_storage()
            .and_then(|s| s.get_item("theme").ok())
            .flatten()
    }

    fn system_scheme() -> Theme {
        web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: light)").ok())
            .flatten()
            .map(|mq| {
                if mq.matches() {
                    Theme::Light
                } else {
                    Theme::Dark
                }
            })
            .unwrap_or(Theme::Dark)
    }

    #[wasm_bindgen_test]
    fn detect_wasm_reads_light_from_storage() {
        clear_theme();
        set_theme("light");
        assert_eq!(Theme::detect(), Theme::Light);
        clear_theme();
    }

    #[wasm_bindgen_test]
    fn detect_wasm_reads_dark_from_storage() {
        clear_theme();
        set_theme("dark");
        assert_eq!(Theme::detect(), Theme::Dark);
        clear_theme();
    }

    #[wasm_bindgen_test]
    fn detect_wasm_defaults_to_dark_on_unknown_value() {
        clear_theme();
        set_theme("not-a-theme");
        assert_eq!(Theme::detect(), Theme::Dark);
        clear_theme();
    }

    #[wasm_bindgen_test]
    fn detect_wasm_defaults_to_system_scheme() {
        clear_theme();
        assert_eq!(Theme::detect(), system_scheme());
    }

    #[wasm_bindgen_test]
    fn label_wasm_matches_theme() {
        assert_eq!(Theme::Dark.label(), "☀️");
        assert_eq!(Theme::Light.label(), "🌙");
    }

    #[wasm_bindgen_test]
    fn persist_wasm_writes_to_storage() {
        clear_theme();
        Theme::Light.persist();
        assert_eq!(get_theme().as_deref(), Some("light"));
        Theme::Dark.persist();
        assert_eq!(get_theme().as_deref(), Some("dark"));
        clear_theme();
    }
}

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
}

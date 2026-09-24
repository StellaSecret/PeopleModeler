#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

// Persisted "last successful Drive backup" timestamp (ms since epoch), stored
// with the same platform-aware scheme as the auth token: localStorage key on
// wasm, a dotfile next to the token on native (cwd / Android files dir).

const LAST_BACKUP_KEY: &str = "pm_last_backup_ms";

#[cfg(not(target_arch = "wasm32"))]
fn path() -> Option<PathBuf> {
    crate::auth::state_path(LAST_BACKUP_KEY)
}

pub fn last_backup_ms() -> Option<i64> {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        return gloo_storage::LocalStorage::get(LAST_BACKUP_KEY).ok();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let p = path()?;
        std::fs::read_to_string(&p)
            .ok()
            .and_then(|s| s.trim().parse::<i64>().ok())
    }
}

pub fn set_last_backup_ms(ms: i64) {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_storage::Storage;
        let _ = gloo_storage::LocalStorage::set(LAST_BACKUP_KEY, ms);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(p) = path() {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = std::fs::write(&p, ms.to_string());
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn last_backup_roundtrip() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        let _ = std::env::set_current_dir(dir.path());
        assert_eq!(last_backup_ms(), None);
        set_last_backup_ms(12345);
        assert_eq!(last_backup_ms(), Some(12345));
        let _ = std::env::set_current_dir(&orig);
    }
}

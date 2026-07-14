use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static JVM: OnceLock<jni::JavaVM> = OnceLock::new();
static FILES_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Set to true by the JNI callback when the token file is written.
pub(crate) static TOKEN_SAVED: AtomicBool = AtomicBool::new(false);

/// Returns the app's internal files directory, used for token storage.
pub(crate) fn get_files_dir() -> Option<&'static std::path::Path> {
    FILES_DIR.get().map(|p| p.as_path())
}

/// Called from Kotlin `GoogleDriveHelper.nativeInit()` to store JVM reference and files dir.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_GoogleDriveHelper_nativeInit(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    files_dir: jni::objects::JString,
) {
    if let Ok(jvm) = env.get_java_vm() {
        JVM.set(jvm).ok();
        eprintln!("[android_auth] JVM stored");
    } else {
        eprintln!("[android_auth] FAILED to get JVM from env");
    }
    if let Ok(path) = env.get_string(&files_dir) {
        let path_str: String = path.into();
        FILES_DIR.set(PathBuf::from(path_str.clone())).ok();
        eprintln!("[android_auth] filesDir: {path_str}");
    } else {
        eprintln!("[android_auth] FAILED to get filesDir string");
    }
}

/// Called from Kotlin `GoogleDriveHelper.nativeOnTokenSaved()` after token file written.
/// Sets a global flag so the UI can pick up the change.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_GoogleDriveHelper_nativeOnTokenSaved(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    eprintln!("[android_auth] token saved callback received");
    TOKEN_SAVED.store(true, Ordering::Release);
}

/// Called from `auth::start_oauth()` to trigger native Google Sign-In.
pub fn start_sign_in() {
    eprintln!("[android_auth] start_sign_in called");
    match JVM.get() {
        Some(jvm) => {
            eprintln!("[android_auth] JVM found, attaching thread");
            match jvm.attach_current_thread() {
                Ok(mut env) => {
                    eprintln!(
                        "[android_auth] thread attached, calling GoogleDriveHelper.startSignIn"
                    );
                    match env.call_static_method(
                        "com/stellasecret/peoplemodeler/GoogleDriveHelper",
                        "startSignIn",
                        "()V",
                        &[],
                    ) {
                        Ok(_) => eprintln!("[android_auth] startSignIn JNI call succeeded"),
                        Err(e) => {
                            // Clear pending Java exception so next JNI call doesn't crash
                            let _ = env.exception_clear();
                            eprintln!("[android_auth] startSignIn JNI call FAILED: {e:?}");
                        }
                    }
                }
                Err(e) => eprintln!("[android_auth] FAILED to attach thread: {e:?}"),
            }
        }
        None => eprintln!("[android_auth] JVM not initialized — was nativeInit() called?"),
    }
}

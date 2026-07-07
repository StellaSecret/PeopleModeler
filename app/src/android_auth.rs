use std::sync::{Mutex, OnceLock};

static JVM: OnceLock<jni::JavaVM> = OnceLock::new();
static TOKEN_RDY: OnceLock<Mutex<Option<tokio::sync::oneshot::Sender<()>>>> = OnceLock::new();

/// Called from Kotlin `GoogleDriveHelper.nativeInit()` to store JVM reference.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_GoogleDriveHelper_nativeInit(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    if let Ok(jvm) = env.get_java_vm() {
        JVM.set(jvm).ok();
        eprintln!("[android_auth] JVM stored");
    } else {
        eprintln!("[android_auth] FAILED to get JVM from env");
    }
}

/// Called from Kotlin `GoogleDriveHelper.nativeOnTokenSaved()` after token file written.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_GoogleDriveHelper_nativeOnTokenSaved(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    eprintln!("[android_auth] token saved callback received");
    if let Some(lock) = TOKEN_RDY.get() {
        if let Ok(mut guard) = lock.lock() {
            if let Some(tx) = guard.take() {
                let _ = tx.send(());
            }
        }
    }
}

/// Register a oneshot sender to be notified when token is saved.
/// Only one sender at a time; previous is dropped.
pub fn set_token_callback(tx: tokio::sync::oneshot::Sender<()>) {
    let lock = TOKEN_RDY.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = lock.lock() {
        *guard = Some(tx);
    }
}

/// Called from `auth::start_oauth()` to trigger native Google Sign-In.
pub fn start_sign_in() {
    eprintln!("[android_auth] start_sign_in called");
    match JVM.get() {
        Some(jvm) => {
            eprintln!("[android_auth] JVM found, attaching thread");
            match jvm.attach_current_thread() {
                Ok(mut env) => {
                    eprintln!("[android_auth] thread attached, calling GoogleDriveHelper.startSignIn");
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

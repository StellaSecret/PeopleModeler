use std::sync::{Mutex, OnceLock};

static JVM: OnceLock<jni::JavaVM> = OnceLock::new();
static IMPORT_RDY: OnceLock<Mutex<Option<tokio::sync::oneshot::Sender<String>>>> = OnceLock::new();

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_FileShareHelper_nativeInit(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    if let Ok(jvm) = env.get_java_vm() {
        JVM.set(jvm).ok();
        eprintln!("[android_share] JVM stored");
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_FileShareHelper_nativeOnImportReady(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    eprintln!("[android_share] import ready callback");
    let path = "/data/data/com.stellasecret.peoplemodeler/files/.pm_import_data";
    let content = std::fs::read_to_string(path).unwrap_or_default();
    if content.is_empty() {
        eprintln!("[android_share] import data file empty");
    }
    if let Some(lock) = IMPORT_RDY.get() {
        if let Ok(mut guard) = lock.lock() {
            if let Some(tx) = guard.take() {
                let _ = tx.send(content);
            }
        }
    }
}

pub fn share_export_file(json: &str) {
    eprintln!("[android_share] share_export_file called");
    let Some(jvm) = JVM.get() else {
        eprintln!("[android_share] JVM not initialized");
        return;
    };
    let Ok(mut env) = jvm.attach_current_thread() else {
        eprintln!("[android_share] attach thread FAILED");
        return;
    };
    let j_json = env.new_string(json).expect("new_string failed");
    if env
        .call_static_method(
            "com/stellasecret/peoplemodeler/FileShareHelper",
            "launchExport",
            "(Ljava/lang/String;)V",
            &[(&j_json).into()],
        )
        .is_err()
    {
        let _ = env.exception_clear();
        eprintln!("[android_share] launchExport JNI call FAILED");
    }
}

pub fn start_import() {
    eprintln!("[android_share] start_import called");
    let Some(jvm) = JVM.get() else {
        eprintln!("[android_share] JVM not initialized");
        return;
    };
    let Ok(mut env) = jvm.attach_current_thread() else {
        eprintln!("[android_share] attach thread FAILED");
        return;
    };
    if env
        .call_static_method(
            "com/stellasecret/peoplemodeler/FileShareHelper",
            "launchImport",
            "()V",
            &[],
        )
        .is_err()
    {
        let _ = env.exception_clear();
        eprintln!("[android_share] launchImport JNI call FAILED");
    }
}

pub fn set_import_callback(tx: tokio::sync::oneshot::Sender<String>) {
    let lock = IMPORT_RDY.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = lock.lock() {
        *guard = Some(tx);
    }
}

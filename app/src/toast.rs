use dioxus::prelude::*;

pub fn provide_toast() -> Signal<Option<String>> {
    let s = use_signal(|| None::<String>);
    use_context_provider(|| s);
    s
}

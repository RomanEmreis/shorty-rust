use leptos::prelude::window;

pub fn copy_to_clipboard(value: &str) {
    let _ = window()
        .navigator()
        .clipboard()
        .write_text(value);
}
use crate::components::App;

pub mod components;
pub mod utils;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App)
}

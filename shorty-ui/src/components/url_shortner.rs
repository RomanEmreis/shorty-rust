use crate::utils::copy_to_clipboard;
use std::rc::Rc;
use self::{api::ShortyApi, state::UrlInputState};
use leptos::{
    ev::{MouseEvent, KeyboardEvent, FocusEvent},
    prelude::*
};

mod state;
mod api;

#[component]
pub fn UrlShortner() -> impl IntoView {
    let state = Rc::new(UrlInputState::new());
    let api = Rc::new(ShortyApi::new()); 
    let url = state.url();
    let error = state.error();
    let input = state.input();
        
    let create_url = Action::new_local(
        move |_| create_short_url(api.clone(), state.clone()));
    
    let on_submit = move |ev: MouseEvent| {
        ev.prevent_default();
        create_url.dispatch(());
    };

    let on_key_up = move |ev: KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }
        ev.prevent_default();
        create_url.dispatch(());
    };
    
    let on_focus = move |ev: FocusEvent| {
        ev.prevent_default();
        input
            .get()
            .expect("input should be mounted")
            .select();
    };
    
    let on_copy_to_clipboard = move |ev: MouseEvent| {
        ev.prevent_default();
        copy_to_clipboard(url.get().as_str());
    };

    view! {
        <div class="center-screen">
            <div class="url-input">
                <input type="text"
                    placeholder="https://www.very-long-url.com"
                    value=move || url.get()
                    node_ref=input
                    on:focus=on_focus
                    on:keyup=on_key_up/>
                <button class="generate-btn" on:click=on_submit>
                    <img alt="generate-img" src="/static/icons/generate.svg" width="38" height="38" />
                </button>
            </div>
            <Show when=move || !error.get().is_empty()>
                <div class="error-field">
                    <span>{move || error.get()}</span>
                </div>
            </Show>
            <Show when=move || !url.get().is_empty()>
                <div class="result-field">
                    <a href={move || url.get()} target="_blank">{move || url.get()}</a>
                    <button class="copy-btn" on:click=on_copy_to_clipboard>
                        <img alt="copy-img" src="/static/icons/copy.svg" width="38" height="38" />
                    </button>
                </div>
            </Show>
            <div class="copyright">Created by Roman Emreis</div>
        </div>
    }
}

async fn create_short_url(api: Rc<ShortyApi>, state: Rc<UrlInputState>) {
    if let Some(value) = state.validate() {
        match api.create_url(value).await {
            Ok(url) => state.set_url(url),
            Err(err) => state.set_error(err.to_string()),
        }
    }
}
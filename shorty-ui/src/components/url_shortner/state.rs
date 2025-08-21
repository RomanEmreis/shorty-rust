use leptos::{prelude::*, html::Input};

pub(super) struct UrlInputState {
    input_element: NodeRef<Input>,
    url: RwSignal<String>,
    error: RwSignal<String>,
}

impl UrlInputState {
    pub(super) fn new() -> Self {
        Self {
            input_element: NodeRef::new(),
            url: RwSignal::new(String::new()),
            error: RwSignal::new(String::new()),
        }
    }
    
    pub(super) fn url(&self) -> RwSignal<String> {
        self.url
    }
    
    pub(super) fn error(&self) -> RwSignal<String> {
        self.error
    }
    
    pub(super) fn input(&self) -> NodeRef<Input> {
        self.input_element
    }

    pub(super) fn set_url(&self, url: String) {
        self.url.set(url);
        self.error.set(String::new());
    }

    pub(super) fn set_error(&self, error: String) {
        self.error.set(error);
        self.url.set(String::new());
    }

    pub(super) fn validate(&self) -> Option<String> {
        self.input_element
            .get()
            .map(|v| v.value())
            .and_then(|value| {
                match reqwest::Url::parse(&value) {
                    Ok(_) => Some(value),
                    Err(err) => {
                        self.set_error(err.to_string());
                        None
                    }
                }
            })
    }
}
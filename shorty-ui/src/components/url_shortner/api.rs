use leptos::serde_json::json;
use reqwest::{Client, Error};

pub(super) struct ShortyApi {
    api_base: String,
    create_api: String,
    client: Client
}

impl ShortyApi {
    pub(super) fn new() -> Self {
        let api_base = std::env::var("API_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".into());
        Self {
            create_api: format!("{api_base}/create"),
            client: Client::new(),
            api_base,
        }
    }
    
    pub(super) async fn create_url(&self, url: String) -> Result<String, Error> {
        let payload = json!({ "url": url });
        self.client
            .post(&self.create_api)
            .json(&payload).send()
            .await?
            .json::<String>().await
            .map(|s| format!("{}/{s}", self.api_base))
    }
}


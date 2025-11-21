use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_valkey::Client;
use volga::error::Error;

pub(crate) struct Cache {
    client: Arc<Mutex<Client>>,
}

impl Cache {
    pub(crate) async fn new() -> Self {
        let cache_url = std::env::var("CACHE_URL")
            .expect("CACHE_URL must be set");

        let client = Client::connect(cache_url)
            .await
            .expect("Cache init failed");
        Self { client: Arc::new(Mutex::new(client)) }
    }

    #[inline]
    pub(crate) async fn get(&self, key: &str) -> Result<Option<String>, Error> {
        let mut guard = self.client.lock().await;
        guard.get(key)
            .await
            .map_err(|err| Error::server_error(err))
    }

    #[inline]
    pub(crate) async fn set(&self, key: &str, value: &str) -> Result<(), Error> {
        let mut guard = self.client.lock().await;
        guard.set(key, value)
            .await
            .map_err(|err| Error::server_error(err))
    }
}
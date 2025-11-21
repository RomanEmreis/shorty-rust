use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_valkey::Client;
use volga::error::Error;

pub(crate) struct Cache {
    url: Option<String>,
    client: Option<Arc<Mutex<Client>>>,
}

impl Cache {
    pub(crate) fn new() -> Self {
        let cache_url = std::env::var("CACHE_URL")
            .expect("CACHE_URL must be set");

        Self { client: None, url: Some(cache_url) }
    }

    pub(crate) async fn connect(mut self) -> Self {
        let url = self.url.take()
            .expect("Already connected to the cache");

        let client = Client::connect(url)
            .await
            .expect("Cache init failed");

        self.client = Some(Arc::new(Mutex::new(client)));
        self
    }

    #[inline]
    pub(crate) async fn get(&self, key: &str) -> Result<Option<String>, Error> {
        let mut guard = self.client
            .as_ref()
            .ok_or_else(CacheError::connection_lost)?
            .lock()
            .await;

        guard.get(key)
            .await
            .map_err(CacheError::query_error)
    }

    #[inline]
    pub(crate) async fn set(&self, key: &str, value: &str) -> Result<(), Error> {
        let mut guard = self.client
            .as_ref()
            .ok_or_else(CacheError::connection_lost)?
            .lock()
            .await;

        guard.set(key, value)
            .await
            .map_err(CacheError::query_error)
    }
}

pub(crate) struct CacheError;
impl CacheError {
    #[inline]
    pub(crate) fn query_error(err: tokio_valkey::Error) -> Error {
        Error::server_error(format!("Query error: {err}"))
    }

    #[inline]
    pub(crate) fn connection_lost() -> Error {
        Error::server_error("Connection lost".to_string())
    }
}
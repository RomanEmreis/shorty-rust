use std::sync::Arc;
use crate::token::MIN_VALUE;
use crate::cache::Cache;
use volga::{
    di::{Container, Inject, error::Error as DiError},
    error::Error
};

const COUNT_KEY: &str = "cnt";

pub(crate) struct Counter {
    cache: Arc<Cache>
}

impl Inject for Counter {
    #[inline]
    fn inject(container: &Container) -> Result<Self, DiError> {
        let cache = container.resolve_shared::<Cache>()?;
        Ok(Self { cache })
    }
}

impl Counter {
    pub(crate) async fn increment(&self) -> Result<u64, Error> {
        let cnt = 1 + if let Some(cnt) = self.cache.get(COUNT_KEY).await? {
            cnt.parse::<u64>()
                .map_err(|_| Error::server_error("Invalid cache value"))?
        } else {
            MIN_VALUE
        };
        self.cache.set(COUNT_KEY, &cnt.to_string()).await?;
        Ok(cnt)
    }
}
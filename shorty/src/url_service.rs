use std::sync::Arc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use volga::{
    di::{Inject, Container, error::Error as DiError},
    error::Error
};
use crate::{
    db::{DbContext, DbError},
    cache::Cache,
    counter::Counter,
    models::ShortUrl,
    schema::shorty_urls,
    token::Token
};

pub(crate) struct UrlService {
    db: Arc<DbContext>,
    cache: Arc<Cache>,
    counter: Arc<Counter>
}

impl Inject for UrlService {
    #[inline]
    fn inject(container: &Container) -> Result<Self, DiError> {
        let db = container.resolve_shared::<DbContext>()?;
        let counter = container.resolve_shared::<Counter>()?;
        let cache = container.resolve_shared::<Cache>()?;
        Ok(Self { db, cache, counter })
    }
}

impl UrlService {
    pub(crate) async fn create_short_url(&self, url: String) -> Result<ShortUrl, Error> {
        let count = self.counter.increment().await?;
        let token = Token::new(count)?;
        let record = ShortUrl::new(url, token);

        let mut conn = self.db.get_connection().await?;
        let _res = diesel::insert_into(shorty_urls::table)
            .values(&record)
            .returning(ShortUrl::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(DbError::query_error)?;

        self.cache.set(&record.token, &record.url).await?;

        Ok(record)
    }

    pub(crate) async fn get_short_url(&self, token: String) -> Result<Option<String>, Error> {
        if let Some(url) = self.cache.get(&token).await? {
            return Ok(Some(url));
        }

        let mut conn = self.db.get_connection().await?;
        let res: Vec<String> = shorty_urls::table
            .filter(shorty_urls::token.eq(&token))
            .limit(1)
            .select(shorty_urls::url)
            .load(&mut conn)
            .await
            .map_err(DbError::query_error)?;
        
        Ok(res.into_iter().next())
    }
}

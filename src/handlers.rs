use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use volga::{
    HttpResult, 
    Json, 
    error::Error, 
    di::Dc, 
    ok, status, redirect, problem
};

use crate::{
    db::{DbContext, DbError},
    schema::shorty_urls,
    models::ShortUrl,
    counter::Counter,
    token::Token
};

#[derive(serde::Deserialize)]
pub(crate) struct NewUrl { 
    pub url: String,
}

pub(crate) async fn create_url(
    Json(new_url): Json<NewUrl>,
    mut counter: Dc<Counter>,
    db_ctx: Dc<DbContext>
) -> HttpResult {
    let count = counter.increment();
    let token = Token::new(count)?;
    
    let record = ShortUrl::new(new_url.url, token);
    
    let mut conn = db_ctx.get_connection().await?;
    let _res = diesel::insert_into(shorty_urls::table)
        .values(&record)
        .returning(ShortUrl::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(DbError::query_error)?;
    
    ok!(record.token)
}

pub(crate) async fn get_url(token: String, db_ctx: Dc<DbContext>) -> HttpResult {
    let mut conn = db_ctx.get_connection().await?;
    
    let res: Vec<String> = shorty_urls::table
        .filter(shorty_urls::token.eq(&token))
        .limit(1)
        .select(shorty_urls::url)
        .load(&mut conn)
        .await
        .map_err(DbError::query_error)?;
    
    if res.is_empty() {
        tracing::trace!("no url found for token: {}", token);
        status!(404)
    } else {
        redirect!(&res[0])
    }
}

pub(crate) async fn error(err: Error) -> HttpResult {
    tracing::error!("{:?}", err);
    let (status, instance, err) = err.into_parts();
    problem! {
        "status": status.as_u16(),
        "detail": (err.to_string()),
        "instance": instance,
    }
}
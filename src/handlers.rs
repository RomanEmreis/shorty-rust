use chrono::Local;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use volga::{HttpResult, Json, di::Dc, ok, status, redirect};

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
    let mut conn = db_ctx.get_connection().await?;
    
    let count = counter.increment();
    let token = Token::new(count)?.to_string();
    
    let record = ShortUrl { 
        url: new_url.url, 
        created_at: Local::now().naive_local(),
        token
    };
    
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
        .filter(shorty_urls::token.eq(token))
        .limit(1)
        .select(shorty_urls::url)
        .load(&mut conn)
        .await
        .map_err(DbError::query_error)?;
    
    if res.is_empty() { 
        status!(404)
    } else {
        redirect!(&res[0])
    }
}
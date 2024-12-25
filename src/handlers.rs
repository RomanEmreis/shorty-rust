use chrono::Local;
use diesel::prelude::*;
use volga::{ok, HttpResult, Json};
use volga::di::Dc;

use crate::DbContext;
use crate::models::ShortUrl;
use crate::schema::shorty_urls;
use crate::token::Token;

#[derive(serde::Deserialize)]
pub struct NewUrl { 
    pub url: String,
}

pub async fn create_url(new_url: Json<NewUrl>, db_ctx: Dc<DbContext>) -> HttpResult {
    let mut conn = db_ctx.get_connection();

    let token = Token::new(57_000_000_000).to_string(); 
    let record = ShortUrl { 
        url: new_url.url.clone(), 
        created_at: Local::now().naive_local(),
        token
    };
    
    let _res = diesel::insert_into(shorty_urls::table)
        .values(&record)
        .returning(ShortUrl::as_returning())
        .get_result(&mut conn)
        .unwrap();
    
    ok!()
}

pub async fn get_url(token: String, db_ctx: Dc<DbContext>) -> HttpResult {
    let mut conn = db_ctx.get_connection();
    
    let res = shorty_urls::table
        .filter(shorty_urls::token.eq(token))
        .limit(1)
        .select(ShortUrl::as_select())
        .load(&mut conn)
        .unwrap();
    
    ok!(&res[0].url)
}
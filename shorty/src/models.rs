use diesel::prelude::*;
use chrono::{NaiveDateTime, Local};
use crate::token::Token;

#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::shorty_urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortUrl {
    pub token: String,
    pub url: String,
    pub created_at: NaiveDateTime
}

impl ShortUrl {
    pub(super) fn new(url: String, token: Token) -> Self {
        Self { 
            created_at: Local::now().naive_local(), 
            token: token.into(),
            url
        }
    }
}
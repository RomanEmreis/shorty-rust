use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::shorty_urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortUrl {
    pub token: String,
    pub url: String,
    pub created_at: NaiveDateTime
}
use crate::url_service::UrlService;
use volga::{
    HttpResult, Json,
    ok, status, redirect,
    di::Dc,
};

#[derive(serde::Deserialize)]
pub(crate) struct NewUrl {
    url: String,
}

pub(crate) async fn create_url(Json(new_url): Json<NewUrl>, svc: Dc<UrlService>) -> HttpResult {
    let record = svc.create_short_url(new_url.url).await?;
    ok!(record.token)
}

pub(crate) async fn get_url(token: String, svc: Dc<UrlService>) -> HttpResult {
    let res = svc.get_short_url(token).await?;
    res.map_or_else(
        || status!(404),
        |url| redirect!(url)
    )
}

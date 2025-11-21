use crate::url_service::UrlService;
use volga::{
    HttpResult, Json,
    ok, status, redirect, problem,
    error::Error,
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

pub(crate) async fn error(err: Error) -> HttpResult {
    tracing::error!("{:?}", err);
    let (status, instance, err) = err.into_parts();
    problem! {
        "status": status.as_u16(),
        "detail": (err.to_string()),
        "instance": instance,
    }
}
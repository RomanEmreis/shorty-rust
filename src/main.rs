use volga::App;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager}, 
    AsyncPgConnection
};

pub(crate) mod db;
pub(crate) mod token;
pub(crate) mod handlers;
pub(crate) mod counter;
pub mod schema;
pub mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new().bind("0.0.0.0:8080");
    
    let db_ctx = build_db_context().await?;
    
    app.register_singleton(db_ctx);
    app.register_singleton(counter::Counter::default());
    
    app.map_get("/{token}", handlers::get_url);
    app.map_post("/create", handlers::create_url);
    
    app.run().await
}

async fn build_db_context() -> std::io::Result<db::DbContext> {
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(config)
        .await
        .map_err(db::DbError::pool_error)?;

    Ok(db::DbContext::new(pool))
}

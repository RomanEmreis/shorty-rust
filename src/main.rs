use volga::App;
use tracing_subscriber::prelude::*;

pub(crate) mod db;
pub(crate) mod token;
pub(crate) mod handlers;
pub(crate) mod counter;
pub(crate) mod schema;
pub(crate) mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,volga=debug,diesel=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let mut app = App::new()
        .bind("0.0.0.0:8080")
        .with_tracing(|tracing| tracing.with_header());
    
    let db_ctx = db::DbContext::new()
        .create_pool()
        .await;
    
    app
        .add_singleton(db_ctx)
        .add_singleton(counter::Counter::default());
    
    app.use_tracing()
        .map_get("/{token}", handlers::get_url)
        .map_post("/create", handlers::create_url)
        .map_err(handlers::error);
    
    app.run().await
}

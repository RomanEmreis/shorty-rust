use volga::{App, tracing::TracingConfig};
use tracing_subscriber::prelude::*;

pub(crate) mod db;
pub(crate) mod token;
pub(crate) mod handlers;
pub(crate) mod counter;
pub mod schema;
pub mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let mut app = App::new()
        .bind("0.0.0.0:8080")
        .with_tracing(TracingConfig::new()
            .with_header());
    
    let db_ctx = db::DbContext::new().await?;
    
    app
        .add_singleton(db_ctx)
        .add_singleton(counter::Counter::default());
    
    app.use_tracing()
        .map_get("/{token}", handlers::get_url)
        .map_post("/create", handlers::create_url);
    
    app.run().await
}

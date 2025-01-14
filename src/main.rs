use volga::App;

pub(crate) mod db;
pub(crate) mod token;
pub(crate) mod handlers;
pub(crate) mod counter;
pub mod schema;
pub mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new().bind("0.0.0.0:8080");
    
    let db_ctx = db::DbContext::new().await?;
    
    app
        .add_singleton(db_ctx)
        .add_singleton(counter::Counter::default());
    
    app
        .map_get("/{token}", handlers::get_url)
        .map_post("/create", handlers::create_url);
    
    app.run().await
}

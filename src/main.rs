use volga::App;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager}, 
    AsyncPgConnection
};

pub(crate) mod db;
pub(crate) mod token;
pub(crate) mod handlers;
pub mod schema;
pub mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new().bind("0.0.0.0:8080");

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    
    // set up connection pool
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder().build(config).await.unwrap();
    app.register_singleton(db::DbContext::new(pool));

    app.map_get("/{token}", handlers::get_url);
    app.map_post("/create", handlers::create_url);
    
    app.run().await
}

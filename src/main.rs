use volga::App;

use diesel::prelude::*;

pub mod schema;
pub mod handlers;
pub mod models;
pub mod token;

#[derive(Default, Clone)]
pub struct DbContext;

impl DbContext {
    fn get_connection(&self) -> PgConnection {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        PgConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new().bind("0.0.0.0:8080");

    app.register_singleton(DbContext);

    app.map_get("/{token}", handlers::get_url);
    app.map_post("/create", handlers::create_url);
    
    app.run().await
}

use volga::{error::Error, di::{Container, Inject}};
use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection, RunError},
    pooled_connection::AsyncDieselConnectionManager,
    AsyncPgConnection
};

pub(crate) struct DbContext {
    pool: Option<Pool<AsyncPgConnection>>, 
    connection_string: String,
}

impl Inject for DbContext {
    fn inject(_: &Container) -> Result<Self, Error> {
        Ok(Self::new())
    }
}

impl DbContext {
    pub(crate) fn new() -> DbContext{
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self { connection_string: db_url, pool: None }
    }
    
    pub(crate) async fn create_pool(mut self) -> Self {
        if self.pool.is_some() {
            return self;
        }
        
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&self.connection_string);
        let pool = Pool::builder()
            .build(config)
            .await
            .expect("Unable to establish connection to database");
        self.pool = Some(pool);
        self   
    }
    
    pub(crate) async fn get_connection(&self) -> Result<PooledConnection<AsyncPgConnection>, Error> {
        self.pool
            .as_ref()
            .ok_or_else(DbError::connection_lost)?
            .get()
            .await
            .map_err(DbError::connection_error)
    }
}

pub(crate) struct DbError;

impl DbError {
    pub(crate) fn connection_error(err: RunError) -> Error {
        Error::server_error(format!("DB connection error: {}", err))
    }

    pub(crate) fn query_error(err: diesel::result::Error) -> Error {
        Error::server_error(format!("Query error: {}", err))
    }
    
    pub(crate) fn connection_lost() -> Error {
        Error::server_error("Connection lost".to_string())
    }
}
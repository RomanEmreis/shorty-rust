use volga::{error::Error, di::{Container, Inject}};
use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection, RunError},
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
    AsyncPgConnection
};

pub(crate) struct DbContext {
    pool: Pool<AsyncPgConnection>
}

impl Inject for DbContext {
    async fn inject(_: &Container) -> Result<Self, Error> {
        Self::new().await
    }
}

impl Clone for DbContext {
    fn clone(&self) -> Self {
        Self { pool: self.pool.clone() }
    }
}

impl DbContext {
    pub(crate) async fn new() -> Result<DbContext, Error> {
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder()
            .build(config)
            .await
            .map_err(DbError::pool_error)?;

        Ok(Self { pool })
    }
    
    pub(crate) async fn get_connection(&self) -> Result<PooledConnection<AsyncPgConnection>, Error> {
        self.pool.get()
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

    pub(crate) fn pool_error(err: PoolError) -> Error {
        Error::server_error(format!("Query error: {}", err))
    }
}
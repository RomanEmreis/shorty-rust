use std::io::{Error, ErrorKind};
use volga::di::{Container, Inject};
use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection, RunError},
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
    AsyncPgConnection
};

pub(crate) struct DbContext {
    pool: Pool<AsyncPgConnection>
}

impl Inject for DbContext {
    async fn inject(_: &mut Container) -> Result<Self, Error> {
        Self::new().await
    }
}

impl Clone for DbContext {
    fn clone(&self) -> Self {
        Self { pool: self.pool.clone() }
    }
}

impl DbContext {
    pub(crate) async fn new() -> std::io::Result<DbContext> {
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not set");

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder()
            .build(config)
            .await
            .map_err(DbError::pool_error)?;

        Ok(Self { pool })
    }

    pub(crate) async fn get_connection(&self) -> std::io::Result<PooledConnection<AsyncPgConnection>> {
        self.pool.get_owned()
            .await
            .map_err(DbError::connection_error)
    }
}

pub(crate) struct DbError;

impl DbError {
    pub(crate) fn connection_error(err: RunError) -> Error {
        Error::new(ErrorKind::Other, format!("DB connection error: {}", err))
    }

    pub(crate) fn query_error(err: diesel::result::Error) -> Error {
        Error::new(ErrorKind::Other, format!("Query error: {}", err))
    }

    pub(crate) fn pool_error(err: PoolError) -> Error {
        Error::new(ErrorKind::Other, format!("Query error: {}", err))
    }
}
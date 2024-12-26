use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection, RunError},
    pooled_connection::PoolError,
    AsyncPgConnection
};

pub(crate) struct DbContext {
    pool: Pool<AsyncPgConnection>
}

impl Default for DbContext {
    fn default() -> Self {
        unreachable!()
    }
}

impl Clone for DbContext {
    fn clone(&self) -> Self {
        Self { pool: self.pool.clone() }
    }
}

impl DbContext {
    pub(crate) fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }

    pub(crate) async fn get_connection(&self) -> std::io::Result<PooledConnection<AsyncPgConnection>> {
        self.pool.get_owned()
            .await
            .map_err(DbError::connection_error)
    }
}

pub(crate) struct DbError;

impl DbError {
    pub(crate) fn connection_error(err: RunError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, format!("DB connection error: {}", err))
    }

    pub(crate) fn query_error(err: diesel::result::Error) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Query error: {}", err))
    }

    pub(crate) fn pool_error(err: PoolError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Query error: {}", err))
    }
}
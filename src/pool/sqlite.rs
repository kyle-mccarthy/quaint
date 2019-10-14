use crate::{
    connector::{Queryable, Sqlite, DBIO},
    error::Error,
};
use tokio_resource_pool::{Manage, Status, RealDependencies, CheckOut};
use std::path::PathBuf;
use futures::future;

pub struct SqliteManager {
    file_path: PathBuf,
}

impl SqliteManager {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

impl Manage for SqliteManager {
    type Resource = Sqlite;
    type Dependencies = RealDependencies;
    type CheckOut = CheckOut<Self>;
    type Error = Error;
    type CreateFuture = DBIO<'static, Self::Resource>;
    type RecycleFuture = DBIO<'static, Option<Self::Resource>>;

    fn create(&self) -> Self::CreateFuture {
        match Sqlite::new(self.file_path.clone()) {
            Ok(conn) => DBIO::new(future::ok(conn)),
            Err(e) => DBIO::new(future::err(e))
        }
    }

    fn status(&self, _: &Self::Resource) -> Status {
        Status::Valid
    }

    fn recycle(&self, connection: Self::Resource) -> Self::RecycleFuture {
        DBIO::new(async {
            connection.query_raw("SELECT 1", &[]).await?;
            Ok(Some(connection))
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_default_connection_limit() {
        let conn_string = format!("file:db/test.db",);
        let pool = crate::pool::sqlite(&conn_string).unwrap();

        assert_eq!(num_cpus::get_physical() * 2 + 1, pool.capacity());
    }

    #[test]
    fn test_custom_connection_limit() {
        let conn_string = format!("file:db/test.db?connection_limit=10",);
        let pool = crate::pool::sqlite(&conn_string).unwrap();

        assert_eq!(10, pool.capacity());
    }
}

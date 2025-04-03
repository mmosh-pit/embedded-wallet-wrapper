pub mod model;
use rocksdb::{DB, Options};
use r2d2::{ManageConnection, Pool, PooledConnection};
use std::sync::Arc;


/// Custom RocksDB Connection Manager for r2d2
pub struct RocksDbManager {
    path: String,
    options: Options,
}

impl RocksDbManager {
    pub fn new(path: &str) -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);
        Self {
            path: path.to_string(),
            options,
        }
    }
}

impl ManageConnection for RocksDbManager {
    type Connection = Arc<DB>;
    type Error = rocksdb::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let db = DB::open(&self.options, &self.path)?;
        Ok(Arc::new(db))
    }

    fn is_valid(&self, _: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

/// Type alias for the RocksDB connection pool
pub type RocksDbPool = Pool<RocksDbManager>;
pub type RocksDbConn = PooledConnection<RocksDbManager>;

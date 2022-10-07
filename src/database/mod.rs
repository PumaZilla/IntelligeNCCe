pub mod graphql;
pub mod models;
pub mod schema;

use crate::error::{Error, Result};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::time::Duration;

pub type DBConnection = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(address: &str) -> Result<DBConnection> {
    log::debug!("connecting to database at {}", address);
    Ok(Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .min_idle(Some(0))
        .idle_timeout(Some(Duration::from_secs(600)))
        .max_lifetime(Some(Duration::from_secs(30)))
        .connection_timeout(Duration::from_secs(30))
        .build(ConnectionManager::new(address))
        .map_err(|_| Error::DatabaseConnectionError(address.to_string()))?)
}

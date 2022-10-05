pub mod graphql;
pub mod models;
pub mod schema;

use crate::error::{Error, Result};

pub type Connection = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

pub fn establish_connection(address: &str) -> Result<Connection> {
    log::debug!("connecting to database at {}", address);
    Ok(diesel::r2d2::Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .min_idle(Some(0))
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .max_lifetime(Some(std::time::Duration::from_secs(30)))
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(diesel::r2d2::ConnectionManager::new(address))
        .map_err(|_| Error::DatabaseConnectionError(address.to_string()))?)
}
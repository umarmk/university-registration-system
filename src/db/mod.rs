use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use crate::config::DBPoolConfig;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(config: &DBPoolConfig) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(&config.url);
    r2d2::Pool::builder()
        .max_size(config.max_size)
        .min_idle(config.min_idle)
        .connection_timeout(config.connection_timeout)
        .build(manager)
        .expect("Failed to create pool")
}

pub mod repository;
pub mod error; 
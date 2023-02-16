use std::sync::Arc;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub mod movie;

pub type SharedState = Arc<AppState>;
pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct AppState {
    pub connection_pool: ConnectionPool,
}

use std::{net::SocketAddr, sync::Arc};

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;

use models::{AppState, SharedState};
use tokio_postgres::NoTls;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod data;
mod error;
mod extractors;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = init().await;

    let app = routes::get_router(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn init() -> SharedState {
    // set up connection pool
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost dbname=app user=postgres password=P4SSw0rd111",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    let shared_state = Arc::new(AppState {
        connection_pool: pool,
    });

    shared_state
}

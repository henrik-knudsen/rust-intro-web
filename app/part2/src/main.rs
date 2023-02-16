use std::net::SocketAddr;

mod data;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::get_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

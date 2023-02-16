use std::net::SocketAddr;

use axum::{extract::Path, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // TODO: Configure routes for greetings handler and foo.

    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// TODO: Implement a GET handler which returns a 'Hello world' string. Return type may need to be updated.
async fn hello_world() {
    todo!()
}

// TODO: Implement a GET handler which returns a 'Greetings, {name}' string, where name is extracted from the path. Return type and arguments may need to be updated.
// TIP: Path extractor.
async fn greetings() {
    todo!()
}

#[derive(Deserialize, Serialize)]
struct Foo {
    id: u32,
    name: String,
}

// TODO: Implement a POST handler which receives a Foo object (extract from Request) and returns a new Foo object with a new name (you decide!), serialized to Json. Return type and arguments may need to be updated.
// TIP: Path extractor.
async fn foo() {
    todo!()
}

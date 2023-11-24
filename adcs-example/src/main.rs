use std::{env, net::SocketAddr};

use axum::{routing::get, Router};
use dotenvy::dotenv;

async fn index() -> &'static str {
    "Hello, World!"
}

fn app() -> Router {
    Router::new().route("/", get(index))
}

#[tokio::main]
async fn main() {
    // load the .env file if it exists
    dotenv().ok();

    //let db = establish_connection().await;

    let addr: SocketAddr = env::var("SOCKET_ADDR")
        .expect("SOCKET_ADDR must be set")
        .parse()
        .expect("SOCKET_ADDR must be a valid address");

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

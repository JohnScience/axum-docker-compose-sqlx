use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
};

use axum::{extract::State, routing::get, Router};
use dotenvy::dotenv;
use sqlx::PgPool;

pub(crate) async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url)
        .await
        .unwrap_or_else(|e| panic!("Error connecting to {database_url}: {e}"));
    db
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn test_connection(State(db): State<PgPool>) -> &'static str {
    let result: i32 = sqlx::query_scalar!("SELECT 1 + 1 AS result")
        .fetch_one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result, 2);
    "Connection to the DB tested!"
}

fn app(db: PgPool) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/test_connection", get(test_connection))
        .with_state(db)
}

#[tokio::main]
async fn main() {
    // load the .env file if it exists
    dotenv().ok();

    let db: sqlx::Pool<sqlx::Postgres> = establish_connection().await;

    let addr: SocketAddr = {
        let mut iter = env::var("SOCKET_ADDR")
            .expect("SOCKET_ADDR must be set")
            .to_socket_addrs()
            .expect("Failed to get the list of socket addresses for SOCKET_ADDR");
        iter.next()
            .expect("Failed to get the first socket address for SOCKET_ADDR")
    };

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app(db).into_make_service())
        .await
        .unwrap();
}

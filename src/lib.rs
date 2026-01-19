mod handlers;
mod db;
use sqlx;
use handlers::*;
use axum::{handler, routing::{get, post}, Router};
use std::net::SocketAddr;

pub async fn new_app() {
    tracing_subscriber::fmt::init();
    let pool=db::pool::init_pool().await;
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/test",get(test_handler))
        .route("/get-user-by-id/{id}",get(get_user))
        .route("/login",post(login_handler))
        .route("/register",post(register_handler))
        .with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root_handler() -> &'static str {
    "Hello, Rust Backend!"
}
async fn test_handler()->&'static str{
    "Hello I am Test Route."
}

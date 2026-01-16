use axum::{
    routing::{get,post},
    Router,
    Json
};
use serde::{Deserialize,Serialize};
use std::net::SocketAddr;
#[derive(Deserialize, Debug)]
struct LoginRequest{
    username:String,
    password:String
}
#[derive(Serialize)]
struct LoginResponse {
    message: String,
    success: bool,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/test",get(test_handler))
        .route("/login",post(login_handler));

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
async fn login_handler(Json(payload):Json<LoginRequest>)->Json<LoginResponse> {
    println!("This is the payload : {:?}",payload);
    Json(LoginResponse {
        message: "Login Successful!".to_string(),
        success: true,
    })
}
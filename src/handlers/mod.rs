use axum::Json;
use serde::{Deserialize, Serialize};
use crate::db::get_user_by_id::{get_user_by_id, User};
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    success: bool,
}
#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    success: bool,
}
pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    println!("Login Attempt for user : {}", payload.username);
    Json(LoginResponse {
        message: "Login Successful".to_string(),
        success: true,
    })
}
pub async fn register_handler(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    println!("Registering User with username : {}", payload.username);
    Json(RegisterResponse {
        message: "Register Successful".to_string(),
        success: true,
    })
}
pub async fn get_user(Json(payload):Json<i32>)->User{
    let user=get_user_by_id(payload).await.unwrap();
    user
}

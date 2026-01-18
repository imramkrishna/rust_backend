use axum::Json;
use axum::{extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
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
pub struct GetUserResponse {
    message: String,
    success: bool,
}
#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    success: bool,
}
#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
    success: bool,
}
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
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
pub async fn get_user(
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<GetUserResponse>), (StatusCode, Json<ErrorMessage>)> {
    let pool = crate::db::pool::init_pool().await;
    println!("Getting User with id : {}", id);

    let result: Result<User, _> =
        sqlx::query_as("SELECT id, name, email, phone FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await;

    match result {
        Ok(user) => {
            println!("Found user: {:?}", user);
            Ok((
                StatusCode::OK,
                Json(GetUserResponse {
                    message: format!("Found user: {}", user.name),
                    success: true,
                }),
            ))
        }
        Err(e) => {
            println!("Database error for id {}: {:?}", id, e); // Log the actual error
            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorMessage {
                    message: "User not found".to_string(),
                    success: false,
                }),
            ))
        }
    }
}


use axum::Json;
use axum::{extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
}
#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    name: String,
    phone: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    user: User,
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
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
}
pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorMessage>)> {
    println!("Login Attempt for user : {}", payload.email);
    let result = sqlx::query_as("SELECT id, name, email, phone FROM users WHERE email = $1")
        .bind(payload.email)
        .fetch_optional(&pool)
        .await;
    match result {
        Ok(Some(user)) => Ok((
            StatusCode::OK,
            Json(LoginResponse {
                message: "Login Successful".to_string(),
                user,
                success: true,
            }),
        )),
        Ok(None) => Err((
            // No user with that email
            StatusCode::UNAUTHORIZED,
            Json(ErrorMessage {
                message: "Invalid Credentials".to_string(),
                success: false,
            }),
        )),
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorMessage {
                message: "Invalid Credentials".to_string(),
                success: false,
            }),
        )),
    }
}
pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), (StatusCode, Json<ErrorMessage>)> {
    println!("Registering User with username : {}", payload.name);
    let result = sqlx::query("INSERT INTO users(name,email,phone) VALUES($1,$2,$3)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.phone)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(RegisterResponse {
                message: "User Registered Successfully".to_string(),
                success: true,
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: format!("Error while registering user: {}", e),
                success: false,
            }),
        )),
    }
}
pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<GetUserResponse>), (StatusCode, Json<ErrorMessage>)> {
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

use sqlx::Pool;
use sqlx::Postgres;
use crate::db::pool::init_pool;

#[derive(Debug, sqlx::FromRow)]
pub struct User{
    id:i32,
    name:String,
    email:String,
    phone:String
}
pub async fn get_user_by_id(
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    match db_get_user(id, &crate::db::pool::get_pool()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
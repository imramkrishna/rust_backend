use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
pub async fn init_pool() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost:5432/rust").await.unwrap();
    println!("Connected to Database");
    pool
}
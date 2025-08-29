use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

pub async fn start_connection() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let migrate_check = sqlx::migrate!("./src/database/migrations")
        .run(&pool)
        .await;
    match migrate_check {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => println!("Migrations failed: {}", e),
    }

    pool
}

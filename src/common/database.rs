use sqlx::{query, sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        ).await.expect("Couldn't connect to DB");
    
    // create config table if it does not exist
    let create_session_sql = format!(
        "CREATE TABLE IF NOT EXISTS sessions (
            session_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            expire_at INTEGER NOT NULL
        );"
    );

    let create_users_sql = format!(
        "CREATE TABLE IF NOT EXISTS users (
            user_id TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            hashed_password TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE
        );"
    );

    query(&create_session_sql)
        .execute(&pool)
        .await
        .unwrap();

    query(&create_users_sql)
        .execute(&pool)
        .await
        .unwrap();
    
    pool
}
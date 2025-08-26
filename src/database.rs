//! # Database Module
//!
//! PostgreSQL database connection and pool management using SQLx.

use sqlx::{PgPool, Row};
use std::env;
use std::time::Duration;

/// Initialize the database connection pool
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    init_pool_with_url(None).await
}

/// Initialize the database connection pool with optional URL override
pub async fn init_pool_with_url(
    database_url_override: Option<&str>,
) -> Result<PgPool, sqlx::Error> {
    let database_url = match database_url_override {
        Some(url) => url.to_string(),
        None => {
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment or .env file")
        }
    };

    println!("🗄️  Connecting to PostgreSQL database...");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .connect(&database_url)
        .await?;

    println!("✅ Database connection pool established");
    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    println!("🔄 Running database migrations...");

    sqlx::migrate!("./migrations").run(pool).await?;

    println!("✅ Database migrations completed");
    Ok(())
}

/// Test database connectivity
pub async fn test_connection(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT 1 as test").fetch_one(pool).await?;

    let test_value: i32 = row.get("test");
    Ok(test_value == 1)
}

/// Get database connection info for health checks
pub async fn get_connection_info(pool: &PgPool) -> Result<DatabaseInfo, sqlx::Error> {
    let row = sqlx::query(
        "SELECT 
            version() as version,
            current_database() as database_name,
            current_user as username,
            NOW() as server_time",
    )
    .fetch_one(pool)
    .await?;

    Ok(DatabaseInfo {
        version: row.get("version"),
        database_name: row.get("database_name"),
        username: row.get("username"),
        server_time: row.get("server_time"),
        pool_connections: pool.size(),
        idle_connections: pool.num_idle(),
    })
}

/// Database information structure for health checks
#[derive(Debug, serde::Serialize)]
pub struct DatabaseInfo {
    pub version: String,
    pub database_name: String,
    pub username: String,
    pub server_time: chrono::DateTime<chrono::Utc>,
    pub pool_connections: u32,
    pub idle_connections: usize,
}

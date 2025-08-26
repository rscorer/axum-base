use axum::Router;
use axum_base::{auth::PasswordService, database, models::User};
use sqlx::PgPool;
use std::sync::Once;

static INIT: Once = Once::new();

pub struct TestDatabase {
    pub pool: PgPool,
}

impl TestDatabase {
    /// Initialize test database with migrations
    pub async fn new() -> Self {
        INIT.call_once(|| {
            // Set test environment
            unsafe {
                std::env::set_var("RUST_LOG", "debug");
            }
        });

        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/axum_base_test".to_string());

        let pool = database::init_pool_with_url(Some(&database_url))
            .await
            .expect("Failed to connect to test database");

        // Run migrations
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations on test database");

        Self { pool }
    }

    /// Clean all tables for test isolation
    pub async fn cleanup(&self) {
        // Clean tables in reverse dependency order
        let tables = [
            "trip_cards",
            "travelers",
            "dog_sitters",
            "sessions",
            "users",
            "categories",
        ];

        for table in &tables {
            // Use IF EXISTS to avoid errors if table doesn't exist
            let result = sqlx::query(&format!(
                "DO $$ 
                BEGIN 
                    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = '{}') THEN 
                        TRUNCATE TABLE {} RESTART IDENTITY CASCADE; 
                    END IF; 
                END $$;",
                table, table
            ))
            .execute(&self.pool)
            .await;

            if let Err(e) = result {
                eprintln!("Warning: Failed to clean table {}: {}", table, e);
            }
        }
    }

    /// Create a test user and return the User struct
    pub async fn create_test_user(&self, username: &str, email: &str, password: &str) -> User {
        let password_hash =
            PasswordService::hash_password(password).expect("Failed to hash password");

        // Use a regular query to avoid type conversion issues
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash, email_verified, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, false, true, NOW(), NOW())
             RETURNING id, username, email, password_hash, email_verified, is_active, 
                       created_at, updated_at, last_login"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .expect("Failed to create test user")
    }

    /// Create a testable Axum app instance with test database  
    /// This creates a test router with only API endpoints to avoid template issues
    pub async fn create_test_app(&self) -> Router {
        use axum::{Router, routing::get};
        use axum_base::api::{api_hello, health_check};
        use axum_base::web::handler_404;

        // Create a simplified router for testing that doesn't require templates
        // API endpoints should only return JSON, not HTML
        Router::new()
            .route("/health", get(health_check))
            .route("/api/hello", get(api_hello))
            .fallback(handler_404)
            .with_state(self.pool.clone())
    }
}

/// Test helper to verify JSON response structure
pub fn assert_json_response_structure(body: &str, expected_fields: &[&str]) {
    let json: serde_json::Value =
        serde_json::from_str(body).expect("Response should be valid JSON");

    for field in expected_fields {
        assert!(
            json.get(field).is_some(),
            "Response should contain field: {}. Got: {}",
            field,
            body
        );
    }
}

/// Create a default test .env configuration
pub fn setup_test_env() {
    unsafe {
        std::env::set_var("DATABASE_URL", "postgresql://localhost/axum_base_test");
        std::env::set_var("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test");
        std::env::set_var("PORT", "0"); // Use random available port for tests
    }
}

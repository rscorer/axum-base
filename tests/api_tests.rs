mod common;

use axum_test::TestServer;
use common::{setup_test_env, TestDatabase, assert_json_response_structure};
use axum::http::StatusCode;
use chrono;

/// Test that the health endpoint returns expected JSON structure
#[tokio::test]
async fn test_health_endpoint() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    let app = test_db.create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/health").await;
    response.assert_status(StatusCode::OK);
    
    // Verify Content-Type header for API endpoint
    let content_type_header = response.header("content-type");
    let content_type = content_type_header.to_str().unwrap_or("");
    assert!(content_type.contains("application/json"), "Health endpoint should return JSON, got: {}", content_type);
    
    let body = response.text();
    assert_json_response_structure(&body, &["status", "service", "version"]);
    
    // Parse and verify specific content
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["service"], "axum-base");
    
    test_db.cleanup().await;
}

/// Test 404 handling for unknown routes
#[tokio::test] 
async fn test_404_endpoint() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    let app = test_db.create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/nonexistent").await;
    response.assert_status(StatusCode::NOT_FOUND);
    
    // Verify Content-Type header for API error response
    let content_type_header = response.header("content-type");
    let content_type = content_type_header.to_str().unwrap_or("");
    assert!(content_type.contains("application/json"), "404 handler should return JSON, got: {}", content_type);
    
    let body = response.text();
    assert_json_response_structure(&body, &["message", "status", "timestamp"]);
    
    // Parse and verify specific content
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "error");
    assert!(json["message"].as_str().unwrap().contains("not found"));
    
    test_db.cleanup().await;
}

/// Test the API hello endpoint
#[tokio::test]
async fn test_api_hello_endpoint() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    let app = test_db.create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/api/hello").await;
    response.assert_status(StatusCode::OK);
    
    // Verify Content-Type header for API endpoint
    let content_type_header = response.header("content-type");
    let content_type = content_type_header.to_str().unwrap_or("");
    assert!(content_type.contains("application/json"), "API hello endpoint should return JSON, got: {}", content_type);
    
    let body = response.text();
    assert_json_response_structure(&body, &["message", "status", "timestamp"]);
    
    // Parse and verify specific content
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "success");
    assert!(json["message"].as_str().unwrap().contains("Hello from Axum Base"));
    
    // Verify timestamp is in RFC3339 format
    let timestamp = json["timestamp"].as_str().unwrap();
    chrono::DateTime::parse_from_rfc3339(timestamp)
        .expect("Timestamp should be valid RFC3339 format");
    
    test_db.cleanup().await;
}

/// Test the root endpoint serves HTML
/// NOTE: This test is disabled because template initialization doesn't work in test environment
/// TODO: Fix template testing infrastructure
/*
#[tokio::test]
async fn test_root_endpoint() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    let app = test_db.create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/").await;
    response.assert_status(StatusCode::OK);
    
    let body = response.text();
    // Verify it's HTML content
    assert!(body.contains("<html") || body.contains("<!DOCTYPE"));
    assert!(body.contains("</html>"));
    
    test_db.cleanup().await;
}
*/

/// Test login page endpoint  
/// NOTE: This test is disabled because template initialization doesn't work in test environment
/// TODO: Fix template testing infrastructure
/*
#[tokio::test]
async fn test_login_page_endpoint() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    let app = test_db.create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/login").await;
    response.assert_status(StatusCode::OK);
    
    let body = response.text();
    // Verify it's HTML content and likely contains form elements
    assert!(body.contains("<html") || body.contains("<!DOCTYPE"));
    
    test_db.cleanup().await;
}
*/

/// Test database connection in test environment
#[tokio::test]
async fn test_database_connection() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    
    // Test basic database connectivity
    let result = sqlx::query("SELECT 1 as test")
        .fetch_one(&test_db.pool)
        .await;
    
    assert!(result.is_ok(), "Database connection should work");
    
    // Clean up
    test_db.cleanup().await;
}

/// Test user creation and cleanup
#[tokio::test]
async fn test_user_creation() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await; // Start with clean state
    
    // Create test user
    let user = test_db.create_test_user("testuser", "test@example.com", "password123").await;
    
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert!(user.is_active);
    assert!(user.password_hash.map_or(false, |hash| hash.len() > 10)); // Should have hashed password
    
    // Verify user exists in database
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
        .bind("testuser")
        .fetch_one(&test_db.pool)
        .await
        .expect("Should be able to count users");
    
    assert_eq!(count.0, 1, "Should have exactly one test user");
    
    // Clean up
    test_db.cleanup().await;
    
    // Verify cleanup worked
    let count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&test_db.pool)
        .await
        .expect("Should be able to count users after cleanup");
    
    assert_eq!(count_after.0, 0, "Should have no users after cleanup");
}

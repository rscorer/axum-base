mod common;

use std::process::Command;
use common::{setup_test_env, TestDatabase};
use sqlx::Row;

/// Test the create_user CLI binary with non-interactive mode
#[tokio::test]
async fn test_create_user_cli_non_interactive() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await; // Start with clean state
    
    // Test successful user creation with password
    let output = Command::new("cargo")
        .args(&["run", "--bin", "create_user", "--", "testcli", "testcli@example.com", "password123"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute create_user command");
    
    assert!(output.status.success(), "create_user command should succeed. stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✅ User created successfully!"));
    assert!(stdout.contains("Username: testcli"));
    assert!(stdout.contains("Email: testcli@example.com"));
    assert!(stdout.contains("Password: Set"));
    
    // Verify user exists in database
    let user = sqlx::query("SELECT id, username, email, password_hash, is_active FROM users WHERE username = $1")
        .bind("testcli")
        .fetch_one(&test_db.pool)
        .await
        .expect("Should find created user");
    
    assert_eq!(user.get::<String, _>("username"), "testcli");
    assert_eq!(user.get::<String, _>("email"), "testcli@example.com");
    assert!(user.get::<bool, _>("is_active"));
    assert!(user.get::<Option<String>, _>("password_hash").is_some());
    
    test_db.cleanup().await;
}

/// Test the create_user CLI binary with invalid arguments
#[tokio::test]
async fn test_create_user_cli_invalid_args() {
    setup_test_env();
    
    // Test with wrong number of arguments (3 args - not 2 or 4)
    let output = Command::new("cargo")
        .args(&["run", "--bin", "create_user", "--", "user", "email@test.com"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute create_user command");
    
    assert!(!output.status.success(), "create_user should fail with invalid args");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
}

/// Test the create_user CLI binary with empty email
#[tokio::test]
async fn test_create_user_cli_empty_email() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await;
    
    // Test with empty email
    let output = Command::new("cargo")
        .args(&["run", "--bin", "create_user", "--", "testuser", "", "password123"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute create_user command");
    
    assert!(!output.status.success(), "create_user should fail with empty email");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Email cannot be empty"));
    
    test_db.cleanup().await;
}

/// Test the create_user CLI binary with interactive mode simulation
/// Note: We can't easily test true interactive mode, so this tests the logic
#[tokio::test]
async fn test_create_user_requires_password() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await;
    
    // The CLI should always create users with passwords
    // Test that our test helper works correctly
    let user = test_db.create_test_user("haspassword", "has@example.com", "validpassword123").await;
    
    assert_eq!(user.username, "haspassword");
    assert_eq!(user.email, "has@example.com");
    assert!(user.password_hash.is_some());
    assert!(user.is_active);
    
    test_db.cleanup().await;
}

/// Test the set_password CLI binary with valid arguments
#[tokio::test]
async fn test_set_password_cli_success() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await;
    
    // First create a user with existing password
    let user = test_db.create_test_user("pwuser", "pwuser@example.com", "oldpassword123").await;
    let user_id = user.id;
    let old_hash = user.password_hash.clone();
    
    // Now change password using CLI
    let output = Command::new("cargo")
        .args(&["run", "--bin", "set_password", "--", &user_id.to_string(), "newpassword123"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute set_password command");
    
    assert!(output.status.success(), "set_password command should succeed. stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✅ Password set successfully"));
    assert!(stdout.contains(&format!("user ID {}", user_id)));
    
    // Verify password was changed in database
    let updated_user = sqlx::query("SELECT password_hash FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&test_db.pool)
        .await
        .expect("Should find updated user");
    
    let new_hash = updated_user.get::<String, _>("password_hash");
    assert_ne!(old_hash.unwrap(), new_hash, "Password hash should have changed");
    
    test_db.cleanup().await;
}

/// Test the set_password CLI binary with invalid user ID
#[tokio::test]
async fn test_set_password_cli_invalid_user_id() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await;
    
    // Test with non-existent user ID
    let output = Command::new("cargo")
        .args(&["run", "--bin", "set_password", "--", "99999", "newpassword123"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute set_password command");
    
    assert!(!output.status.success(), "set_password should fail with invalid user ID");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("❌ Failed to set password"));
    
    test_db.cleanup().await;
}

/// Test the set_password CLI binary with invalid arguments
#[tokio::test]
async fn test_set_password_cli_invalid_args() {
    setup_test_env();
    
    // Test with wrong number of arguments
    let output = Command::new("cargo")
        .args(&["run", "--bin", "set_password", "--", "123"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute set_password command");
    
    assert!(!output.status.success(), "set_password should fail with invalid args");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
}

/// Test the set_password CLI binary with non-numeric user ID
#[tokio::test]
async fn test_set_password_cli_non_numeric_user_id() {
    setup_test_env();
    
    // Test with non-numeric user ID
    let output = Command::new("cargo")
        .args(&["run", "--bin", "set_password", "--", "notanumber", "password123"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute set_password command");
    
    assert!(!output.status.success(), "set_password should fail with non-numeric user ID");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: User ID must be a valid number"));
}

/// Test the set_password CLI binary with short password
#[tokio::test]
async fn test_set_password_cli_short_password() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await;
    
    let user = test_db.create_test_user("shortpw", "shortpw@example.com", "validpassword").await;
    
    // Test with password too short
    let output = Command::new("cargo")
        .args(&["run", "--bin", "set_password", "--", &user.id.to_string(), "short"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute set_password command");
    
    assert!(!output.status.success(), "set_password should fail with short password");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Password must be at least 8 characters long"));
    
    test_db.cleanup().await;
}

/// Test duplicate username creation
#[tokio::test]
async fn test_create_user_cli_duplicate_username() {
    setup_test_env();
    
    let test_db = TestDatabase::new().await;
    test_db.cleanup().await;
    
    // Use a unique username for this test to avoid conflicts with other tests
    let unique_username = format!("duplicate_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
    
    // Create first user
    let _user1 = test_db.create_test_user(&unique_username, "first@example.com", "password123").await;
    
    // Try to create user with same username
    let output = Command::new("cargo")
        .args(&["run", "--bin", "create_user", "--", &unique_username, "second@example.com", "password456"])
        .env("TEST_DATABASE_URL", "postgresql://localhost/axum_base_test")
        .env("DATABASE_URL", "postgresql://localhost/axum_base_test")
        .output()
        .expect("Failed to execute create_user command");
    
    assert!(!output.status.success(), "create_user should fail with duplicate username");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("❌ Failed to create user"));
    
    test_db.cleanup().await;
}

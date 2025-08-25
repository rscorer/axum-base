//! # Data Models
//!
//! Shared data structures used across the application.

use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

// =============================================================================
// Time Conversion Utilities
// =============================================================================

/// Convert time::OffsetDateTime to chrono::DateTime<Utc>
#[allow(dead_code)]
pub fn time_to_chrono(dt: OffsetDateTime) -> DateTime<Utc> {
    DateTime::from_timestamp(dt.unix_timestamp(), dt.nanosecond())
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap())
}

/// Convert chrono::DateTime<Utc> to time::OffsetDateTime
#[allow(dead_code)]
pub fn chrono_to_time(dt: DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(dt.timestamp())
        .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH)
        .replace_nanosecond(dt.nanosecond())
        .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH)
}

/// Convert Option<time::OffsetDateTime> to Option<chrono::DateTime<Utc>>
#[allow(dead_code)]
pub fn time_opt_to_chrono_opt(dt: Option<OffsetDateTime>) -> Option<DateTime<Utc>> {
    dt.map(time_to_chrono)
}

/// Convert Option<chrono::DateTime<Utc>> to Option<time::OffsetDateTime>
#[allow(dead_code)]
pub fn chrono_opt_to_time_opt(dt: Option<DateTime<Utc>>) -> Option<OffsetDateTime> {
    dt.map(chrono_to_time)
}

// =============================================================================
// API Response Models
// =============================================================================

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub status: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub database: Option<DatabaseHealthInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseHealthInfo {
    pub connected: bool,
    pub database_name: String,
    pub pool_connections: u32,
    pub idle_connections: usize,
}

// =============================================================================
// Database Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub email_verified: bool,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub category_name: String,
    pub display_name: String,
    pub is_visible: bool,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub data: Option<serde_json::Value>, // Flexible JSON field for custom data
    pub is_active: bool,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// Request/Response DTOs
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CreateItemRequest {
    pub title: String,
    pub description: Option<String>,
    pub data: Option<serde_json::Value>,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ItemWithCategory {
    #[serde(flatten)]
    pub item: Item,
    pub category: Category,
}

// =============================================================================
// Authentication Models
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UpdateProfileRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
}

// Convert User to UserResponse (hiding sensitive fields)
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            email_verified: user.email_verified,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

// Convert User to AuthenticatedUser
impl From<User> for AuthenticatedUser {
    fn from(user: User) -> Self {
        AuthenticatedUser {
            id: user.id,
            username: user.username,
            email: user.email,
            is_active: user.is_active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    
    #[test]
    fn test_user_to_user_response_conversion() {
        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: Some("hashed_password".to_string()),
            email_verified: true,
            is_active: true,
            last_login: None,
            created_at: DateTime::from_timestamp(1640995200, 0).unwrap(), // 2022-01-01
            updated_at: DateTime::from_timestamp(1640995200, 0).unwrap(),
        };
        
        let user_response: UserResponse = user.into();
        
        assert_eq!(user_response.id, 1);
        assert_eq!(user_response.username, "testuser");
        assert_eq!(user_response.email, "test@example.com");
        assert_eq!(user_response.email_verified, true);
        assert_eq!(user_response.is_active, true);
        // Verify that sensitive data (password_hash) is not included in UserResponse
    }
    
    #[test]
    fn test_time_conversion_functions() {
        let chrono_dt = DateTime::from_timestamp(1640995200, 123456789).unwrap();
        let time_dt = chrono_to_time(chrono_dt);
        let back_to_chrono = time_to_chrono(time_dt);
        
        // Should be approximately equal (nanoseconds may differ slightly due to conversion)
        assert_eq!(chrono_dt.timestamp(), back_to_chrono.timestamp());
    }
    
    #[test]
    fn test_optional_time_conversions() {
        let some_chrono = Some(DateTime::from_timestamp(1640995200, 0).unwrap());
        let none_chrono: Option<DateTime<Utc>> = None;
        
        let some_time = chrono_opt_to_time_opt(some_chrono);
        let none_time = chrono_opt_to_time_opt(none_chrono);
        
        assert!(some_time.is_some());
        assert!(none_time.is_none());
        
        let back_to_some_chrono = time_opt_to_chrono_opt(some_time);
        let back_to_none_chrono = time_opt_to_chrono_opt(none_time);
        
        assert!(back_to_some_chrono.is_some());
        assert!(back_to_none_chrono.is_none());
    }
    
    #[test]
    fn test_api_response_serialization() {
        let response = ApiResponse {
            message: "Test message".to_string(),
            status: "success".to_string(),
            timestamp: "2022-01-01T00:00:00Z".to_string(),
        };
        
        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ApiResponse = serde_json::from_str(&json).expect("Should deserialize");
        
        assert_eq!(deserialized.message, "Test message");
        assert_eq!(deserialized.status, "success");
        assert_eq!(deserialized.timestamp, "2022-01-01T00:00:00Z");
    }
    
    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{
            "username": "testuser",
            "password": "testpass"
        }"#;
        
        let login_req: LoginRequest = serde_json::from_str(json).expect("Should deserialize");
        
        assert_eq!(login_req.username, "testuser");
        assert_eq!(login_req.password, "testpass");
    }
}

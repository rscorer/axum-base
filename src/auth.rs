//! # Authentication Module
//!
//! Handles password hashing, session management, and user authentication.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use sqlx::PgPool;

use crate::models::{AuthenticatedUser, User};

// =============================================================================
// Password Hashing Service
// =============================================================================

pub struct PasswordService;

impl PasswordService {
    /// Hash a password using Argon2
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    /// Verify a password against a hash
    pub fn verify_password(
        password: &str,
        hash: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

// =============================================================================
// Authentication Service
// =============================================================================

pub struct AuthService;

#[allow(dead_code)]
impl AuthService {
    /// Authenticate a user with username and password
    pub async fn authenticate_user(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<Option<AuthenticatedUser>, sqlx::Error> {
        // Get user by username
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, email_verified, is_active, last_login, created_at, updated_at 
             FROM users 
             WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        if let Some(user) = user {
            // Check if user has a password hash
            if let Some(hash) = &user.password_hash {
                // Verify password
                match PasswordService::verify_password(password, hash) {
                    Ok(true) => {
                        // Update last login time
                        let now = Utc::now();
                        sqlx::query(
                            "UPDATE users SET last_login = $1, updated_at = $1 WHERE id = $2"
                        )
                        .bind(now)
                        .bind(user.id)
                        .execute(pool)
                        .await?;

                        Ok(Some(user.into()))
                    }
                    Ok(false) => Ok(None), // Wrong password
                    Err(_) => Ok(None), // Hash verification error
                }
            } else {
                Ok(None) // No password set
            }
        } else {
            Ok(None) // User not found or not active
        }
    }

    /// Set password for a user (used for initial setup or admin password resets)
    pub async fn set_user_password(
        pool: &PgPool,
        user_id: i32,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let password_hash = PasswordService::hash_password(password)
            .map_err(|e| format!("Password hashing error: {}", e))?;
        let now = Utc::now();

        let result = sqlx::query(
            "UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(password_hash)
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(format!("User with ID {} not found", user_id).into());
        }

        Ok(())
    }

    /// Change user password (requires current password verification)
    pub async fn change_user_password(
        pool: &PgPool,
        user_id: i32,
        current_password: &str,
        new_password: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Get current user
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, email_verified, is_active, last_login, created_at, updated_at 
             FROM users 
             WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some(user) = user {
            if let Some(current_hash) = &user.password_hash {
                // Verify current password
                if PasswordService::verify_password(current_password, current_hash)
                    .map_err(|e| format!("Password verification error: {}", e))? {
                    // Hash new password and update
                    let new_hash = PasswordService::hash_password(new_password)
                        .map_err(|e| format!("Password hashing error: {}", e))?;
                    let now = Utc::now();

                    sqlx::query(
                        "UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3"
                    )
                    .bind(new_hash)
                    .bind(now)
                    .bind(user_id)
                    .execute(pool)
                    .await?;

                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Update user profile (email, etc.)
    pub async fn update_user_profile(
        pool: &PgPool,
        user_id: i32,
        email: &str,
    ) -> Result<bool, sqlx::Error> {
        let now = Utc::now();
        let result = sqlx::query(
            "UPDATE users SET email = $1, updated_at = $2 WHERE id = $3 AND is_active = true"
        )
        .bind(email)
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Create a new user (for admin use since registration is disabled)
    pub async fn create_user(
        pool: &PgPool,
        username: &str,
        email: &str,
        password: Option<&str>,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let password_hash = if let Some(pwd) = password {
            Some(PasswordService::hash_password(pwd)
                .map_err(|e| format!("Password hashing error: {}", e))?)
        } else {
            None
        };

        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash, email_verified, is_active, created_at, updated_at) 
             VALUES ($1, $2, $3, false, true, $4, $4) 
             RETURNING id, username, email, password_hash, email_verified, is_active, last_login, created_at, updated_at"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

// =============================================================================
// Authentication Middleware
// =============================================================================

use axum::{
    extract::Request,
    middleware::Next,
    response::{Redirect, Response},
};
use tower_sessions::Session;

/// Middleware to require authentication
#[allow(dead_code)]
pub async fn require_auth(session: Session, request: Request, next: Next) -> Result<Response, Redirect> {
    // Check if user is authenticated
    match session.get::<AuthenticatedUser>(USER_SESSION_KEY).await {
        Ok(Some(_user)) => {
            // User is authenticated, proceed
            Ok(next.run(request).await)
        }
        _ => {
            // User is not authenticated, redirect to login
            Err(Redirect::to("/login"))
        }
    }
}

/// Middleware to inject current user into request extensions (optional auth)
#[allow(dead_code)]
pub async fn inject_user(session: Session, mut request: Request, next: Next) -> Response {
    // Try to get current user and add to request extensions
    if let Ok(Some(user)) = session.get::<AuthenticatedUser>(USER_SESSION_KEY).await {
        request.extensions_mut().insert(user);
    }
    
    next.run(request).await
}

// =============================================================================
// Session Keys
// =============================================================================

pub const USER_SESSION_KEY: &str = "user";

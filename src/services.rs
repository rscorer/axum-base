//! # Business Logic Services
//!
//! Service layer for handling business logic and database operations.

use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;

use crate::models::{
    Category, CreateItemRequest, CreateUserRequest, Item, ItemWithCategory, User, UserResponse,
    time_opt_to_chrono_opt, time_to_chrono,
};

// =============================================================================
// User Service
// =============================================================================

#[allow(dead_code)]
pub struct UserService;

#[allow(dead_code)]
impl UserService {
    /// Get user by ID
    pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, username, email, password_hash, email_verified, is_active, last_login, created_at, updated_at 
             FROM users 
             WHERE id = $1 AND is_active = true",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.id,
                username: row.username,
                email: row.email,
                password_hash: Some(row.password_hash),
                email_verified: row.email_verified,
                is_active: row.is_active,
                last_login: time_opt_to_chrono_opt(row.last_login),
                created_at: time_to_chrono(row.created_at),
                updated_at: time_to_chrono(row.updated_at),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Get user by username
    pub async fn get_user_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, username, email, password_hash, email_verified, is_active, last_login, created_at, updated_at 
             FROM users 
             WHERE username = $1 AND is_active = true",
            username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.id,
                username: row.username,
                email: row.email,
                password_hash: Some(row.password_hash),
                email_verified: row.email_verified,
                is_active: row.is_active,
                last_login: time_opt_to_chrono_opt(row.last_login),
                created_at: time_to_chrono(row.created_at),
                updated_at: time_to_chrono(row.updated_at),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Verify user password
    pub async fn verify_password(
        password: &str,
        hash: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Hash password
    pub async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    /// Update user's last login time
    pub async fn update_last_login(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE users SET last_login = NOW() WHERE id = $1", user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Update user's email
    pub async fn update_user_email(
        pool: &PgPool,
        user_id: i32,
        new_email: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2",
            new_email,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Update user's password
    pub async fn update_user_password(
        pool: &PgPool,
        user_id: i32,
        new_password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            new_password_hash,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Create new user
    pub async fn create_user(
        pool: &PgPool,
        request: &CreateUserRequest,
    ) -> Result<UserResponse, sqlx::Error> {
        let password_hash = Self::hash_password(&request.password)
            .await
            .map_err(|e| sqlx::Error::Protocol(format!("Password hashing failed: {}", e)))?;

        let row = sqlx::query!(
            "INSERT INTO users (username, email, password_hash) 
             VALUES ($1, $2, $3) 
             RETURNING id, username, email, password_hash, email_verified, is_active, last_login, created_at, updated_at",
            request.username,
            request.email,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.id,
            username: row.username,
            email: row.email,
            password_hash: Some(row.password_hash),
            email_verified: row.email_verified,
            is_active: row.is_active,
            last_login: time_opt_to_chrono_opt(row.last_login),
            created_at: time_to_chrono(row.created_at),
            updated_at: time_to_chrono(row.updated_at),
        };

        Ok(UserResponse::from(user))
    }
}

// =============================================================================
// Category Service
// =============================================================================

#[allow(dead_code)]
pub struct CategoryService;

#[allow(dead_code)]
impl CategoryService {
    /// Get all visible categories
    pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, category_name, display_name, is_visible, display_order, created_at, updated_at 
             FROM category 
             WHERE is_visible = true 
             ORDER BY display_order, display_name"
        )
        .fetch_all(pool)
        .await?;

        let categories: Vec<Category> = rows
            .into_iter()
            .map(|row| Category {
                id: row.id,
                category_name: row.category_name,
                display_name: row.display_name,
                is_visible: row.is_visible,
                display_order: row.display_order,
                created_at: time_to_chrono(row.created_at),
                updated_at: time_to_chrono(row.updated_at),
            })
            .collect();

        Ok(categories)
    }

    /// Get category by ID
    pub async fn get_category_by_id(
        pool: &PgPool,
        category_id: i32,
    ) -> Result<Option<Category>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, category_name, display_name, is_visible, display_order, created_at, updated_at 
             FROM category 
             WHERE id = $1 AND is_visible = true",
            category_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let category = Category {
                id: row.id,
                category_name: row.category_name,
                display_name: row.display_name,
                is_visible: row.is_visible,
                display_order: row.display_order,
                created_at: time_to_chrono(row.created_at),
                updated_at: time_to_chrono(row.updated_at),
            };
            Ok(Some(category))
        } else {
            Ok(None)
        }
    }
}

// =============================================================================
// Item Service
// =============================================================================

#[allow(dead_code)]
pub struct ItemService;

#[allow(dead_code)]
impl ItemService {
    /// Get all items with their categories
    pub async fn get_all_items(pool: &PgPool) -> Result<Vec<ItemWithCategory>, sqlx::Error> {
        let items = sqlx::query!(
            "SELECT 
                i.id, i.title, i.description, i.data, i.is_active, i.category_id, 
                i.created_at, i.updated_at,
                c.id as cat_id, c.category_name, c.display_name, c.is_visible,
                c.display_order, c.created_at as cat_created_at, c.updated_at as cat_updated_at
             FROM items i 
             JOIN category c ON i.category_id = c.id 
             WHERE c.is_visible = true AND i.is_active = true
             ORDER BY i.created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        let result = items
            .into_iter()
            .map(|row| ItemWithCategory {
                item: Item {
                    id: row.id,
                    title: row.title,
                    description: row.description,
                    data: row.data,
                    is_active: row.is_active,
                    category_id: row.category_id,
                    created_at: time_to_chrono(row.created_at),
                    updated_at: time_to_chrono(row.updated_at),
                },
                category: Category {
                    id: row.cat_id,
                    category_name: row.category_name,
                    display_name: row.display_name,
                    is_visible: row.is_visible,
                    display_order: row.display_order,
                    created_at: time_to_chrono(row.cat_created_at),
                    updated_at: time_to_chrono(row.cat_updated_at),
                },
            })
            .collect();

        Ok(result)
    }

    /// Get items by category
    pub async fn get_items_by_category(
        pool: &PgPool,
        category_id: i32,
    ) -> Result<Vec<Item>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, description, data, is_active, category_id, created_at, updated_at
             FROM items 
             WHERE category_id = $1 AND is_active = true
             ORDER BY created_at DESC",
            category_id
        )
        .fetch_all(pool)
        .await?;

        let items: Vec<Item> = rows
            .into_iter()
            .map(|row| Item {
                id: row.id,
                title: row.title,
                description: row.description,
                data: row.data,
                is_active: row.is_active,
                category_id: row.category_id,
                created_at: time_to_chrono(row.created_at),
                updated_at: time_to_chrono(row.updated_at),
            })
            .collect();

        Ok(items)
    }

    /// Create new item
    pub async fn create_item(
        pool: &PgPool,
        request: &CreateItemRequest,
    ) -> Result<Item, sqlx::Error> {
        let row = sqlx::query!(
            "INSERT INTO items (title, description, data, category_id) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, title, description, data, is_active, category_id, created_at, updated_at",
            request.title,
            request.description,
            request.data,
            request.category_id
        )
        .fetch_one(pool)
        .await?;

        let item = Item {
            id: row.id,
            title: row.title,
            description: row.description,
            data: row.data,
            is_active: row.is_active,
            category_id: row.category_id,
            created_at: time_to_chrono(row.created_at),
            updated_at: time_to_chrono(row.updated_at),
        };

        Ok(item)
    }
}

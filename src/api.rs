//! # API Handlers
//!
//! Handlers for JSON API endpoints.

use axum::{extract::State, response::Json};
use sqlx::PgPool;
use std::env;

use crate::database::get_connection_info;
use crate::models::{ApiResponse, DatabaseHealthInfo, HealthResponse};

/// Health check endpoint with database connectivity check
pub async fn health_check(State(pool): State<PgPool>) -> Json<HealthResponse> {
    // Check database connectivity
    let database_info = match get_connection_info(&pool).await {
        Ok(info) => Some(DatabaseHealthInfo {
            connected: true,
            database_name: info.database_name,
            pool_connections: info.pool_connections,
            idle_connections: info.idle_connections,
        }),
        Err(err) => {
            eprintln!("Database health check failed: {}", err);
            Some(DatabaseHealthInfo {
                connected: false,
                database_name: "unknown".to_string(),
                pool_connections: 0,
                idle_connections: 0,
            })
        }
    };

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "axum-base".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: database_info,
    })
}

/// API hello endpoint
pub async fn api_hello() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello from Axum Base! A modern Rust web server template built with Axum."
            .to_string(),
        status: "success".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

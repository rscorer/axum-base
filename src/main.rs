//! # Axum Base - Modern Rust Web Server Template
//!
//! A modular, production-ready web server built with Axum 0.7, SQLx, and Tera templating.
//! Includes authentication, database migrations, and comprehensive testing.

mod api;
mod auth;
mod context;
mod database;
mod models;
mod routes;
mod server;
mod services;
mod web;

use server::start_server;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    start_server().await;
}

//! # Router Configuration
//!
//! Configures all routes and middleware for the application.

use axum::{
    Router,
    routing::{get, get_service, post},
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use crate::api::{api_hello, health_check};
use crate::web::{handler_404, serve_index, serve_landing, serve_login, handle_login, handle_logout, serve_profile, handle_profile_update};

/// Creates the main application router with all routes and middleware
pub async fn create_router(pool: PgPool) -> Router {
    // Create session store using the database
    let session_store = PostgresStore::new(pool.clone());
    if let Err(e) = session_store.migrate().await {
        eprintln!("❌ Failed to migrate session store: {}", e);
        std::process::exit(1);
    }

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_expiry(Expiry::OnInactivity(tower_sessions::cookie::time::Duration::days(30))); // 30 days

    Router::new()
        // Root route serves the welcome page
        .route("/", get(serve_index))
        // Landing page route
        .route("/landing", get(serve_landing))
        // Authentication routes
        .route("/login", get(serve_login).post(handle_login))
        .route("/logout", post(handle_logout))
        .route("/profile", get(serve_profile).post(handle_profile_update))
        // Health check endpoint
        .route("/health", get(health_check))
        // API routes
        .route("/api/hello", get(api_hello))
        // Serve static files from the static directory
        .nest_service("/static", get_service(ServeDir::new("static")))
        // 404 fallback for any other routes
        .fallback(handler_404)
        // Add middleware for sessions, error handling and logging
        .layer(
            ServiceBuilder::new()
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .layer(tower_http::cors::CorsLayer::permissive())
                .layer(session_layer),
        )
        .with_state(pool)
}

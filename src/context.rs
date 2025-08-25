//! # Axum Base Project Context
//! 
//! This file provides context for AI assistants working with this codebase.
//! This is a generic Rust web application template.
//! 
//! ## Architecture
//! - Modular web server split across modules in `src/`
//! - Axum 0.7 framework with Tokio async runtime
//! - Tera 1.19 template engine for dynamic HTML generation
//! - All API responses use consistent JSON structure with message, status, timestamp
//! 
//! ## Response Pattern
//! ```rust,ignore
//! struct ApiResponse {
//!     message: String,
//!     status: String,     // "success" or "error"
//!     timestamp: String,  // chrono::Utc::now().to_rfc3339()
//! }
//! ```
//! 
//! ## Handler Pattern
//! ```rust,ignore
//! async fn handler() -> Result<Json<ApiResponse>, (StatusCode, String)> {
//!     // Implementation with proper error handling
//! }
//! ```
//! 
//! ## Current Endpoints
//! - GET / -> HTML welcome page
//! - GET /health -> JSON health check
//! - GET /api/hello -> JSON greeting
//! - GET /static/* -> Static file serving
//! - Fallback -> 404 JSON response
//! 
//! ## Template Engine (Tera)
//! ```rust,ignore
//! // Template handler pattern
//! pub async fn serve_template() -> Result<Html<String>, (StatusCode, String)> {
//!     let tera = get_templates();
//!     let mut context = Context::new();
//!     
//!     context.insert("title", "Page Title");
//!     context.insert("version", env!("CARGO_PKG_VERSION"));
//!     
//!     let rendered = tera.render("template.html", &context)
//!         .map_err(|err| {
//!             eprintln!("Template error: {}", err);
//!             (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render".to_string())
//!         })?;
//!     
//!     Ok(Html(rendered))
//! }
//! ```
//! 
//! ## Template Syntax
//! ```html,ignore
//! <!-- Variables -->
//! {{ title }}
//! {{ user_name | default(value="world") }}
//! 
//! <!-- Loops -->
//! {% for item in items %}
//!     <div>{{ item.name }}</div>
//! {% endfor %}
//! 
//! <!-- Conditionals -->
//! {% if version %}
//!     <span>Version: {{ version }}</span>
//! {% endif %}
//! ```
//! 
//! ## Development
//! - Default port: 3093 (configurable via PORT env var)
//! - Templates in `templates/` directory
//! - Static files in `static/` directory
//! - Initialize templates before starting server
//! - Always run `cargo fmt` before commits
//! - Use `eprintln!` for error logging

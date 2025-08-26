//! # Web Handlers
//!
//! Handlers for HTML pages, static files, and error responses.

use axum::{
    extract::{Form, State},
    http::{StatusCode, Uri},
    response::{Html, Json, Redirect},
};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::OnceLock;
use tera::{Context, Tera};
use tower_sessions::Session;

use crate::auth::{AuthService, USER_SESSION_KEY};
use crate::models::{ApiResponse, AuthenticatedUser, LoginRequest};

/// Global Tera instance
static TEMPLATES: OnceLock<Tera> = OnceLock::new();

/// Initialize the template engine
pub fn init_templates() -> Result<(), tera::Error> {
    let tera = Tera::new("templates/**/*")?;
    TEMPLATES
        .set(tera)
        .map_err(|_| tera::Error::msg("Failed to initialize template engine"))?;
    Ok(())
}

/// Get the global Tera instance
fn get_templates() -> &'static Tera {
    TEMPLATES.get().expect("Templates not initialized")
}

/// Format a UTC DateTime to a human-readable format
/// Example: "Sept 27th, 2025 @ 4:13pm"
fn format_human_time(dt: DateTime<Utc>) -> String {
    // Convert to local time if needed, but for now use UTC
    let month = match dt.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "Aug",
        9 => "Sept",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Unknown",
    };

    let day = dt.day();
    let day_suffix = match day % 10 {
        1 if day != 11 => "st",
        2 if day != 12 => "nd",
        3 if day != 13 => "rd",
        _ => "th",
    };

    let hour = dt.hour();
    let (hour_12, am_pm) = if hour == 0 {
        (12, "am")
    } else if hour < 12 {
        (hour, "am")
    } else if hour == 12 {
        (12, "pm")
    } else {
        (hour - 12, "pm")
    };

    format!(
        "{} {}{}, {} @ {}:{:02}{}",
        month,
        day,
        day_suffix,
        dt.year(),
        hour_12,
        dt.minute(),
        am_pm
    )
}

/// Create base template context with common variables
/// Pass additional variables as a HashMap
fn create_base_context(additional_vars: HashMap<&str, serde_json::Value>) -> Context {
    let mut context = Context::new();

    // Add common variables that appear in all templates
    context.insert("service_name", "Axum Base");
    context.insert("version", env!("CARGO_PKG_VERSION"));
    context.insert("server_time", &format_human_time(Utc::now()));

    // Add any additional variables passed in
    for (key, value) in additional_vars {
        context.insert(key, &value);
    }

    context
}

/// Create base template context with user information
fn create_base_context_with_user(
    additional_vars: HashMap<&str, serde_json::Value>,
    user: Option<&AuthenticatedUser>,
) -> Context {
    let mut context = Context::new();

    // Add common variables that appear in all templates
    context.insert("service_name", "Axum Base");
    context.insert("version", env!("CARGO_PKG_VERSION"));
    context.insert("server_time", &format_human_time(Utc::now()));

    // Add user information if available
    context.insert("current_user", &user);
    context.insert("is_authenticated", &user.is_some());

    // Add any additional variables passed in
    for (key, value) in additional_vars {
        context.insert(key, &value);
    }

    context
}

/// Render a template with error handling
fn render_template(
    template_name: &str,
    context: &Context,
) -> Result<Html<String>, (StatusCode, String)> {
    let tera = get_templates();
    let rendered = tera.render(template_name, context).map_err(|err| {
        eprintln!("Failed to render template '{}': {}", template_name, err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", template_name),
        )
    })?;

    Ok(Html(rendered))
}

/// Handler for the landing page - serves a generic landing page
pub async fn serve_landing(session: Session) -> Result<Html<String>, (StatusCode, String)> {
    // Define landing page specific features
    let landing_features = json!([
        {
            "title": "Modern Architecture",
            "description": "Built with Rust, Axum, and PostgreSQL for maximum performance and reliability.",
            "icon_path": "M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H6.911a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661Z",
            "link": "/api/hello"
        },
        {
            "title": "Authentication Ready",
            "description": "Complete user authentication system with sessions and secure password handling.",
            "icon_path": "M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z",
            "link": "/login"
        },
        {
            "title": "Production Ready",
            "description": "Includes health checks, database migrations, comprehensive testing, and error handling.",
            "icon_path": "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
            "link": "/health"
        }
    ]);

    // Create context with base variables plus page-specific data
    let mut page_vars = HashMap::new();
    page_vars.insert("page_title", json!("Modern Rust Web Application Template"));
    page_vars.insert("page_description", json!("A production-ready foundation for building fast, secure web applications with Rust and Axum."));
    page_vars.insert("landing_features", landing_features);

    let current_user = get_current_user(&session).await;
    let context = create_base_context_with_user(page_vars, current_user.as_ref());
    render_template("landing.html", &context)
}

/// Handler for the root path - serves the welcome page using Tera templates
pub async fn serve_index(session: Session) -> Result<Html<String>, (StatusCode, String)> {
    // Define index page specific features
    let features = json!([
        {
            "icon": "🚀",
            "title": "Fast & Efficient",
            "description": "Built with Rust's performance and safety guarantees"
        },
        {
            "icon": "🔧",
            "title": "Modern Stack",
            "description": "Powered by Axum web framework and Tokio runtime"
        },
        {
            "icon": "📡",
            "title": "API Ready",
            "description": "RESTful endpoints with JSON support out of the box"
        },
        {
            "icon": "🎨",
            "title": "Template Engine",
            "description": "Dynamic content with Tera template engine"
        }
    ]);

    let endpoints = json!([
        {"name": "Try API", "path": "/api/hello"},
        {"name": "Health Check", "path": "/health"}
    ]);

    // Create context with base variables plus page-specific data
    let mut page_vars = HashMap::new();
    page_vars.insert("title", json!("Home"));
    page_vars.insert(
        "description",
        json!("A fast and modern Rust web application template built with Axum"),
    );
    page_vars.insert("features", features);
    page_vars.insert("endpoints", endpoints);

    let current_user = get_current_user(&session).await;
    let context = create_base_context_with_user(page_vars, current_user.as_ref());
    render_template("index.html", &context)
}

// =============================================================================
// Authentication Handlers
// =============================================================================

/// Helper function to get the current user from session
async fn get_current_user(session: &Session) -> Option<AuthenticatedUser> {
    session.get(USER_SESSION_KEY).await.ok().flatten()
}

/// Login page handler
pub async fn serve_login(session: Session) -> Result<Html<String>, Redirect> {
    // If user is already logged in, redirect to home
    if get_current_user(&session).await.is_some() {
        return Err(Redirect::to("/"));
    }

    let mut page_vars = HashMap::new();
    page_vars.insert("title", json!("Login"));
    page_vars.insert("error", json!(null));

    let context = create_base_context(page_vars);

    match render_template("login.html", &context) {
        Ok(html) => Ok(html),
        Err(_) => Err(Redirect::to("/")),
    }
}

/// Login form handler
pub async fn handle_login(
    State(pool): State<PgPool>,
    session: Session,
    Form(login_data): Form<LoginRequest>,
) -> Result<Redirect, Html<String>> {
    // Attempt to authenticate the user
    match AuthService::authenticate_user(&pool, &login_data.username, &login_data.password).await {
        Ok(Some(user)) => {
            // Store user in session
            if (session.insert(USER_SESSION_KEY, &user).await).is_err() {
                let mut page_vars = HashMap::new();
                page_vars.insert("title", json!("Login"));
                page_vars.insert("error", json!("Session error. Please try again."));
                page_vars.insert("username", json!(login_data.username));

                let context = create_base_context(page_vars);
                return Err(render_template("login.html", &context)
                    .unwrap_or_else(|_| Html("Login error".to_string())));
            }

            Ok(Redirect::to("/"))
        }
        Ok(None) => {
            // Authentication failed
            let mut page_vars = HashMap::new();
            page_vars.insert("title", json!("Login"));
            page_vars.insert("error", json!("Invalid username or password"));
            page_vars.insert("username", json!(login_data.username));

            let context = create_base_context(page_vars);
            Err(render_template("login.html", &context)
                .unwrap_or_else(|_| Html("Login error".to_string())))
        }
        Err(_) => {
            // Database error
            let mut page_vars = HashMap::new();
            page_vars.insert("title", json!("Login"));
            page_vars.insert("error", json!("System error. Please try again later."));
            page_vars.insert("username", json!(login_data.username));

            let context = create_base_context(page_vars);
            Err(render_template("login.html", &context)
                .unwrap_or_else(|_| Html("Login error".to_string())))
        }
    }
}

/// Logout handler
pub async fn handle_logout(session: Session) -> Redirect {
    // Remove user from session
    let _ = session.remove::<AuthenticatedUser>(USER_SESSION_KEY).await;
    // Clear the entire session
    let _ = session.clear().await;

    Redirect::to("/login")
}

/// Profile page handler
pub async fn serve_profile(session: Session) -> Result<Html<String>, Redirect> {
    // Check if user is authenticated
    let user = match get_current_user(&session).await {
        Some(user) => user,
        None => return Err(Redirect::to("/login")),
    };

    let mut page_vars = HashMap::new();
    page_vars.insert("title", json!("Profile"));
    page_vars.insert("user", json!(user));
    page_vars.insert("success", json!(null));
    page_vars.insert("error", json!(null));

    let context = create_base_context_with_user(page_vars, Some(&user));

    match render_template("profile.html", &context) {
        Ok(html) => Ok(html),
        Err(_) => Err(Redirect::to("/")),
    }
}

/// Profile update handler
pub async fn handle_profile_update(
    State(pool): State<PgPool>,
    session: Session,
    Form(form_data): Form<serde_json::Value>,
) -> Result<Html<String>, Redirect> {
    // Check if user is authenticated
    let user = match get_current_user(&session).await {
        Some(user) => user,
        None => return Err(Redirect::to("/login")),
    };

    let mut success_message = None;
    let mut error_message = None;

    // Handle profile update (email)
    if let (Some(email), Some(action)) = (
        form_data.get("email").and_then(|v| v.as_str()),
        form_data.get("action").and_then(|v| v.as_str()),
    )
        && action == "update_profile" {
            match AuthService::update_user_profile(&pool, user.id, email).await {
                Ok(true) => {
                    success_message = Some("Profile updated successfully!".to_string());
                    // Update session with new email
                    let mut updated_user = user.clone();
                    updated_user.email = email.to_string();
                    let _ = session.insert(USER_SESSION_KEY, &updated_user).await;
                }
                Ok(false) => error_message = Some("Failed to update profile".to_string()),
                Err(_) => error_message = Some("Database error".to_string()),
            }
        }

    // Handle password change
    if let (Some(current_password), Some(new_password), Some(confirm_password), Some(action)) = (
        form_data.get("current_password").and_then(|v| v.as_str()),
        form_data.get("new_password").and_then(|v| v.as_str()),
        form_data.get("confirm_password").and_then(|v| v.as_str()),
        form_data.get("action").and_then(|v| v.as_str()),
    )
        && action == "change_password" {
            if new_password != confirm_password {
                error_message = Some("New passwords do not match".to_string());
            } else if new_password.len() < 8 {
                error_message = Some("Password must be at least 8 characters".to_string());
            } else {
                match AuthService::change_user_password(
                    &pool,
                    user.id,
                    current_password,
                    new_password,
                )
                .await
                {
                    Ok(true) => {
                        success_message = Some("Password changed successfully!".to_string())
                    }
                    Ok(false) => error_message = Some("Current password is incorrect".to_string()),
                    Err(_) => error_message = Some("Error changing password".to_string()),
                }
            }
        }

    let mut page_vars = HashMap::new();
    page_vars.insert("title", json!("Profile"));
    page_vars.insert("user", json!(user));
    page_vars.insert("success", json!(success_message));
    page_vars.insert("error", json!(error_message));

    let context = create_base_context_with_user(page_vars, Some(&user));

    match render_template("profile.html", &context) {
        Ok(html) => Ok(html),
        Err(_) => Err(Redirect::to("/")),
    }
}

/// 404 handler
pub async fn handler_404(uri: Uri) -> (StatusCode, Json<ApiResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            message: format!(
                "The requested path '{}' was not found on this server",
                uri.path()
            ),
            status: "error".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }),
    )
}

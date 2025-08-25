//! # Server Configuration
//!
//! Server startup and configuration logic.

use std::env;
use std::net::IpAddr;

use crate::database::{init_pool, run_migrations, test_connection};
use crate::routes::create_router;
use crate::web::init_templates;

/// Gets all available network interfaces and their IP addresses
fn get_network_addresses() -> Vec<String> {
    let mut addresses = Vec::new();

    // Add localhost variants
    addresses.push("localhost".to_string());
    addresses.push("127.0.0.1".to_string());

    // Try to get actual network interfaces
    if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
        for (name, ip) in interfaces {
            // Skip loopback interfaces
            if name != "lo" && name != "lo0" && !ip.is_loopback() {
                match ip {
                    IpAddr::V4(ipv4) => {
                        // Only include IPv4 addresses (skip IPv6)
                        if !ipv4.is_link_local() {
                            addresses.push(ipv4.to_string());
                        }
                    }
                    IpAddr::V6(_) => {
                        // Skip all IPv6 addresses to reduce clutter
                        continue;
                    }
                }
            }
        }
    }

    addresses
}

/// Starts the Axum Base server
pub async fn start_server() {
    // Get port from environment variable, default to 3093
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3093".to_string())
        .parse::<u16>()
        .unwrap_or(3093);

    let addr = format!("0.0.0.0:{}", port);

    // Initialize database connection pool
    let db_pool = match init_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("❌ Failed to initialize database pool: {}", err);
            std::process::exit(1);
        }
    };

    // Test database connectivity
    match test_connection(&db_pool).await {
        Ok(true) => println!("✅ Database connectivity verified"),
        Ok(false) => {
            eprintln!("❌ Database connectivity test failed: unexpected result");
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("❌ Database connectivity test failed: {}", err);
            std::process::exit(1);
        }
    }

    // Run database migrations
    if let Err(err) = run_migrations(&db_pool).await {
        eprintln!("❌ Failed to run database migrations: {}", err);
        std::process::exit(1);
    }
    println!("✅ Database migrations completed successfully");

    // Initialize template engine
    if let Err(err) = init_templates() {
        eprintln!("❌ Failed to initialize templates: {}", err);
        std::process::exit(1);
    }

    // Create the Axum router with all routes and session management
    let app = create_router(db_pool).await;

    // Start the server
    println!("🚀 Axum Base server starting...");
    println!("🌟 Server ready! Access via:");

    // Get all available network addresses
    let addresses = get_network_addresses();
    for address in addresses {
        println!("   http://{}:{}", address, port);
    }

    println!();
    println!("📡 Available endpoints:");
    println!("   GET  /         - Welcome page (using base template)");
    println!("   GET  /landing  - Landing page");
    println!("   GET  /login    - Login page");
    println!("   POST /login    - Login form submission");
    println!("   POST /logout   - Logout");
    println!("   GET  /profile  - User profile (authenticated)");
    println!("   POST /profile  - Update profile (authenticated)");
    println!("   GET  /health   - Health check");
    println!("   GET  /api/hello - JSON API endpoint");
    println!("   GET  /static/* - Static file serving");
    println!("💡 Press Ctrl+C to stop the server");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|err| {
            eprintln!("❌ Failed to bind to address {}: {}", addr, err);
            std::process::exit(1);
        });

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("❌ Server error: {}", err);
        std::process::exit(1);
    }
}

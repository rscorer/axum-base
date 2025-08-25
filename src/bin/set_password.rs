//! # Password Management CLI
//!
//! Command-line utility for setting user passwords.

use std::env;

use axum_base::auth::AuthService;
use axum_base::database::init_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <user_id> <password>", args[0]);
        std::process::exit(1);
    }

    let user_id: i32 = match args[1].parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error: User ID must be a valid number");
            std::process::exit(1);
        }
    };

    let password = &args[2];

    if password.len() < 8 {
        eprintln!("Error: Password must be at least 8 characters long");
        std::process::exit(1);
    }

    // Initialize database connection
    let pool = init_pool().await?;

    // Set the password
    match AuthService::set_user_password(&pool, user_id, password).await {
        Ok(()) => {
            println!("✅ Password set successfully for user ID {}", user_id);
        }
        Err(e) => {
            eprintln!("❌ Failed to set password: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

//! # User Management CLI
//!
//! Command-line utility for creating users since registration is disabled.

use std::env;
use std::io::{self, Write};

use axum_base::auth::AuthService;
use axum_base::database::init_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 && args.len() != 4 {
        eprintln!("Usage: {} <username> [email] [password]", args[0]);
        eprintln!("       {} <username>  # Interactive mode", args[0]);
        std::process::exit(1);
    }

    let username = &args[1];
    
    let (email, password) = if args.len() == 4 {
        // Non-interactive mode
        (args[2].clone(), Some(args[3].clone()))
    } else {
        // Interactive mode
        print!("Email: ");
        io::stdout().flush()?;
        let mut email = String::new();
        io::stdin().read_line(&mut email)?;
        let email = email.trim().to_string();

        print!("Set password now? (y/N): ");
        io::stdout().flush()?;
        let mut set_password = String::new();
        io::stdin().read_line(&mut set_password)?;
        
        let password = if set_password.trim().to_lowercase() == "y" {
            print!("Password: ");
            io::stdout().flush()?;
            let mut password = String::new();
            io::stdin().read_line(&mut password)?;
            Some(password.trim().to_string())
        } else {
            None
        };

        (email, password)
    };

    if email.is_empty() {
        eprintln!("Error: Email cannot be empty");
        std::process::exit(1);
    }

    // Initialize database connection
    let pool = init_pool().await?;

    // Create the user
    match AuthService::create_user(&pool, username, &email, password.as_deref()).await {
        Ok(user) => {
            println!("✅ User created successfully!");
            println!("   ID: {}", user.id);
            println!("   Username: {}", user.username);
            println!("   Email: {}", user.email);
            println!("   Active: {}", user.is_active);
            
            if password.is_some() {
                println!("   Password: Set");
            } else {
                println!("   Password: Not set (user will need admin to set password)");
                println!();
                println!("💡 To set password later, use:");
                println!("   cargo run --bin set_password {} <password>", user.id);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create user: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

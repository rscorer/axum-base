use std::process::Command;
use std::env;

fn main() {
    // Determine input and output paths
    let input_path = "input.css";
    let output_path = "static/style.css";

    println!("cargo:rerun-if-changed={}", input_path);
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=tailwind.config.js");

    // Check if we are in release mode to decide on minification
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let mut cmd = Command::new("tailwindcss");
    cmd.args(&["-i", input_path, "-o", output_path]);

    if profile == "release" {
        cmd.arg("--minify");
    }

    println!("cargo:warning=Running Tailwind CSS build for {} mode...", profile);

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("cargo:warning=Tailwind CSS build complete!");
        }
        Ok(status) => {
            eprintln!("Tailwind CSS build failed with status: {}", status);
            // We might not want to fail the whole build if Tailwind is missing globally, 
            // but for this request, it's safer to fail if it's supposed to be there.
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute tailwindcss CLI: {}. Is it installed globally?", e);
            // Decide if you want to fail the build or just warn. 
            // Failing is usually better for consistency.
            std::process::exit(1);
        }
    }
}

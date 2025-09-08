.PHONY: run watch test test-api test-cli test-all check clean-test tailwind-dev tailwind-build fmt clippy create-user set-password sqlx-prepare dev-setup clean dev

# Run the application (default target)
run:
	cargo run

# Run with auto-reload on file changes
watch:
	cargo watch -x run

# Run all tests with selective threading optimization
test:
	cargo test

# Run only API tests (database tests run serially via #[serial])
test-api:
	cargo test --test api_tests

# Run only CLI tests (database tests run serially via #[serial])
test-cli:
	cargo test --test cli_tests

# Run all tests with output
test-all:
	cargo test --nocapture

# Quick compile check
check:
	cargo check --tests

# Clean and test
clean-test:
	cargo clean && make test

# Tailwind CSS development (watch mode)
tailwind-dev:
	./tw.sh

# Tailwind CSS production build
tailwind-build:
	./tw-build.sh

# Code formatting
fmt:
	cargo fmt

# Code linting
clippy:
	cargo clippy

# Database operations
create-user:
	cargo run --bin create_user

set-password:
	cargo run --bin set_password

# SQLx operations
sqlx-prepare:
	cargo sqlx prepare

# Development setup (build deps + tailwind)
dev-setup: 
	cargo build
	./tw-build.sh

# Clean everything
clean:
	cargo clean
	rm -f static/style.css

# Full development workflow
dev: dev-setup
	@echo "🚀 Starting development servers..."
	@echo "📝 Run 'make tailwind-dev' in another terminal for CSS watching"
	make watch

# 🚀 Axum Base - Production-Ready Rust Web Server Template

A modern, secure, and scalable Rust web server template built with **Axum 0.8**, featuring authentication, database integration, templating, and comprehensive AI coding assistant support.

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/postgresql-supported-blue.svg)](https://www.postgresql.org)
[![SQLx](https://img.shields.io/badge/sqlx-compile%20time%20checked-green.svg)](https://github.com/launchbadge/sqlx)

## ✨ Features

### 🏗️ **Modern Architecture**
- **Axum 0.8** - High-performance async web framework
- **Tokio** - Robust async runtime with full feature set
- **Modular Design** - Clean separation of concerns across dedicated modules
- **Rust 2024 Edition** - Latest language features and improvements

### 🗄️ **Database Integration**
- **PostgreSQL** with **SQLx** for compile-time checked queries
- **Database Migrations** - Sequential, reproducible schema changes
- **Connection Pooling** - Optimized resource management
- **Type Safety** - Prevent SQL injection with compile-time verification

### 🔐 **Security & Authentication**
- **tower-sessions** - Secure session management with PostgreSQL store
- **Argon2** password hashing - Industry-standard, memory-hard algorithm
- **Input Validation** - Comprehensive request validation and sanitization
- **CSRF Protection** - Built-in protection against cross-site request forgery

### 🎨 **Frontend & Templating**
- **Tera Templates** - Django/Jinja2-like syntax with safe HTML escaping
- **Static File Serving** - Efficient static asset delivery
- **Template Inheritance** - Reusable layouts and components

### 🧪 **Testing & Quality**
- **Comprehensive Test Suite** - Unit and integration tests
- **axum-test** - Specialized testing utilities for HTTP endpoints
- **Database Testing** - Proper test isolation and cleanup
- **Code Quality** - Clippy linting and rustfmt formatting

### 🤖 **AI Coding Assistants Support**
This project includes comprehensive configuration for **10+ AI coding tools**:

| AI Tool | Configuration File | Purpose |
|---------|-------------------|---------|
| 🔥 **WARP Terminal** | `.warp.md` | Terminal AI assistance |
| 🎯 **Cursor** | `.cursorrules` | Pair programming in Cursor IDE |
| 🛠️ **Aider** | `.aider.conf.yml` | Command-line AI pair programming |
| 📝 **Continue.dev** | `.continuerc.json` | VS Code AI extension |
| 🧠 **JetBrains AI** | `.jetbrains-ai.md` | IntelliJ/RustRover context |
| 🌊 **Windsurf** | `.windsurf.md` | AI code completion |
| 🎭 **Claude** | `.claude.md` | Deep analysis & refactoring |
| 💎 **Gemini** | `.gemini.md` | Code generation & optimization |
| 🐙 **GitHub Copilot** | `.copilotignore` | Privacy protection |
| 📚 **Master Docs** | `.ai-context.md` | Central AI context |

> 🔗 **See [AI-ROBOT-FRIENDS.md](AI-ROBOT-FRIENDS.md) for complete setup guide**

## 🚀 Quick Start

### Prerequisites
- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **PostgreSQL** - [Install PostgreSQL](https://www.postgresql.org/download/)
- **Tailwind CSS CLI** - [Install Tailwind CSS](https://tailwindcss.com/blog/standalone-cli)
- **Git** - [Install Git](https://git-scm.com/)

### 1. Clone & Setup
```bash
git clone https://github.com/rscorer/axum-base.git
cd axum-base
```

### 2. Database Setup
```bash
# Create database
createdb axum_base_dev

# Copy environment configuration
cp .env.example .env

# Edit .env with your database credentials
# DATABASE_URL=postgres://username:password@localhost:5432/axum_base_dev
```

### 3. Build Tailwind CSS & Install Dependencies
```bash
# Install dependencies
cargo build

# Build Tailwind CSS for development (run in separate terminal)
./tw.sh

# OR build Tailwind CSS once for production
./tw-build.sh
```

### 4. Run Application
```bash
# Run database migrations (automatic on first run)
cargo run

# Server starts on http://localhost:3093
```

### 5. Create Test User (Optional)
```bash
# Create a user via CLI
cargo run --bin create_user

# Or set password for existing user
cargo run --bin set_password
```

## 📁 Project Structure

```
src/
├── main.rs           # 🚪 Application entry point
├── server.rs         # 🏗️ Server initialization and configuration
├── context.rs        # 🎯 Application state and dependency injection
├── database.rs       # 🗄️ Database connection and configuration
├── routes.rs         # 🛣️ Route registration and middleware setup
│
├── api.rs            # 🔌 JSON API handlers and responses
├── web.rs            # 🌐 HTML handlers with Tera integration
├── services.rs       # ⚙️ Core business logic layer
├── models.rs         # 📊 Data structures and database schemas
└── auth.rs           # 🔐 Authentication middleware and utilities

migrations/           # 🔄 Database schema migrations
├── 0001_create_initial_schema.sql
├── 0002_seed_categories.sql
└── 0003_seed_sample_items.sql

tests/               # 🧪 Integration and unit tests
├── api_tests.rs
└── cli_tests.rs

templates/           # 🎨 Tera HTML templates
├── base.html
├── index.html
├── login.html
└── ...

static/              # 📦 Static assets (CSS, JS, images)
├── style.css        #   Generated CSS from Tailwind (served to browsers)

input.css            # 🎨 Tailwind CSS source (@import "tailwindcss")
tw.sh                # ⚡ Development Tailwind build (--watch)
tw-build.sh          # 🚀 Production Tailwind build (--minify)

Dockerfile           # 🐳 Multi-stage Docker build configuration
.dockerignore        # 🚫 Docker build context exclusions
compose.yml          # 📋 Docker Compose orchestration
```

## 🛠️ Development Commands

### Quick Start
```bash
# Full development workflow (builds deps + starts server)
make dev

# Or step by step:
make run                    # Start the application
make watch                  # Start with auto-reload on file changes
```

### Makefile Commands
```bash
# Development
make run                    # Run the application
make watch                  # Run with auto-reload (cargo watch)
make dev                    # Full development setup + watch
make dev-setup              # Build dependencies + Tailwind CSS

# Tailwind CSS
make tailwind-dev           # Development mode (watch)
make tailwind-build         # Production build (minified)

# Testing (with selective threading optimization)
make test                   # Run all tests (unit tests parallel, DB tests serial)
make test-api               # Run API integration tests (with #[serial] for DB safety)
make test-cli               # Run CLI utility tests (with #[serial] for DB safety)
make test-all               # Run tests with output (optimized threading)
make check                  # Quick compile check
make clean-test             # Clean + test

# Code Quality
make fmt                    # Format code with rustfmt
make clippy                 # Lint code with clippy
make sqlx-prepare           # Check SQLx queries

# Database Operations
make create-user            # Create new user via CLI
make set-password           # Set user password via CLI

# Utilities
make clean                  # Clean build artifacts + CSS
```

### Direct Cargo Commands
```bash
# Alternative to Makefile (if preferred)
cargo run
cargo watch -x run
cargo test
cargo fmt
cargo clippy
./tw.sh                     # Tailwind development
./tw-build.sh               # Tailwind production
```

## 🎨 Tailwind CSS Integration

This project uses **Tailwind CSS** for styling, with a streamlined build process using the Tailwind standalone CLI.

### 🏗️ Architecture
- **Input**: `input.css` (imports Tailwind directives)
- **Output**: `static/style.css` (served to browsers)
- **Template Integration**: Referenced in `templates/base.html`

### ⚡ Development Workflow
```bash
# Start Tailwind in watch mode (monitors file changes)
./tw.sh

# In a separate terminal, start your Axum server
cargo run
```

### 🚀 Production Build
```bash
# One-time build with minification for production
./tw-build.sh

# OR use Makefile
make tailwind-build
```

### 📁 File Structure
```
input.css          # Tailwind source (imports @tailwindcss)
static/style.css   # Generated CSS (served by Axum)
tw.sh             # Development script (--watch)
tw-build.sh       # Production script (--minify)
```

### 💡 Usage Tips
- Run `./tw.sh` in a **separate terminal** for development
- The build process **automatically detects** class usage in templates
- Production builds are **minified** for optimal performance
- All Tailwind classes in templates are **purged** if unused

## 🔧 Configuration

### Environment Variables
Key configuration options in `.env`:

```bash
# Database (Required)
DATABASE_URL=postgres://username:password@localhost:5432/axum_base_dev

# Server (Optional)
PORT=3093
HOST=0.0.0.0

# Session (Optional)
SESSION_SECRET=your-secret-key-here
```

### Database Configuration
- **Connection Pool**: 20 max connections, 5 minimum
- **Query Timeout**: 3 seconds
- **Migration**: Automatic in development, manual in production

## 🧪 Testing

### 🏃‍♂️ Selective Test Threading
This project uses an **intelligent testing strategy** that automatically optimizes performance while ensuring reliability:

- **Unit Tests** (in `src/models.rs`): Run in **parallel** for fast execution (~0.00s)
- **Database Tests** (in `tests/`): Run **serially** via `#[serial]` attribute to prevent race conditions (~10-15s)
- **Smart Implementation**: Uses `serial_test` crate to selectively control threading per test
- **Performance**: 60-80% faster than traditional single-threaded testing

#### How It Works
Instead of forcing ALL tests to run single-threaded (slow), we use the `#[serial]` attribute only on tests that need it:

```rust
// ✅ Unit tests run in PARALLEL (fast)
#[test] 
fn test_validation() { /* no database = parallel */ }

// ✅ Database tests run in SERIAL (safe)
use serial_test::serial;

#[tokio::test]
#[serial]  // This attribute ensures serial execution
async fn test_database() { /* database access = serial */ }
```

### Unit Tests
```bash
# Test business logic (runs in parallel)
cargo test --lib

# Test specific module
cargo test models::tests

# Test with coverage
cargo test --lib --features coverage
```

### Integration Tests
```bash
# Test HTTP endpoints (runs serially with #[serial])
cargo test --test api_tests

# Test CLI utilities (runs serially with #[serial])
cargo test --test cli_tests

# Run all tests with optimal threading
cargo test --all
```

### Testing Architecture
- **Selective Threading**: `serial_test` crate enables per-test thread control
- **No Global Restrictions**: No `--test-threads=1` needed - threading is controlled per test
- **Database Safety**: Tests marked with `#[serial]` prevent database race conditions
- **Test Isolation**: Each database test includes proper setup/teardown
- **Automatic Optimization**: Rust's test runner handles parallel execution for unmarked tests

### Writing New Tests
```rust
// Unit tests (parallel execution)
#[test]
fn test_data_validation() {
    // Fast, isolated logic tests
}

// Database/integration tests (serial execution)
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_database_operation() {
    // Database tests with proper isolation
}
```

## 🔒 Security Features

- **Password Hashing**: Argon2 with configurable work factors
- **Session Security**: HTTP-only, secure cookies with CSRF protection  
- **Input Validation**: Comprehensive request validation using `validator`
- **SQL Injection Prevention**: Compile-time checked queries via SQLx
- **Dependency Security**: Regular `cargo audit` checks

## 📈 Performance

- **Async Throughout**: Non-blocking I/O with Tokio
- **Connection Pooling**: Optimized database resource usage
- **Compile-time Optimization**: SQLx compile-time query checking
- **Static Assets**: Efficient static file serving with caching headers

## 🚀 Deployment

### Docker (Recommended)

The project includes a comprehensive multi-stage Dockerfile that:
- ✅ Builds Rust application with optimized dependencies caching
- ✅ Builds Tailwind CSS with minification for production
- ✅ Creates secure runtime image with non-root user
- ✅ Includes health checks and proper signal handling

#### Quick Start with Docker Compose
```bash
# Build and start all services (app + PostgreSQL)
docker compose up --build

# Run in background
docker compose up -d --build

# View logs
docker compose logs -f app

# Stop services
docker compose down
```

#### Build Docker Image
```bash
# Build the image
docker build -t axum-base .

# Run with external database
docker run -p 3093:3093 \
  -e DATABASE_URL=postgres://user:pass@host:5432/db \
  -e SESSION_SECRET=your-secret-key \
  axum-base
```

### Production Environment

#### Environment Variables
```bash
# Required
export DATABASE_URL=postgres://prod_user:prod_pass@db:5432/axum_base
export SESSION_SECRET=your-production-secret-key

# Optional
export RUST_LOG=info
export PORT=3093
export HOST=0.0.0.0
```

#### Manual Deployment
```bash
# Build Tailwind CSS for production
./tw-build.sh

# Build Rust application
cargo build --release

# Run migrations
sqlx migrate run

# Start server
./target/release/axum-base
```

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** following the established patterns
4. **Add tests** for new functionality
5. **Run quality checks** (`cargo fmt && cargo clippy && cargo test`)
6. **Commit your changes** (`git commit -m 'feat: add amazing feature'`)
7. **Push to the branch** (`git push origin feature/amazing-feature`)
8. **Open a Pull Request**

### Development Guidelines
- Follow **Rust 2024 edition** best practices
- Use **SQLx macros** for all database operations
- Keep handlers **lightweight** - move logic to services
- Write **comprehensive tests** for new features
- Update **AI configurations** if adding new patterns

## 📚 Documentation

- **[AI Robot Friends Guide](AI-ROBOT-FRIENDS.md)** - Complete AI assistant setup
- **[API Documentation](docs/api.md)** - REST API reference
- **[Database Schema](docs/schema.md)** - Database design and relationships
- **[Deployment Guide](docs/deployment.md)** - Production deployment instructions

## 🔗 Related Resources

- **[Axum Documentation](https://docs.rs/axum/)** - Web framework docs
- **[SQLx Documentation](https://docs.rs/sqlx/)** - Database toolkit docs  
- **[Tokio Documentation](https://docs.rs/tokio/)** - Async runtime docs
- **[Tera Documentation](https://keats.github.io/tera/)** - Template engine docs

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[Axum Team](https://github.com/tokio-rs/axum)** - For the excellent web framework
- **[SQLx Team](https://github.com/launchbadge/sqlx)** - For compile-time checked SQL
- **[Tokio Team](https://github.com/tokio-rs/tokio)** - For the robust async runtime
- **Rust Community** - For creating an amazing ecosystem

---

**Built with ❤️ and 🦀 Rust**

*Ready to build something amazing? Your AI coding assistants are configured and waiting to help!* 🤖✨

# 🚀 Axum Base - Production-Ready Rust Web Server Template

A modern, secure, and scalable Rust web server template built with **Axum 0.7**, featuring authentication, database integration, templating, and comprehensive AI coding assistant support.

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/postgresql-supported-blue.svg)](https://www.postgresql.org)
[![SQLx](https://img.shields.io/badge/sqlx-compile%20time%20checked-green.svg)](https://github.com/launchbadge/sqlx)

## ✨ Features

### 🏗️ **Modern Architecture**
- **Axum 0.7** - High-performance async web framework
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

### 3. Install Dependencies & Run
```bash
# Install dependencies
cargo build

# Run database migrations (automatic on first run)
cargo run

# Server starts on http://localhost:3093
```

### 4. Create Test User (Optional)
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
```

## 🛠️ Development Commands

```bash
# Development server with auto-reload
cargo watch -x run

# Run all tests
cargo test

# Run specific test suite
cargo test --test api_tests

# Format code
cargo fmt

# Lint code
cargo clippy

# Check SQLx queries against database
cargo sqlx prepare

# Database operations
cargo run --bin create_user
cargo run --bin set_password
```

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

### Unit Tests
```bash
# Test business logic
cargo test --lib

# Test with coverage
cargo test --lib --features coverage
```

### Integration Tests
```bash
# Test HTTP endpoints
cargo test --test api_tests

# Test CLI utilities
cargo test --test cli_tests
```

### Database Tests
Uses real PostgreSQL with proper setup/teardown for reliable testing.

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
```dockerfile
FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/axum-base .
EXPOSE 3093
CMD ["./axum-base"]
```

### Production Environment
```bash
# Set production environment
export RUST_LOG=info
export DATABASE_URL=postgres://prod_user:prod_pass@db:5432/axum_base

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

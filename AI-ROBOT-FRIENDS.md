# 🤖 AI Robot Friends Configuration Guide

This document provides an overview of all the AI configuration files included in this Axum Base project to optimize your experience with various AI coding assistants and tools.

## 📁 Configuration Files Overview

### Core AI Context Files

#### `.ai-context.md` - Master Documentation
- **Purpose**: Comprehensive project documentation for all AI tools
- **Contents**: Architecture overview, technology decisions, development practices
- **Usage**: Reference document for understanding project structure and conventions
- **Best For**: Getting AI tools up to speed on project context

#### `.warp.md` - WARP Terminal AI
- **Purpose**: Configuration for WARP terminal's built-in AI assistant
- **Contents**: Project context, coding guidelines, module structure
- **Usage**: Automatically loaded by WARP for intelligent terminal suggestions
- **Best For**: Command-line assistance and terminal-based development

### Code Assistant Configurations

#### `.cursorrules` - Cursor AI Editor
- **Purpose**: Comprehensive coding standards for Cursor AI
- **Contents**: Detailed guidelines, security requirements, testing patterns
- **Usage**: Integrated directly into Cursor IDE for context-aware assistance
- **Best For**: Pair programming and code generation in Cursor

#### `.aider.conf.yml` - Aider AI Pair Programming
- **Purpose**: Optimized configuration for Aider command-line AI assistant
- **Contents**: Model settings, file exclusions, custom instructions
- **Usage**: Used by Aider CLI tool for AI-assisted development
- **Best For**: Command-line pair programming and refactoring

#### `.continuerc.json` - Continue.dev VS Code Extension
- **Purpose**: Configuration for Continue.dev AI coding assistant
- **Contents**: Custom commands, context providers, project-specific rules
- **Usage**: Loaded by Continue.dev VS Code extension
- **Best For**: VS Code users wanting advanced AI assistance

#### `.jetbrains-ai.md` - JetBrains AI Assistant
- **Purpose**: Context for IntelliJ IDEA/RustRover AI features
- **Contents**: Project patterns, code completion preferences, refactoring guidelines
- **Usage**: Reference for JetBrains AI features and suggestions
- **Best For**: IntelliJ IDEA and RustRover users

#### `.windsurf.md` - Windsurf AI (formerly Codeium)
- **Purpose**: Configuration for Windsurf AI code completion
- **Contents**: Coding patterns, common suggestions, security guidelines
- **Usage**: Provides context for Windsurf's AI suggestions
- **Best For**: AI-powered code completion and suggestions

#### `.claude.md` - Claude AI
- **Purpose**: Configuration for Anthropic's Claude AI assistant
- **Contents**: Comprehensive project context, coding standards, security guidelines
- **Usage**: Provides detailed context for Claude's code assistance
- **Best For**: In-depth code analysis, refactoring, and architectural discussions

#### `.gemini.md` - Gemini AI
- **Purpose**: Configuration for Google's Gemini AI assistant
- **Contents**: Project architecture, development patterns, optimization guidelines
- **Usage**: Provides structured context for Gemini's development assistance
- **Best For**: Code generation, performance optimization, and testing strategies

### Security & Privacy

#### `.copilotignore` - GitHub Copilot
- **Purpose**: Excludes sensitive files from Copilot context
- **Contents**: File patterns for environment configs, secrets, build artifacts
- **Usage**: Automatically respected by GitHub Copilot
- **Best For**: Protecting sensitive information from AI training/suggestions

## 🛠️ How Each Tool Uses These Files

### WARP Terminal
```bash
# WARP automatically loads .warp.md for terminal AI assistance
# No additional setup required
```

### Cursor Editor
```bash
# Place .cursorrules in project root
# Cursor automatically detects and applies rules
```

### Aider CLI
```bash
# Install Aider: pip install aider-chat
aider  # Automatically uses .aider.conf.yml
```

### Continue.dev (VS Code)
```bash
# Install Continue.dev extension in VS Code
# Copy .continuerc.json to project root
# Extension automatically loads configuration
```

### GitHub Copilot
```bash
# .copilotignore is automatically respected
# No additional configuration needed
```

### Windsurf
```bash
# Install Windsurf extension for your editor
# Reference .windsurf.md for optimal usage patterns
```

### Claude AI
```bash
# Use Claude through web interface or API
# Reference .claude.md for project-specific context
```

### Gemini AI
```bash
# Use Gemini through Google AI Studio or API
# Reference .gemini.md for development guidelines
```

### JetBrains IDEs
```bash
# Enable AI Assistant in IntelliJ IDEA/RustRover
# Reference .jetbrains-ai.md for context
```

## 🎯 Recommended Workflow

### 1. Choose Your Primary AI Tool
- **For VS Code users**: Continue.dev (`.continuerc.json`)
- **For terminal-heavy workflows**: WARP (`.warp.md`) + Aider (`.aider.conf.yml`)
- **For Cursor users**: Built-in AI with `.cursorrules`
- **For JetBrains users**: AI Assistant with `.jetbrains-ai.md`
- **For Claude users**: Anthropic Claude with `.claude.md` context
- **For Gemini users**: Google Gemini with `.gemini.md` guidelines
- **For Windsurf users**: AI code completion with `.windsurf.md`

### 2. Security Setup
- Ensure `.copilotignore` is in place to protect sensitive files
- Review environment variables and add any project-specific secrets to ignore files
- Verify that no `.env` files or credentials are exposed

### 3. Testing the Configuration
```bash
# Test with your chosen AI tool:
# 1. Ask about project architecture
# 2. Request code generation following project patterns
# 3. Verify security guidelines are respected
```

## 🧪 Testing Methodology for AI Assistants

This project uses **selective test threading** for optimal performance. AI assistants should understand this approach when suggesting or generating tests:

### Testing Architecture
- **Unit Tests**: Run in parallel for fast execution (no `#[serial]` needed)
- **Database Tests**: Run serially with `#[serial]` attribute to prevent race conditions
- **Dependencies**: Uses `serial_test` crate for selective serialization

### When to Use `#[serial]` Attribute
```rust
// ✅ Use #[serial] for database/integration tests
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_user_creation_api() {
    // Tests that interact with database
}

// ✅ Do NOT use #[serial] for unit tests
#[test]
fn test_data_validation() {
    // Pure logic tests run in parallel
}
```

### AI Guidelines for Test Generation
1. **Database Tests**: Always include `use serial_test::serial;` and `#[serial]` attribute
2. **Unit Tests**: No special attributes needed - let them run in parallel
3. **Test Isolation**: Generate proper setup/teardown for database tests
4. **Performance**: Prefer unit tests over integration tests when possible

### Test File Patterns
- `src/models.rs` tests: Unit tests (parallel)
- `tests/api_tests.rs`: Integration tests (serial)
- `tests/cli_tests.rs`: CLI tests (serial)

## 🔒 Security Considerations

All configuration files are designed to:
- **Exclude sensitive information** from AI context
- **Enforce security best practices** in code generation
- **Prevent credential exposure** in suggestions
- **Maintain production-ready code standards**

### Protected File Types
- Environment configuration (`.env*`)
- Build artifacts (`/target/`, `*.log`)
- Database credentials and connection strings
- Session secrets and API keys
- Temporary and backup files

## 🚀 Getting Started

1. **Clone this repository** and navigate to the project root
2. **Choose your AI coding assistant** from the list above
3. **Install the tool** following its documentation
4. **Verify the configuration** is loaded correctly
5. **Start coding** with AI assistance optimized for this Axum project!

## 📚 Additional Resources

- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Tokio Documentation](https://docs.rs/tokio/)
- [Tower Sessions Documentation](https://docs.rs/tower-sessions/)

## 🤝 Contributing

When adding new AI tool configurations:
1. Follow the established patterns in existing files
2. Include security considerations and file exclusions
3. Document the configuration in this README
4. Test with the AI tool to ensure optimal performance

---

**Happy coding with your AI robot friends! 🤖✨**

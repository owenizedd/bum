# Contributing to Bum

Thank you for your interest in contributing to Bum! We welcome contributions from everyone.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) 1.3.0+
- [Node.js](https://nodejs.org/) 22+

### Development Setup

```bash
# Clone the repository
git clone https://github.com/owenizedd/bum.git
cd bum

# Install dependencies
bun install

# Build the project
cargo build --release

# Run tests
cargo test --lib
cargo clippy
cargo fmt --check
```

## Making Changes

### Code Style

- Follow Rust best practices and idioms
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Write tests for new features

### Testing

We use **behavior-based testing** (blackbox testing):

```bash
# Run all tests
cargo test --lib

# Run specific test
cargo test test_version_persistence

# Run with output
cargo test -- --nocapture
```

Tests should focus on **user behavior** rather than implementation details:
- ✅ Test what users care about (version switching, persistence)
- ❌ Don't test internal implementation details

### Commit Messages

Use clear, descriptive commit messages:

```bash
# Good
git commit -m "Add retry logic to install script"
git commit -m "Fix version persistence when switching between versions"

# Avoid
git commit -m "fix bug"
git commit -m "update code"
```

## Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** your changes
4. **Test** thoroughly (`cargo test --lib`, `cargo clippy`)
5. **Commit** with clear messages
6. **Push** to your fork (`git push origin feature/amazing-feature`)
7. **Open** a Pull Request

### PR Guidelines

- Describe what your PR does and why
- Reference any related issues
- Ensure all CI checks pass
- Keep PRs focused (one feature/fix per PR)

## Architecture Overview

Bum is built with:
- **Rust** - Core CLI logic and business logic
- **NAPI-RS** - Node.js bindings for npm distribution
- **Bun** - Package manager and build tooling

### Project Structure

```
bum/
├── src/
│   ├── main.rs        # CLI entry point
│   ├── lib.rs         # NAPI bindings
│   ├── commands.rs    # Core commands (use, remove, list)
│   └── bun.rs         # Bun version management
├── .github/workflows/
│   ├── ci.yml         # npm publish workflow
│   └── deploy.yml     # GitHub Releases workflow
├── npm/               # Platform-specific npm packages
└── install.sh         # Installation script
```

## Release Process

See [RELEASE.md](./RELEASE.md) for detailed release instructions.

## Questions?

- Open an [issue](https://github.com/owenizedd/bum/issues) for bug reports or feature requests
- Check existing issues before creating a new one

## License

By contributing, you agree that your contributions will be licensed under the MIT License.


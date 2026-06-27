# AGENTS.md — Bum

> Context for AI agents working on the **Bum** Bun version manager.

## Project overview

Bum is a fast Bun version manager written in Rust and distributed in two ways:

1. **npm package** (`@owenizedd/bum`) — Rust core compiled to a NAPI native binding (`src/lib.rs`) and exposed through `index.js` / `bin.js`.
2. **Standalone binary** — built from `src/main.rs` and released via GitHub Releases; installed by `install.sh`.

Both entry points share the same command logic in `src/commands.rs`.

## Architecture

```
src/
├── main.rs      # Native CLI entry point (binary target)
├── lib.rs       # NAPI-RS entry point for npm distribution
├── commands.rs  # Core commands: use, remove, list, list-remote
├── bun.rs       # Bun version management: download, extract, get active version
├── os.rs        # Architecture/platform detection for Bun release artifacts
└── utils.rs     # Small helpers (folder existence check)
```

### Key directories used at runtime

| Path | Purpose |
|------|---------|
| `~/.bum/bun-versions/<version>/` | Extracted Bun versions stored locally. |
| `~/.bun/bin/bun` | The currently "active" Bun binary (copied/switched by `bum use`). |
| `~/.bum/bin/bum` | Standalone bum binary installed by `install.sh`. |

`~` is resolved with the `resolve-path` crate (respects `$HOME`).

## Build & test commands

```bash
# Install Node/Bun dependencies
bun install

# Build the native NAPI binding (requires cargo in PATH)
bun run build

# Run Rust unit tests
cargo test --lib

# Lint & format
cargo clippy
cargo fmt --check

# npm package integration test (network required — downloads Bun from GitHub)
bash test-npm.sh

# E2E test that verifies fresh-install behavior (missing ~/.bun/bin)
bash e2e-test.sh
```

**Note:** `bun run build` shells out to `cargo metadata governmental --format-version 1`, so `cargo` must be available. In environments where Rust was installed via rustup, source `$HOME/.cargo/env` first.

## Code style

- Follow Rust idioms and keep changes minimal.
- Run `cargo fmt` before committing.
- Keep `cargo clippy` clean.
- Tests are behavior-based: test user-visible outcomes (e.g., " activating a version copies the binary") rather than implementation details.

## Common pitfalls

### `~/.bun/bin` must exist before copying the active binary

`commands::activate_bun` copies the extracted Bun binary to `~/.bun/bin/bun`. If the parent directory does not exist, `fs::copy` fails with:

```text
No such file or directory (os error 2)
```

The command now creates the parent directory before copying.

### `~/.bun/bin` must be on the user's PATH

`bun::get_active_version()` runs `bun -v` to detect the active version. If `~/.bun/bin` is not in `PATH`, this command fails. `install.sh` adds both `~/.bum/bin` (for bum) and `~/.bun/bin` (for bun) to the shell config. The npm package path cannot modify the user's shell config, so the active-version detection falls back to an empty string instead of panicking.

### NAPI build requires a clean Cargo environment

The `@napi-rs/cli` build step invokes `cargo metadata`. If `cargo` is not on `PATH`, the build fails with a Cargo.toml parse error even though the file is valid.

## Release/distribution notes

- Version numbers are kept in sync across `Cargo.toml`, `package.json`, and the optional npm packages under `npm/`.
- `install.sh` hardcodes a `VERSION` variable that must match the GitHub release tag.
- Native bindings are produced by `napi build --platform --release` and committed/released per platform package.

## Useful references

- `technical-docs/CONTRIBUTING.md` — development setup and PR process.
- `technical-docs/RELEASE.md` — release instructions.
- `README.md` — user-facing install and usage docs.

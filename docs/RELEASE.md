# Release Process

This document outlines the process for releasing a new version of Bum.

## Overview

Bum has **two release channels**:

1. **GitHub Releases** - Standalone Rust binaries (via `install.sh`)
2. **npm Registry** - NAPI-RS packages (via `npm install`)

Both are triggered automatically when you push a version tag.

## Version Update Checklist

Before releasing, update the version in **3 files**:

### 1. `Cargo.toml` (line 3)
```toml
[package]
name = "bum"
version = "0.7.0"  # Update this
edition = "2021"
```

### 2. `package.json` (line 3)
```json
{
  "name": "@owenizedd/bum",
  "version": "0.7.0",  // Update this
  ...
}
```

### 3. `install.sh` (line 4)
```bash
#!/usr/bin/env bash
set -euo pipefail

VERSION="v0.7.0"  # Update this (note the 'v' prefix)
```

## Release Steps

### 1. Update Version Numbers

```bash
# Manually update Cargo.toml, package.json, and install.sh
# Or use this helper:
./scripts/bump-version.sh 0.7.0
```

### 2. Update `Cargo.lock`

```bash
cargo build --release
```

### 3. Commit Changes

```bash
git add Cargo.toml Cargo.lock package.json install.sh
git commit -m "Bump version to 0.7.0"
git push origin main
```

### 4. Create and Push Tag

```bash
# Create the tag
git tag v0.7.0

# Push the tag to trigger releases
git push origin v0.7.0
```

## What Happens Automatically

When you push a tag (e.g., `v0.7.0`), two workflows are triggered:

### 1. `deploy.yml` - GitHub Releases

Builds standalone binaries for:
- macOS (Intel + ARM64)
- Linux (x64 + ARM64, glibc + musl)
- Windows (x64)

These are uploaded to **GitHub Releases** and used by `install.sh`.

### 2. `ci.yml` - npm Publish

Builds NAPI-RS packages for:
- All platforms above
- Universal macOS binary (Intel + ARM64 combined)

These are published to **npm registry** as `@owenizedd/bum`.

## Monitoring Release

### Check GitHub Actions

1. Go to [Actions tab](https://github.com/owenizedd/bum/actions)
2. Look for two workflow runs:
   - **Deploy** (GitHub Releases)
   - **CI** (npm publish)
3. Ensure all jobs pass ✅

### Verify GitHub Release

1. Go to [Releases](https://github.com/owenizedd/bum/releases)
2. Verify assets are uploaded (`.tar.gz` and `.zip` files)

### Verify npm Package

```bash
# Check if published
npm view @owenizedd/bum

# Test installation
npx @owenizedd/bum@0.7.0 --version
```

### Verify install.sh

```bash
# Test installation script
curl -fsSL https://github.com/owenizedd/bum/raw/main/install.sh | bash
bum --version
```

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (`1.0.0`) - Breaking changes
- **MINOR** (`0.7.0`) - New features (backward compatible)
- **PATCH** (`0.7.1`) - Bug fixes

### Examples

```bash
# Bug fix
0.6.1 → 0.6.2

# New feature
0.6.1 → 0.7.0

# Breaking change
0.6.1 → 1.0.0
```

## Troubleshooting

### Tag Already Exists

If you need to recreate a tag:

```bash
# Delete local tag
git tag -d v0.7.0

# Delete remote tag
git push origin :refs/tags/v0.7.0

# Create new tag
git tag v0.7.0

# Push new tag
git push origin v0.7.0
```

### Workflow Failed

1. Check the workflow logs in GitHub Actions
2. Fix the issue
3. Commit the fix
4. Delete and recreate the tag (see above)

### npm Publish Failed

Common issues:
- **NPM_TOKEN expired** - Update in GitHub Secrets
- **Version already published** - Bump to next version
- **Publish check failed** - Ensure tag format is `v0.7.0`

## Post-Release

After a successful release:

1. **Announce** on social media / community channels
2. **Update** documentation if needed
3. **Close** related issues/PRs
4. **Monitor** for issues from users

## Release Checklist

Use this checklist for each release:

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `package.json`
- [ ] Update version in `install.sh`
- [ ] Run `cargo build --release` to update `Cargo.lock`
- [ ] Commit changes with message "Bump version to X.Y.Z"
- [ ] Push to main
- [ ] Create tag: `git tag vX.Y.Z`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Verify GitHub Actions pass
- [ ] Verify GitHub Release created
- [ ] Verify npm package published
- [ ] Test `install.sh`
- [ ] Test `npx @owenizedd/bum@X.Y.Z`
- [ ] Announce release


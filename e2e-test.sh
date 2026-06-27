#!/bin/bash
# E2E test that verifies bum works when ~/.bun/bin is missing initially.
# Runs entirely inside an isolated temporary $HOME so it does not touch
# the developer's real Bun installation.
set -euo pipefail

if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi

PROJECT_ROOT="$(cd "$(dirname "$0")" && pwd)"

# Build against the host environment (needs bun, cargo, node on PATH).
echo "🔨 Building native binding..."
(
  cd "$PROJECT_ROOT"
  bun run build
)

echo ""
echo "📦 Testing bin.js --version..."
VERSION=$(node "$PROJECT_ROOT/bin.js" --version)
echo "Version: $VERSION"

# Create an isolated home directory for the real test.
ISOLATED_HOME=$(mktemp -d)
echo ""
echo "🏠 Using isolated HOME: $ISOLATED_HOME"

# Make sure the directory is truly empty.
rm -rf "$ISOLATED_HOME/.bun" "$ISOLATED_HOME/.bum"

run_in_home() {
  HOME="$ISOLATED_HOME" "$@"
}

echo ""
echo "📦 Testing bin.js use 1.3.3 with missing ~/.bun/bin..."
run_in_home node "$PROJECT_ROOT/bin.js" use 1.3.3

echo ""
echo "📦 Verifying bun was installed in isolated home..."
run_in_home "$ISOLATED_HOME/.bun/bin/bun" --version

echo ""
echo "📦 Testing bin.js list in isolated home..."
run_in_home node "$PROJECT_ROOT/bin.js" list

echo ""
echo "🧹 Cleaning up isolated home..."
rm -rf "$ISOLATED_HOME"

echo ""
echo "✅ All e2e tests passed!"

#!/bin/bash
# E2E test that verifies bum works when ~/.bun/bin is missing initially.
# Tests both distribution paths:
#   1. npm package  -> node bin.js
#   2. standalone   -> target/release/bum
# Each path runs entirely inside its own isolated temporary $HOME so the
# developer's real Bun installation is never touched.
set -euo pipefail

if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUM_VERSION="1.3.3"

# Build both distribution artifacts against the host environment.
echo "🔨 Building native npm binding..."
(
  cd "$PROJECT_ROOT"
  bun run build
)

echo ""
echo "🔨 Building standalone binary..."
(
  cd "$PROJECT_ROOT"
  cargo build --release
)

# $1 = distribution name, $2... = bum executable + any fixed args
test_distribution() {
  local name="$1"
  shift
  local bum_cmd=("$@")
  local isolated_home
  isolated_home=$(mktemp -d)

  echo ""
  echo "═══════════════════════════════════════════════════════════════"
  echo "  Testing distribution: $name"
  echo "  Command: ${bum_cmd[*]}"
  echo "  Isolated HOME: $isolated_home"
  echo "═══════════════════════════════════════════════════════════════"

  run_in_home() {
    HOME="$isolated_home" "$@"
  }

  echo ""
  echo "📦 ${name}: --version"
  run_in_home "${bum_cmd[@]}" --version

  echo ""
  echo "📦 ${name}: use ${BUM_VERSION} with missing ~/.bun/bin"
  run_in_home "${bum_cmd[@]}" use "$BUM_VERSION"

  echo ""
  echo "📦 ${name}: verify active bun version"
  run_in_home "$isolated_home/.bun/bin/bun" --version

  echo ""
  echo "📦 ${name}: list installed versions"
  run_in_home "${bum_cmd[@]}" list

  echo ""
  echo "🧹 ${name}: cleaning up isolated home"
  rm -rf "$isolated_home"
}

# Test npm package path.
test_distribution "npm package" node "$PROJECT_ROOT/bin.js"

# Test standalone binary path.
test_distribution "standalone binary" "$PROJECT_ROOT/target/release/bum"

echo ""
echo "✅ All e2e tests passed for both npm and standalone binary distributions!"

#!/bin/bash
# E2E test that simulates the reported user scenario:
#   npm install -g @owenizedd/bum
# on a system where Bun is NOT pre-installed.
#
# Uses a multi-stage container build: the native binding is compiled in a
# builder image that has Rust + Bun, then copied into a runtime image that
# only has Node.js. Inside the runtime container bum is installed globally
# from a tarball, and `bum use 1.3.3` is run in an isolated $HOME.
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENGINE="${DOCKER:-podman}"
TAG="bum-npm-install-e2e"

echo "🐳 Building multi-stage container image..."
$ENGINE build \
  --build-arg "BUM_VERSION=1.3.3" \
  -t "$TAG" \
  -f "$PROJECT_ROOT/e2e/Dockerfile.npm-install" \
  "$PROJECT_ROOT"

echo ""
echo "🚀 Running npm-install E2E test inside container (no pre-installed Bun)..."
$ENGINE run --rm "$TAG"

echo ""
echo "✅ npm install E2E test passed! bum works on a system without pre-installed Bun."

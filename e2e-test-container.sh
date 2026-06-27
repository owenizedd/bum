#!/bin/bash
# Run the E2E test inside a container for full filesystem isolation.
# Uses podman by default; set DOCKER=docker to use Docker instead.
set -euo pipefail

ENGINE="${DOCKER:-podman}"
TAG="bum-e2e"

echo "🐳 Building container image with $ENGINE..."
$ENGINE build -t "$TAG" -f e2e/Dockerfile .

echo ""
echo "🚀 Running E2E test inside container..."
$ENGINE run --rm "$TAG"

echo ""
echo "✅ Containerized E2E test passed!"

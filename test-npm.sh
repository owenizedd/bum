#!/bin/bash
# Integration test for npm package
# This tests the full flow: build -> version check -> use command

set -e

echo "🔨 Building native binding..."
bun run build

echo ""
echo "📦 Testing bin.js --version..."
VERSION=$(node bin.js --version)
echo "Version: $VERSION"

echo ""
echo "📦 Testing bin.js use 1.3.3..."
node bin.js use 1.3.3

echo ""
echo "📦 Verifying bun version..."
~/.bun/bin/bun --version

echo ""
echo "✅ All npm package tests passed!"


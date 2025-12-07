#!/bin/bash
# Quick local test to verify bin.js works with the native binding

set -e

echo "🔨 Building native binding..."
bun run build

echo "📦 Testing bin.js --version..."
node bin.js --version

echo "📦 Testing bin.js use 1.3.3..."
node bin.js use 1.3.3

echo "✅ npm package test passed!"


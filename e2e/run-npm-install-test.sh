#!/bin/bash
# Runs inside the npm-install E2E container.
set -euo pipefail

ISOLATED_HOME=$(mktemp -d)
echo "🏠 Isolated HOME: $ISOLATED_HOME"

HOME="$ISOLATED_HOME" bum --version
HOME="$ISOLATED_HOME" bum use "$BUM_VERSION"
HOME="$ISOLATED_HOME" "$ISOLATED_HOME/.bun/bin/bun" --version
HOME="$ISOLATED_HOME" bum list

rm -rf "$ISOLATED_HOME"
echo "✅ npm-install E2E test passed inside container"

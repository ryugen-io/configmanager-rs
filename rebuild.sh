#!/bin/bash
# Rebuild script for Config Manager frontend
# Run this after modifying theme.toml

set -e

cd "$(dirname "$0")"

echo "🎨 Rebuilding Config Manager with custom theme..."
echo ""

# Build frontend
cd frontend
echo "📦 Building WASM frontend with Trunk..."
trunk build --release

echo ""
echo "✅ Rebuild complete!"
echo "🌐 Refresh your browser at http://10.1.1.30:3000 to see changes"

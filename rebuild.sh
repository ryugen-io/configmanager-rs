#!/bin/bash
# Rebuild script for Config Manager
# Performs full build cycle: format, build backend and frontend

set -e

cd "$(dirname "$0")"

echo "[INFO] Rebuilding Config Manager..."
echo ""

# Format code
echo "[INFO] Formatting code..."
cargo fmt --all

# Build backend with auditable
echo "[INFO] Building backend with cargo-auditable..."
cargo auditable build --release

# Build frontend
cd frontend
echo "[INFO] Formatting frontend..."
cargo fmt

echo "[INFO] Building WASM frontend with Trunk..."
trunk build --release

cd ..

echo ""
echo "[OK] Rebuild complete!"
echo "[INFO] Backend built with audit metadata"
echo "[INFO] Refresh your browser at http://10.1.1.30:3000 to see changes"

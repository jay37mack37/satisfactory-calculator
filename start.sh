#!/usr/bin/env bash
# Satisfactory Calculator - Startup Script (Linux/macOS)
#
# The frontend is fully self-contained: it bundles a JavaScript port of the
# recipe engine, so it needs no backend. This script launches the Vite dev
# server. The Rust backend in ./backend is an optional reference server you
# can run separately with `cargo run` from that directory.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "============================================"
echo "  Satisfactory Calculator - Startup"
echo "============================================"
echo ""

# Check for Node.js
if ! command -v node &> /dev/null; then
    echo "[ERROR] Node.js not found. Install: https://nodejs.org"
    exit 1
fi

# Install frontend dependencies if needed
if [ ! -d "$SCRIPT_DIR/frontend/node_modules" ]; then
    echo "[INFO] Installing frontend dependencies..."
    cd "$SCRIPT_DIR/frontend"
    npm install
fi

# Start frontend dev server
echo "[1/1] Starting frontend (Vite dev server)..."
cd "$SCRIPT_DIR/frontend"
npm run dev &
FRONTEND_PID=$!
echo "       Frontend PID: $FRONTEND_PID"

echo ""
echo "============================================"
echo "  Frontend: http://localhost:5173 (or 5174)"
echo ""
echo "  The app runs entirely in the browser."
echo "  Press Ctrl+C to stop."
echo "============================================"
echo ""

# Trap Ctrl+C to kill the process
trap "echo 'Shutting down...'; kill $FRONTEND_PID 2>/dev/null; exit 0" SIGINT SIGTERM

wait $FRONTEND_PID 2>/dev/null || true
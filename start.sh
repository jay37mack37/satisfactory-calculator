#!/usr/bin/env bash
# Satisfactory Calculator - Startup Script (Linux/macOS)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "============================================"
echo "  Satisfactory Calculator - Startup"
echo "============================================"
echo ""

# Check for Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Cargo not found. Install Rust: https://rustup.rs"
    exit 1
fi

# Check for Node.js
if ! command -v node &> /dev/null; then
    echo "[ERROR] Node.js not found. Install: https://nodejs.org"
    exit 1
fi

# Start backend in background
echo "[1/2] Starting backend (Rust/Axum on port 3000)..."
cd "$SCRIPT_DIR/backend"
cargo run &
BACKEND_PID=$!
echo "       Backend PID: $BACKEND_PID"

# Wait a moment for backend to compile/start
echo "       Compiling backend... (this may take a moment on first run)"
sleep 3

# Install frontend dependencies if needed
if [ ! -d "$SCRIPT_DIR/frontend/node_modules" ]; then
    echo "[INFO] Installing frontend dependencies..."
    cd "$SCRIPT_DIR/frontend"
    npm install
fi

# Start frontend in background
echo "[2/2] Starting frontend (Vite dev server)..."
cd "$SCRIPT_DIR/frontend"
npm run dev &
FRONTEND_PID=$!
echo "       Frontend PID: $FRONTEND_PID"

echo ""
echo "============================================"
echo "  Both servers are starting!"
echo ""
echo "  Backend:  http://localhost:3000"
echo "  Frontend: http://localhost:5173 (or 5174)"
echo ""
echo "  Press Ctrl+C to stop both servers."
echo "============================================"
echo ""

# Trap Ctrl+C to kill both processes
trap "echo 'Shutting down...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit 0" SIGINT SIGTERM

# Wait for either process to exit
wait -n $BACKEND_PID $FRONTEND_PID 2>/dev/null || true

echo "[WARN] One of the servers stopped. Shutting down..."
kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true
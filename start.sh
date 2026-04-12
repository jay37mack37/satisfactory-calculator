#!/usr/bin/env bash
# Satisfactory Calculator - Startup Script (Linux/macOS)
#
# Usage: ./start.sh [OPTIONS]
#   --host HOST           Bind address for both servers (default: 0.0.0.0)
#   --port PORT           Backend port (default: 3000)
#   --frontend-port PORT  Frontend port (default: 5173)
#   --help                Show this help message
#
# Environment variables:
#   BACKEND_HOST     Backend bind address (default: 0.0.0.0)
#   BACKEND_PORT     Backend port (default: 3000)
#   FRONTEND_HOST    Frontend bind address (default: 0.0.0.0)
#   FRONTEND_PORT    Frontend port (default: 5173)
#   VITE_API_URL     API base URL for frontend (default: http://localhost:BACKEND_PORT)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Defaults from env vars or fallbacks
BACKEND_HOST="${BACKEND_HOST:-0.0.0.0}"
BACKEND_PORT="${BACKEND_PORT:-3000}"
FRONTEND_HOST="${FRONTEND_HOST:-0.0.0.0}"
FRONTEND_PORT="${FRONTEND_PORT:-5173}"

# Parse CLI arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --host)
            BACKEND_HOST="$2"
            FRONTEND_HOST="$2"
            shift 2
            ;;
        --port)
            BACKEND_PORT="$2"
            shift 2
            ;;
        --frontend-port)
            FRONTEND_PORT="$2"
            shift 2
            ;;
        --help|-h)
            echo ""
            echo "  Satisfactory Calculator - Startup Script"
            echo ""
            echo "  Usage: ./start.sh [OPTIONS]"
            echo ""
            echo "  Options:"
            echo "    --host HOST           Bind address for both servers (default: 0.0.0.0)"
            echo "    --port PORT           Backend port (default: 3000)"
            echo "    --frontend-port PORT  Frontend port (default: 5173)"
            echo "    --help                Show this help message"
            echo ""
            echo "  Environment variables:"
            echo "    BACKEND_HOST     Backend bind address (default: 0.0.0.0)"
            echo "    BACKEND_PORT     Backend port (default: 3000)"
            echo "    FRONTEND_HOST    Frontend bind address (default: 0.0.0.0)"
            echo "    FRONTEND_PORT    Frontend port (default: 5173)"
            echo "    VITE_API_URL     API base URL for frontend (default: http://localhost:BACKEND_PORT)"
            echo ""
            echo "  Examples:"
            echo "    ./start.sh                              # defaults: 0.0.0.0:3000 + 0.0.0.0:5173"
            echo "    ./start.sh --port 8080                  # backend on port 8080"
            echo "    ./start.sh --host 127.0.0.1             # localhost only"
            echo "    ./start.sh --port 8080 --frontend-port 3000"
            exit 0
            ;;
        *)
            echo "[ERROR] Unknown option: $1"
            echo "Run './start.sh --help' for usage."
            exit 1
            ;;
    esac
done

echo "============================================"
echo "  Satisfactory Calculator - Startup"
echo "============================================"
echo ""
echo "  Backend:   ${BACKEND_HOST}:${BACKEND_PORT}"
echo "  Frontend:  ${FRONTEND_HOST}:${FRONTEND_PORT}"
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
echo "[1/2] Starting backend..."
cd "$SCRIPT_DIR/backend"
BACKEND_HOST="$BACKEND_HOST" BACKEND_PORT="$BACKEND_PORT" cargo run -- "$BACKEND_HOST" "$BACKEND_PORT" &
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
echo "[2/2] Starting frontend..."
cd "$SCRIPT_DIR/frontend"
VITE_API_URL="http://localhost:${BACKEND_PORT}" \
FRONTEND_HOST="$FRONTEND_HOST" \
FRONTEND_PORT="$FRONTEND_PORT" \
npm run dev &
FRONTEND_PID=$!
echo "       Frontend PID: $FRONTEND_PID"

echo ""
echo "============================================"
echo "  Both servers are starting!"
echo ""
echo "  Backend:   http://${BACKEND_HOST}:${BACKEND_PORT}"
echo "  Frontend:  http://${FRONTEND_HOST}:${FRONTEND_PORT}"
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
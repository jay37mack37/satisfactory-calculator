@echo off
echo ============================================
echo   Satisfactory Calculator - Startup
echo ============================================
echo.

:: Check for Rust/Cargo
where cargo >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Cargo not found. Install Rust: https://rustup.rs
    exit /b 1
)

:: Check for Node.js
where node >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Node.js not found. Install: https://nodejs.org
    exit /b 1
)

:: Start backend
echo [1/2] Starting backend (Rust/Axum on port 3000)...
start "Satisfactory Calculator - Backend" cmd /k "cd /d "%~dp0backend" && cargo run"

:: Wait a moment for backend to compile/start
echo        Compiling backend... (this may take a moment on first run)
timeout /t 3 /nobreak >nul

:: Install frontend dependencies if needed
if not exist "%~dp0frontend\node_modules" (
    echo [INFO] Installing frontend dependencies...
    cd /d "%~dp0frontend"
    call npm install
)

:: Start frontend
echo [2/2] Starting frontend (Vite dev server)...
start "Satisfactory Calculator - Frontend" cmd /k "cd /d "%~dp0frontend" && npm run dev"

echo.
echo ============================================
echo   Both servers are starting!
echo.
echo   Backend:  http://localhost:3000
echo   Frontend:  http://localhost:5173 (or 5174)
echo.
echo   Close the terminal windows to stop.
echo ============================================
echo.
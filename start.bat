@echo off
REM Satisfactory Calculator - Startup Script (Windows)
REM
REM The frontend is fully self-contained: it bundles a JavaScript port of the
REM recipe engine, so it needs no backend. This script launches the Vite dev
REM server. The Rust backend in .\backend is an optional reference server you
REM can run separately with `cargo run` from that directory.

echo ============================================
echo   Satisfactory Calculator - Startup
echo ============================================
echo.

:: Check for Node.js
where node >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Node.js not found. Install: https://nodejs.org
    exit /b 1
)

:: Install frontend dependencies if needed
if not exist "%~dp0frontend\node_modules" (
    echo [INFO] Installing frontend dependencies...
    cd /d "%~dp0frontend"
    call npm install
)

:: Start frontend
echo [1/1] Starting frontend (Vite dev server)...
start "Satisfactory Calculator - Frontend" cmd /k "cd /d "%~dp0frontend" && npm run dev"

echo.
echo ============================================
echo   Frontend: http://localhost:5173 (or 5174)
echo.
echo   The app runs entirely in the browser.
echo   Close the terminal window to stop.
echo ============================================
echo.
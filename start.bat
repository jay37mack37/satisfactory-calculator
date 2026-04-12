@echo off
setlocal enabledelayedexpansion

echo ============================================
echo   Satisfactory Calculator - Startup
echo ============================================
echo.

:: Defaults — override with args or env vars
set "BACKEND_HOST=%BACKEND_HOST%"
set "BACKEND_PORT=%BACKEND_PORT%"
set "FRONTEND_HOST=%FRONTEND_HOST%"
set "FRONTEND_PORT=%FRONTEND_PORT%"

if "%BACKEND_HOST%"=="" set "BACKEND_HOST=0.0.0.0"
if "%BACKEND_PORT%"=="" set "BACKEND_PORT=3000"
if "%FRONTEND_HOST%"=="" set "FRONTEND_HOST=0.0.0.0"
if "%FRONTEND_PORT%"=="" set "FRONTEND_PORT=5173"

:: Parse CLI args: start.bat [backend_port] [frontend_port] [host]
:argloop
if "%~1"=="" goto :doneargs
if "%~1"=="--help" goto :usage
if "%~1"=="-h" goto :usage
if "%~1"=="--port" (
    shift
    set "BACKEND_PORT=%~1"
    shift
    goto :argloop
)
if "%~1"=="--frontend-port" (
    shift
    set "FRONTEND_PORT=%~1"
    shift
    goto :argloop
)
if "%~1"=="--host" (
    shift
    set "BACKEND_HOST=%~1"
    set "FRONTEND_HOST=%~1"
    shift
    goto :argloop
)
:: Positional: port, frontend_port, host
if not defined POS1 (
    set "BACKEND_PORT=%~1"
    set "POS1=1"
) else if not defined POS2 (
    set "FRONTEND_PORT=%~1"
    set "POS2=1"
) else if not defined POS3 (
    set "BACKEND_HOST=%~1"
    set "FRONTEND_HOST=%~1"
    set "POS3=1"
)
shift
goto :argloop
:doneargs

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

echo   Backend:  %BACKEND_HOST%:%BACKEND_PORT%
echo   Frontend: %FRONTEND_HOST%:%FRONTEND_PORT%
echo.

:: Start backend
echo [1/2] Starting backend...
start "Satisfactory Calculator - Backend" cmd /k "cd /d "%~dp0backend" && set BACKEND_HOST=%BACKEND_HOST%&& set BACKEND_PORT=%BACKEND_PORT%&& cargo run -- %BACKEND_HOST% %BACKEND_PORT%"

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
echo [2/2] Starting frontend...
set "VITE_API_URL=http://localhost:%BACKEND_PORT%"
start "Satisfactory Calculator - Frontend" cmd /k "cd /d "%~dp0frontend" && set FRONTEND_HOST=%FRONTEND_HOST%&& set FRONTEND_PORT=%FRONTEND_PORT%&& set VITE_API_URL=http://localhost:%BACKEND_PORT%&& npm run dev"

echo.
echo ============================================
echo   Both servers are starting!
echo.
echo   Backend:   http://%BACKEND_HOST%:%BACKEND_PORT%
echo   Frontend:  http://%FRONTEND_HOST%:%FRONTEND_PORT%
echo.
echo   Close the terminal windows to stop.
echo ============================================
echo.
goto :eof

:usage
echo.
echo   Usage: start.bat [OPTIONS]
echo.
echo   Options:
echo     --host HOST           Bind address for both servers (default: 0.0.0.0)
echo     --port PORT           Backend port (default: 3000)
echo     --frontend-port PORT  Frontend port (default: 5173)
echo.
echo   Environment variables:
echo     BACKEND_HOST    Backend bind address (default: 0.0.0.0)
echo     BACKEND_PORT    Backend port (default: 3000)
echo     FRONTEND_HOST   Frontend bind address (default: 0.0.0.0)
echo     FRONTEND_PORT   Frontend port (default: 5173)
echo     VITE_API_URL    API base URL for frontend (default: http://localhost:BACKEND_PORT)
echo.
echo   Examples:
echo     start.bat                          # defaults: 0.0.0.0:3000 + 0.0.0.0:5173
echo     start.bat --port 8080              # backend on port 8080
echo     start.bat --host 127.0.0.1         # localhost only
echo     start.bat --port 8080 --frontend-port 3000
echo.
@echo off
echo ========================================
echo  ChainForge - Run Script
echo ========================================
echo.

REM Check if binary exists
if not exist "target\release\chain-forge.exe" (
    echo ERROR: Binary not found. Please run build.bat first.
    pause
    exit /b 1
)

echo Starting ChainForge server...
echo.

REM Check if .env exists
if not exist ".env" (
    echo WARNING: .env file not found
    echo Please create .env from .env.example
    echo.
)

echo Press Ctrl+C to stop the server
echo.

target\release\chain-forge.exe

pause

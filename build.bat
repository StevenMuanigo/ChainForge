@echo off
echo ========================================
echo  ChainForge - Build Script
echo ========================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo [1/3] Checking Rust installation...
cargo --version
echo.

echo [2/3] Running cargo check...
cargo check
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo check failed
    pause
    exit /b 1
)
echo.

echo [3/3] Building in release mode...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Build failed
    pause
    exit /b 1
)
echo.

echo ========================================
echo  Build completed successfully!
echo  Executable: target\release\chain-forge.exe
echo ========================================
echo.

pause

@echo off
setlocal enabledelayedexpansion

echo ========================================
echo  Markdown My Project - Release Builder
echo ========================================
echo.

REM Set paths
set "PROJECT_DIR=%~dp0"
set "BIN_DIR=%PROJECT_DIR%bin"
set "RELEASE_DIR=%PROJECT_DIR%target\release"

REM Clean and create bin directory
echo [1/4] Cleaning bin directory...
if exist "%BIN_DIR%" rmdir /s /q "%BIN_DIR%"
mkdir "%BIN_DIR%"

REM Build release version
echo [2/4] Building release version...
cd "%PROJECT_DIR%"
cargo build --release
if errorlevel 1 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)
echo      Build successful!

REM Copy executable
echo [3/4] Copying executable...
copy "%RELEASE_DIR%\markdown_my_project.exe" "%BIN_DIR%\" >nul
if errorlevel 1 (
    echo ERROR: Failed to copy executable!
    pause
    exit /b 1
)
echo      Copied: markdown_my_project.exe

REM Copy configuration files
echo [4/4] Copying configuration files...

REM Copy languages.yml
copy "%PROJECT_DIR%languages.yml" "%BIN_DIR%\" >nul
echo      Copied: languages.yml

REM Copy log4rs.yml
copy "%PROJECT_DIR%log4rs.yml" "%BIN_DIR%\" >nul
echo      Copied: log4rs.yml

REM Copy projects directory
mkdir "%BIN_DIR%\projects"
xcopy "%PROJECT_DIR%projects\*.yml" "%BIN_DIR%\projects\" /Y >nul
echo      Copied: projects\*.yml

echo.
echo ========================================
echo  Build Complete!
echo ========================================
echo.
echo  Output directory: %BIN_DIR%
echo.
echo  Files:
echo    - markdown_my_project.exe
echo    - languages.yml
echo    - log4rs.yml
echo    - projects\
echo.
echo  Usage:
echo    cd bin
echo    markdown_my_project.exe
echo.
pause

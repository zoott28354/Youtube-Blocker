@echo off
:: YouTube Blocker - Avvio sviluppo (richiede privilegi admin)
:: Author : zoott28354
:: GitHub : https://github.com/zoott28354/Youtube-Blocker
:: ---

:: Controlla privilegi admin
net session >nul 2>&1
if errorlevel 1 (
    echo Elevazione privilegi admin richiesta...
    powershell -NoProfile -Command "Start-Process cmd -ArgumentList '/k \"%~f0\"' -Verb RunAs"
    exit /b
)

echo ==========================================
echo  YouTube Blocker - Dev mode
echo ==========================================
cd /d "%~dp0..\site-blocker"

echo.
echo Avvio Tauri dev...
call npm run tauri dev
pause

@echo off
:: YouTube Blocker - Build installer NSIS
:: Author : zoott28354
:: GitHub : https://github.com/zoott28354/Youtube-Blocker
:: ---

echo ==========================================
echo  YouTube Blocker - Build installer
echo ==========================================
cd /d "%~dp0..\site-blocker"
set CARGO_TARGET_DIR=%CD%\target

:: Legge versione da tauri.conf.json
for /f "tokens=2 delims=:, " %%V in ('findstr /r "\"version\"" src-tauri\tauri.conf.json') do (
    set RAW=%%V
)
set APP_VER=%RAW:"=%
echo Versione: %APP_VER%
echo.

echo Build in corso (puo' richiedere qualche minuto)...
call npm run tauri build
if errorlevel 1 (
    echo.
    echo ERRORE: build fallita. Controlla l'output sopra.
    pause
    exit /b 1
)

echo.
echo ==========================================
echo  Build completata.
echo  Output: target\release\bundle\nsis\
echo ==========================================
pause

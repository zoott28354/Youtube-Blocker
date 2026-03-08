@echo off
:: YouTube Blocker - Build portable (.exe senza installer)
:: Author : zoott28354
:: GitHub : https://github.com/zoott28354/Youtube-Blocker
:: ---

echo ==========================================
echo  YouTube Blocker - Build portable
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

echo Build portable in corso...
call npm run tauri:portable
if errorlevel 1 (
    echo.
    echo ERRORE: build fallita. Controlla l'output sopra.
    pause
    exit /b 1
)

:: Copia e rinomina exe con versione nella root del repo
set SRC=target\release\youtube-blocker.exe
set DST=..\YouTubeBlocker_v%APP_VER%.exe

if exist "%SRC%" (
    copy /y "%SRC%" "%DST%" >nul
    echo.
    echo ==========================================
    echo  Build completata.
    echo  Portable: YouTubeBlocker_v%APP_VER%.exe
    echo ==========================================
) else (
    echo.
    echo ERRORE: exe non trovato in %SRC%
)
pause

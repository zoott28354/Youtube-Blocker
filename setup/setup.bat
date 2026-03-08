@echo off
setlocal enabledelayedexpansion
rem YouTube Blocker - Setup
rem Author : zoott28354
rem GitHub : https://github.com/zoott28354/Youtube-Blocker

:menu
cls
echo ==========================================
echo  YouTube Blocker - Setup
echo ==========================================
echo.
echo  [1] Installa dipendenze sviluppo (npm install)
echo  [2] Scarica installer  da GitHub Releases
echo  [3] Scarica portable   da GitHub Releases
echo  [4] Esci
echo.
set /p SCELTA=Scelta:

if "!SCELTA!"=="1" goto dev_setup
if "!SCELTA!"=="2" goto download_releases
if "!SCELTA!"=="3" goto download_releases
if "!SCELTA!"=="4" exit /b 0
echo Scelta non valida.
pause
goto menu

rem ------------------------------------------
:dev_setup
echo.
echo Controllo prerequisiti...
echo.

rem Verifica Node.js
node --version >nul 2>&1
if errorlevel 1 (
    echo [MANCANTE] Node.js non trovato.
    echo Download: https://nodejs.org
    echo.
    set /p OPEN=Aprire il browser per scaricare Node.js? (s/n):
    if /i "!OPEN!"=="s" start "" "https://nodejs.org"
    echo.
    echo Installa Node.js, riavvia il terminale e riesegui setup.bat.
    pause
    exit /b 1
)
for /f %%V in ('node --version') do set NODE_VER=%%V
echo [OK] Node.js !NODE_VER!

rem Verifica Cargo / Rust
cargo --version >nul 2>&1
if errorlevel 1 (
    echo [MANCANTE] Rust/Cargo non trovato.
    echo Download: https://rustup.rs
    echo.
    echo Nota: dopo aver installato Rust occorre anche
    echo       "Microsoft C++ Build Tools" (Visual Studio Installer).
    echo.
    set /p OPEN=Aprire il browser per scaricare Rust? (s/n):
    if /i "!OPEN!"=="s" start "" "https://rustup.rs"
    echo.
    echo Installa Rust, riavvia il terminale e riesegui setup.bat.
    pause
    exit /b 1
)
for /f %%V in ('cargo --version') do set CARGO_VER=%%V
echo [OK] !CARGO_VER!

echo.
echo Prerequisiti OK. Installazione dipendenze npm...
echo.
cd /d "%~dp0..\site-blocker"
call npm install
if errorlevel 1 (
    echo.
    echo ERRORE: npm install fallito. Controlla l'output sopra.
    pause
    exit /b 1
)

echo.
echo ==========================================
echo  Setup completato.
echo  Usa dev.bat per avviare in modalita' sviluppo.
echo ==========================================
pause
exit /b 0

rem ------------------------------------------
:download_releases
echo.
echo Apertura pagina GitHub Releases...
start "" "https://github.com/zoott28354/Youtube-Blocker/releases"
echo.
echo Scarica il file .exe dalla pagina che si e' aperta nel browser.
pause
goto menu

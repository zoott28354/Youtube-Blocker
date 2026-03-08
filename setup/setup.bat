@echo off
setlocal enabledelayedexpansion
rem YouTube Blocker - Setup
rem Author : zoott28354
rem GitHub : https://github.com/zoott28354/Youtube-Blocker

:menu
cls

rem --- Rileva prerequisiti ---
set NODE_OK=0
set CARGO_OK=0
set NODE_VER=n/d
set CARGO_VER=n/d

where node >nul 2>&1
if errorlevel 1 goto check_cargo
for /f %%V in ('node --version') do set NODE_VER=%%V
set NODE_OK=1

:check_cargo
where cargo >nul 2>&1
if errorlevel 1 goto show_menu
for /f "tokens=2" %%V in ('cargo --version') do set CARGO_VER=%%V
set CARGO_OK=1

:show_menu
echo ==========================================
echo  YouTube Blocker - Setup
echo ==========================================
echo.
echo  Prerequisiti rilevati:
if "!NODE_OK!"=="1" (
    echo  [OK]       Node.js    !NODE_VER!
) else (
    echo  [MANCANTE] Node.js    - https://nodejs.org
)
if "!CARGO_OK!"=="1" (
    echo  [OK]       Rust/Cargo !CARGO_VER!
) else (
    echo  [MANCANTE] Rust/Cargo - https://rustup.rs
)
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
if "!NODE_OK!"=="1" goto node_ok
echo [MANCANTE] Node.js non trovato.
echo Download: https://nodejs.org
echo.
set /p OPEN=Aprire il browser per scaricare Node.js? [s/n]:
if /i "!OPEN!"=="s" start "" "https://nodejs.org"
echo.
echo Installa Node.js, riavvia il terminale e riesegui setup.bat.
pause
exit /b 1

:node_ok
echo [OK] Node.js !NODE_VER!
if "!CARGO_OK!"=="1" goto cargo_ok
echo [MANCANTE] Rust/Cargo non trovato.
echo Download: https://rustup.rs
echo.
echo Nota: dopo aver installato Rust occorre anche
echo       "Microsoft C++ Build Tools" (Visual Studio Installer).
echo.
set /p OPEN=Aprire il browser per scaricare Rust? [s/n]:
if /i "!OPEN!"=="s" start "" "https://rustup.rs"
echo.
echo Installa Rust, riavvia il terminale e riesegui setup.bat.
pause
exit /b 1

:cargo_ok
echo [OK] Rust/Cargo !CARGO_VER!
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

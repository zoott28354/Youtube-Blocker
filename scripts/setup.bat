@echo off
:: YouTube Blocker - Setup dipendenze
:: Author : zoott28354
:: GitHub : https://github.com/zoott28354/Youtube-Blocker
:: ---

echo ==========================================
echo  YouTube Blocker - Setup
echo ==========================================
cd /d "%~dp0..\site-blocker"

echo.
echo Installazione dipendenze npm...
call npm install
if errorlevel 1 (
    echo ERRORE: npm install fallito. Controlla che Node.js sia installato.
    pause
    exit /b 1
)

echo.
echo ==========================================
echo  Setup completato.
echo  Usa dev.bat per avviare in modalita' sviluppo.
echo ==========================================
pause

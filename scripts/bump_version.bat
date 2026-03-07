@echo off
:: YouTube Blocker - Aggiornamento versione
:: Aggiorna: tauri.conf.json, Cargo.toml, package.json
:: Author : zoott28354
:: GitHub : https://github.com/zoott28354/Youtube-Blocker
:: ---

echo ==========================================
echo  YouTube Blocker - Bump versione
echo ==========================================
cd /d "%~dp0.."

echo.
set /p NEW_VER=Nuova versione (es. 1.1.0):

if "%NEW_VER%"=="" (
    echo Versione non specificata. Annullato.
    pause
    exit /b 1
)

echo.
echo Aggiornamento versione a %NEW_VER%...

:: tauri.conf.json
powershell -NoProfile -Command ^
    "(Get-Content 'site-blocker\src-tauri\tauri.conf.json' -Raw) -replace '\"version\":\s*\"[^\"]+\"', '\"version\": \"%NEW_VER%\"' | Set-Content 'site-blocker\src-tauri\tauri.conf.json' -NoNewline"
echo  [OK] tauri.conf.json

:: Cargo.toml (solo la riga version del [package], non delle dipendenze)
powershell -NoProfile -Command ^
    "$f = Get-Content 'site-blocker\src-tauri\Cargo.toml'; $done = $false; $f = $f | ForEach-Object { if (-not $done -and $_ -match '^version\s*=') { $done = $true; 'version = \"%NEW_VER%\"' } else { $_ } }; $f | Set-Content 'site-blocker\src-tauri\Cargo.toml'"
echo  [OK] Cargo.toml

:: package.json
powershell -NoProfile -Command ^
    "(Get-Content 'site-blocker\package.json' -Raw) -replace '\"version\":\s*\"[^\"]+\"', '\"version\": \"%NEW_VER%\"' | Set-Content 'site-blocker\package.json' -NoNewline"
echo  [OK] package.json

echo.
echo ==========================================
echo  Versione aggiornata a %NEW_VER%
echo  Ricorda di committare i file modificati.
echo ==========================================
pause

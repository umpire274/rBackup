@echo off
setlocal enabledelayedexpansion

:: === SELEZIONE PIATTAFORMA ===
echo ========================================
echo Seleziona la piattaforma da compilare:
echo   [1] Windows (x86_64-pc-windows-msvc)
echo   [2] Linux   (x86_64-unknown-linux-gnu)
echo   [3] macOS   (x86_64-apple-darwin)
echo   [4] macOS Apple Silicon (aarch64-apple-darwin)
echo ========================================
set /p CHOICE=Inserisci il numero della piattaforma [1-4]:

if "%CHOICE%"=="1" (
    set "TARGET=x86_64-pc-windows-msvc"
    set "EXT=.exe"
    set "ARCHIVE_EXT=.zip"
    set "USE_ZIP=1"
) else if "%CHOICE%"=="2" (
    set "TARGET=x86_64-unknown-linux-gnu"
    set "EXT="
    set "ARCHIVE_EXT=.tar.gz"
    set "USE_ZIP=0"
) else if "%CHOICE%"=="3" (
    set "TARGET=x86_64-apple-darwin"
    set "EXT="
    set "ARCHIVE_EXT=.tar.gz"
    set "USE_ZIP=0"
) else if "%CHOICE%"=="4" (
    set "TARGET=aarch64-apple-darwin"
    set "EXT="
    set "ARCHIVE_EXT=.tar.gz"
    set "USE_ZIP=0"
) else (
    echo Errore: selezione non valida.
    exit /b 1
)

echo(
echo Inizio procedura...
echo(

:: === CONFIGURAZIONE ===
set "APP_NAME=rbackup"

:: Estrae la versione da Cargo.toml
for /f "tokens=2 delims== " %%A in ('findstr /b "version" Cargo.toml') do (
    set "VERSION=%%~A"
    set "VERSION=!VERSION:"=!"
)

:: Directory di output
set "DIST_DIR=%CD%\dist"
set "OUT_DIR=%CD%\target\%TARGET%\release"
set "TEMP_DIR=%CD%\temp_build"

:: Genera nome archivio e file di log con timestamp
for /f %%A in ('powershell -NoProfile -Command "Get-Date -Format yyyyMMdd_HHmmss"') do set "TS=%%A"
set "ARCHIVE_BASE=%APP_NAME%-%VERSION%-%TARGET%"
set "ARCHIVE_FULL=%DIST_DIR%\%ARCHIVE_BASE%%ARCHIVE_EXT%"
set "CHECKSUM_FILE=%DIST_DIR%\%ARCHIVE_BASE%.sha256"
set "LOG_FILE=%DIST_DIR%\%ARCHIVE_BASE%-%TS%.log"

:: Crea directory di output
if not exist "%DIST_DIR%" mkdir "%DIST_DIR%"
if exist "%TEMP_DIR%" rmdir /s /q "%TEMP_DIR%"
echo [BUILDING] Creazione directory temporanea... >> "%LOG_FILE%"
mkdir "%TEMP_DIR%"

echo [BUILDING] Compilazione per %TARGET%... >> "%LOG_FILE%"
cargo build --release --target %TARGET% >> "%LOG_FILE%" 2>&1
if errorlevel 1 (
    echo [ERROR] Compilazione fallita. Vedi il log: %LOG_FILE%
    exit /b 1
)

:: Copia file nel TEMP_DIR
copy "%OUT_DIR%\%APP_NAME%%EXT%" "%TEMP_DIR%\" >> "%LOG_FILE%" 2>&1
copy "LICENSE" "%TEMP_DIR%\" >> "%LOG_FILE%" 2>&1
copy "README.md" "%TEMP_DIR%\" >> "%LOG_FILE%" 2>&1
copy "CHANGELOG.md" "%TEMP_DIR%\" >> "%LOG_FILE%" 2>&1

:: Rimuove archivio se già esiste
if exist "%ARCHIVE_FULL%" del "%ARCHIVE_FULL%"

:: Comprimi
if "%USE_ZIP%"=="1" (
    echo [ARCHIVE] Compressione in ZIP... >> "%LOG_FILE%"
    powershell -NoProfile -Command "Compress-Archive -Path '%TEMP_DIR%\*' -DestinationPath '%ARCHIVE_FULL%'" >> "%LOG_FILE%" 2>&1
) else (
    echo [ARCHIVE] Compressione in TAR.GZ... >> "%LOG_FILE%"
    powershell -NoProfile -Command ^
        "$dest='%ARCHIVE_FULL%';" ^
        "$temp='%TEMP_DIR%';" ^
        "if (Test-Path $dest) { Remove-Item $dest };" ^
        "tar -czf $dest -C $temp ."
)

:: Rimuove archivio se già esiste
if exist "%CHECKSUM_FILE%" del "%CHECKSUM_FILE%"

:: Calcola SHA256
echo [CHECKSUM] Calcolo SHA256... >> "%LOG_FILE%"
powershell -NoProfile -Command "(Get-FileHash -Path '%ARCHIVE_FULL%' -Algorithm SHA256).Hash | Out-File -Encoding ascii -FilePath '%CHECKSUM_FILE%'"

:: Rimuove la cartella temporanea
rmdir /s /q "%TEMP_DIR%"

:: Messaggi finali
echo [OK] Archivio creato: %ARCHIVE_FULL%
echo [OK] Checksum salvato: %CHECKSUM_FILE%
echo [OK] Log salvato: %LOG_FILE%

echo(
echo Fine procedura.

endlocal

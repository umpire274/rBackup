@echo off
setlocal enabledelayedexpansion

:: === SELEZIONE PIATTAFORMA ===
echo ========================================
echo Seleziona la piattaforma da compilare:
echo   [1] Windows (x86_64-pc-windows-msvc)
echo   [2] Linux   (x86_64-unknown-linux-gnu)
echo   [3] macOS   (x86_64-apple-darwin)
echo   [4] macOS Apple Silicon (aarch64-apple-darwin)
echo   [5] FreeBSD (x86_64-unknown-freebsd)
echo ========================================
set /p CHOICE=Inserisci il numero della piattaforma [1-5] o [0] per uscire:

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
) else if "%CHOICE%"=="5" (
    set "TARGET=x86_64-unknown-freebsd"
    set "EXT="
    set "ARCHIVE_EXT=.tar.gz"
    set "USE_ZIP=0"
) else if "%CHOICE%"=="0" (
	exit 0
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

:: === Verifica e installazione automatica del target ===
echo  [INFO] Verifica se il target "%TARGET%" è installato... >> "%LOG_FILE%"
rustup target list --installed | findstr /C:"%TARGET%" >nul
if errorlevel 1 (
	echo [INFO] Target non trovato. Installo... >> "%LOG_FILE%"
	rustup target add %TARGET% >> "%LOG_FILE%" 2>&1
	if errorlevel 1 (
		echo [ERROR] Impossibile installare il target %TARGET%. Lo script viene interrotto. >> "%LOG_FILE%"
		echo Errore: installazione del target fallita. Vedi il log: %LOG_FILE%
		exit /b 1
	) else (
		echo [OK] Target %TARGET% installato con successo. >> "%LOG_FILE%"
	)
) else (
	echo [OK] Target %TARGET% già installato. >> "%LOG_FILE%"
)

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

:: Calcola SHA256
if exist "%CHECKSUM_FILE%" del "%CHECKSUM_FILE%"
echo [CHECKSUM] Calcolo SHA256... >> "%LOG_FILE%"
powershell -NoProfile -Command "(Get-FileHash -Path '%ARCHIVE_FULL%' -Algorithm SHA256).Hash | Out-File -Encoding ascii -FilePath '%CHECKSUM_FILE%'"

:: Firma con GPG (se disponibile)
set "SIGNATURE_FILE=%ARCHIVE_FULL%.sig"
where gpg >nul 2>&1
if %errorlevel% equ 0 (
    echo [GPG] Firma dell'archivio... >> "%LOG_FILE%"
    gpg --output "%SIGNATURE_FILE%" --detach-sign --armor "%ARCHIVE_FULL%" >> "%LOG_FILE%" 2>&1
    if exist "%SIGNATURE_FILE%" (
        echo [OK] Firma GPG salvata: %SIGNATURE_FILE%
    ) else (
        echo [WARN] Firma GPG non riuscita.
    )
) else (
    echo [WARN] gpg non trovato. Firma GPG saltata.
)

:: Rimuove la cartella temporanea
rmdir /s /q "%TEMP_DIR%"

:: Messaggi finali
echo [OK] Archivio creato: %ARCHIVE_FULL%
echo [OK] Checksum salvato: %CHECKSUM_FILE%
if exist "%SIGNATURE_FILE%" echo [OK] Firma GPG salvata: %SIGNATURE_FILE%
echo [OK] Log salvato: %LOG_FILE%

echo(
echo Fine procedura.

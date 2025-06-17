#!/bin/bash
set -e

# === SELEZIONE PIATTAFORMA ===
echo "========================================"
echo "Seleziona la piattaforma da compilare:"
echo "  [1] Windows (x86_64-pc-windows-msvc)"
echo "  [2] Linux   (x86_64-unknown-linux-gnu)"
echo "  [3] macOS   (x86_64-apple-darwin)"
echo "  [4] macOS Apple Silicon (aarch64-apple-darwin)"
echo "========================================"
read -rp "Inserisci il numero della piattaforma [1-4]: " CHOICE

case $CHOICE in
    1)
        TARGET="x86_64-pc-windows-msvc"
        EXT=".exe"
        ARCHIVE_EXT=".zip"
        USE_ZIP=1
        ;;
    2)
        TARGET="x86_64-unknown-linux-gnu"
        EXT=""
        ARCHIVE_EXT=".tar.gz"
        USE_ZIP=0
        ;;
    3)
        TARGET="x86_64-apple-darwin"
        EXT=""
        ARCHIVE_EXT=".tar.gz"
        USE_ZIP=0
        ;;
    4)
        TARGET="aarch64-apple-darwin"
        EXT=""
        ARCHIVE_EXT=".tar.gz"
        USE_ZIP=0
        ;;
    *)
        echo "Errore: selezione non valida."
        exit 1
        ;;
esac

echo
APP_NAME="rbackup"
VERSION=$(grep '^version =' Cargo.toml | cut -d '"' -f2)
TS=$(date +"%Y%m%d_%H%M%S")

DIST_DIR="$(pwd)/dist"
OUT_DIR="$(pwd)/target/$TARGET/release"
TEMP_DIR="$(pwd)/temp_build"
ARCHIVE_BASE="$APP_NAME-$VERSION-$TARGET"
ARCHIVE_FULL="$DIST_DIR/$ARCHIVE_BASE$ARCHIVE_EXT"
CHECKSUM_FILE="$DIST_DIR/$ARCHIVE_BASE.sha256"
LOG_FILE="$DIST_DIR/$ARCHIVE_BASE-$TS.log"

mkdir -p "$DIST_DIR"
rm -rf "$TEMP_DIR"
echo "[BUILDING] Creazione directory temporanea..." | tee -a "$LOG_FILE"
mkdir "$TEMP_DIR"

echo "[BUILDING] Compilazione per $TARGET..." | tee -a "$LOG_FILE"
cargo build --release --target "$TARGET" >> "$LOG_FILE" 2>&1

cp "$OUT_DIR/$APP_NAME$EXT" "$TEMP_DIR/"
cp LICENSE README.md CHANGELOG.md "$TEMP_DIR/"

if [ -f "$ARCHIVE_FULL" ]; then rm "$ARCHIVE_FULL"; fi

if [ "$USE_ZIP" = "1" ]; then
    echo "[ARCHIVE] Compressione in ZIP..." | tee -a "$LOG_FILE"
    (cd "$TEMP_DIR" && zip -r "$ARCHIVE_FULL" .) >> "$LOG_FILE" 2>&1
else
    echo "[ARCHIVE] Compressione in TAR.GZ..." | tee -a "$LOG_FILE"
    tar -czf "$ARCHIVE_FULL" -C "$TEMP_DIR" . >> "$LOG_FILE" 2>&1
fi

if [ -f "$CHECKSUM_FILE" ]; then rm "$CHECKSUM_FILE"; fi

echo "[CHECKSUM] Calcolo SHA256..." | tee -a "$LOG_FILE"
sha256sum "$ARCHIVE_FULL" | cut -d ' ' -f1 > "$CHECKSUM_FILE"

rm -rf "$TEMP_DIR"

echo "[OK] Archivio creato: $ARCHIVE_FULL"
echo "[OK] Checksum salvato: $CHECKSUM_FILE"
echo "[OK] Log salvato: $LOG_FILE"
echo
echo "Fine procedura."
exit 0

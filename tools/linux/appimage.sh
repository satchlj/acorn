#!/usr/bin/env bash
# Builds dist/<Name>-<version>-<arch>.AppImage for this machine's architecture with
# linuxdeploy (fetched into dist/appimage-tools on first use). With <PREFIX>_UPDATE_KEY_FILE
# set to the ed25519 release key, also writes the .sig that acorn-update checks before it
# installs an update. Used by the release workflow, and by hand.
set -euo pipefail
source ./acorn-app.env
KEY_VAR=${ENV_PREFIX}_UPDATE_KEY_FILE
KEY_FILE=${!KEY_VAR:-}

VERSION=$(awk -F\" '/^version/ {print $2; exit}' Cargo.toml)
ARCH=$(uname -m)
OUT="dist/$APP_NAME-$VERSION-$ARCH.AppImage"

cargo build --profile dist -p "$APP_PACKAGE"

TOOLS=dist/appimage-tools
LD="$TOOLS/linuxdeploy-$ARCH.AppImage"
mkdir -p "$TOOLS"
if [[ ! -x $LD ]]; then
  curl -fsSL -o "$LD" "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-$ARCH.AppImage"
  chmod +x "$LD"
fi

STAGE=dist/appimage-stage
rm -rf dist/AppDir "$STAGE" "$OUT"
mkdir -p "$STAGE"
cp "target/dist/$APP_CARGO_BIN" "$STAGE/$APP_BIN"
sed "s|@BIN@|$APP_BIN|" "$LINUX_DESKTOP" > "$STAGE/$APP_ID.desktop"
cp "$LINUX_PNG" "$STAGE/$APP_ID.png"

# Extract-and-run: CI machines and some desktops have no FUSE.
APPIMAGE_EXTRACT_AND_RUN=1 ARCH="$ARCH" LDAI_OUTPUT="$OUT" OUTPUT="$OUT" "$LD" \
  --appdir dist/AppDir \
  --executable "$STAGE/$APP_BIN" \
  --desktop-file "$STAGE/$APP_ID.desktop" \
  --icon-file "$STAGE/$APP_ID.png" \
  --output appimage
rm -rf dist/AppDir "$STAGE"

if [[ -n $KEY_FILE ]]; then
  openssl pkeyutl -sign -inkey "$KEY_FILE" -rawin -in "$OUT" -out "$OUT.sig"
fi
echo "$OUT"

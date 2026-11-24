#!/usr/bin/env bash
# Builds the app and installs it for the current user: the binary in ~/.local/bin, a desktop
# entry (which also registers $LINUX_SCHEME links, if set) and the icon. PREFIX=/usr/local
# installs system-wide instead (run as root).
set -euo pipefail
source ./acorn-app.env

PREFIX="${PREFIX:-$HOME/.local}"
BIN="$PREFIX/bin/$APP_BIN"
APPS="$PREFIX/share/applications"
ICONS="$PREFIX/share/icons/hicolor/512x512/apps"

cargo build --profile dist -p "$APP_PACKAGE"

install -Dm755 "target/dist/$APP_CARGO_BIN" "$BIN"
install -Dm644 "$LINUX_PNG" "$ICONS/$APP_ID.png"
mkdir -p "$APPS"
sed "s|@BIN@|$BIN|" "$LINUX_DESKTOP" > "$APPS/$APP_ID.desktop"

command -v update-desktop-database >/dev/null && update-desktop-database "$APPS" || true
command -v gtk-update-icon-cache >/dev/null && gtk-update-icon-cache -qtf "$PREFIX/share/icons/hicolor" || true
if [[ -n ${LINUX_SCHEME:-} ]]; then
  command -v xdg-mime >/dev/null && xdg-mime default "$APP_ID.desktop" "x-scheme-handler/$LINUX_SCHEME" || true
fi

echo "Installed $BIN"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="${VERSION:-$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT_DIR/Cargo.toml" | head -n1)}"
BIN_PATH="${BIN_PATH:-$ROOT_DIR/target/release/bongocat-studio}"
DIST_DIR="${DIST_DIR:-$ROOT_DIR/dist}"
ARCH="$(uname -m)"

if [[ ! -x "$BIN_PATH" ]]; then
    echo "error: release binary not found at $BIN_PATH (run 'make release' first)" >&2
    exit 1
fi

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

PKG_NAME="bongocat-studio-${VERSION}-linux-${ARCH}"
STAGE="$WORK_DIR/$PKG_NAME"
mkdir -p "$STAGE" "$DIST_DIR"

install -Dm755 "$BIN_PATH" "$STAGE/bongocat-studio"
install -Dm644 "$ROOT_DIR/assets/icons/bongocat-studio.svg" "$STAGE/bongocat-studio.svg"
install -Dm644 "$ROOT_DIR/assets/desktop/bongocat-studio.desktop.in" "$STAGE/bongocat-studio.desktop.in"
install -Dm644 "$ROOT_DIR/LICENSE" "$STAGE/LICENSE"
install -Dm644 "$ROOT_DIR/THIRD_PARTY_LICENSES.md" "$STAGE/THIRD_PARTY_LICENSES.md"
install -Dm644 "$ROOT_DIR/README.md" "$STAGE/README.md"

cat <<'EOF' > "$STAGE/install.sh"
#!/usr/bin/env sh
set -eu
PREFIX="${PREFIX:-$HOME/.local}"
install -Dm755 bongocat-studio "$PREFIX/bin/bongocat-studio"
install -Dm644 bongocat-studio.svg "$PREFIX/share/icons/hicolor/scalable/apps/bongocat-studio.svg"
install -Dm644 LICENSE "$PREFIX/share/licenses/bongocat-studio/LICENSE"
install -Dm644 THIRD_PARTY_LICENSES.md "$PREFIX/share/licenses/bongocat-studio/THIRD_PARTY_LICENSES.md"
mkdir -p "$PREFIX/share/applications"
sed "s|@BIN_PATH@|$PREFIX/bin/bongocat-studio|g" bongocat-studio.desktop.in \
  > "$PREFIX/share/applications/bongocat-studio.desktop"
chmod 644 "$PREFIX/share/applications/bongocat-studio.desktop"
command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$PREFIX/share/applications" || true
if command -v gtk4-update-icon-cache >/dev/null 2>&1; then
  gtk4-update-icon-cache -f -t "$PREFIX/share/icons/hicolor" 2>/dev/null || true
elif command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -f -t "$PREFIX/share/icons/hicolor" 2>/dev/null || true
fi
echo "Installed BongoCat Studio to $PREFIX"
EOF

cat <<'EOF' > "$STAGE/uninstall.sh"
#!/usr/bin/env sh
set -eu
PREFIX="${PREFIX:-$HOME/.local}"
rm -f "$PREFIX/bin/bongocat-studio"
rm -f "$PREFIX/share/applications/bongocat-studio.desktop"
rm -f "$PREFIX/share/icons/hicolor/scalable/apps/bongocat-studio.svg"
rm -f "$PREFIX/share/icons/hicolor/icon-theme.cache"
rm -rf "$PREFIX/share/licenses/bongocat-studio"
rm -f "$HOME/.config/autostart/bongocat-studio.desktop"
command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$PREFIX/share/applications" || true
echo "Uninstalled BongoCat Studio from $PREFIX"
EOF

chmod 755 "$STAGE/install.sh" "$STAGE/uninstall.sh"

OUT_FILE="$DIST_DIR/${PKG_NAME}.tar.gz"
tar -C "$WORK_DIR" -czf "$OUT_FILE" "$PKG_NAME"
echo "Built tarball: $OUT_FILE"

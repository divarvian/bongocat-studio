#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="${VERSION:-$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT_DIR/Cargo.toml" | head -n1)}"
BIN_PATH="${BIN_PATH:-$ROOT_DIR/target/release/bongocat-studio}"
DIST_DIR="${DIST_DIR:-$ROOT_DIR/dist}"

if [[ ! -x "$BIN_PATH" ]]; then
    echo "error: release binary not found at $BIN_PATH (run 'make release' first)" >&2
    exit 1
fi

case "$(uname -m)" in
    x86_64)  DEB_ARCH="amd64" ;;
    aarch64) DEB_ARCH="arm64" ;;
    *)       DEB_ARCH="$(uname -m)" ;;
esac

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

PKG_ROOT="$WORK_DIR/pkg"
mkdir -p "$PKG_ROOT/DEBIAN" "$DIST_DIR"

install -Dm755 "$BIN_PATH" "$PKG_ROOT/usr/bin/bongocat-studio"
install -Dm644 "$ROOT_DIR/assets/icons/bongocat-studio.svg" \
    "$PKG_ROOT/usr/share/icons/hicolor/scalable/apps/bongocat-studio.svg"
install -Dm644 "$ROOT_DIR/packaging/deb/copyright" \
    "$PKG_ROOT/usr/share/doc/bongocat-studio/copyright"
install -Dm644 "$ROOT_DIR/LICENSE" \
    "$PKG_ROOT/usr/share/licenses/bongocat-studio/LICENSE"
install -Dm644 "$ROOT_DIR/THIRD_PARTY_LICENSES.md" \
    "$PKG_ROOT/usr/share/licenses/bongocat-studio/THIRD_PARTY_LICENSES.md"

mkdir -p "$PKG_ROOT/usr/share/applications"
sed 's|@BIN_PATH@|/usr/bin/bongocat-studio|g' \
    "$ROOT_DIR/assets/desktop/bongocat-studio.desktop.in" \
    > "$PKG_ROOT/usr/share/applications/bongocat-studio.desktop"
chmod 644 "$PKG_ROOT/usr/share/applications/bongocat-studio.desktop"

INSTALLED_SIZE="$(du -sk "$PKG_ROOT/usr" | awk '{print $1}')"

sed -e "s|@VERSION@|$VERSION|g" \
    -e "s|@ARCH@|$DEB_ARCH|g" \
    -e "s|@INSTALLED_SIZE@|$INSTALLED_SIZE|g" \
    "$ROOT_DIR/packaging/deb/control.in" \
    > "$PKG_ROOT/DEBIAN/control"

install -Dm755 "$ROOT_DIR/packaging/deb/postinst" "$PKG_ROOT/DEBIAN/postinst"
install -Dm755 "$ROOT_DIR/packaging/deb/postrm" "$PKG_ROOT/DEBIAN/postrm"

(cd "$PKG_ROOT" && find usr -type f -print0 | sort -z | xargs -0 md5sum > DEBIAN/md5sums)
chmod 644 "$PKG_ROOT/DEBIAN/control" "$PKG_ROOT/DEBIAN/md5sums"

OUT_FILE="$DIST_DIR/bongocat-studio_${VERSION}_${DEB_ARCH}.deb"
rm -f "$OUT_FILE"

if command -v dpkg-deb >/dev/null 2>&1; then
    dpkg-deb --root-owner-group --build "$PKG_ROOT" "$OUT_FILE"
else
    # Pure POSIX fallback using ar + tar + xz (works cleanly on Fedora/Arch without dpkg)
    printf '2.0\n' > "$WORK_DIR/debian-binary"
    (cd "$PKG_ROOT/DEBIAN" && tar --owner=0 --group=0 -cJf "$WORK_DIR/control.tar.xz" .)
    (cd "$PKG_ROOT" && tar --owner=0 --group=0 -cJf "$WORK_DIR/data.tar.xz" ./usr)
    (cd "$WORK_DIR" && ar rcsD "$OUT_FILE" debian-binary control.tar.xz data.tar.xz)
fi

echo "Built DEB package: $OUT_FILE"

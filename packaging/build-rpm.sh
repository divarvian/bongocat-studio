#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="${VERSION:-$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT_DIR/Cargo.toml" | head -n1)}"
BIN_PATH="${BIN_PATH:-$ROOT_DIR/target/release/bongocat-studio}"
DIST_DIR="${DIST_DIR:-$ROOT_DIR/dist}"

if ! command -v rpmbuild >/dev/null 2>&1; then
    echo "error: rpmbuild not found (install rpm-build on Fedora or rpm on Ubuntu/Debian)" >&2
    exit 1
fi

if [[ ! -x "$BIN_PATH" ]]; then
    echo "error: release binary not found at $BIN_PATH (run 'make release' first)" >&2
    exit 1
fi

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

STAGE_DIR="$WORK_DIR/stage"
RPMBUILD_DIR="$WORK_DIR/rpmbuild"
mkdir -p "$STAGE_DIR" "$WORK_DIR/tmp" "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS} "$DIST_DIR"

install -Dm755 "$BIN_PATH" "$STAGE_DIR/bongocat-studio"
install -Dm644 "$ROOT_DIR/assets/icons/bongocat-studio.svg" "$STAGE_DIR/bongocat-studio.svg"
install -Dm644 "$ROOT_DIR/LICENSE" "$STAGE_DIR/LICENSE"
install -Dm644 "$ROOT_DIR/THIRD_PARTY_LICENSES.md" "$STAGE_DIR/THIRD_PARTY_LICENSES.md"
sed 's|@BIN_PATH@|/usr/bin/bongocat-studio|g' \
    "$ROOT_DIR/assets/desktop/bongocat-studio.desktop.in" \
    > "$STAGE_DIR/bongocat-studio.desktop"

rpmbuild -bb \
    --define "_topdir $RPMBUILD_DIR" \
    --define "_tmppath $WORK_DIR/tmp" \
    --define "_pkg_version $VERSION" \
    --define "_stage_dir $STAGE_DIR" \
    --define "dist %{nil}" \
    "$ROOT_DIR/packaging/rpm/bongocat-studio.spec"

RPM_FILE="$(find "$RPMBUILD_DIR/RPMS" -name '*.rpm' -type f | head -n1)"
if [[ -z "$RPM_FILE" ]]; then
    echo "error: rpmbuild did not produce an RPM file" >&2
    exit 1
fi

OUT_NAME="$(basename "$RPM_FILE")"
cp "$RPM_FILE" "$DIST_DIR/$OUT_NAME"
echo "Built RPM package: $DIST_DIR/$OUT_NAME"

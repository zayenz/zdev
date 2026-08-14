#!/bin/sh
set -eu

fail() {
    printf 'cargo-dist bootstrap failed: %s\n' "$*" >&2
    exit 1
}

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P) ||
    fail "cannot locate the bootstrap script"
repository_root=$(dirname -- "$script_dir")
config="$repository_root/dist-workspace.toml"
checksums="$script_dir/cargo-dist-checksums.txt"

version=$(awk -F '"' \
    '/^cargo-dist-version = / { print $2; exit }' \
    "$config")
[ -n "$version" ] || fail "cannot read the pinned cargo-dist version"

case "$(uname -s):$(uname -m)" in
    Darwin:arm64|Darwin:aarch64) target=aarch64-apple-darwin ;;
    Darwin:x86_64) target=x86_64-apple-darwin ;;
    Linux:aarch64|Linux:arm64) target=aarch64-unknown-linux-gnu ;;
    Linux:x86_64) target=x86_64-unknown-linux-gnu ;;
    *) fail "unsupported bootstrap host: $(uname -s) $(uname -m)" ;;
esac

archive="cargo-dist-$target.tar.xz"
expected_hash=$(awk -v archive="$archive" \
    '$2 == archive { print $1; exit }' \
    "$checksums")
[ -n "$expected_hash" ] || fail "no reviewed SHA-256 for $archive"

scratch=$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/zdev-cargo-dist.XXXXXX")
cleanup() {
    cleanup_status=$?
    trap - 0 1 2 15
    rm -rf "$scratch" || cleanup_status=1
    exit "$cleanup_status"
}
trap cleanup 0 1 2 15

url="https://github.com/axodotdev/cargo-dist/releases/download/v$version/$archive"
curl --proto '=https' --tlsv1.2 -LsSf "$url" -o "$scratch/$archive"

if command -v sha256sum >/dev/null 2>&1; then
    actual_hash=$(sha256sum "$scratch/$archive" | awk '{ print $1 }')
elif command -v shasum >/dev/null 2>&1; then
    actual_hash=$(shasum -a 256 "$scratch/$archive" | awk '{ print $1 }')
else
    fail "SHA-256 verification requires sha256sum or shasum"
fi
[ "$actual_hash" = "$expected_hash" ] || fail "SHA-256 mismatch for $archive"

mkdir -p "$scratch/unpacked"
tar -xJf "$scratch/$archive" -C "$scratch/unpacked"
binary="$scratch/unpacked/cargo-dist-$target/dist"
[ -x "$binary" ] || fail "archive does not contain an executable dist binary"

install_root=${CARGO_HOME:-"$HOME/.cargo"}
mkdir -p "$install_root/bin"
cp "$binary" "$install_root/bin/dist"
chmod +x "$install_root/bin/dist"
actual_version=$("$install_root/bin/dist" --version)
case "$actual_version" in
    "cargo-dist $version"|"dist $version") ;;
    *) fail "expected cargo-dist $version, found $actual_version" ;;
esac
printf '%s\n' "$actual_version"

if [ -n "${GITHUB_PATH:-}" ]; then
    printf '%s\n' "$install_root/bin" >> "$GITHUB_PATH"
fi

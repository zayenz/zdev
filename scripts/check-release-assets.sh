#!/bin/sh
set -eu

fail() {
    printf 'release asset check failed: %s\n' "$*" >&2
    exit 1
}

if [ "$#" -lt 2 ] || [ "$#" -gt 3 ]; then
    fail "usage: check-release-assets.sh DIRECTORY APP_NAME [--with-dist-manifest]"
fi

asset_root=$1
app_name=$2
include_dist_manifest=false
if [ "$#" -eq 3 ]; then
    [ "$3" = "--with-dist-manifest" ] ||
        fail "usage: check-release-assets.sh DIRECTORY APP_NAME [--with-dist-manifest]"
    include_dist_manifest=true
fi
[ -d "$asset_root" ] || fail "asset directory does not exist: $asset_root"

scratch=$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/zdev-release-assets.XXXXXX")
cleanup() {
    cleanup_status=$?
    trap - 0 1 2 15
    rm -rf "$scratch" || cleanup_status=1
    exit "$cleanup_status"
}
trap cleanup 0 1 2 15

expected_files="$scratch/expected-files"
expected_checksums="$scratch/expected-checksums"
actual_files="$scratch/actual-files"
actual_checksums="$scratch/actual-checksums"
targets='aarch64-apple-darwin
aarch64-unknown-linux-musl
x86_64-apple-darwin
x86_64-unknown-linux-musl'

{
    printf '%s\n' \
        sha256.sum \
        source.tar.gz \
        source.tar.gz.sha256
    if [ "$include_dist_manifest" = true ]; then
        printf '%s\n' dist-manifest.json
    fi
    for target in $targets; do
        printf '%s\n' \
            "${app_name}-${target}.tar.xz" \
            "${app_name}-${target}.tar.xz.sha256"
    done
} | LC_ALL=C sort > "$expected_files"

find "$asset_root" \
    -maxdepth 1 \
    -type f \
    ! -name '*-dist-manifest.json' \
    -exec basename {} \; |
    LC_ALL=C sort > "$actual_files"
diff -u "$expected_files" "$actual_files" ||
    fail "asset filenames differ from the release allowlist"

{
    printf '%s\n' source.tar.gz
    for target in $targets; do
        printf '%s\n' "${app_name}-${target}.tar.xz"
    done
} | LC_ALL=C sort > "$expected_checksums"

awk 'NF >= 2 { sub(/^\*/, "", $2); print $2 }' \
    "$asset_root/sha256.sum" |
    LC_ALL=C sort > "$actual_checksums"
diff -u "$expected_checksums" "$actual_checksums" ||
    fail "sha256.sum members differ from the archive allowlist"

(
    cd "$asset_root"
    sha256sum --check sha256.sum
    while IFS= read -r archive; do
        companion="${archive}.sha256"
        companion_member=$(awk '
            NF {
                if (NF != 2 || count != 0) {
                    invalid = 1
                }
                count += 1
                member = $2
            }
            END {
                if (invalid || count != 1) {
                    exit 1
                }
                sub(/^\*/, "", member)
                print member
            }
        ' "$companion") || fail "invalid companion checksum: $companion"
        [ "$companion_member" = "$archive" ] ||
            fail "$companion names $companion_member instead of $archive"
        sha256sum --check "$companion"
    done < "$expected_checksums"
)

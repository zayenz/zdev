#!/bin/sh
set -eu

fail() {
    printf 'release workflow sync failed: %s\n' "$*" >&2
    exit 1
}

mode=write
if [ "$#" -gt 1 ]; then
    fail "usage: sync-release-workflow.sh [--check]"
fi
if [ "$#" -eq 1 ]; then
    [ "$1" = "--check" ] ||
        fail "usage: sync-release-workflow.sh [--check]"
    mode=check
fi

repository_root=$(git rev-parse --show-toplevel 2>/dev/null) ||
    fail "not inside a Git repository"
cd "$repository_root"

workflow=.github/workflows/release.yml
hardening_patch=scripts/release-workflow-hardening.patch
dist_config=dist-workspace.toml
dist_path=${DIST_BIN:-}
if [ -z "$dist_path" ]; then
    dist_path=$(command -v dist 2>/dev/null) ||
        fail "dist is not on PATH"
fi

expected_dist=$(awk -F '"' \
    '/^cargo-dist-version = / { print $2; exit }' \
    dist-workspace.toml)
[ -n "$expected_dist" ] || fail "cannot read the pinned dist version"
actual_dist=$("$dist_path" --version)
case "$actual_dist" in
    "cargo-dist $expected_dist"|"dist $expected_dist") ;;
    *) fail "expected dist $expected_dist, found $actual_dist" ;;
esac

scratch=$(mktemp -d "${TMPDIR:-/tmp}/zdev-release-workflow.XXXXXX")
cp "$dist_config" "$scratch/dist-workspace.toml"

cleanup_sync() {
    exit_code=$?
    trap - 0 1 2 15
    cp "$scratch/dist-workspace.toml" "$dist_config" || exit_code=1
    if [ "$mode" = check ] && [ -f "$scratch/release.yml" ]; then
        cp "$scratch/release.yml" "$workflow"
    fi
    rm -rf "$scratch" || exit_code=1
    exit "$exit_code"
}
trap cleanup_sync 0 1 2 15

if [ "$mode" = check ]; then
    cp "$workflow" "$scratch/release.yml"
fi

# `allow-dirty = ["ci"]` tells normal dist commands that zdev owns the final
# workflow bytes. Temporarily remove that setting so `generate` still produces
# the upstream baseline that the reviewed patch must apply to.
awk '!/^[[:space:]]*allow-dirty[[:space:]]*=/' \
    "$scratch/dist-workspace.toml" \
    > "$dist_config"
"$dist_path" generate
cp "$scratch/dist-workspace.toml" "$dist_config"
patch --dry-run --silent -p1 < "$hardening_patch"
patch --silent -p1 < "$hardening_patch"

if [ "$mode" = check ]; then
    if ! cmp -s "$scratch/release.yml" "$workflow"; then
        diff -u "$scratch/release.yml" "$workflow" >&2 || true
        fail "the committed release workflow is out of date"
    fi
    printf 'release workflow is current\n'
else
    printf 'generated and hardened %s\n' "$workflow"
fi

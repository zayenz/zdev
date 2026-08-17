#!/bin/sh
set -eu

fail() {
    printf 'release check failed: %s\n' "$*" >&2
    exit 1
}

if [ "$#" -ne 1 ]; then
    echo "usage: check-release.sh vMAJOR.MINOR.PATCH[-PRERELEASE]" >&2
    exit 2
fi

tag=$1
case "$tag" in
    v[0-9]*.[0-9]*.[0-9]*) ;;
    *) fail "release tag must start with v and contain a semantic version" ;;
esac
expected_version=${tag#v}

repository_root=$(git rev-parse --show-toplevel 2>/dev/null) ||
    fail "not inside a Git repository"
cd "$repository_root"

check_clean() {
    changes=$(git status --porcelain=v1 --untracked-files=all)
    [ -z "$changes" ] || {
        printf '%s\n' "$changes" >&2
        fail "the Git worktree is not clean"
    }
}

check_clean

package_id=$(cargo pkgid --locked)
package_version=${package_id##*#}
package_version=${package_version##*@}
[ "$package_version" = "$expected_version" ] ||
    fail "tag version $expected_version does not match Cargo version $package_version"

expected_dist=$(awk -F '"' \
    '/^cargo-dist-version = / { print $2; exit }' \
    dist-workspace.toml)
[ -n "$expected_dist" ] || fail "cannot read the pinned dist version"

dist_path=$(command -v dist 2>/dev/null) || fail "dist is not on PATH"
actual_dist=$("$dist_path" --version)
case "$actual_dist" in
    "cargo-dist $expected_dist"|"dist $expected_dist") ;;
    *) fail "expected dist $expected_dist, found $actual_dist" ;;
esac

scratch=$(mktemp -d "${TMPDIR:-/tmp}/zdev-release-check.XXXXXX")
cleanup() {
    cleanup_status=$?
    trap - 0
    rm -rf "$scratch" || cleanup_status=1
    exit "$cleanup_status"
}
trap cleanup 0

cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
cargo package --locked
cargo install --path . --locked --root "$scratch/install"
sh scripts/release-smoke.sh \
    "$scratch/install/bin/zdev" \
    "$expected_version"

DIST_BIN="$dist_path" scripts/sync-release-workflow.sh --check
"$dist_path" plan \
    --tag="$tag" \
    --output-format=json \
    --no-local-paths \
    > "$scratch/dist-plan.json"

check_clean
printf 'release check passed: %s\n' "$tag"

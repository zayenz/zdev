#!/bin/sh
set -eu

fail() {
    printf 'release smoke failed: %s\n' "$*" >&2
    exit 1
}

if [ "$#" -ne 2 ]; then
    echo "usage: release-smoke.sh UNPACKED_ZD EXPECTED_VERSION" >&2
    exit 2
fi

binary=$1
expected_version=$2
[ -x "$binary" ] || fail "binary is not executable: $binary"

actual_version=$($binary --version)
[ "$actual_version" = "zdev $expected_version" ] ||
    fail "expected zdev $expected_version, found $actual_version"

scratch=$(mktemp -d "${TMPDIR:-/tmp}/zdev-release-smoke.XXXXXX")
cleanup() {
    status=$?
    trap - 0 1 2 15
    rm -rf "$scratch" || status=1
    exit "$status"
}
trap cleanup 0 1 2 15

project="$scratch/project"
mkdir -p "$project"
git -C "$project" init -q
git -C "$project" config user.name "Zdev Release Smoke"
git -C "$project" config user.email "zdev@example.invalid"
[ -z "$(git -C "$project" remote)" ] || fail "fixture unexpectedly has a Git remote"

$binary --root "$project" init --record project
trunk=$(git -C "$project" branch --show-current)
$binary --root "$project" config trunk "$trunk"
$binary --root "$project" skill install codex --scope project
printf '%s\n' \
    '' \
    'Use `cargo test --locked` for validation.' \
    'Do not use a remote repository or open a pull request.' \
    >> "$project/.zdev/guidance.md"
$binary --root "$project" skill install codex --scope project --force
$binary --root "$project" skill install claude --scope project
$binary --root "$project" skill install opencode --scope project
$binary --root "$project" skill install pi --scope project
$binary --root "$project" skill install omp --scope project
$binary --root "$project" skill check codex --scope project
$binary --root "$project" skill check claude --scope project
$binary --root "$project" skill check opencode --scope project
$binary --root "$project" skill check pi --scope project
$binary --root "$project" skill check omp --scope project

$binary --root "$project" area create smoke \
    --title "Release smoke" \
    --objective "Exercise the standalone lean task loop."

bundle="$scratch/tasks.json"
printf '%s\n' \
    '{' \
    '  "schema_version": 1,' \
    '  "area": "smoke",' \
    '  "tasks": [' \
    '    {' \
    '      "key": "standalone",' \
    '      "title": "Verify the standalone release",' \
    '      "blocked_by": [],' \
    '      "outcome": "The copied binary runs the lean task loop.",' \
    '      "boundaries": ["Keep release smoke state limited to this task."],' \
    '      "done_when": ["The task can be selected and completed."],' \
    '      "validation": ["Run the release smoke test."]' \
    '    }' \
    '  ]' \
    '}' > "$bundle"

$binary --root "$project" tasks import smoke --from "$bundle"
next=$($binary --root "$project" next smoke --format json)
printf '%s\n' "$next" | grep -Fq '"id": "smoke-001"' ||
    fail "next did not select smoke-001"

$binary --root "$project" task done smoke smoke-001 \
    --summary "The standalone binary completed the lean task loop." \
    --validation "Release smoke test passed."

git -C "$project" add .zdev .codex .claude .opencode .pi .omp
commit_output=$($binary --root "$project" commit -m "test: complete release smoke")
printf '%s\n' "$commit_output" | grep -Eq '^Committed [0-9a-f]+ \(Z[0-9a-f]{64}\): test: complete release smoke$' ||
    fail "commit output did not report its commit and stable change ID"
$binary --root "$project" change inspect HEAD
$binary --root "$project" check smoke

[ -f "$project/.zdev/smoke/tasks/001-verify-the-standalone-release.md" ] ||
    fail "task file was not created"
grep -Fq '## Boundaries' \
    "$project/.zdev/smoke/tasks/001-verify-the-standalone-release.md" ||
    fail "task boundaries were not rendered"
[ -f "$project/.zdev/smoke/TASKS.md" ] || fail "task summary was not created"
[ -z "$(git -C "$project" remote)" ] || fail "zdev introduced a Git remote"

printf 'release smoke passed: %s\n' "$actual_version"

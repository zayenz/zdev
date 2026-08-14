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
[ "$actual_version" = "zd $expected_version" ] ||
    fail "expected zd $expected_version, found $actual_version"

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

init_output=$($binary --root "$project" init)
printf '%s\n' "$init_output" |
    grep -Fq 'zd skill check <codex|claude|opencode|pi|omp> --scope user' ||
    fail "init output did not point to the integration check"
trunk=$(git -C "$project" branch --show-current)
$binary --root "$project" config trunk "$trunk"
$binary --root "$project" skill install codex --scope project
printf '%s\n' \
    '' \
    'Use `cargo test --locked` for validation.' \
    'Do not use a remote repository or open a pull request.' \
    >> "$project/.zd/guidance.md"
$binary --root "$project" skill install codex --scope project --force
$binary --root "$project" skill install claude --scope project
$binary --root "$project" skill install opencode --scope project
$binary --root "$project" skill install pi --scope project
$binary --root "$project" skill install omp --scope project
check_output=$($binary --root "$project" skill check codex --scope project)
printf '%s\n' "$check_output" | grep -Fq 'Codex zdev integration is ready' ||
    fail "skill check output did not confirm readiness"
$binary --root "$project" skill check claude --scope project
$binary --root "$project" skill check opencode --scope project
$binary --root "$project" skill check pi --scope project
$binary --root "$project" skill check omp --scope project

grep -Fq 'guidance = ".zd/guidance.md"' "$project/.zd/config.toml" ||
    fail "project guidance selection was not persisted"
grep -Fq 'Use `cargo test --locked` for validation.' \
    "$project/.codex/skills/zdev/SKILL.md" ||
    fail "Codex skill did not render project guidance"
[ -f "$project/.codex/skills/zdev/agents/openai.yaml" ] ||
    fail "Codex skill did not install UI metadata"
[ "$(find "$project/.codex/skills/zdev" -type f | wc -l | tr -d ' ')" -eq 12 ] ||
    fail "Codex skill did not install its complete inventory"
grep -Fq 'Use `cargo test --locked` for validation.' \
    "$project/.claude/skills/zdev/skills/zdev/SKILL.md" ||
    fail "Claude skill did not render persisted project guidance"
grep -Fq 'Use `cargo test --locked` for validation.' \
    "$project/.opencode/skills/zdev-opencode/SKILL.md" ||
    fail "OpenCode skill did not render persisted project guidance"
grep -Fq '@zdev-implementer' "$project/.opencode/skills/zdev-opencode/SKILL.md" ||
    fail "OpenCode skill did not name its implementation subagent"
grep -Fq 'edit: deny' "$project/.opencode/agents/zdev-verifier.md" ||
    fail "OpenCode verifier gained edit permission"
grep -Fq 'Use `cargo test --locked` for validation.' \
    "$project/.pi/skills/zdev-pi/SKILL.md" ||
    fail "Pi skill did not render persisted project guidance"
grep -Fq 'read,bash,edit,write,grep,find,ls' \
    "$project/.pi/extensions/zdev-subagent.ts" ||
    fail "Pi implementer lost its edit tools"
grep -Fq 'read,bash,grep,find,ls' \
    "$project/.pi/extensions/zdev-subagent.ts" ||
    fail "Pi verifier gained edit tools"
grep -Fq '"--no-extensions"' \
    "$project/.pi/extensions/zdev-subagent.ts" ||
    fail "Pi child processes can load the delegation extension"
grep -Fq 'Use `cargo test --locked` for validation.' \
    "$project/.omp/skills/zdev/SKILL.md" ||
    fail "Oh My Pi skill did not render persisted project guidance"
grep -Fq 'tools: read, grep, bash' \
    "$project/.omp/agents/zdev-verifier.md" ||
    fail "Oh My Pi verifier gained edit tools"
grep -Fq 'zdev:zdev-implementer' \
    "$project/.claude/skills/zdev/skills/zdev/SKILL.md" ||
    fail "Claude skill did not name the packaged implementer"
grep -Fq 'tools: Read, Bash, Grep, Glob' \
    "$project/.claude/skills/zdev/agents/zdev-verifier.md" ||
    fail "Claude verifier gained write tools"
grep -Fq 'while (/^REWORK\b/.test(verdict))' \
    "$project/.claude/skills/zdev/workflows/zdev-task.js" ||
    fail "Claude task workflow lost its repeated rework loop"
if grep -Fq 'Mechanical: yes' \
    "$project/.claude/skills/zdev/workflows/zdev-task.js"; then
    fail "Claude task workflow retained the obsolete mechanical verdict field"
fi

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
    '      "boundaries": ["Do not create obsolete run state."],' \
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

git -C "$project" add .zd .codex .claude .opencode .pi .omp
commit_output=$($binary --root "$project" commit -m "test: complete release smoke")
printf '%s\n' "$commit_output" | grep -Eq '^Committed [0-9a-f]+ \(Z[0-9a-f]{64}\): test: complete release smoke$' ||
    fail "commit output did not report its commit and stable change ID"
$binary --root "$project" change inspect HEAD
$binary --root "$project" check smoke

[ -f "$project/.zd/smoke/tasks/001-verify-the-standalone-release.md" ] ||
    fail "task file was not created"
grep -Fq '## Boundaries' \
    "$project/.zd/smoke/tasks/001-verify-the-standalone-release.md" ||
    fail "task boundaries were not rendered"
[ -f "$project/.zd/smoke/TASKS.md" ] || fail "task summary was not created"
[ -z "$(git -C "$project" remote)" ] || fail "zdev introduced a Git remote"

printf 'release smoke passed: %s\n' "$actual_version"

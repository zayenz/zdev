# Multi-lens audit handoff measurements

## Question

Would transient files reduce total prompt transport for explicit one-to-four
lens audits without adding coordinator rounds or ceremony?

## Reproduction

Run these commands from the repository root at commit
`c038e76619c316449a086a0ea5c9587adb1ca8a2`:

```sh
wc -c templates/zdev/audit.md \
  skills/zdev-audit/SKILL.md \
  templates/zdev/codex-audit-skill.md \
  templates/zdev/claude/workflows/zdev-audit.js \
  templates/zdev/opencode/commands/zdev-audit.md \
  templates/zdev/pi/prompts/zdev-audit.md \
  templates/zdev/omp/prompts/zdev-audit.md

for n in 1 2 3 4; do
  printf '%s lenses: workers=%s prompt_copies=%s file_reads_if_file=%s\n' \
    "$n" "$((n+1))" "$((2*n))" "$n"
done

rg -n "pipeline\(|Reviewer output|one independent verifier|one blocking agent" \
  templates/zdev/claude/workflows/zdev-audit.js \
  templates/zdev/opencode/commands/zdev-audit.md \
  templates/zdev/pi/prompts/zdev-audit.md \
  templates/zdev/omp/prompts/zdev-audit.md \
  skills/zdev-audit/SKILL.md
```

The byte counts are:

| File | Bytes |
| --- | ---: |
| Canonical audit contract, `templates/zdev/audit.md` | 1,243 |
| Generated Codex audit skill | 2,108 |
| Codex audit skill template | 1,057 |
| Claude audit workflow template | 2,672 |
| OpenCode audit command template | 542 |
| Pi audit prompt template | 575 |
| Oh My Pi audit prompt template | 545 |

These sizes describe the checked-in sources and generated Codex entrypoint.
They are not token estimates and do not include harness system prompts.

## Observations

Claude is the only adapter here with executable orchestration that exposes the
complete construction. For `n` explicit lenses it starts `n` reviewers through
one pipeline and then one fresh vetter, for `n + 1` worker calls. Each reviewer
gets the 1,243-byte canonical contract. The vetter gets that contract again and
the non-empty reviewer outputs joined inline. A reviewer byte is therefore
emitted once and transported once more as vetter input: two transported copies
in this observable boundary.

| Lenses | Worker calls | Reviewer-output copies transported | Extra reads if outputs move to files |
| ---: | ---: | ---: | ---: |
| 1 | 2 | 2 | 1 |
| 2 | 3 | 4 | 2 |
| 3 | 4 | 6 | 3 |
| 4 | 5 | 8 | 4 |

If each reviewer produces `R` bytes, Claude's reviewer evidence accounts for
`2nR` bytes across reviewer output plus vetter input. The final inline prompt
contains `nR` of those bytes. Replacing that inline section with paths would
make the constructed prompt smaller, but the vetter must open and receive the
same `nR` bytes to check the evidence. Total evidence delivered across the
workers remains `2nR`; the file design also adds at least one write per reviewer
and one or more file-read tool calls. It does not reduce worker calls.

Codex, OpenCode, Pi, and Oh My Pi are declarative rather than an observed
workflow implementation. Their entrypoints all require one verifier per lens
and a different final verifier to open, check, and deduplicate candidate
locations. Thus their declared worker count is also `n + 1`. The repository
does not expose whether a harness internally serializes those results inline,
through native task state, or another mechanism, so no byte claim is made for
their hidden transport.

The fixed canonical contract is sent to every Claude worker, contributing
`(n + 1) * 1,243` bytes, or 2,486 through 6,215 bytes for one through four
lenses. A reviewer-output file does not affect that fixed cost. Reducing it
would be a separate contract-loading question, already addressed by installed
worker-contract references, not evidence for audit result files.

## Conclusion

Do not add transient file handoffs for multi-lens audits. They reduce the
literal size of Claude's final prompt expression but do not reduce the evidence
bytes the final model must receive. They add filesystem writes, reads, cleanup
and failure cases without removing a worker call or coordinator round. This is
text moved into a file, not a transport saving under the task's criterion.

No derived implementation task is warranted.

Confidence is high for the checked-in Claude workflow and its structural byte
accounting. Confidence is moderate for the cross-harness conclusion because
the other adapters specify semantics but their native runtimes' internal
transport is not present in this repository. Runtime caching, provider-side
prompt deduplication, tokenization, and latency were not measured; model
evaluations were intentionally out of scope.

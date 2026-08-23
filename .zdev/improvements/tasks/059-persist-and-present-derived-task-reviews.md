+++
schema_version = 1
id = "improvements-059"
key = "store-derived-proposals"
area = "improvements"
status = "open"
complexity = "standard"
blocked_by = []
+++
# Persist and present derived-task reviews

## Outcome

Manual derived-task review uses an actual stored proposal and Markdown document that zdev can show and apply by opaque identity, without replaying proposal bytes or transporting a fingerprint.

## Context

Ordinary task review is filesystem-backed, but derived review still returns the complete proposal, Markdown, and approval fingerprint and requires them again at apply. Reuse the established Git administrative storage pattern while preserving the approval-free automatic path.

## Boundaries

- Keep one current stored derived review per area and source handoff; replacement invalidates the earlier identity.
- Store the exact proposal JSON, rendered Markdown, and internal metadata; expose show and apply-by-reviewed-identity operations.
- Manual flow reviews from input once, shows the stored Markdown, and applies the approved identity.
- Automatic-authority flow remains one direct apply --from operation and gains no mandatory review or storage round-trip.
- Retain direct input and approval compatibility, but add no lineage, ledger, history UI, or user-visible fingerprint.

## Done when

- [ ] Derived review JSON is compact and zdev presents the actual stored Markdown on demand.
- [ ] Reviewed apply rereads the stored proposal and revalidates source task, proposal kind, area, canonical content, mechanical authority, ownership, and current Git state under the apply lock.
- [ ] Missing, corrupt, replaced, cross-area, and wrong-source artifacts fail before publication.
- [ ] Canonical and generated guidance removes proposal reconstruction and fingerprint transport from the normal manual flow.

## Validation

- Add focused coverage for stored show/apply, replacement, corruption, cross-area and wrong-source rejection, linked worktrees, direct automatic apply, compatibility input, and rollback.
- Run the repository standard full validation.

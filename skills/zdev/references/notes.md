# Notes for future work

## When

Use this route for **add note**, **list notes**, and **make area from note**.
A note may be one file or a folder of related files under `.zdev/_notes/`.
Notes hold unfinished plans, future ideas, open questions, and collected
research documents. They have no required format, metadata, status, or index.
They do not enter task selection or authorize implementation.

These are skill interactions using ordinary filesystem tools, not `zdev note`
CLI commands. Work in the current checkout for saving and listing notes; no
area or branch selection is needed. Follow the repository's record policy for
whether notes are tracked. When saving requires a new `.zdev` record, follow
[Set up zdev](setup.md) and reuse any record policy the user already chose.

## Add note

Save the supplied idea, partial plan, or documents when the user requests it.
Use a descriptive filename such as `.zdev/_notes/search.md`. For several
related documents, use a topic folder such as `.zdev/_notes/search/`, with
nested folders when useful. Preserve supplied research documents together;
capture does not require settled conclusions or an index of every file.
An optional `README.md` can explain a folder's purpose or where to resume.

Reuse the destination the user names. Otherwise choose a descriptive path and
report it. Preserve existing material when adding to a folder. Keep unresolved
questions visible rather than inventing decisions to fill a template. A request
to save existing material does not by itself request more research or planning.
Report the saved paths, then stop unless the user requested another interaction.

## List notes

List individual files and topic folders under `.zdev/_notes/` in path order,
omitting the root convention `README.md`. Give each entry's path and a short
description when its title or a folder README supplies one. List a topic folder
as one collection by default; expand its contents, including nested folders,
when the user requests that folder or a recursive listing. Avoid reading every
research document just to build an inventory.

If the directory is absent or has no notes, report that without creating files.
Listing is read-only and does not select an area or start work.

## Make area from note

Resolve the named file or folder. Ask which one only when the reference is
ambiguous. Read the note, or a folder's README and the documents relevant to the
objective; inspect further files as needed to understand the proposed work.
Treat draft plans and research conclusions as inputs to discussion, preserving
open questions and checking repository facts again where they affect scope.

Follow [Explore an objective](shape-work.md) to create the area and shape its
authoritative `brief.md`, reusing settled choices from the note and the user's
workspace authority. Link useful source notes from the brief. If selected
research is ready to become area background, move it under
`.zdev/<area>/background/` and index it from the brief under the ordinary
background rules. Leave other material in `_notes/` for future use.

Report the resulting area and retained source paths. Area creation alone does
not request tasks or implementation. Remove obsolete drafts when the user asks
for cleanup; keep source notes by default so unfinished material is preserved.

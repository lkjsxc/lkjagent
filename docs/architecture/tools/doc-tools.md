# Document Tools

## Purpose

Define live document and artifact tools for structured artifacts. The semantic
tree and graph contract lives in
[../document-structure/](../document-structure/README.md).

## Address Reducer Contract

`artifact.plan`, `artifact.next`, `artifact.audit`, and `doc.audit` call the
shared address reducer before any filesystem, audit, or ledger effect. The
reducer owns root versus weak path classification, Markdown leaf refusals,
old `.md` directory refusals, normalized roots, and copyable non-content
action surfaces. `fs.write` and `fs.batch_write` remain the only tools that
write Markdown leaves.

## Removed Live Scaffold Writers

Prompt-visible scaffold writers are not live tools. The former document and
artifact scaffold writers are absent from the executable registry, prompts,
admission, recovery examples, and tests. Structure and identity are created by model-
authored `fs.batch_write` actions that pass stored write contracts.

## doc.audit

Audits a document root for README presence, local child links, forbidden
sequence-only names, catalog metadata presence, line caps, and optional
file-count target. Approximate count targets accept extra Markdown files; exact
mode requires the target count. Passing audits can satisfy document-structure
evidence when the active graph gate accepts that audit-owned evidence.

## artifact.plan

Records a semantic content artifact identity without writing files. Parameters
are `root`, `title`, `kind`, optional `scale`, and optional `sections`. It
preserves full owner wording in metadata while short path aliases, such as
`stories/novel`, remain the filesystem root.

## artifact.audit

Audits a semantic artifact root. Parameters are `root`, optional `kind`,
optional `count`, and optional `mode`. Passing artifact audit is the only
audit-owned readiness proof for artifact-readiness evidence. If `root` resolves
to a file, artifact.audit reports `root_is_file` and renders a root-directory
next action instead of surfacing an OS error. Graph notes and raw file existence
do not satisfy artifact readiness.

## artifact.next

Plans the next bounded artifact write batch from readiness gaps. Parameters are
`root`, optional `path`, and optional `kind`. It returns exact paths, limits,
required sections, forbidden weak phrase classes, and
`next_decision_required=true`. It never writes files and never returns generated
body prose. The next persisted decision may render a content-write surface that
requires the model to author a singular `fs.batch_write` action.

If `root` resolves to a file, artifact.next must not claim missing=0 or render
artifact.audit for that file. It resolves the owning artifact root or asks for a
directory inspection action.

## Address Contract

The artifact address contract is [artifact-addresses.md](artifact-addresses.md).
The documentation root contract is
[../../documentation-system/root-path-contract.md](../../documentation-system/root-path-contract.md).

## Status

open for this redesign. Live tools use audits, plans, contracts, and
model-authored line-protocol writes; scaffold writers are not prompt-visible.

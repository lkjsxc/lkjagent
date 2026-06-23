# Document Tools

## Purpose

Define graph-native document construction helpers for structured artifacts.
The semantic tree and graph contract lives in
[../document-structure/](../document-structure/README.md).

## doc.scaffold

Creates a compact semantic tree under `root`. Parameters are `root`, optional
`kind`, optional `count`, optional `mode` (`exact` or `approx`), required
`title`, and optional newline sections. It generates README indexes and
compact `catalog.toml` metadata for documentation roots.

## doc.audit

Audits a document root for README presence, local child links, forbidden
sequence-only names, catalog metadata presence, line caps, and
optional file-count target. Passing audits can satisfy document-structure
evidence when the active graph gate accepts that evidence.

## artifact.plan

Plans a semantic content artifact without writing files. Parameters are
`root`, `title`, `kind`, optional `scale`, and optional `sections`.

## artifact.apply

Writes a semantic artifact scaffold by reusing the document planner and writer.
Parameters are `root`, optional `title`, optional `kind`, optional `mode`, and
optional `sections`. The output includes README files and `catalog.toml`
as the current manifest. `root` is a directory address and `.md` roots are
refused before mutation.

## artifact.audit

Audits a semantic artifact root. Parameters are `root`, optional `kind`,
optional `count`, and optional `mode`. When `kind` is story or cookbook, audit
rejects a generic project-doc manifest for that artifact request. Passing
artifact audit is the only audit-owned readiness proof for artifact-readiness
evidence. If `root` resolves to a file, artifact.audit reports `root_is_file`
and renders a root-directory next action instead of surfacing an OS error.

## artifact.next

Plans the next bounded artifact write batch from readiness gaps. Parameters
are `root`, optional `path`, and optional `kind`. It returns exact weak paths,
required sections, and a copyable `fs.batch_write` example only when the
example already contains profile-specific content. When no current weak path
remains in the durable cursor window, it requests audit or focused reads
instead of rendering a placeholder write.

If `root` resolves to a file, artifact.next must not claim missing=0 or render
artifact.audit for that file. It resolves the owning artifact root or asks for a
directory inspection action.

## Address Contract

The artifact address contract is [artifact-addresses.md](artifact-addresses.md).
The documentation root contract is
[../../documentation-system/root-path-contract.md](../../documentation-system/root-path-contract.md).

## Status

partially implemented; doc tools, artifact wrappers, bounded next-batch
planning, and scaffold-only rejection exist. Root adoption and repair planning
remain open.

# Document Tools

## Purpose

Define graph-native document construction helpers for structured artifacts.
The semantic tree and graph contract lives in
[../document-structure/](../document-structure/README.md).

## doc.scaffold

Creates a compact semantic tree under `root`. Parameters are `root`, optional
`kind`, optional `count`, optional `mode` (`exact` or `approx`), required
`title`, and optional newline sections. It generates README indexes and a graph
manifest for documentation roots.

## doc.audit

Audits a document root for README presence, local child links, forbidden
sequence-only names, graph manifest presence, graph references, line caps, and
optional file-count target. Passing audits can satisfy document-structure
evidence when the active graph gate accepts that evidence.

## artifact.plan

Plans a semantic content artifact without writing files. Parameters are
`root`, `title`, `kind`, optional `scale`, and optional `sections`.

## artifact.apply

Writes a semantic artifact scaffold by reusing the document planner and writer.
Parameters are `root`, optional `title`, optional `kind`, optional `mode`, and
optional `sections`. The output includes README files and `.lkj-doc-graph.md`
as the current manifest.

## artifact.audit

Audits a semantic artifact root. Parameters are `root`, optional `kind`,
optional `count`, and optional `mode`. When `kind` is story or cookbook, audit
rejects a generic project-doc manifest for that artifact request. Passing
artifact audit is the only audit-owned readiness proof for artifact-readiness
evidence.

## artifact.next

Plans the next bounded artifact write batch from readiness gaps. Parameters
are `root` and optional `kind`. It returns exact weak paths, required sections,
and a copyable `fs.batch_write` example only when the example already contains
profile-specific content. When no current weak path remains in the durable
cursor window, it requests audit or focused reads instead of rendering a
placeholder write.

## Status

partially implemented; doc tools, artifact wrappers, bounded next-batch
planning, and scaffold-only rejection exist. Root adoption and repair planning
remain open.

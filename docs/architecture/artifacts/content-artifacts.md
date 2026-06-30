# Content Artifacts

## Purpose

State the common contract for large content deliverables.

## Contract

Long stories, dictionaries, cookbooks, guides, encyclopedias, knowledge bases,
and other large deliverables route to semantic artifact roots. The root
contains a README, manifest, semantic child files, audit evidence, and
completion evidence.

Artifact states are defined in [lifecycle.md](lifecycle.md). Scaffold creates
topology only. Content pass writes meaningful leaves through bounded
[content-atoms.md](content-atoms.md). Audit proves topology, atom status, and
content readiness. Completion requires graph evidence plus artifact readiness.

## Payload Safety

After a max-token write fault, unclosed content tag, or raw large-write retry,
the controller routes to artifact planning or bounded section writes. It must
not retry one giant `fs.write`.

## Roots

Default roots are `stories/<semantic-title>`, `dictionaries/<semantic-title>`,
`cookbooks/<semantic-title>`, `guides/<semantic-title>`, and
`knowledge/<semantic-title>`. Generic `docs/` is used only when the owner asks
for project documentation.

## Status

partially implemented; classification, scaffold profiles, bounded write
batches, and shared content atoms exist. Root adoption remains open.

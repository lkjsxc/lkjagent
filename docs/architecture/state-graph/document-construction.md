# Document Construction

## Purpose

Define graph treatment for large, counted, or structured document tasks.

## Topology First

Document tasks route through document-profile, topology, scaffold,
section-plan, write, audit, repair, and completion-check nodes. The graph
requires topology evidence before bulk writing and audit evidence before
completion.

The document state ledger carries root, kind, language when detected, count
target, count mode, root README status, docs/main split, section map, coverage
map, first and last main path, sequence status, audit status, and repair
needs.

Long-form content requests are content artifacts when the owner asks for a long
story, very long story, novel, book, big cookbook, encyclopedia, large guide,
corpus, many files, structured output, or when a write attempt hits max tokens
or an unclosed content tag. These tasks must not use one giant `fs.write`.

The route creates a semantic artifact root, a root README table of contents,
manifest, semantic child directories or files, bounded content-bearing
sections, and an audit report. Names such as part-001.md are valid only when
the owner asks for numbered parts.

Story and cookbook profiles are examples, not hard-coded universal output. The
planner chooses semantic roles for the owner objective and repairs or adopts
an existing equivalent root before creating new files.

## Completion

Document completion requires README or index evidence, topology evidence,
count or scale evidence, link audit evidence, manifest evidence, content
presence evidence, and a restart or read-order signal when relevant.

The model cannot complete a large document task by saying it is done; the
completion gate requires deterministic audit evidence.

For content artifacts, completion also requires an artifact root, README,
manifest, semantic children, content-bearing files, plan evidence, observation
evidence, and verification or audit evidence. Planning alone and generic
scaffold alone are never completion.

The artifact lifecycle and completion gates are defined in
[../artifacts/lifecycle.md](../artifacts/lifecycle.md) and
[../artifacts/completion-gates.md](../artifacts/completion-gates.md).

## Status

partially implemented; long story and bread cookbook classification now route
to `content-artifact`, scaffold profiles produce semantic story/cookbook paths,
scaffold-only output cannot satisfy document-structure evidence, and
`doc.audit` rejects scaffold-only story and cookbook leaves. Manifest adoption
and broader artifact repair remain open.

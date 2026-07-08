# Product

## Purpose

Define the owner-visible product: daemon lifecycle, owner-turn intake, matter
and record surfaces, status, console, and workbench.

## Table of Contents

- [daemon.md](daemon.md): daemon loop, idle, waiting, and crash resume.
- [cli.md](cli.md): owner command surface and exit-code discipline.
- [queue.md](queue.md): owner-turn intake, answer routing, and semantic routing.
- [status-and-console.md](status-and-console.md): status fields, matter display,
  log output, proof visibility, and watch layout.
- [workbench.md](workbench.md): daily-driver terminal workbench with durable
  transcript and Japanese input requirements.

## Product Architecture

The product is workspace-first. The owner sees one readable workspace plus
matter, record, artifact, transcript, status, and proof surfaces. SQLite rows and
persisted `RuntimeDecision` rows remain the authority for routing, effects, and
completion.

Every owner turn leaves workspace evidence. Ordinary conversation appends a
durable transcript entry. Ambiguous save-like text writes an inbox trace or asks
one clarification. Record-like text writes a Markdown record, row, history,
fingerprint, README coverage, index artifact, state cells, and queue route
evidence before reporting success.

Model-dependent work uses selected runtime state. The prompt shows bounded
source-linked context, the selected XML-like envelope, and only the tools
admissible for that decision. It does not show the global tool catalog, ask for
JSON, or let model prose decide completion.

## Product Units

The owner sees turns, matters, records, artifacts, transcripts, decisions,
events, and proof. Plan-family rows may appear only as bridge evidence until
removed from the implementation.

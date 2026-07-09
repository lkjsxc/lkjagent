# Product

## Purpose

Define the owner-visible product: daemon lifecycle, owner-turn intake, matter
and record surfaces, status, console, and workbench.

## Table of Contents

- [authority-boundaries.md](authority-boundaries.md): workspace, SQLite,
  projection, model, harness, and configuration ownership.
- [configuration-registry.md](configuration-registry.md): exact flat scalar
  settings, bounds, precedence, reload policy, and consumers.
- [daemon.md](daemon.md): daemon loop, quiescence, waiting, and crash resume.
- [cli.md](cli.md): owner command surface and exit-code discipline.
- [queue.md](queue.md): owner-turn intake, answer routing, and semantic routing.
- [status-and-console.md](status-and-console.md): status fields, matter display,
  log output, proof visibility, and watch layout.
- [workbench.md](workbench.md): daily-driver terminal workbench with durable
  transcript and Japanese input requirements.

## Target Product Architecture

The product is workspace-first. The owner sees one readable workspace that acts
as an auxiliary memory device: daily records, calendar notes, TODOs, finance,
project work, artifacts, transcripts, indexes, and proof bundles are inspectable
without asking the agent. SQLite rows and persisted `RuntimeDecision` rows remain
the authority for routing, effects, recovery, and completion.

Every owner turn leaves workspace evidence. Ordinary conversation appends a
durable transcript entry. Ambiguous save-like text writes an inbox trace or asks
one clarification. Record-like text writes a Markdown record at the canonical
family path before reporting success: journals by human date, TODOs by state,
calendar items by date, finance items by month, and work notes by project or
repository slug. The owner command remains trace evidence; the record body is
structured unless the owner explicitly asks for verbatim storage. Recording also
writes row, history, fingerprint, README, index artifact, state-cell, and queue
route evidence.

Model-dependent work uses selected runtime state. The prompt shows bounded
source-linked context, the selected attribute-less XML-like envelope, and only
the tools admissible for that decision. It does not show the global tool catalog,
ask for JSON, or let model prose decide completion. Artifact success requires a
file, artifact row, check row, and response path evidence. Earlier blocked,
active, failed, pending, or unsuperseded skipped work blocks later response work
until recovery or supersession evidence exists.

## Product Units

The owner sees turns, matters, records, artifacts, transcripts, decisions,
events, and proof. No retired projection is part of the production contract.

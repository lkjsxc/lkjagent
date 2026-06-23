# Data Directory

## Purpose

Explain the local runtime evidence stored under `data/` and the inspection rule
for agents working on runtime behavior.

## Contents

- `lkjagent.json`: local runtime configuration with secrets omitted.
- `lkjagent.sqlite3*`: local SQLite store and SQLite sidecar files.
- `logs/`: generated model handoff and provider exchange evidence.
- `workspace/`: generated artifact workspace used by runtime tools.

## Git Policy

Runtime databases stay ignored. Small README files, reduced fixtures, and
intentional diagnostic evidence may be tracked when they document a replay case
or implementation contract.

## Inspection Rule

Inspect existing `data/` evidence before changing runtime authority, recovery,
artifact, compaction, maintenance, or logging behavior. Write inspection notes
to `tmp/data-inspection.md` and record files, SQLite tables, error patterns,
fixtures created, and evidence not inspected.

## Safety

Do not delete runtime evidence casually. Summarize large logs into reduced
fixtures before cleanup. Never commit secrets, API keys, authorization headers,
full environment dumps, or unredacted remote endpoint credentials.

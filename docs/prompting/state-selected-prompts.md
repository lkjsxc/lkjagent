# State Selected Prompts

## Purpose

State-selected prompts own prompt mode selection. The compiler chooses a mode
from hard state and guard tracks, then inserts only the slices needed for the
next safe action.

## Modes

- Intake: classify owner text and build a task contract; no mutation.
- Semantic seed: create the smallest connected documentation structure.
- Expansion: grow one local neighborhood and update links.
- Relation: add relation pages and backlinks.
- Audit: inspect and report exact failures; no mutation.
- Repair: fix exact audit failures and rerun the owning audit.
- Recovery: show canonical grammar or schema and require a smaller action.
- Maintenance: scan structural health and create a change or suppression key.

## Guard Overrides

Parse recovery, artifact drift, context pressure, queue interruption, and
model-specific naming can override the nominal mode and force recovery or
classification before mutation.

## Status

design-only

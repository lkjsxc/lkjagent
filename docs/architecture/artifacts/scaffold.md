# Scaffold

## Purpose

Define the non-live scaffold concept that artifact contracts replaced.

## Behavior

Prompt-visible scaffold writers are not live tools. The runtime can reason
about identity, required files, and weak paths, but the model must author
content through contract-bound `fs.batch_write` actions.

## Writes

Structure writes happen only when a stored write contract admits exact paths.
The contract may include root identity files, README navigation, manifest data,
and semantic leaves. Empty placeholders never count as completed content.

For content artifacts, any identity or navigation batch must be followed by
bounded content batches and a new audit before completion. See
[write-batches.md](write-batches.md) and [repair.md](repair.md).

## Duplicate Prevention

Duplicate detection uses artifact kind, normalized title, owner objective hash,
root role, README title, manifest artifact key, and section role.

## Status

implemented for the current runtime. Scaffold is an internal planning concept,
not a live model-facing writer.

# State Cells

## Purpose

Define the durable facts from which runtime work is selected.

## Cell Shape

A cell stores a stable namespace/name key, status, priority, confidence, typed
payload schema, evidence references, source event, timestamps, optional wake,
and optional conflict group. Unknown namespaces and payloads survive reduction.

Active cells require source evidence. A missing source, stale file revision, or
invalidated check cannot remain silently active.

## Initial Dimensions

- `matter`: open, waiting, blocked, or closed.
- `phase`: orient, modify, review, respond, or idle.
- `need`: target, source revision, edit, check, response, or owner fact.
- `fault`: protocol, admission, stale file, effect, endpoint, check, or stasis.
- `wake`: immediate, time, owner input, file change, or config change.

These dimensions are ordinary cells, not a second hard-coded workflow authority.
A selector derives one operation from their current combination.

## Authority

Matter lifecycle and obligation state are reducer-derived projections. They may
support owner views but cannot select work independently of the state snapshot.
Prompts and dispatchers cannot invent state absent from durable rows.

## Evidence

Evidence references name source kind, stable source ID, and fingerprint. File
facts use SHA-256 revisions. A later source change emits an invalidation event and
suppresses dependent checks or context before the next decision.

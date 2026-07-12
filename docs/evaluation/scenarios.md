# Evaluation Scenarios

## Purpose

Define complete tasks and deterministic outcomes for the direct runtime.

## Edit Scenarios

The tracked `exact-file-edit` alias starts from exact UTF-8 bytes in
`notes/exact-base.txt`. Its first owner turn requests one exact phrase
replacement, automatic verification of the resulting bytes, no other change,
and a truthful report. The second scheduled turn must itself complete; three
later turns test phrase retrieval and matter continuity without expanding the
file scope. The five strictly increasing offsets end at 840 seconds so the same
input supports an endpoint probe and the bounded campaign.

Its declared checks bind the exact resulting SHA-256, the sole allowed changed
path, absence of collateral and scaffold files, and durable decision, provider
exchange, tool admission, edit effect, fresh check, truthful final message, and
second-turn owner/final message rows. These are expected facts, not runtime
results; no endpoint run is claimed by this scenario declaration.

- S1: exact one-line edit with explicit path.
- S2: multiline edit containing XML-sensitive and Japanese text.
- S3: create one absent file without scaffold.
- S4: repeated old text requiring disambiguation.
- S5: external file change between read and edit.
- S6: model proposes final before any effect.
- S7: same filename in two projects with conflicting facts.
- S8: malformed or truncated action at the provider-response boundary.

Each scenario declares initial tree, exact owner turns, expected bytes or safe
block, allowed changed paths, forbidden collateral, config, model identity, and
required durable lineage.

## Daily Scenarios

Journal verifies configured `YYYY/MM/DD`, grounded model-authored body, no copied
command/canned filler/invented fact, token cap, revision, check, and receipt.
Record scenarios add TODO, calendar, finance, note, and project paths one family
at a time.

Recall records a sourced fact in one run, uses it once in a fresh matter, then
proves a newer owner correction wins.

## Report And Activity Scenarios

A short report is one checked file. A longer report has meaningful bounded
children, complete links, no empty part, and restart continuity. Activity
projections bind canonical messages and decision/effect/check receipts without
becoming runtime authority.

## TUI Scenario

A real PTY during a slow call enters Japanese text, resizes, scrolls manually as
new rows arrive, restores bottom Follow, searches/clears, submits or clears the
composer, restarts, and binds visible rows to canonical message IDs.

## False Positives

Fixtures must reject blocked/idle/fake closure, unchanged requested bytes,
placeholder output, untracked evidence, stale hashes, missing exchange/effect,
stale check, duplicate message, short/quiet campaign, and secret-bearing blob.

# Contradictions

## Purpose

Prevent conflicting facts from appearing as simultaneous ordinary context.

## Detection

Before rendering, prompt assembly groups active clean context items by semantic
key. A contradiction exists when incompatible claims share a key and no
resolution state exists. Examples include different target roots, word targets,
owner objectives, or a check result that conflicts with a newer artifact
fingerprint.

## State Representation

Unresolved contradictions create state cells such as:

```text
context:conflict/<semantic-key>
```

The payload names competing item ids, source refs, confidence values, and the
reason the conflict was detected.

## Rendering

Normal prompts do not render both sides as facts. They render one bounded
`Unresolved Conflict` section with source refs and the operation selected by the
runtime, such as inspect evidence, ask owner, or take a conservative path.

## Lineage

Detection writes `context_edges` rows with `contradicts` links between competing
items. A resolution writes `resolved-by` links from each suppressed loser to the
winning item. Proof bundles include these rows with the context items.

## Resolution

A conflict resolves through a durable event: owner answer, check evidence,
artifact inspection, or a deterministic reducer rule. The owner can write a
resolution cell with `context resolve CASE_ID KEY WINNING_ITEM_ID`. Losing
context items are suppressed from normal prompt admission, not deleted.

## Failure This Prevents

The model does not receive contradictory instructions as if both were true.

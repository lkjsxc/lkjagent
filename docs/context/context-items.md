# Context Items

## Purpose

Define durable source-tagged facts used to assemble prompts.

## Item Fields

A context candidate stores item ID, semantic key and claim scope, bounded body
or excerpt, source type, ID, path and fingerprint, trust, effective time,
staleness, contamination, estimated tokens, relevance features, provenance
edges, and expiry or suppression reason when present.

## Prompt Admission

The decision derives information needs from obligations and the selected
operation. Prompt compilation selects candidates by semantic key, trust,
freshness, novelty, dependency value, and budget. It renders compact source refs
so the model can distinguish owner
instructions, file evidence, check results, memory, model-authored content,
observations, and recovery diagnoses.

The renderer never dumps a transcript. It admits bounded current items, accepted
memory, active state payloads, required artifact tails, workspace record/index
summaries with fingerprints, and unresolved-conflict summaries.

## Compaction

Compaction creates new items with provenance edges. It does not delete older
items. The reducer may suppress older items from normal prompt admission when a
clean compacted item is current.

## Failure This Prevents

Prompts remain bounded while preserving enough source identity to avoid mixing
unverified claims with measured evidence.

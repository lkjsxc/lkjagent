# Context Items

## Purpose

Define durable source-tagged facts used to assemble prompts.

## Item Fields

A context item stores item id, semantic key, text or structured JSON, source
type, source id, source fingerprint, trust class, staleness class,
contamination class, artifact refs, optional decision id, created time, and
expiry or suppression reason when present.

## Prompt Admission

Prompt assembly selects items by semantic key, trust class, active state need,
and budget. It renders compact source refs so the model can distinguish owner
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

# Resolvers

## Purpose

Define total resolver plans and how they become runtime decisions.

## Resolver Plan Set

`TotalResolverPlan` is one of:

- `RuntimeEffect`: compaction, idle, maintenance defer, or other deterministic
  runtime work that needs no model content.
- `ExactInspection`: an exact zero-content read, stat, list, audit, or graph
  state action.
- `SemanticWriteContract`: a persisted write contract rendered as a singular
  `fs.batch_write` surface.
- `Audit`: a document or artifact audit action after write progress exists.
- `EvidenceRecording`: direct evidence recording for graph-owned requirements.
- `OwnerWait`: wait for outside owner input.
- `BlockedHandoff`: store exact blockers and stop looping.
- `CloseCase`: close only after the central completion gate passes.

## Decision Mapping

A resolver plan always produces one `RuntimeDecision`; there is no mission
fallback branch after resolver planning. The decision kind is derived from the
plan variant. The decision names admitted and blocked tools, the forced tool or
runtime effect, the content write contract, completion blockers, progress key,
and the rule explanation. Prompt frames and admission views are rendered from
the same decision id and staleness fingerprint.

## Root Missing Mapping

When facts contain `ArtifactRootStatus::Missing`, the resolver emits
`SemanticWriteContract` with `fs.batch_write`, not `Audit`. The contract uses
flat root identity paths that can pass topology and content checks. Same-root
`doc.audit` is blocked until the progress key changes through a write success
or blocked handoff.

## Artifact Next Mapping

`artifact.next` observations are candidate facts, not dispatch authority. When
an observation says `candidate_action=fs.batch_write` and
`next_decision_required=true`, the following decision may render the batch
write contract. When it says `candidate_action=artifact.audit`, the following
decision may audit instead of repeating a placeholder write.

# Prompt Assembly

## Purpose

Define decision-first selection and compilation of one bounded model request.

## Boundary

A prompt is a projection of a persisted `RuntimeDecision`. Compilation starts
after selection and cannot change the operation. It consumes decision ID,
prompt state, expected envelope, tool-view identity, information needs, source
candidates, lane budgets, conflict state, and recovery policy.

## Pipeline

1. discover candidates through state-routed indexes and exact paths;
2. validate source fingerprints, trust, staleness, and contamination;
3. normalize semantic identity and claim scope;
4. resolve or block material contradictions;
5. remove exact and near duplicates across every prompt region;
6. score utility, freshness, novelty, dependency value, and token cost;
7. select mandatory facts, then optional facts under lane and total caps;
8. render source-linked attribute-free cards;
9. validate budgets, uniqueness, decision binding, and output reserve.

## Lanes

The stable prefix contains kernel, owner policy, state grammar, sorted current
tool schemas, and output grammar. The volatile suffix contains objective,
selected workspace and conversation excerpts, observations, conflicts, and
recovery facts.

Every selected and excluded candidate records its ID, reason, rank, source
fingerprint, lane, token estimate, and decision. Full workspace files, raw
transcripts, raw failed output, unresolved conflicting values, and harness JSON
are not prompt regions.

## Fingerprints

Compilation records kernel, prompt-state, lane, tool-view, grammar, stable-prefix,
and full-prompt fingerprints. The provider exchange and decision settlement
reference the same full frame. A source change invalidates the frame before an
effect can use it.

## Failure Response

After a failure, the next model call must change context, tool view, budget,
grammar, strategy, or external condition. A bounded recovery card names the
fault and change; it never quotes the failed body.

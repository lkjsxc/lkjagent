# Matter Brief

## Purpose

Define the rolling summary that helps a persisted decision understand the active
matter without replaying the full transcript.

## Shape

A matter brief is an engine-maintained summary stored in durable rows and
referenced by prompt-frame cards. It includes:

- matter title and lifecycle state;
- latest owner turn summary;
- current objective or record intent;
- relevant workspace record ids and fingerprints;
- unresolved questions or blockers;
- fresh checks or stale evidence needs;
- bounded memory facts admitted for this decision.

## Source Discipline

Each sentence in the brief must be traceable to owner turns, records, artifacts,
context items, checks, or state events. If evidence is stale or contradicted,
the brief says so and links the conflict cell instead of picking a winner.

## Update Rule

Reducers update the brief through state events. Prompt rendering reads the
latest admitted brief for the selected `RuntimeDecision`; it does not append raw
chat history.

## Failure This Prevents

The model receives the current matter and workspace situation without seeing a
large transcript or stale failed output.

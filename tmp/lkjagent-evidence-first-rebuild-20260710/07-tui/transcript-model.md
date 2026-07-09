# Transcript Model

## Canonical Table

Persist first-class conversation messages with:

- globally monotonic sequence;
- stable logical message ID;
- owner-turn, matter, decision, and operation refs;
- role: owner, agent, or question;
- lifecycle: draft, final, superseded;
- replacement or draft relation;
- body ref and fingerprint;
- created and committed times.

## Producers

Queue intake creates one owner message transactionally. A report or question
operation creates one agent message. Internal step, state, tool, check, queue,
and proof events link to messages but never become conversation rows.

## Streaming

Streaming deltas update an in-memory or durable draft with the same logical ID.
Finalization replaces that draft once. Crash resume either continues the draft
or marks it interrupted; it does not display both draft and final copies.

## Rendering

Both TTY and line modes consume the same transcript projection and shared
viewport reducer. Separate terminal effects may remain, but message identity,
ordering, windowing, and filtering have one implementation.

## Deduplication

Deduplicate by logical ID and replacement relation. Identical text from distinct
real owner turns remains distinct.

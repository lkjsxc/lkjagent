# Turn Cycle

## Purpose

Define one daemon cycle from queue intake through persistence.

## Cycle

- Intake delivers owner messages at the cycle boundary.
- Selection calls the pure `next_work` function on the task snapshot.
- Rendering builds the step prompt from durable state.
- Calling sends at most `engine.turn.endpoint-calls=1` request to the endpoint.
- Parsing expects the envelope for the selected step kind.
- Effect applies the engine-owned write, tool, message, or plan materialization.
- Effect results are fed back into settlement before any done or closed state is
  committed.
- Checking evaluates attached checks from [completion.md](completion.md).
- Settling marks the step done, records a diagnosis, or invokes
  [retry-and-escalation.md](retry-and-escalation.md).
- Persistence commits the attempt, events, checks, token usage, and state in one
  transaction.

## Aurora Ledger Example

For an Aurora Ledger write step, selection chooses the planned chapter section,
rendering includes the objective, plan digest, section beat, continuity tail,
and word target, calling uses `llm.max-tokens.write`, effect appends to the
planned chapter file, and checking measures words before the step can finish.

## Ordering Guarantees

Queue rows do not interrupt a turn. Pure verify work skips the endpoint. Tool
and shell effects run with the timeouts owned by their adapters. If an effect
fails before commit, the turn records an honest error path and does not mark the
step done or the task closed. A crash resumes from the last committed
transaction.

## Failure This Prevents

The cycle has one decision seam and one persistent writer. Contradictory runtime
instructions cannot accumulate because each turn is a fresh projection of state.

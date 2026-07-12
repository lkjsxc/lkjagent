# Runtime Loop

## Purpose

Define the direct durable cycle that replaces task and step projections.

## Cycle

1. Claim or refresh the daemon lease.
2. Read owner, wake, provider, effect, and file-change events.
3. Reduce events into current state and obligations.
4. Select one eligible operation deterministically.
5. Persist its immutable `RuntimeDecision` selection and exact compiler specs.
6. Compile context and attach source refs, rendered frame, and fingerprints.
7. Persist provider intent or prepare one native effect.
8. Perform at most one provider call or external effect.
9. Persist outcome events, observation, checks, message, and decision settlement.
10. Begin the next cycle from committed rows.

Context compilation cannot change the selected operation. A compile fault settles
the decision and makes no provider call.

## Decisions

A decision stores selected state, operation, exact tool descriptors, grammar,
context needs and caps, model budget, recovery policy, check requirements, exit
policy, source refs, frame fingerprint, and settlement status. A code change that
cannot honor a pending spec blocks it rather than reinterpreting it.

## Prompt State

Every model call rebuilds one system message from stable identity, active phase,
active fault, workspace boundary, exact tool cards, and one output grammar. A
state change therefore changes the prompt and often its visible tools.

## Progress

A progress fingerprint covers obligations, relevant file revisions, successful
observations, fresh checks, recovery strategy, and candidate response. Empty
polls and elapsed time are not progress. Repeated equal fingerprints trigger a
material strategy change rather than an identical model call.

## Quiescence

A per-invocation cycle cap returns control to the daemon and never ends a matter.
Idle is valid only when no operation, due wake, unsettled effect, pending provider
boundary, unanswered question, or recoverable open need exists.

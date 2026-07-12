# Runtime Loop

## Purpose

Define the direct durable cycle that replaces task and step projections.

## Cycle

1. Open and restart-project the native store.
2. Refuse replay of a sent provider exchange or unfinished effect.
3. Hydrate a `RuntimeSnapshot` and `RuntimeState` from active native cells.
4. Call the direct core selector and persist its immutable decision and specs.
5. Compile the prompt after selection from the current owner objective and
   current revision-bound source bytes; attach compiler facts and fingerprints.
6. Persist provider intent, mark sent, call the configured endpoint, and persist
   the bounded outcome before strict parsing.
7. Dispatch only through the persisted tool entry and direct effect key.
8. Settle one read/list/search observation, one exact edit/create effect, or one
   checked final response.
9. Reduce committed edits into three current native checks immediately.
10. Begin the next cycle from committed rows.

Context compilation cannot change the selected operation. Parse, hidden-tool,
and stale-revision failures settle through `reject_model_output`. A sent provider
boundary is ambiguous on restart and never replayed.

## Decisions

A decision stores selected state, operation, exact tool descriptors, grammar,
context needs and caps, model budget, recovery policy, file check requirements,
exit policy, frame fingerprint, and settlement status. A code change that cannot
honor unfinished work blocks it rather than reinterpreting it. The close
transaction excludes only its exact respond decision from blockers and settles
that decision atomically with the canonical final message and matter closure.

## Prompt State

Every model call rebuilds one system message from stable identity, active phase,
active fault, workspace boundary, exact tool cards, and one output grammar. The
selector stores the canonical phase projection on the decision: orient receives
only orient tools, modify only modify tools, and recovery only its intended tool.
Review and respond receive none. Prompt compilation, parsing, admission, and
dispatch consume that persisted view without reconstructing it from a catalog.
A state change therefore changes the prompt and often its visible tools.

## Progress

A progress fingerprint covers obligations, relevant file revisions, successful
observations, fresh checks, recovery strategy, and candidate response. Empty
polls and elapsed time are not progress. Repeated equal fingerprints trigger a
material strategy change rather than an identical model call.

## Quiescence

A per-invocation cycle cap returns control to the daemon and never ends a matter.
Idle is valid only when no operation, due wake, unsettled effect, pending provider
boundary, unanswered question, or recoverable open need exists.

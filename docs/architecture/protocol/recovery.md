# Recovery

## Purpose

The complete taxonomy of turn failures and the bounded response to each.
Recovery is designed so that one failure costs one small notice, the model
sees what failed, and the next turn can choose a narrower action. Repeated
recoverable faults add stronger recovery notices instead of stopping the
task; the task turn budget remains the hard bound.

## Taxonomy

| Fault | Detected by | Harness response |
| --- | --- | --- |
| missing or malformed act | parser ([parsing.md](parsing.md)) | error notice plus one recovery instruction |
| unknown tool | parser | error notice listing valid tool names plus recovery instruction |
| bad or duplicate params | parser | error notice listing every offender plus recovery instruction |
| repeat action | dispatcher: byte-identical act to previous turn | notice pointing at the prior observation plus recovery instruction |
| tool error | tool adapter | observation with status error plus a recovery instruction |
| endpoint error | llm client | capped exponential backoff retries; nothing appended until a completion arrives |
| endpoint overflow | llm client | treated as a harness bug: error event, compaction forced, incident memory row |
| oversize payload | context engine | truncation per [../context/budgets.md](../context/budgets.md) with retrieval path |
| task budget exhausted | loop | budget notice; only agent.ask or agent.done lawful next |

## Repeated Faults

Consecutive faults on one task are counted (resets on any successful
valid parsed action for parse faults and on non-repeat output for repeat
faults):

- 3 consecutive parse-class faults: the harness keeps the task open and adds
  a stronger recovery notice telling the model to emit one simple valid act
  block or ask if blocked.
- 3 consecutive repeat actions: same recovery notice pattern; the repeated
  action is not re-executed.
- Endpoint unreachable beyond the backoff cap (initial contract: 15
  minutes): daemon stays alive, state shows the outage, retries continue at
  the capped interval.

Recovery never discards state: the task stays open, the transcript holds the
fault trail, and the next endpoint turn sees the latest recovery notice.

## Retry Discipline

- Endpoint retries are invisible to the context: the request is re-sent
  unchanged, preserving the cache, and only the final completion is
  appended.
- Parse retries are visible by design: the faulty completion and the error
  notice both stay in the log, because the model needs to see its own
  mistake to stop making it, and the transcript must stay honest per
  [../../agent/honest-state.md](../../agent/honest-state.md).
- Tool errors are never retried by the harness. The observation and recovery
  notice tell the model to inspect the failure, adjust path, command, or
  parameters, and continue with a narrower action.

## Status

implemented.

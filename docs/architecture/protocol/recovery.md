# Recovery

## Purpose

The complete taxonomy of turn failures and the bounded response to each.
Recovery is designed so that one failure costs one small notice, repeated
failure escalates to the owner, and nothing ever crashes the loop or
silently disappears.

## Taxonomy

| Fault | Detected by | Harness response |
| --- | --- | --- |
| missing or malformed act | parser ([parsing.md](parsing.md)) | error notice quoting the grammar line violated |
| unknown tool | parser | error notice listing valid tool names |
| bad or duplicate params | parser | error notice listing every offender at once |
| repeat action | dispatcher: byte-identical act to previous turn | notice pointing at the prior observation; action not re-executed |
| tool error | tool adapter | observation with status error and a bounded cause |
| endpoint error | llm client | capped exponential backoff retries; nothing appended until a completion arrives |
| endpoint overflow | llm client | treated as a harness bug: error event, compaction forced, incident memory row |
| oversize payload | context engine | truncation per [../context/budgets.md](../context/budgets.md) with retrieval path |
| task budget exhausted | loop | budget notice; only agent.ask or agent.done lawful next |

## Escalation

Consecutive faults on one task are counted (resets on any successful
action):

- 3 consecutive parse-class faults: the harness pauses the task, records an
  error event, and sets daemon state to waiting for owner attention via
  `lkjagent status`.
- 3 consecutive repeat actions: same escalation; the loop is wedged and
  pretending otherwise would burn the window.
- Endpoint unreachable beyond the backoff cap (initial contract: 15
  minutes): daemon stays alive, state shows the outage, retries continue at
  the capped interval.

Escalation never discards state: the task stays open, the transcript holds
the fault trail, and the owner resumes by sending guidance through the queue.

## Retry Discipline

- Endpoint retries are invisible to the context: the request is re-sent
  unchanged, preserving the cache, and only the final completion is
  appended.
- Parse retries are visible by design: the faulty completion and the error
  notice both stay in the log, because the model needs to see its own
  mistake to stop making it, and the transcript must stay honest per
  [../../agent/honest-state.md](../../agent/honest-state.md).
- Tool errors are never retried by the harness; deciding whether to retry
  is the model's job with full information.

## Status

design-only.

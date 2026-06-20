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
| endpoint error | llm client | one error event per failed attempt, then capped exponential backoff before the next endpoint call |
| completion oversize | llm client finish_reason length without a closed act | error notice with preview plus instruction to emit one short valid action |
| endpoint overflow | llm client | treated as a harness bug: error event, compaction forced, incident memory row |
| oversize payload | context engine | truncation per [../context/budgets.md](../context/budgets.md) with retrieval path |
| task budget exhausted | loop | budget notice; only agent.ask or agent.done lawful next |

## Repeated Faults

Consecutive faults on one task are counted (resets on any successful
valid parsed action for parse faults and on non-repeat output for repeat
faults):

- 3 consecutive parse-class faults: the harness keeps the task open and adds
  a stronger recovery notice telling the model to emit one simple valid act
  block, switch large generated payloads to `fs.batch_write` or
  `doc.scaffold`, or ask if blocked.
- 3 consecutive repeat actions: same recovery notice pattern; the repeated
  action is not re-executed, and the model is told to inspect state, choose
  a different tool, or switch repetitive writes to `fs.batch_write`.
- Endpoint unreachable beyond the backoff cap (initial contract: 15
  minutes): daemon stays alive, state shows the outage, polls before the
  retry deadline do not append duplicate error events, and retries continue
  at the capped interval.

Recovery never discards state: the task stays open, the transcript holds the
fault trail, and the next endpoint turn sees the latest recovery notice.

## Retry Discipline

- Endpoint retries are invisible to the context until an attempt is made: the
  request is re-sent unchanged after the retry deadline, preserving the
  cache, and polls before the deadline do not append transcript noise.
- Parse retries are visible by design: the faulty completion and the error
  notice both stay in the log, because the model needs to see its own
  mistake to stop making it, and the transcript must stay honest per
  [../../agent/honest-state.md](../../agent/honest-state.md).
- Tool errors are never retried by the harness. The observation and recovery
  notice tell the model to inspect the failure, adjust path, command, or
  parameters, and continue with a narrower action. For shell.run errors, the
  observation adds targeted hints for common non-portable commands such as
  hardcoded /workspace paths or /bin/sh brace expansion.
- Completion oversize is not an endpoint outage. The daemon records it with
  a bounded preview, resets endpoint retry state, and appends a recovery
  notice telling the model to emit one short act block under about 1200
  characters. If the preview shows a bulk write, the notice directs the next
  action toward `fs.batch_write`, `doc.scaffold`, or a smaller `fs.write`,
  and reminds it not to bypass graph policy. A length response that already
  contains one closed act is accepted and passed to the parser.

## Status

implemented.

# Recovery

## Purpose

The complete taxonomy of turn failures and the bounded response to each.
Recovery turns structured faults into a new persisted runtime decision; prompt
wording, graph guidance, and tool observations cannot grant authority.

## Taxonomy

| Fault class | Detected by | Harness response |
| --- | --- | --- |
| provider empty content with usage | LLM wire classifier | provider anomaly route; no parser retry |
| provider reasoning-only message | LLM wire classifier | provider anomaly route; reasoning stays evidence only |
| missing action envelope | parser after provider anomaly classification | route from `MissingActionEnvelope` |
| unclosed action envelope | parser or wire | provider-stop repair or route from `UnclosedActionEnvelope` |
| implicit action envelope | parser and admission | record normalization and require normal admission |
| malformed tag | parser | exact tag grammar notice |
| attribute-like tag | parser | contextual repair tag and value |
| missing required parameter | registry | exact concrete valid example |
| conditional requirement missing | registry | `missing_any=checks|paths` style notice |
| unknown parameter | registry | list unknown names and valid names |
| duplicate parameter | parser | name the duplicate field |
| unsafe path-shaped parameter | dispatcher validator | refuse before mutation and show scoped batch form |
| batch-write payload fault | batch validator | refuse whole batch and show scoped `files` payload |
| payload too large | context or tool validator | route to artifact planning and bounded batches |
| blocked or not-admitted tool | admission | refuse and render current next executable action |
| stale decision | admission | compare full fingerprint and rerender current decision |
| repeat action | dispatcher guard | choose different tool family or deterministic inspection |
| tool runtime failure | tool adapter | observation event, then recovery decision |
| endpoint retry or overflow | llm client | bounded retry, compaction, or incident route |
| context pressure | context engine | runtime-owned compaction or retrieval route |
| verification failure | verifier | repair or blocked handoff route |
| completion blocked | completion reducer | audit or evidence repair action |
| maintenance conflict | runtime kernel | defer, preempt, or refuse stale maintenance action |

## Attribute-Like Tag Repair

A line like this is never a valid parameter:

```text
<path=stories/chronos-fracture</path>
```

The parser emits `AttributeLikeTag` with the invalid tag name and value hint.
The recovery route does not render the malformed line as the next valid action.
For `graph.plan`, the repair is:

```text
repair_tag=paths
repair_value=stories/chronos-fracture
```

For artifact and document root tools, the repair tag is `root`.

## Route Table Shape

Each route record contains:

```text
fault_class
fault_key
first_route
second_same_fault_route
third_same_fault_route
admitted_tools
blocked_tools
exact_valid_example
runtime_effect_when_no_model_content_needed
blocked_handoff_condition
```

The first route may render a concrete repaired action. The second same fault
cannot render the same action text as the only path. The third same fault uses a
deterministic inspection effect or records blocked handoff when no internal path
remains.

## Repeated Faults

Consecutive faults on one task are counted by case, node, tool, parameter shape,
fault class, and action fingerprint. Counts reset on a successful action that
changes state or on a non-repeat action for repeat faults.

- First parse or schema fault: render the exact valid action from the registry
  and current runtime decision.
- Second same fault: switch tool family, such as from `fs.batch_write` to
  `artifact.next` or from malformed planning to `graph.state`.
- Third same fault: execute deterministic inspection when available, otherwise
  record blocked handoff with the remaining evidence gap.

Recovery never discards state. The task stays open, the transcript holds the
fault trail, and the next endpoint turn sees the latest runtime decision.

## Retry Discipline

- Endpoint retries are invisible until an attempt occurs: the request is resent
  unchanged after the retry deadline.
- Provider anomalies are visible as provider events, but they are not parser
  faults and do not replay as invalid assistant examples.
- Parse retries are visible: the faulty completion and error notice remain in
  the log because the transcript must stay honest.
- Tool errors are not retried by the harness. The observation becomes a runtime
  event and the kernel selects the next route.
- Completion oversize records a bounded preview and routes to one short valid
  action, artifact planning, bounded batch writing, or document audit.
- Payload-risk recovery never repeats the same raw oversized write after the
  same fault.

## Observation Authority

Tool observations report facts:

```text
observation_result=...
next_runtime_event=...
next_decision_required=true
```

An observation may include `next_executable_action` only when it cites a
persisted decision id whose admitted tool surface includes that action. The
runtime decision stream, not tool text, supplies executable authority.

## Status

partially implemented. Recovery topology, retry counts, route metadata, and
some shape-changing routes exist. Attribute-like tag faults, strict implicit
envelope records, full route-table coverage, and authority-aware observation
text remain implementation tasks.

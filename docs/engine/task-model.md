# Task Model

## Purpose

Define how the current task record maps into the state-ledger case model.

## Current Rows

The current checkout stores delivered owner work as task rows with objective,
template, state, brief, model-call budget, and terminal summary. This is the
implemented plan-engine shape and remains useful evidence while the state ledger
is introduced.

## Target Rows

The state-ledger target stores owner work as a case plus state cells:

| Current task field | State-ledger owner |
| --- | --- |
| `id` | `cases.id` |
| `queue_id` | `events` source ref from queue intake |
| `objective` | `cases.objective` and clean context item |
| `template` | initial classification event and plan state payload |
| `state` | completion, recovery, and waiting state cells |
| `brief` | context item with provenance edges |
| `budget` | case budget state cell |
| `summary` | terminal report context item and case field |

## State Semantics

Open, waiting, blocked, and closed remain known helper statuses. They are not the
complete active state vector. A case may also carry simultaneous cells for
needed evidence, suppressed tools, unresolved context conflicts, artifact
fingerprints, retry policy, and completion blockers.

## Budget Rule

Endpoint-call budgets are represented as durable state and usage rows. Verify,
recovery, and engine-side work do not consume model-call budget unless the
runtime decision calls the endpoint.

## Failure This Prevents

Task status cannot hide concurrent facts such as waiting owner answer, evidence
needed, tool suppressed, and completion blocked.

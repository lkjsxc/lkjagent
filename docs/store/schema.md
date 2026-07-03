# Schema

## Purpose

Define the SQLite tables read and written by lkjagent.

## Database

The store file is `data/lkjagent.sqlite3`. SQLite runs with WAL and foreign keys
enabled. The schema table count is `store.schema.table-count=10`, counting the
FTS mirror as a table.

| Table | Columns | Writer | Reader |
| --- | --- | --- | --- |
| `queue` | id, content, state, force_new, timestamps, task_id | CLI, intake | intake, CLI |
| `tasks` | id, queue_id, objective, template, state, brief, budget, summary | engine | engine, CLI |
| `steps` | id, task_id, ordinal, kind, instruction, inputs_json, checks_json, state, attempts_used, actions_used, action_budget, split_used | engine | engine, CLI |
| `attempts` | id, step_id, ordinal, fingerprint, exchange_ref, outcome, diagnosis | engine | engine, proof |
| `check_results` | id, step_id, name, params_json, passed, measured_json, created_at | engine | engine, proof |
| `events` | id, task_id, kind, content, created_at | engine, intake | CLI, console |
| `memory` | id, topic, content, task_id, created_at | engine, explore | classifier, CLI |
| `memory_fts` | topic, content | triggers | memory search |
| `token_usage` | id, task_id, attempt_id, prompt_tokens?, completion_tokens?, cached_tokens? | engine | status, proof |
| `config` | key, value | CLI, daemon | daemon |

## Nullable Usage

Token counts are nullable. A null value means the endpoint did not report that
field. Status and proof render null usage as `unknown`, never as zero.

## Durable Owners

- Queue rows own `force_new`; command output alone is not durable routing state.
- Attempts own `exchange_ref`, which points to the exchange log directory for
  the attempt.
- Check results own the active `step_id`, parameters, structured measured value,
  pass flag, and timestamp.
- Token usage rows own prompt, completion, and cached token counts when the
  provider reports them.
- Config rows own settings only; they do not own active task snapshots or plan
  authority.

## Deliberate Non-Tables

There is no separate plan table; ordered steps are the plan. There are no
authority, admission, graph, artifact, personal-record, or provider-exchange
tables. Exchange files carry model request and response bodies.

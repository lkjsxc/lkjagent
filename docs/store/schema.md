# Schema

## Purpose

Define the SQLite tables read and written by lkjagent.

## Database

The store file is `data/lkjagent.sqlite3`. SQLite runs with WAL and foreign keys
enabled. The schema table count is `store.schema.table-count=10`, counting the
FTS mirror as a table.

| Table | Columns | Writer | Reader |
| --- | --- | --- | --- |
| `queue` | id, content, state, timestamps, task_id | CLI, intake | intake, CLI |
| `tasks` | id, objective, template, state, brief, budget, summary | engine | engine, CLI |
| `steps` | id, task_id, ordinal, kind, instruction, inputs, checks | engine | engine, CLI |
| `attempts` | id, step_id, ordinal, fingerprint, exchange_ref, outcome | engine | engine, proof |
| `check_results` | id, step_id, name, params, passed, measured | engine | engine, proof |
| `events` | id, task_id, kind, content, created_at | engine, intake | CLI, console |
| `memory` | id, topic, content, task_id, created_at | engine, explore | classifier, CLI |
| `memory_fts` | topic, content | triggers | memory search |
| `token_usage` | id, task_id, attempt_id, prompt, completion, cached | engine | status, proof |
| `config` | key, value | CLI, daemon | daemon |

## Deliberate Non-Tables

There is no separate plan table; ordered steps are the plan. There are no
authority, admission, graph, artifact, personal-record, or provider-exchange
tables. Exchange files carry model request and response bodies.

# Schema

## Purpose

Define the SQLite tables needed for arbitrary state, decisions, context hygiene,
and evidence-gated completion.

## Database

The store file is `data/lkjagent.sqlite3`. SQLite runs with WAL and foreign keys
enabled. Durable rows own runtime truth. Exchange files may hold large request,
response, or artifact bodies, but rows own resumable facts and references.

## Table Set

State-ledger tables are created beside the current plan-family tables until the
runtime reads only state rows. Required tables:

| Table | Role |
| --- | --- |
| `queue` | owner messages, answer routing, and forced-new intent |
| `cases` | owner objective, lifecycle summary, and terminal report |
| `runtime_events` | append-only runtime facts with optional decision id |
| `state_cells` | current arbitrary state vector |
| `state_history` | audit of applied state patches |
| `runtime_decisions` | persisted `RuntimeDecision` authority rows |
| `prompt_frames` | prompt metadata, fingerprints, and bounded body refs |
| `tool_admissions` | parsed action, result, and view fingerprint |
| `observations` | bounded tool or effect output tied to decisions |
| `context_items` | source-tagged prompt candidates |
| `context_edges` | provenance, suppression, and conflict links between context items |
| `state_edges` | generic relation evidence between state, records, artifacts, checks, and messages |
| `workspace_records` | current record metadata and fingerprints for owner-readable files |
| `workspace_record_history` | record fingerprint history for staleness checks |
| `artifacts` | files, roots, fingerprints, and ownership metadata |
| `check_results` | deterministic and judged evidence |
| `provider_exchanges` | endpoint request and response refs |
| `token_usage` | nullable provider usage fields |
| `memory` | durable owner-useful facts with FTS mirror when useful |
| `config` | owner settings and daemon lease values only |

## JSON Columns

JSON payloads include schema names and are validated at crate boundaries. Prefer
small typed mappers over ad hoc string parsing. Nullable provider usage means the
provider did not report the value, not zero.

## Indexes

Index state cells by case id, key, status, priority, and conflict group. Index
state edges by case id or workspace scope, relation, status, and endpoint refs.
Index context items by semantic key, contamination class, trust class, and source
fingerprint. Index runtime decisions by case id, status, and selected time.

## Failure This Prevents

The runtime can resume and explain why a tool was rendered, admitted, refused,
hidden, or recovered for a specific turn.

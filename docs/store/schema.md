# Schema

## Purpose

Define the SQLite tables needed for arbitrary state, decisions, context hygiene,
and evidence-gated completion.

## Database

The store file is `data/lkjagent.sqlite3`. SQLite runs with WAL and foreign keys
enabled. Durable rows own runtime truth. Exchange files may hold large request,
response, or artifact bodies, but rows own resumable facts and references.

## Table Set

State-ledger tables are created beside transitional plan-family tables until the
runtime reads only matter, state, and record rows. Required tables:

| Table | Role |
| --- | --- |
| `queue` | owner turns, answer routing, and separate-matter intent |
| `cases` | matter objective, lifecycle summary, and terminal report |
| `runtime_events` | append-only runtime facts with optional decision id |
| `state_cells` | current arbitrary state vector |
| `state_history` | audit of applied state patches |
| `runtime_decisions` | persisted `RuntimeDecision` authority rows with selected state key |
| `prompt_frames` | prompt metadata, fingerprints, and bounded body refs |
| `prompt_cards` | prompt-kernel card reasons and section fingerprints |
| `tool_admissions` | parsed action, result, and view fingerprint |
| `observations` | bounded tool or effect output tied to decisions |
| `context_items` | source-tagged prompt candidates |
| `context_edges` | provenance, suppression, and conflict links between context items |
| `state_edges` | generic relation evidence between state, records, artifacts, and checks |
| `workspace_records` | current record metadata and fingerprints for owner-readable files |
| `workspace_record_history` | record fingerprint history for staleness checks |
| `workspace_manifest` | schema-numbered workspace root and directory policy |
| `workspace_path_aliases` | old path to stable entity id and new path mappings |
| `workspace_rebalance_audit` | applied path moves with decision and validation data |
| `artifacts` | files, roots, fingerprints, and ownership metadata |
| `check_results` | deterministic and judged evidence with params, decision ids, and artifact refs |
| `provider_exchanges` | endpoint request and response refs |
| `token_usage` | nullable provider usage fields |
| `memory` | durable owner-useful facts with FTS mirror when useful |
| `config` | bridge settings and daemon lease values only |

## Queue Route Evidence

Queue rows keep nullable deterministic route evidence: lane, desired durability,
title seed, and transformation permission. Delivery refreshes these fields when
waiting-answer context changes a pending turn into an existing-matter answer.

## JSON Columns

JSON payloads include schema names and are validated at crate boundaries. Prefer
small typed mappers over ad hoc string parsing. Nullable provider usage means the
provider did not report the value, not zero.

## Indexes

Index state cells by case id, key, status, priority, and conflict group. Index
state edges by case id or workspace scope, relation, status, and endpoint refs.
Index context items by semantic key, contamination class, trust class, and source
fingerprint. Index runtime decisions by case id, status, and selected time. The decision JSON
also carries the selected state key used for settlement diagnostics.

## Failure This Prevents

The runtime can resume and explain why a tool was rendered, admitted, refused,
hidden, or recovered for a specific turn.

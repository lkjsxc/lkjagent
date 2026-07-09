# Product Contract

## Identity

lkjagent is a local-first personal agent harness for one owner, one continuously
running daemon, one OpenAI-compatible model endpoint, one visible workspace,
and one SQLite control ledger.

## Work Scope

The same harness handles:

- journal, todo, calendar, finance, notes, and personal knowledge;
- retrieval and comparison across recorded information;
- multiple long-lived projects and their decisions, tasks, sessions, and files;
- software repository inspection, editing, verification, and evidence;
- structured reports and other multi-file artifacts;
- deterministic workspace indexing, validation, and maintenance.

## Owner Experience

An imperative request starts work, not a readiness reply. The daemon continues
while executable work exists. It may pause only for a concrete owner answer,
external wake condition, or a truthful exhausted-recovery report.

The owner can inspect workspace files directly. Status and TUI views explain
current work, blockers, next wake conditions, and produced paths without
exposing noisy internal rows in ordinary conversation.

## Completion

Each matter has explicit obligations derived from owner intent. Completion
requires fresh evidence for every obligation. A response message is an effect of
completion, not completion authority.

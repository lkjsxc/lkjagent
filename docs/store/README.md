# Store

## Purpose

Define SQLite persistence, exchange refs, state hydration, transactions, and
crash resume.

## Table of Contents

- [schema.md](schema.md): exact fresh native tables and required fields.
- [schema-constraints.md](schema-constraints.md): identity, lineage, integrity,
  conversation, workspace, and database rules.
- [effect-journal.md](effect-journal.md): effect admission, two transactions,
  settlement, and recovery.
- [exchange-logs.md](exchange-logs.md): file log layout for large provider
  bodies and exchange refs.
- [crash-and-resume.md](crash-and-resume.md): decision-first transaction and
  recovery rules.

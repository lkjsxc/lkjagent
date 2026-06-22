# Observability

## Purpose

This directory defines the owner-visible runtime accounting contract: token
ledger, console deck, status format, and single model handoff log.

## Table of Contents

- [token-ledger.md](token-ledger.md): persisted endpoint and context usage.
- [console-deck.md](console-deck.md): compact live console status.
- [model-log.md](model-log.md): current Markdown handoff file.
- [status-format.md](status-format.md): plain status output contract.

## Local Map

- [token-ledger.md](token-ledger.md): owns counts and unknown handling.
- [console-deck.md](console-deck.md): owns terminal display placement.
- [model-log.md](model-log.md): owns the single external-model handoff.
- [status-format.md](status-format.md): owns `lkjagent status` fields.

## Reading Paths

- Implementation path: token-ledger, status-format, console-deck, model-log.
- Diagnosis path: status-format, token-ledger, model-log.
- Verification path: CLI tests, console render tests, store tests.

## Cross-Links

- Related contract: [../../product/observability.md](../../product/observability.md).
- Owning crate or module: `crates/lkjagent-cli/src`.

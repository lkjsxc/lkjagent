# Observability

## Purpose

This directory defines the owner-visible runtime accounting contract: token
ledger, console deck, status format, single model handoff log, and per-provider
exchange evidence.

## Table of Contents

- [token-ledger.md](token-ledger.md): persisted endpoint and context usage.
- [console-deck.md](console-deck.md): compact live console status.
- [model-log.md](model-log.md): current Markdown handoff file.
- [provider-exchange-log.md](provider-exchange-log.md): raw model request and response evidence.
- [status-format.md](status-format.md): plain status output contract.

## Local Map

- [token-ledger.md](token-ledger.md): owns counts and unknown handling.
- [console-deck.md](console-deck.md): owns terminal display placement.
- [model-log.md](model-log.md): owns the single external-model handoff.
- [provider-exchange-log.md](provider-exchange-log.md): owns per-call replay records.
- [status-format.md](status-format.md): owns `lkjagent status` fields.

## Reading Paths

- Implementation path: token-ledger, status-format, console-deck, model-log, provider-exchange-log.
- Diagnosis path: status-format, token-ledger, model-log, provider-exchange-log.
- Verification path: CLI tests, console render tests, store tests.

## Cross-Links

- Related contract: [../../product/observability.md](../../product/observability.md).
- Owning crate or module: `crates/lkjagent-cli/src`.

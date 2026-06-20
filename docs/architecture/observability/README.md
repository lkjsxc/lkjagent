# Observability

## Purpose

This directory defines the owner-visible runtime accounting contract: token
ledger, console deck, status format, and single GPT handoff log.

## Table of Contents

- [token-ledger.md](token-ledger.md): persisted endpoint and context usage.
- [console-deck.md](console-deck.md): compact live console status.
- [gpt-log.md](gpt-log.md): current Markdown handoff file.
- [status-format.md](status-format.md): plain status output contract.

## Local Map

- [token-ledger.md](token-ledger.md): owns counts and unknown handling.
- [console-deck.md](console-deck.md): owns terminal display placement.
- [gpt-log.md](gpt-log.md): owns the single external-model handoff.
- [status-format.md](status-format.md): owns `lkjagent status` fields.

## Reading Paths

- Implementation path: token-ledger, status-format, console-deck, gpt-log.
- Diagnosis path: status-format, token-ledger, gpt-log.
- Verification path: CLI tests, console render tests, store tests.

## Cross-Links

- Related contract: [../../product/observability.md](../../product/observability.md).
- Owning crate or module: `crates/lkjagent-cli/src`.

# lkjagent-llm Tests

## Purpose

This directory holds endpoint client tests using pure tables and a local
one-shot HTTP test double.

## Table of Contents

- [backoff.rs](backoff.rs): pure retry delay table tests.
- [client.rs](client.rs): local stub-server request and error mapping tests.
- [support/](support/README.md): one-shot HTTP test double helpers.
- [wire.rs](wire.rs): request serialization and response parsing tests.
- [wire_contract.rs](wire_contract.rs): transport, bounded-response, and outcome contract tests.

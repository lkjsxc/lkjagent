# lkjagent-llm Source

## Purpose

This directory holds the endpoint client, wire model, and retry schedule.

## Table of Contents

- [client.rs](client.rs): one-call HTTP adapter and capped retry eligibility.
- [error.rs](error.rs): endpoint error classification.
- [lib.rs](lib.rs): library root.
- [wire.rs](wire.rs): request and response wire subset.
- [wire/](wire/README.md): wire helper modules.
- [message.rs](message.rs): message roles and response closure handling.

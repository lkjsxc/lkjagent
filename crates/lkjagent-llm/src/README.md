# lkjagent-llm Source

## Purpose

This directory holds the endpoint client, wire model, and retry schedule.

## Table of Contents

- [client.rs](client.rs): blocking HTTP request adapter and capped retry schedule.
- [error.rs](error.rs): endpoint error classification.
- [lib.rs](lib.rs): library root.
- [wire.rs](wire.rs): request and response wire subset.
- [wire/](wire/README.md): wire helper modules.
- [closure.rs](closure.rs): closure source module.

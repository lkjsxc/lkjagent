# lkjagent-store Memory Helpers

## Purpose

This directory holds private helpers for the memory store API.

## Table of Contents

- [identity.rs](identity.rs): stable memory identity and duplicate decisions.
- [prune.rs](prune.rs): exact duplicate pruning with real deletes.
- [row.rs](row.rs): memory row struct, required-row lookup, and query mapping.
- [search.rs](search.rs): FTS query normalization and ranked search.

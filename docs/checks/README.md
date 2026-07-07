# Checks

## Purpose

Define deterministic checks and the word counting rule used by completion,
benchmarks, replay, and proof bundles.

## Table of Contents

- [catalog.md](catalog.md): check names, parameters, and pass rules.
- [word-counting.md](word-counting.md): the shared artifact word rule.

## Failure This Prevents

Completion evidence is computed by the engine and shared by evaluation, so the
product and its tests cannot drift into separate truth sources.

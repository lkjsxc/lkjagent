# Source Map

## Core

Expected ownership:

- matter and obligation domain;
- operation graph and feasibility;
- events, reducer, transitions, selector;
- completion predicates and progress vector;
- context candidates, selection, conflict logic;
- prompt cards, tool views, protocol parsing and admission.

## Store

- native schema and transactions;
- event append and reduced projections;
- effect journal and idempotency;
- conversation sequence;
- workspace document and search projections;
- evidence queries for proof.

## Effects

- WorkspaceService;
- Git and verification commands;
- endpoint call boundary;
- atomic file operations and recovery;
- observation redaction.

## App

- queue intake and intent compiler;
- daemon event loop and wake sources;
- decision interpreter;
- CLI and status;
- shared TUI core and terminal backends;
- configuration loading.

## Xtask

- docs and source gates;
- clean checkout;
- failure replay;
- experiment matrix;
- live scenarios and PTY trace;
- proof collection and acceptance.

Avoid bridge-named modules in the final source tree.

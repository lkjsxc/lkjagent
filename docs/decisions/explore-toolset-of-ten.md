# Tool Catalog And Decision Views

## Purpose

Record the decision to derive model-visible tools from a catalog and persisted
runtime decision.

## Context

A fixed tool list cannot express legality by active state, evidence need,
workspace boundary, owner settings,
budget, retry suppression, and recovery constraints. Duplicating tool law across
docs, prompt text, parser, and dispatcher creates drift.

## Decision

There is one tool catalog. For each `RuntimeDecision`, policy layers derive a
`ToolSetView` containing only tools admissible for that turn. Prompt rendering
lists only that view. Parser and admission validate model tool calls against the
same persisted view fingerprint.

## Consequences

A tool the harness would reject for the turn is absent from the prompt. Hidden
and denied tools are recorded for diagnostics and proof bundles, not normal
prompt text. Explore-style tool calls become one decision-visible operation,
not a separate global authority.

## Rejected Alternatives

A broad visible registry with caveats would train the model to probe rejected
tools. A fixed registry would prevent state-derived evidence gathering and
recovery policies.

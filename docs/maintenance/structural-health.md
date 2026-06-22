# Structural Health

## Purpose

Structural health owns metrics for idle maintenance. The scan reports shape,
link density, duplicate patterns, thin files, overgrown files, generic phrase
risk, model-name risk, and recurring audit failures.

## Metrics

```text
directory fan-out
file depth distribution
README coverage
orphan count
broken link count
cross-link density
topic centrality
duplicate heading patterns
near-duplicate content clusters
thin-file count
overgrown-file count
generic phrase density
model-name count
audit failure recurrence
maintenance no-op recurrence
```

## Evidence Rule

A maintenance claim about workspace shape must come from filesystem inspection
or a structural scan, not memory search alone.

## Status

design-only

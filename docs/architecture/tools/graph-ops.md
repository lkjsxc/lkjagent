# Graph Operations

## Purpose

Specify graph tools exposed to the model when automatic harness updates need a
model-visible action surface.

## graph.state

Shows the active case id, objective, task family, phase, node, selected
context packages, evidence gathered, missing requirements, and legal next
transitions. This is read-only. The harness also injects graph state notices
automatically, so the model does not have to call this before every action.

## graph.evidence

Records explicit evidence when the harness cannot infer it from a tool output.
The model supplies `kind`, `summary`, and optional `path`. The runtime links
the evidence to the active case and active graph node.

## Status

implemented.

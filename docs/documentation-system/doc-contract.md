# Documentation Contract

## Purpose

The documentation contract owns what must be known before lkjagent writes docs.
It converts an owner request into central subject, topic roles, relations,
forbidden outputs, required evidence, and completion gates.

## Required Fields

```text
task kind
central subject
requested topics
topic roles
scope boundary
required connections
forbidden assumptions
forbidden outputs
required audits
completion rule
```

## Multi-Topic Rule

When the owner requests several topics together, no topic may exist only as an
isolated root README section. The contract must name at least one relation
among the topics before any file write.

## Example Roles

For `lkjagent`, a named model family, and Rust:

- lkjagent is the central project and runtime.
- the named model topic maps to model interface or adapter evidence.
- Rust maps to implementation substrate, typed state, tools, and reducers.

## Status

implemented

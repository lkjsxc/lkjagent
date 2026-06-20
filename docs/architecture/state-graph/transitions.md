# State Graph Transitions

## Purpose

Specify how lkjagent moves through graph states and how illegal movement is
refused.

## Admission

Every transition request is evaluated by the graph crate before it affects
runtime state. Edges carry typed guards such as plan-ready, context-selected,
completion-ready, family-in, context-pressure, fault-count, and
document-audit-ready. The decision is one of:

- admit: move to the target node and record a graph event.
- defer: stay in place and name the missing evidence or context.
- recover: route to a recovery node with the failure evidence attached.
- refuse: reject the transition because it violates graph policy.

The harness can admit deterministic transitions automatically. Model-authored
actions may request transitions through graph tools, but the harness owns the
state mutation.

## Required Sequence

A new owner task follows this minimum sequence:

1. classify intent.
2. create or update the task case.
3. enter planning.
4. select context packages.
5. render the graph state notice.
6. call the endpoint.

Execution nodes come after planning and context guards pass. Mutating tools
remain blocked until the active graph node allows them. Completion nodes come
after required evidence and pending checks are clear. Recovery nodes are
entered when parsing, endpoint, tool, verification, repetition, pressure, or
budget failures occur.

## Status

implemented.

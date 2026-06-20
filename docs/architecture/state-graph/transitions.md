# State Graph Transitions

## Purpose

Specify how lkjagent moves through graph states and how illegal movement is
refused.

## Admission

Every transition request is evaluated by the graph crate before it affects
runtime state. Edges carry typed guards such as plan-ready, context-selected,
completion-ready, family-in, context-pressure, fault-count, and
document-audit-ready. Edge priority gives deterministic tie-break order when
several edges are legal. The decision is one of:

- admit: move to the target node and record a graph event.
- defer: stay in place and name the missing evidence or context.
- recover: route to a recovery node with the failure evidence attached.
- refuse: reject the transition because it violates graph policy.

The harness can admit deterministic transitions automatically at safe
boundaries. Model-authored actions may request transitions through graph
tools, but the harness owns the state mutation and records every accepted
transition.

## Required Sequence

A new owner task follows this minimum sequence:

1. create or resume a durable graph case.
2. normalize objective, non-goals, constraints, assumptions, risks, and
   success criteria.
3. classify family and route reason.
4. survey workspace shape through bounded native tools or runtime helpers.
5. select context packages from graph policy.
6. build and review a structured plan with evidence requirements.
7. render the graph state notice.
8. call the endpoint.

Execution nodes come after planning and context guards pass. Mutating tools
remain blocked until the active graph node allows them. Completion nodes come
after required evidence and pending checks are clear. Recovery nodes are
entered when parsing, endpoint, tool, verification, repetition, pressure, or
budget failures occur.

The graph is a network. Verification can route to repair, compaction can
return to context rebuild, recovery can inspect state or choose a smaller
scope, and maintenance can record policy candidates without editing source.

## Status

implemented.

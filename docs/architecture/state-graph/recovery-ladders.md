# Recovery Ladders

## Purpose

Define how typed faults move through graph recovery without turning generic
errors into owner waiting.

## Fault Ledger

Every parse, repeat, tool, shell, endpoint, verification, context, budget, or
completion fault records:

- kind, active node, action fingerprint, summary, count, and timestamp.
- recovery ladder position and selected strategy.
- recent-fault health state rendered in the graph card.

Repeated fingerprints are evidence that the next action class must change.
The runtime records the fault before choosing the next node.

## Ladder

Recovery follows this deterministic order:

1. correct the action format or parameters and retry once.
2. inspect graph state or workspace state and choose a different native tool.
3. reduce scope and create a smaller plan step.
4. route to the matching recovery node and record the decision.
5. use shell only from a shell-admitted recovery node.
6. mark the specific plan step blocked and continue an independent step.

Parameter faults have their own route. Three consecutive parser-level
parameter faults move the graph to `recover-params`, render the valid action
example for the rejected tool, and expose only `graph.state`, `fs.list`,
`workspace.summary`, and `agent.ask` as next actions.

Waiting is valid only for a concrete owner decision, context invalidity,
endpoint outage policy, or sandbox boundary.

`graph.next` is diagnostic. It may inspect one fault route, but repeating it
in the same recovery state records diagnostic exhaustion and forces a
different action class: `graph.recover`, a legal transition, an unused
non-mutating native tool, or a smaller plan step.

Recovery nodes never allow endless inspection. Every inspection route has a
state-changing consequence: alternate tool, smaller scope, shell-admitted
escape, blocked step, or owner-required question.

`graph.note` accepts only constraint, assumption, risk, decision, question,
invariant, success, and path. Registry examples and refusal text must render
only those accepted kinds.

Waiting is forbidden when an internal transition, alternate native tool, or
smaller independent step can continue.

## Status

implemented.

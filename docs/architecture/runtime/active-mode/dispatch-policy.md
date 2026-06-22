# Dispatch Policy

## Purpose

Define how tool execution consumes turn authority.

## Order

Dispatch performs these steps in order:

1. parse exactly one closed `act` block.
2. normalize safe aliases that are unambiguous for the target tool.
3. validate parameter names and values against the registry.
4. check repeated action fingerprints.
5. refresh or reuse `TurnAuthority.effective_policy` as allowed by
   [turn-authority.md](turn-authority.md).
6. reject stale maintenance, compaction, or idle actions when current authority
   no longer admits them.
7. route the tool.
8. record observation or refusal.
9. update graph evidence and fault state.

## Policy Ownership

Owner task and recovery modes may inherit graph policy. Maintenance mode uses
maintenance policy only unless its own policy explicitly admits inspection
tools. Compaction and closed idle admit no model-authored dispatch.

The effective policy is the only policy checked by the dispatcher. Prompt text,
graph suggestions, maintenance directives, and recovery notices cannot admit a
tool that the effective policy refuses.

## Refusals

A refusal names:

- active mode.
- failed tool.
- failed gate.
- admitted tools.
- blocked tools when useful.
- normalized parameters, when normalization happened.
- expected and provided parameter names for schema failures.
- one copyable next action accepted by the same effective policy.

`agent.done` uses completion-specific checks after active-mode admission. A
completion refusal keeps the case open or blocked and returns the same style of
copyable next action.

## Status

partially implemented. Effective-policy dispatch, repeat refusals, registry
examples, and many schema refusals exist. Registry-wide proof that every
rendered recovery example parses, validates, and dispatches remains open.

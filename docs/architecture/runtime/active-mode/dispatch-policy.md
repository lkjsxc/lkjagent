# Dispatch Policy

## Purpose

Define how tool execution consumes turn authority.

## Order

Dispatch performs these steps:

1. parse one closed `act` block.
2. normalize safe aliases.
3. validate parameters against the registry.
4. check repeated action fingerprints.
5. check `TurnAuthority.effective_policy`.
6. route the tool.
7. record observation.
8. update graph evidence and fault state.

## Policy Ownership

Owner task and recovery modes may inherit graph policy. Maintenance mode uses
maintenance policy only unless its own policy explicitly admits inspection
tools. Compaction and closed idle admit no model-authored dispatch.

## Refusals

A refusal names the active mode, failed tool, failed gate, admitted tools, and
one copyable next action that the same policy accepts. `agent.done` has
completion-specific checks, but it must first pass through the active
completion policy.

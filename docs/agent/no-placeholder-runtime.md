# No Placeholder Runtime

## Purpose

Make the project honesty rule concrete for runtime behavior, artifacts, tests,
fixtures, and prompt claims.

## Product Code

Product code may not return fake success, stub data, placeholder bodies, or
silent no-op effects. A path that lacks required facts returns a typed error,
structured refusal, open blocker, or blocked handoff. It does not pretend the
effect happened.

A prompt-only instruction is not an implementation. Runtime behavior is claimed
only when code, focused tests, and the documented gates prove the behavior.

## Artifacts

Scaffold topology, file counts, README links, manifests, headings, and planning
notes are structure evidence only. They cannot satisfy content readiness.
Scaffold phrases, status-only files, empty headings, generic "this file
records" prose, and unsupported verification claims keep completion blocked.

## Memory And Maintenance

Memory rows record observed events and useful distilled facts. No-op
maintenance does not write rows saying nothing changed. Exact duplicate memory
rows are skipped, and high-overlap same-title rows are merged or pruned by the
memory policy.

## Fixtures And Tests

Fixtures are either captured evidence or explicit contract fixtures. A fixture
must not assert that live endpoint behavior exists when only a contract exists.
Known-bad fixtures stay known-bad until runtime code and gates prove the
failure pattern cannot recur.

## Close Claims

`agent.done`, graph close, recovery handoff, turn-budget handoff, console close,
and daemon shutdown handoff all use the same completion evidence. A summary
names only observed files, tool results, audits, verification results, and
blocked evidence.

## Status

implemented as a rule; enforcement is partial and tracked in the blocker queue.

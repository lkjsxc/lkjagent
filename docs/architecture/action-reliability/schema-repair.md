# Schema Repair

## Purpose

Define deterministic repair for action parse faults, parameter faults, invalid
kinds, duplicate reads, and repeated invalid actions.

## Contract

Fault classes are `ParseFault`, `ParameterFault`, `ToolRuntimeFault`,
`RepeatFault`, `PolicyContradiction`, `PayloadTooLarge`,
`VerificationMismatch`, `CompletionRefused`, `CompactionPressure`, and
`MaintenanceConflict`.

Each fault class has a finite ladder. A parameter fault renders the exact
schema for the failed tool, retries once through canonical parameters when the
tool is still needed, then chooses an alternate observation or emits a policy
contradiction. A payload fault never retries raw `fs.write`; it routes to
artifact planning, `artifact.next`, or bounded `fs.batch_write`.

## Schema Source

One registry owns tool name, required fields, optional fields, field encoding,
canonical example, normalization rules, dispatcher parser, and admission
metadata. Every valid example shown to the model must parse, validate, and
pass dispatcher admission in tests.

## Batch Write Rules

The canonical `fs.batch_write` example is:

```text
<act>
<tool>fs.batch_write</tool>
<files>
path: some/path.md
content:
# Title

Body.

-- lkjagent-next-file --
path: other/path.md
content:
# Title

Body.
</files>
</act>
```

Normalization may accept `path:foo`, `<path>foo</path>`, XML-ish accidental
wrappers, and extra blank lines before `path:`. Rendered examples always use
canonical format. If the same syntax fault repeats, authority switches to
normalized parse, one-file fallback, deterministic writer, or blocked handoff.

## Invariants

- A refusal shows exactly one canonical valid action example.
- The example must parse, validate, and pass the same dispatcher admission.
- Invalid `graph.evidence` kinds list allowed evidence requirements.
- Unknown `scale` in `artifact.apply` names accepted parameters.
- Repeated invalid `graph.state` must select a different next action.
- Nested `<path>` child parameters are valid for `fs.read` and `fs.stat`.
- Large payload recovery never suggests another raw large write.

## Failure Cases

- Invalid `graph.evidence kind=evidence` loops into generic graph state.
- Unknown `scale` in `artifact.apply` renders multiple conflicting examples.
- Repeated `graph.state` survives repeat protection unchanged.
- `fs.read` or `fs.stat` rejects a child `<path>` parameter.
- Large payload recovery suggests another raw large write.

## Verification

Focused tests cover child tags for `fs.read`, `fs.stat`, and `fs.list`,
allowed evidence kinds, unknown parameter rejection, exact example rendering,
no repeated invalid example loop, payload-to-batch recovery, XML-ish accidental
wrappers, and dispatch of every generated valid example.

## Related Files

- [parameter-contract.md](parameter-contract.md)
- [recovery.md](recovery.md)
- [error-messages.md](error-messages.md)

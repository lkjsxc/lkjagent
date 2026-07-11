# Toolset View And Admission

## Purpose

Tie prompt-visible tools and tool-call admission to the persisted runtime
decision.

## ToolSetView

`ToolSetView` is produced from the catalog, policy layers, and active state
vector. It contains only tools admissible for the current decision; the global
catalog is never rendered by default. Default explore views exclude `shell.run`.
Shell-like tools are present only when the persisted decision carries an
explicit shell-capable view for development, verification, or recovery work.
Each entry renders the tool name, purpose, required arguments, optional
arguments, `ToolFieldSpec` value classes, value rules, relevant limits, and one
concise XML-like skeleton when budget allows. Persisted views retain only
canonical catalog entries. Admission rejects a removed action, including
`finish`, even when malformed stored data names it.

## View Fingerprint

The tool-view fingerprint is stored on `RuntimeDecision`, `PromptFrame`, and
`ToolAdmission`. Fingerprints are deterministic over canonical data, not debug
strings.

## Parser

The parser receives the expected envelope and view from the decision. Action
turns must use exactly one dedicated `lkjagent_action` block and one `input`
block containing direct decision-approved field tags. Name/value argument
wrappers, unknown blocks, attributes, prose outside the block, duplicate scalar
tags, stale decision ids, missing arguments, wrong primitive classes,
JSON-looking bodies, and tools absent from the view produce structured faults.
Unknown means absent from the decision view, not absent from a hidden global
list.

## Admission

Admission validates the parsed tool call against the same view fingerprint and
then runs final deterministic checks such as placeholder rejection, value-class
validation, path canonicalization, budget remaining, state suppressors, and
recovery constraints. Admitted and rejected parsed actions persist
`ToolAdmission` rows with the decision id, action tool, status, reason, parsed
action, and tool-view fingerprint before effects run. A prompt/admission
mismatch is rejected with a `tool-view mismatch:` reason and is a high-severity
runtime event.

## Fault Handling

Raw failed model output is stored in exchange logs and marked contaminated.
Normal retry prompts include only bounded diagnoses, invalid-excerpt hashes, and
the exact required XML-like action shape. Empty executable values and
placeholder-looking executable values such as `...`, `PATH`, `TODO`, `VALUE`,
`FIELD_VALUE`, `<path>`, or `[path]` are rejected before effects.

## Gate Coverage

Before protocol-tool evidence can pass, its named Docker gate must execute
nonempty behavioral suites for parser faults, decision and view binding, UTF-8
byte bounds for every scalar, direct field, and full action, path containment,
repeated admissions, typed recovery, and exact
admission-to-effect lineage. It also rejects a special `finish` catalog entry:
a typed report cannot settle an operation or completion. The corpus includes
Japanese and multiline values, XML entities, truncation, stale decisions,
unknown and duplicate fields, and wrong primitive values. A printed metric or
an editable label is not evidence.

## Failure This Prevents

A model tool call cannot sneak through a dispatcher path that was absent from
the prompt, and a legal prompt tool call cannot be refused by a stale parser
registry.

# Toolset View And Admission

## Purpose

Tie prompt-visible tools and tool-call admission to the persisted runtime
decision.

## ToolSetView

`ToolSetView` is produced from the catalog, policy layers, and active state
vector. It contains only tools admissible for the current decision. Each entry
renders the tool name, purpose, exact XML `<tool_call>` shape, required fields,
optional fields, relevant limits, and one concise example when budget allows.

## View Fingerprint

The tool-view fingerprint is stored on `RuntimeDecision`, `PromptFrame`, and
`ToolAdmission`. Fingerprints are deterministic over canonical data, not debug
strings.

## Parser

The parser receives the expected envelope and view from the decision. Unknown
blocks, empty blocks, duplicate parameters, missing required parameters, unknown
parameters, and tools absent from the view produce structured faults. Unknown
means absent from the decision view, not absent from a hidden global list.

## Admission

Admission validates the parsed tool call against the same view fingerprint and
then runs final deterministic checks such as path canonicalization, budget
remaining, state suppressors, and recovery constraints. A prompt/admission
mismatch is a high-severity runtime event.

## Fault Handling

Raw failed model output is stored in exchange logs and marked contaminated.
Normal retry prompts include only bounded diagnoses and the exact required
change.

## Failure This Prevents

A model tool call cannot sneak through a dispatcher path that was absent from
the prompt, and a legal prompt tool call cannot be refused by a stale parser
registry.

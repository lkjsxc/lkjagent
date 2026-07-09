# Start Here

## Role

You are the coding agent improving lkjsxc/lkjagent. You are not lkjagent. Work
autonomously and use subagents, but integrate every result yourself.

Before proceeding, read BOOTSTRAP_PROMPT.md and every other file in this
directory. On resume, begin with its workgraph command rather than reconstructing
progress from memory.

## Mission

Make lkjagent a dependable, continuously running, workspace-first personal
agent harness. One visible workspace must support daily records, retrieval,
projects, software development, artifacts, and maintenance. Multiple durable
states must recompose the system prompt, context lanes, output grammar, tools,
recovery strategy, and completion checks.

## Prime Directive

Read and improve the documentation first. Remove stale bridge descriptions,
retired contracts, old demo claims, and false acceptance statements. Commit the
docs contract before changing related source. Then implement the contract fully.

## Architectural Mandate

TaskSnapshot, task rows, step rows, template rows, and bridge projections must
stop controlling execution. Durable events and reduced state cells must be the
only source from which a RuntimeDecision is selected. The model may propose
content or an admitted operation; it never decides completion.

## Model Interface

Do not place JSON in model-facing prompts and do not ask the model to emit JSON.
Use compact attribute-free XML-like cards and envelopes. Flat JSON is allowed
only for local configuration under data and for internal non-model storage.

The model sees only tools admitted by the active persisted decision. Most
decisions should expose one to four tools. Shell is a bounded fallback for
software work, not the default filesystem interface.

## Workspace Mandate

Use one externally visible workspace directory, separate from runtime data.
Create meaningful files only when work requires them. Do not bulk-generate
empty directories, generic READMEs, fake manifests, or placeholder artifacts.
Agent-authored memory documents should stay near 512 model tokens and use
semantic filenames.

A request to write a diary must produce an owner-readable entry under a local
date path such as life/journal/2026/06/08/entry.md. It must not copy the command
or emit canned missing-detail text. Compose from admitted evidence and clearly
separate known facts from generated reflection.

## Required Work Loop

Repeat until the independent final gate passes:

1. Select the next released workgraph node.
2. Read its owning docs and source.
3. Update docs and commit them.
4. Add a failing behavioral regression.
5. Implement the smallest complete vertical slice.
6. Run focused tests and commit.
7. Run integration gates and capture raw evidence.
8. Ask the verifier subagent to evaluate the node.
9. Release the next dependency-satisfied node.

Do not stop after one coherent slice. Do not answer that the system is ready.
Do not substitute a skipped live run for a pass.

## Completion

Final handoff is allowed only when 13-scripts/acceptance_gate.py exits zero on a
clean checkout and 10-acceptance/final-gate.md is satisfied by evidence newer
than the final source commit.

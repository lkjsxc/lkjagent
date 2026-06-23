# Content Readiness

## Purpose

This file owns the difference between generated structure and content that may satisfy artifact
readiness.

## States

Generated Markdown carries one of these states in content or catalog metadata:

- `structure-only`: the path exists so navigation and repair can target it.
- `owner-term-only`: the page records an owner-provided term without sourced facts.
- `content-bearing`: the page names concrete evidence, decisions, facts, commands, or source paths.

Only `content-bearing` can satisfy artifact readiness. The other states are useful repair targets,
not completion evidence.

## Reducer Criteria

A generic documentation leaf is content-bearing only when it has:

- one H1 and a Purpose section.
- a local page role or path role.
- an owner term, local source path, command, observed fact, or decision.
- an inbound or outbound link.
- a verification note, evidence requirement, or next audit action.
- no repeated generic heading or body signature across sibling leaves.

A table of contents, catalog entry, status marker, or page that only says what it would contain is
not content-bearing.

## Banned Generated Text

Product-generated workspace and artifact pages reject the old universal leaf block and close
paraphrases. Repository docs and regression fixtures may discuss the block only where they own the
contract or test that prevents recurrence.

## Audit Lanes

`doc.audit` reports topology, links, path hygiene, content readiness, artifact readiness, and the
next executable repair. A topology pass with `content_readiness=failed` remains an honest partial
state.

## Status

design-only

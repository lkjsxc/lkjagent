# Documentation Standards

## Purpose

The shape, topology, and content rules for every Markdown file. These rules
exist so an LLM can navigate from any README to any contract in at most
three links and trust what it reads.

## File Shape

- First line: `# Title`. Then a `## Purpose` section stating what the file
  owns. Then the content.
- ASCII only. Prose lines at most 120 characters; table rows and fenced
  code blocks are exempt from the width rule. Tables at most 6 columns.
- Leaf specifications for unbuilt behavior end with a `## Status` section
  holding one of: implemented, design-only, not implemented, out of scope,
  open question.
- Filenames are kebab-case nouns; directories are kebab-case singulars or
  established plurals.

## Topology

- Every docs directory holds exactly one README.md and at least two other
  children (files or directories).
- Every README.md carries a `## Table of Contents` linking every sibling
  child with a one-clause description. A child not in its README does not
  exist.
- [../README.md](../README.md) additionally carries the flat All Files
  manifest for single-scan discovery.
- Depth is bounded by need: a directory appears when a topic outgrows one
  file per [line-limits.md](line-limits.md), not before.

## Content Rules

- One rule, one owner. Restating another file's rule is a defect; link it.
- Links are relative paths; bare URLs appear only when naming an external
  reference deliberately.
- State current contracts directly. No history narration, no future tense
  for shipped behavior, no compatibility framing, no milestone or release
  shorthand anywhere.
- Banned tokens, enforced by the check-docs gate: release shorthand and
  compatibility vocabulary as listed in the gate's table, including the
  word that ends in -ersion and single-letter-plus-number release tags.
  Spell out what is actually meant instead.
- Mark uncertainty as an open question instead of implying behavior exists.
- Examples must be real: real paths, real commands, real tag grammar. An
  example that would fail if executed is a defect per
  [../agent/honest-state.md](../agent/honest-state.md).

## Voice

Declarative and specific. "The dispatcher refuses duplicate reads" beats
"duplicate reads should generally be avoided". Hedging words (should,
generally, ideally) are rationed to genuinely soft contracts.

## Enforcement

The check-docs gate enforces shape, topology, TOC completeness, ASCII,
prose width, table width, and banned tokens. Until it exists, the interim
checks in [../operations/verification.md](../operations/verification.md)
cover line caps, README topology, TOC completeness, and banned tokens.

# Templates

## Purpose

Define transitional templates that project owner intents into initial state
cells, checks, and workspace refs.

## Table of Contents

- [docs-tree.md](docs-tree.md): structured documentation artifacts.
- [file-work.md](file-work.md): create, revise, organize, and summarize files.
- [question.md](question.md): direct answer matters.
- [journal.md](journal.md): personal record file capture.
- [generic.md](generic.md): fallback matter shape when no specific template fits.

## Contract

A template is pure data. It reads the owner turn, state snapshot, and config. It
returns initial operations plus matter-level checks from `lkjagent-core`. The
app persists rows and events; templates never perform effects.

## Extensibility

Matter families stay as data over the engine. A family cannot grow a hidden
tool registry, prompt-only policy, or separate completion rule.

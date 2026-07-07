# Templates

## Purpose

Define the pure template functions that turn owner objectives into initial
plans and task checks.

## Table of Contents

- [docs-tree.md](docs-tree.md): structured Markdown knowledge-base plan.
- [file-work.md](file-work.md): workspace create and revise work.
- [question.md](question.md): direct answer tasks.
- [journal.md](journal.md): personal record file tasks.
- [generic.md](generic.md): bounded exploration fallback.

## Shared Contract

A template is a pure function over extracted objective fields, memory facts,
and config. It returns initial steps plus task checks from
[../completion.md](../completion.md). It performs no IO.

## Failure This Prevents

Task families stay as data over the engine. A family cannot grow a hidden tool
or control plane that bypasses checks.

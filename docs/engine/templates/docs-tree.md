# Docs Tree Template

## Purpose

Define how lkjagent writes structured Markdown documentation trees.

## Objective Fields

The classifier extracts root, topic, requested sections, page budget, and any
required filenames. The default cap is `template.docs-tree.max-pages=24` unless
the objective asks for fewer.

## Initial Plan

- A plan step proposes Markdown page paths and titles using the plan-line
  grammar.
- Write steps author each page.
- A verify step runs `readme_coverage` and `links_resolve` from
  [../completion.md](../completion.md).
- Revise steps repair missing child links or broken relative links.
- A respond step reports the tree and check results.

## README Rule

Every directory under the requested root receives a README table of contents.
The template writes READMEs as ordinary write steps and never relies on a hidden
scaffold.

## Failure This Prevents

The tree closes only when link and README checks pass, preventing document sets
that look complete in prose but are not navigable.

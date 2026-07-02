# Documentation Standards

## Purpose

Define the required shape and topology for authored Markdown.

## File Shape

- ASCII prose only.
- First line is one `# Title`.
- The first section is `## Purpose`.
- Filenames are kebab-case.
- Lines stay within `repository.markdown.line-width=100` characters.
- No `## Status` sections outside [../current-state.md](../current-state.md).

## Directory Shape

Every docs directory has one README table of contents and at least two children.
The README links every direct child with a one-line description. Every docs page
is reachable from [../README.md](../README.md) within
`repository.docs.max-link-depth=3` links.

## Content Rules

State the current contract directly. Do not preserve reference copies, history
narration, release shorthand, compatibility framing, TODO markers, or dead
links. Examples must be real paths, commands, fixtures, or protocol blocks.

## Ownership

One rule has one owning page. Other pages link instead of restating. When docs
and code disagree, update the contract and ledger before claiming success.

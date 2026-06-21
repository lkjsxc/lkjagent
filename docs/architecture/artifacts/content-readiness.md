# Content Readiness

## Purpose

Define how audits decide whether leaf files contain the requested content.

## Contract

Content-bearing files must contain domain-specific sections, concrete facts,
and enough non-boilerplate prose to satisfy the artifact kind. Scaffold
phrases, status-only files, empty headings, and generic "this file records"
language fail readiness.

Content readiness inspects actual file content. File count, line count,
manifest shape, README links, and scaffold topology can satisfy structural
audit only. They cannot satisfy readiness by themselves.

## Invariants

- Dictionary readiness requires entry fields requested by the owner objective.
- Cookbook readiness requires actual recipes or technique leaves.
- Unsupported verification claims fail readiness.
- Structural audit can pass while content readiness fails.

## Failure Cases

- A shallow bread term list is accepted as a detailed dictionary.
- A 100-file cookbook scaffold is accepted without recipe content.
- A verification note claims IPA, etymology, or examples that are absent.

## Dictionary Rules

A dictionary entry needs a term, part of speech or term class, non-trivial
definition, usage example when requested, and any requested pronunciation,
origin, variants, or cross-reference fields. A shallow term list fails a
detailed dictionary request even when it has many entries.

## Cookbook Rules

A recipe file needs title, purpose or description, ingredients, method,
timing or yield, and notes or troubleshooting. A technique file needs concept,
procedure, signals, common mistakes, and corrective action. A reference file
needs concrete lookup content.

A scaffold-only cookbook with README files, manifests, empty recipe leaves, or
status prose can pass structure but fails readiness. A 100-file cookbook tree
with weak leaves is not close eligible.

## Evidence

Audit failures name exact weak paths and missing requirements. Passing content
readiness may contribute artifact evidence only when topology and manifest
checks also pass and unsupported verification claims are absent.

## Related Files

- [dictionary-profile.md](dictionary-profile.md)
- [cookbook-profile.md](cookbook-profile.md)
- [completion-gates.md](completion-gates.md)

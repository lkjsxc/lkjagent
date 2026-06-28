# Content Readiness

## Purpose

Define how audits decide whether leaf files contain the requested content.

## Contract

Content readiness inspects actual file content. File count, line count,
manifest shape, README links, and scaffold topology can satisfy structural
audit only. They cannot satisfy readiness by themselves.

Content-bearing files must contain domain-specific sections, concrete facts,
and enough non-boilerplate prose to satisfy the artifact kind and path role.
Scaffold phrases, status-only files, empty headings, owner-term-only pages, and
generic "this file records" language fail readiness. The same normalized
scaffold-phrase detector gates content audit, `fs.write`, and `fs.batch_write`.

## Records

```text
ArtifactKind = cookbook | dictionary | story | reference | generic_content
SemanticArtifactId = owner_case + kind + normalized_topic + requested_scale
ReadinessRequirement = named field required by kind and path role
WeakPath = path + missing_requirements + weak_signals + semantic_mismatch
ContentSignal = concrete fact, recipe field, technique signal, or lookup row
UnsupportedClaim = claimed fact absent from the artifact body
SemanticMismatch = path or content domain conflicts with owner objective
ReadinessEvidence = artifact_id + audit_id + passed_requirements + weak_paths
```

Readiness evidence is valid only for the current `SemanticArtifactId`.

## Invariants

- Dictionary readiness requires entry fields requested by the owner objective.
- Cookbook readiness requires actual recipes, techniques, or reference leaves.
- Unsupported verification claims fail readiness.
- Structural audit can pass while content readiness fails.
- Planning evidence and graph notes cannot satisfy content readiness.

## Cookbook Rules

A recipe file needs recipe title, dish category, servings or yield, time,
ingredient list with quantities, method steps, timing or sensory signals,
notes or troubleshooting, and semantic relevance to the requested cuisine or
topic.

A technique file needs concept, procedure, signals, common mistakes,
corrective action, and applicability.

A reference file needs concrete lookup content, not status prose.

A Japanese cookbook cannot satisfy readiness with generic bread scaffold files
unless the owner explicitly asks for Japanese bread and the content is
semantically Japanese. Bread-like leaves in a Japanese cookbook fail readiness
with `semantic_mismatch` and their missing cookbook requirements.

## Story Rules

A story bible requires role-specific content for premise, timeline, cosmology,
technology rules, locations, society, factions, protagonist, antagonist,
supporting cast, relationships, logline, themes, conflict lattice, act
structure, chapter spine, continuity rules, and completion evidence. A single
page that only lists those labels fails. Each role needs concrete story facts,
constraints, causality, verification notes, or cross references appropriate to
that role.

## Dictionary Rules

A dictionary entry needs a term, part of speech or term class, non-trivial
definition, usage example when requested, and any requested pronunciation,
origin, variants, or cross-reference fields. A shallow term list fails a
detailed dictionary request even when it has many entries.

## Failure Cases

- A shallow bread term list is accepted as a detailed dictionary.
- A 100-file cookbook scaffold is accepted without recipe content.
- Bread categories are accepted for a Japanese cookbook without Japanese food
  semantics.
- A shallow story page is accepted because it contains every required label.
- A verification note claims IPA, etymology, or examples that are absent.

## Evidence

Audit failures name exact weak paths and missing requirement labels. Passing
content readiness may contribute artifact evidence only when topology and
manifest checks also pass and unsupported verification claims are absent.

## Implemented Slice

The shared scaffold-phrase detector rejects generic `coming soon`, `to be
written`, `this file records`, `this section describes`, placeholder-content,
stub-content, scaffold-only, future-work, and table-of-contents-without-body
prose before `fs.write` or `fs.batch_write` mutates files. `artifact.next`
examples avoid the observed generic `This section contains...` and `The body
names facts...` phrase families. Scaffold phrase refusals now name the path,
phrase, phrase class, why it is scaffold-like, and an acceptable replacement
pattern.

## Related Files

- [dictionary-profile.md](dictionary-profile.md)
- [cookbook-profile.md](cookbook-profile.md)
- [completion-gates.md](completion-gates.md)

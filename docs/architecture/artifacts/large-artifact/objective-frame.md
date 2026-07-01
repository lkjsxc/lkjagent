# Objective Frame

## Purpose

Define the deterministic owner request extraction used before artifact planning.
It is a pure conversion from owner text and tool parameters into plain data.

## Fields

| Field | Meaning |
| --- | --- |
| `raw_text` | Exact owner request or plan text. |
| `normalized_title` | Display title and root naming seed. |
| `artifact_kind` | `story`, `manuscript`, `report`, `documentation`, `study-set`, `dictionary`, `cookbook`, or `generic`. |
| `root` | Exact admitted workspace root. |
| `requested_paths` | Exact paths named by the owner. |
| `measurement_kind` | `words`, `characters`, `items`, `files`, `cards`, `lessons`, or `token-estimate`. |
| `requested_total` | Requested count when the owner gives one. |
| `accepted_floor` | Deterministic count floor required for completion. |
| `section_count` | Chapters, lessons, sections, entries, recipes, or parts requested. |
| `audience` | Reader, learner, maintainer, operator, or unspecified. |
| `language_hint` | Detected language hint, or unspecified. |
| `forbidden_roots` | Generic roots and roots contradicted by explicit owner paths. |
| `evidence_requirements` | Proof required before completion. |

## Extraction Rules

Extraction does not call the model. English and Japanese signals classify novels,
reports, documentation, study sets, dictionaries, cookbooks, and generic works.
Numeric phrases set totals and section counts. Explicit paths win over generic
roots. When the request names a concrete root, `structured-output`, `output`,
`artifact`, and `work-product` are forbidden roots.

## Completion Floor

A numeric request sets the accepted floor directly for count-based artifacts.
A qualitative request maps to the profile default floor. The floor is stored in
`artifact_plans` and projected from `artifact_readiness`; observation strings are
not the durable source of this fact.

## Status

implemented.

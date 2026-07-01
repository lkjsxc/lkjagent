# Profile Library

## Purpose

Define the reusable profile library that turns an objective frame into atom
templates, measurement rules, weak classes, assembly rules, and readiness rules.

## Required Profiles

| Profile | Required atoms |
| --- | --- |
| `story` | Premise, setting, cast, plot, continuity, and completion evidence. |
| `manuscript` | Story bible support, scene atoms, assembled chapters, and manuscript floor evidence. |
| `report` | Executive summary, evidence, analysis, recommendations, risks, and appendices. |
| `documentation` | Overview, usage, architecture, operations, verification, and examples. |
| `study-set` | Objectives, lessons, flashcards, drills, quizzes, and review plan. |
| `dictionary` | Entry atoms, index, cross references, and completion evidence. |
| `cookbook` | Recipe atoms, ingredients, procedure, variations, and index. |
| `generic` | Objective, structure, content parts, evidence, and completion evidence. |

## Template Fields

Each atom template names a role, relative path pattern, measurement kind, target
count, count floor, byte budget, required sections, weak classes, and assembly
relationship. Template expansion is deterministic from `ObjectiveFrame` totals
and section counts.

## Weak Classes

Profiles share these weak classes unless the profile adds stricter classes:
`missing-file`, `below-count-floor`, `missing-required-section`,
`scaffold-only`, `outline-only`, `placeholder`, `story-bible-only`,
`owner-terms-only`, and `generic-example`.

## Status

implemented.

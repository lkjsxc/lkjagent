# Content Atoms

## Purpose

Define bounded semantic content units shared by long artifact profiles.

## Contract

A content atom is the smallest runtime-owned unit that can be requested,
audited, retried, and counted for a large work product. It has a relative path,
profile role, minimum concrete prose, and weak-content classes that cannot
satisfy readiness.

The shared atom profiles are:

| Profile | Required atoms |
| --- | --- |
| story | `premise.md`, `setting.md`, `characters.md`, `plot.md`, `completion-evidence.md` |
| report | `executive-summary.md`, `evidence.md`, `analysis.md`, `recommendations.md`, `risks.md` |
| documentation | `overview.md`, `usage.md`, `architecture.md`, `operations.md`, `verification.md` |
| generic | `objective.md`, `structure.md`, `content.md`, `evidence.md`, `completion-evidence.md` |

Story manuscript chapter prose is a manuscript atom family described by
[manuscript-lifecycle.md](manuscript-lifecycle.md). It does not replace the
story-bible atoms above.

## Selection

`artifact.next` selects missing atoms after root identity and before generic
weak-path repair. Story roots keep the story-specific readiness and repair
rules. Report, documentation, and generic artifact roots use the shared atom
profile directly.

Generic roots such as `structured-output`, `output`, `artifact`, and
`work-product` are blocked when the owner objective names a more specific root
or target path. The runtime reports the selected generic root and the requested
target instead of repairing the wrong root.

## Audit Facts

Artifact audits emit these facts when an atom profile is active:

- `artifact_atom_profile` names the selected profile;
- `atom_status` is `ready` or `missing`;
- `atom_missing_count` is the number of required atom gaps;
- `next_atom` is the first missing atom or `none`;
- `required_atoms` is the exact required atom set.

Completion refuses while `atom_missing_count` is nonzero. Refusals include the
missing count and `next_atom` so recovery can request the same bounded unit.

## Status

implemented for report, documentation, generic, and story audit facts.

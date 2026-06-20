# Topic Map

## Purpose

Define how broad owner requests become a semantic graph instead of duplicate
folders with similar files.

## Shape

A Topic owns Subtopics. Subtopics own Claims, Decisions, Artifacts, or Tasks.
Artifacts own DocumentNodes and Sections. Evidence links to the claim,
section, task, or completion gate it proves.

Topic identity uses the normalized owner objective plus the chosen scope path.
Subtopic identity adds the semantic role, such as foundations, characters,
recipes, troubleshooting, chapters, or verification.

## Adoption

Before writing, the runtime searches existing records and files under the
candidate scope. Matching semantic roles are adopted. Empty or generic nodes
are repair targets. Equivalent roots are reused rather than duplicated.

## Status

design, implementation pending

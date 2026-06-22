# Objective Drift

## Purpose

Define the audit that prevents an artifact profile or content body from moving
away from the owner objective.

## Contract

Every large artifact has an objective-matched contract before content work:
artifact kind, subject, scale, required topic areas, forbidden drift terms, and
readiness gates. The contract is derived deterministically from the owner task
and scaffold inputs. Low-confidence profile selection fails closed.

For `Create a very big cookbook about japanese foods.` the contract is:

- artifact kind: cookbook.
- subject: Japanese food.
- scale: large.
- required topic areas: foundations, rice, noodles, soups, grilled dishes,
  simmered dishes, fried dishes, hot pots, pickles, sauces, regional cuisine,
  sweets, menus, techniques, and reference.
- forbidden drift: bread cookbook unless the owner explicitly requests bread.
- readiness gates: topology audit, content specificity audit,
  objective-match audit, and artifact-readiness audit.

## Drift Signals

The objective-match audit fails on:

- Path subject mismatch.
- Profile mismatch.
- Forbidden drift terms.
- Scaffold-only or generic body text.
- Unrelated profile vocabulary.
- README topology mismatch.
- Content that does not serve the owner objective.

Japanese cookbook roots must reject `baking.md`, `fermentation.md`,
`flour-water-salt-yeast.md`, `kneading.md`, `shaping.md`, `ciabatta.md`,
`focaccia.md`, `rye-bread.md`, `sourdough-country-loaf.md`, and the phrase
`This bread cookbook section`.

## Guard Behavior

When drift is detected:

- artifact-drift rises to guard strength.
- artifact-readiness drops sharply.
- artifact.next and artifact.apply are blocked for that root.
- The next safe action is objective-match audit and drift repair.
- Drifted paths are deleted or replaced because no stale compatibility promise
  exists.

## Implementation Hooks

- Scaffold profile selection lives under `crates/lkjagent-tools/src/doc/`.
- Next-batch examples live in `crates/lkjagent-tools/src/artifact_next_example.rs`.
- Drift checks live in the artifact audit path.
- Uploaded-run fixtures live in `crates/lkjagent-benchmark`.

## Completion Check

The Japanese cookbook regression passes only when bread paths are absent by
default, introduced bread drift is detected, and mutation tools are blocked
until audit or repair removes the drift.

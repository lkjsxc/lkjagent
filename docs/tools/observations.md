# Observations

## Purpose

Define the bounded observation returned after an explore action.

## Shape

Every tool returns the same envelope:

```text
<observation>
<status>ok</status>
<content>
...bounded text...
</content>
</observation>
```

The content cap is `tools.observation.max-tokens=1500`. Large output is
head-and-tail truncated with `context.truncation.marker=[...]`.

## Real Exchange Example

Action adapted from the checked-in failure fixture:

```text
<action>
<tool>fs.read</tool>
<path>data/logs/current-model-run.md</path>
<count>20</count>
</action>
```

Observation:

```text
<observation>
<status>ok</status>
<content>
Historical failure fixture for the iwanna manuscript run. The active case is in
recovery, the observed root is stories/novel-named, and final verification is
pending.
</content>
</observation>
```

Only the latest observation enters the next explore prompt; it replaces the
prior observation.

## Failure This Prevents

Tool output cannot grow into a transcript. The model sees the current goal and
latest bounded observation, not every previous refusal.

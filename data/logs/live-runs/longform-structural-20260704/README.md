# Longform Structural Trial 20260704

## Purpose

Preserve a bounded, redacted live-run fixture for planning improvements to
lkjagent longform manuscript generation.

## Scope

This is historical evidence only. It does not prove the current checkout passes
any gate. The binary SQLite store is intentionally not committed; row extracts
are stored as TSV.

## Trial Request

```text
Create a novel manuscript of about 10000 words in 10 chapters at stories/structural-trial-20260704. First write a structural settings document with premise, world rules, cast, timeline, motifs, chapter architecture, and continuity facts. Then write the manuscript chapters as finished prose. Use original English fiction. Avoid TODO, placeholder text, and notes to self. Ensure the manuscript files total at least 10000 words.
```

## Observed Result

- Store path: `data/live-longform-structural-20260704`.
- Generated workspace path: `workspace/stories/structural-trial-20260704`.
- The daemon stopped after endpoint errors during chapter 02.
- `settings.md` was generated with 536 words.
- `manuscript/chapter-01.md` was generated with 425 words.
- Total generated markdown words: 961.
- No check rows were recorded.

## Key Findings

- The settings file is plausible structural material.
- Chapter 01 repeats a settings-style structure instead of finished prose.
- Chapter prompts did not include the settings content as continuity context.
- Step 4 reached high endpoint attempts while the stale lock still rendered the
  daemon as `working`.
- The original data directory had a permission incident; ownership was repaired
  before this fixture was captured.

## Redactions

Endpoint host values were replaced with `<LKJAGENT_ENDPOINT>`. No API key was
present in the captured exchange files.

## Contents

- `cli/`: owner-visible command snapshots.
- `generated-workspace/`: generated markdown artifacts.
- `raw-redacted/exchange-logs/`: redacted request, response, outcome, and timing
  files from endpoint exchanges.
- `raw-redacted/daemon.log`: daemon stderr/stdout capture.
- `sqlite-extracts/`: TSV extracts of durable store rows.
- `word-counts.tsv`: generated markdown word counts.
- `manifest.json`: capture metadata.

## Suggested Planning Use

Use this fixture to plan a clean longform retry, prompt-frame improvements,
continuity admission, stale-lock diagnostics, and endpoint failure recovery.

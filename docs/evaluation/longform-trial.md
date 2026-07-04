# Longform Trial

## Purpose

Record the live longform manuscript trial evidence that informs future endpoint
and daemon tuning.

## Trial Objective

The owner asked lkjagent to create an original English longform manuscript named
Aurora Ledger under `stories/aurora-ledger`, starting with structured setting
notes and then ten chapter files totaling at least 10,000 words.

## Unstable Endpoint Run

The first run used `data/longform-test`. It created:

- `stories/aurora-ledger/settings.md`;
- `stories/aurora-ledger/manuscript/chapter-01.md`.

The run then stalled on chapter 02 because the endpoint needed more than the
client's default 60 second timeout for write-step completions. The attempt rows
recorded repeated endpoint errors for the configured OpenAI-compatible chat
completion path on `http://100.92.39.123:1234` with retry delays reaching 900
seconds. A manual probe using the logged chapter request completed in 106.1
seconds, which showed the request shape was viable but the timeout was too
short.

Measured output from the partial run was 899 words across the setting file and
chapter 01. The task stayed open with blocked and active write steps, so it did
not satisfy live proof criteria.

## Stable Endpoint Run

The successful run used `data/longform-stable` with endpoint timeout set to 240
seconds through config and environment. The daemon ran without human
intervention between `send` and terminal state.

Observed status progression:

- queued task: `queue: 1 new=false`;
- settings completed at poll 7;
- chapters 01 through 09 completed by poll 216;
- verify completed by poll 240;
- task became idle by poll 243.

The poll interval was 5 seconds, so the observed elapsed polling window was
about 20 minutes and 15 seconds. The task closed through engine checks.

## Stable Run Outputs

Output root:

`data/longform-stable/workspace/stories/aurora-ledger/`

Files:

- `settings.md`;
- `manuscript/chapter-01.md` through `manuscript/chapter-10.md`.

Engine check results:

- `file_count` over `stories/aurora-ledger/manuscript/*.md` measured `10` and
  passed the exact ten chapter requirement;
- `min_words_total` over the same manuscript glob measured `12587` and passed
  the 10,000 word requirement.

An external word scan measured 12,824 manuscript words, 451 setting-file words,
and 13,275 words across all generated Markdown files.

Token usage rows summed to 3,965 prompt tokens and 17,899 completion tokens
across 12 endpoint attempts. All stable-run attempt outcomes were `ok`.

## Improvement Evidence

Long write steps can exceed the default 60 second endpoint timeout even when the
endpoint is healthy. Future endpoint tuning should keep a longer write timeout
or make write-step timeout explicit in typed configuration.

The live proof succeeded only after timeout configuration changed. The earlier
partial run remains useful as a failure fixture for endpoint patience and
operator feedback, but it is not proof of task completion.

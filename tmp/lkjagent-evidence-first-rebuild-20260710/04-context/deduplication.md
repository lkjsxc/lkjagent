# Deduplication

## Layers

1. exact source identity and fingerprint;
2. normalized semantic key and effective time;
3. normalized body hash;
4. near-duplicate text fingerprint;
5. claim-equivalence across objective, brief, transcript, and evidence.

## Precedence

When equivalent candidates compete, prefer:

1. current explicit owner statement;
2. current measured workspace or check evidence;
3. current owner-authored file;
4. verified summary with source refs;
5. model-authored observation;
6. stale or external material.

Keep the loser as provenance, not prompt text.

## Repeated Failures

Do not inject every failed observation. Aggregate a failure lineage into one
recovery card containing count, latest bounded reason, tried strategies, and
next strategy. Store full raw evidence outside the prompt.

## Acceptance

With 10,000 candidates in shuffled insertion orders, the same high-utility set
must be selected and each semantic claim must appear once.

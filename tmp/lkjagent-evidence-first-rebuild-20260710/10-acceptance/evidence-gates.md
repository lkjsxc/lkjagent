# Evidence Gates

## Required Campaigns

At least three final-commit live endpoint campaigns and one PTY campaign meet
their time and scenario contracts. Each uses fresh or fingerprinted seed data.
Each running-executable fingerprint matches the runtime image rebuilt from the
frozen source during final acceptance.

## Active Work

Owner turns are scheduled throughout each live time box. Idle polls are not
counted as turns. Report active, waiting, maintenance, and quiescent seconds.

## Raw-State Truth

The gate reads SQLite and workspace directly. It rejects:

- blocked or unsatisfied required matters;
- summaries whose state differs from the database;
- missing requested files or companion units;
- success claims unsupported by checks;
- duplicate message or effect identities;
- evidence older than the final source commit.

## Comparison

Experiment evidence contains baseline, integrated candidates, at least three
repeats for noisy cells, and adoption decisions. Rejected and conditional
results remain visible.

## Secrets

Committed evidence is redacted and bounded. Secret removal does not remove
state, timing, fingerprints, metrics, or failure class required by the gate.

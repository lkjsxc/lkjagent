# Live Forensics

## Summary Mismatch

The standard 900-second runner called one runtime quantum every 200 milliseconds
for the full duration. After real work became terminal, run_until_idle returned
a synthetic closed task. The runner counted it as a turn and replaced the last
real state with closed.

## Profiles

- personal-workspace: 4,401 loop iterations, zero model calls, and one TODO that
  copied the profile objective.
- software-project: summary said closed, but SQLite showed a blocked task after
  filesystem prefix and action-format failures.
- structured-artifact: summary said closed, but SQLite showed blocked because
  the response path was missing.
- protocol-stress: real work ended after a small number of decisions; most of
  the remaining duration was idle polling.

## Additional Failure

The structured artifact file claimed JSON and CSV companion files were verified,
although those files did not exist. A file-exists check on the report itself was
insufficient evidence for its claims.

## Required Replacement

A live profile is an explicit scenario with scheduled owner goals, expected
effects, negative conditions, and independent checks. It stops measuring a
matter when that matter terminates. It never treats elapsed idle time or the
word ran as success.

# Evidence Layout

## Run Directory

    tmp/lkjagent-acceptance/<source-commit>/<campaign>/
      scenario.tsv
      owner-schedule.tsv
      result.tsv
      process-lifecycle.tsv
      provider-manifest.tsv
      runner.log
      events.tsv
      decisions.tsv
      tool-admissions.tsv
      checks.tsv
      metrics.tsv
      workspace-tree.tsv
      workspace-manifest.tsv
      workspace/
      run.sqlite3
      logs-redacted/
      verifier.md

PTY campaigns also contain tui-trace.tsv and a bounded terminal recording.
The evidence root contains experiment-matrix.tsv and adoption.tsv. Matrix rows
bind experiment, scenario, repeat, configuration fingerprint, and raw evidence
ref. Every row names its tested Git commit. Rejected candidates may bind an
ancestor that still contained the candidate; adopted integrated candidates are
rerun on frozen source. Adoption rows bind every experiment and configuration
to adopt, reject, or conditional.

Configuration evidence is canonical, sorted, compact flat JSON so whitespace or
key order cannot create false candidates. Matrix rows name factor families.
Metrics use metric/value TSV and include every hard floor, primary task success,
protected regression ratio, rendered tokens, endpoint calls, and recovery time.
The gate recomputes unstable-repeat and ten-percent or fifteen-percent adoption
rules rather than trusting the rationale.

The evidence root also contains independent-verifier.tsv, verifier-report.md,
verifier-commands.log, and verifier-artifacts.tsv. The receipt binds source and
raw-evidence material commits, immutable packet tree hash, and SHA-256 of the
other three.
The artifact manifest lists at least ten recomputed raw inputs. Mechanical gates
remain authoritative; the receipt proves what the separate review inspected,
not an unverifiable human identity claim.

Pre-freeze routing evidence lives separately at `tmp/lkjagent-progress/nodes`.
Each node has result.tsv, verifier.md, and a raw directory. The receipt fields
are node ID, status, source commit, completion sequence, anchored gate command,
exit code, dependency receipt hashes, evidence refs and hashes, and verifier ref
and hash. These receipts release dependencies but never substitute for final
raw-evidence acceptance.

events.tsv has monotonic_seconds, kind, event_id, matter_id, and ref. It includes
session.start, every owner.turn, state changes, and session.end.
owner-schedule.tsv has offset_seconds, intent, and text fingerprint. The final
gate verifies that scheduled goals and raw owner events span the time box.

process-lifecycle.tsv contains exactly runner start and end with one PID, run
ID, executable hash, and monotonic nanoseconds. provider-manifest.tsv mirrors
every SQLite exchange ID, request/response SHA-256, and monotonic interval.
runner.log is bounded and hash-bound by result.tsv. These records, rebuilt
binary comparison, endpoint exchanges, and real decisions jointly distinguish
execution from an elapsed-loop summary.

## Raw First

Result and verifier files are derived. The final gate reads raw SQLite,
workspace bytes, event trace, and Git commit. A Markdown statement never
overrides raw state.

Capture run.sqlite3 with SQLite Online Backup from a quiesced read boundary.
At that same boundary, workspace-manifest.tsv records normalized path,
document ID, revision ID, and SHA-256 fingerprint for every current document.
The result records `snapshot_method=sqlite-online-backup` and the manifest
fingerprint. The gate recomputes the file hashes and ledger correspondence.

## Commit Binding

Every result includes source commit, configuration fingerprint, scenario
fingerprint, model identifier, start and end times, seed fingerprint, and hash
of the running executable. Final acceptance rebuilds the runtime image and
requires live, adopted-experiment, and PTY binary hashes to match. Git
history derives a later raw-material commit. The verifier receipt names that
already-existing commit and is added in a still later verification commit, so no
file attempts to contain the hash of its own commit.

## Retention

Commit bounded, redacted, owner-safe evidence. Keep secret-bearing raw bodies
local and record their hashes and redaction procedure.

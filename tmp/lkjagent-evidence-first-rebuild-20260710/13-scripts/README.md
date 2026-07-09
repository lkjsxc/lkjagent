# Scripts

## Purpose

Provide mechanical gates that inspect files and raw evidence.

## Contents

- packet_lint.py: validate this packet.
- controller.py: canonical resume, next-task, and final-acceptance entrypoint.
- clean_checkout_gate.sh: verify tracked-tree behavior.
- focused_gate.py: run named nonempty contract suites and flat configuration gate.
- sqlite_snapshot.py: produce bounded table and state facts.
- live_evidence_gate.py: reject false live completion.
- live_db_checks.py: derive relational and workspace facts for live evidence.
- live_relational_checks.py: enforce operation, admission, effect, observation,
  context, and check cardinality.
- live_source_checks.py: bind scenario bundle, configuration, binary, model,
  wall time, and SQLite snapshot method.
- scenario_policy.py: immutable live floors and required semantic checks.
- scenario_semantic_gate.py: recompute diary, project, and long-artifact
  predicates from workspace bytes and SQLite.
- experiment_gate.py: bind repeated candidate runs and adoption decisions.
- experiment_metrics.py: recompute hard floors, unstable repeats, and adoption
  improvement thresholds.
- experiment_run_checks.py: validate candidate commit, canonical configuration,
  raw duration, SQLite, and metrics.
- verifier_receipt_gate.py: verify independent-report inputs and hashes.
- workspace_gate.py: compare visible workspace bytes, native ledger, managed
  headers, token counts, navigation debt, and snapshot manifest.
- workspace_file_contract.py: deterministic token and record-header checks.
- tui_trace_gate.py: verify scroll and transcript invariants.
- tui_cast_checks.py: parse raw PTY recording and bind duration and input.
- tui_db_checks.py: validate canonical conversation identity and content.
- repository_gate.py: derive anchor and source commit, then run final repository
  and public CI checks.
- repository_ci.py: verify the exact public repository workflow result.
- repository_history.py: derive immutable packet, source, material, and
  verification commit boundaries.
- workgraph_gate.py: validate the graph and release every dependency-ready node.
- acceptance_gate.py: final anchored gate.
- acceptance_runs.py: validate final live, PTY, workspace, semantic, and adopted
  experiment runs.

## Use

Run packet lint immediately after extraction. Commit the packet unchanged as the
anchor. Run the other gates from the downstream repository as their dependent
implementation becomes available.

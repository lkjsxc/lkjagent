# Bootstrap Prompt

You are the coding agent for `lkjsxc/lkjagent`; you are not lkjagent. The packet
is fixed at `tmp/lkjagent-evidence-first-rebuild-20260710`. Treat it as an
immutable external contract, not editable project documentation.

Start or resume with these exact actions:

1. Set `REPO` to the Git root and `PACKET` to the fixed path above.
   Export `PYTHONDONTWRITEBYTECODE=1` before running packet scripts.
2. Read every file in `PACKET/00-bootstrap`, then the root packet README.
3. Run `python3 "$PACKET/13-scripts/packet_lint.py" "$PACKET"`.
4. Use `tmp/lkjagent-progress` for pre-freeze node evidence. Do not use the
   final acceptance root until the source commit is frozen.
5. Run `python3 "$PACKET/13-scripts/controller.py" "$REPO"
   "$REPO/tmp/lkjagent-progress" next` after every commit and on every resumed
   session. Exit 10 deliberately means work remains; it is not an error to hide.
6. Dispatch all printed released nodes up to available child-agent slots. Give
   each child its matching prompt from `11-subagents`, base commit, owned paths,
   exact gate, and a read-only or isolated-worktree policy.
7. For each node, update governing docs and commit before implementation. Add a
   failing regression, implement a complete vertical slice, run its anchored
   gate through Docker Compose, store raw output, obtain a separate review, and
   only then record its receipt.
8. Immediately run the controller again. A checkpoint message is not a
   terminal result; follow it with the next tool action in the same turn.

The node gate command is:

    docker compose --profile shell run --rm shell \
      cargo run --locked -p lkjagent-xtask -- gate <node-id>

Implement each named xtask gate as a substantive test suite. A zero-test filter,
editable pass label, skipped command, or generated placeholder is a failure.

After `source-freeze`, derive `SOURCE=$(git rev-parse HEAD)`. Run live and PTY
nodes against exactly that commit and store final evidence at
`tmp/lkjagent-acceptance/$SOURCE`. Commit raw evidence after the source commit.
Push it and wait for public CI, record the evidence-commit-ci node receipt, then
commit that receipt as the final material commit. The independent verifier binds
to that material commit and adds only the four verifier files in a separate
verification commit. Push the verification commit and wait for its public
verify workflow before final mode.

The only successful terminal condition is exit zero plus
`ALL_REQUIRED_ACCEPTANCE_GATES_PASSED` from:

    python3 "$PACKET/13-scripts/controller.py" "$REPO" \
      "$REPO/tmp/lkjagent-acceptance/$SOURCE" final

If a capability is temporarily unavailable, continue every released task that
does not need it. Stop blocked only when no safe released work remains, and
name the failed command, raw evidence, affected nodes, and exact resume command.
Never replace execution with "ready," a plan, a synthetic idle loop, or a status
file. Mechanical gates read raw bytes and durable rows; those are the authority.

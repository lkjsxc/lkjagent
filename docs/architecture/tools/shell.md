# Shell Tool

## Purpose

Define shell.run, the escape hatch. Ordinary listing, searching, counting,
directory creation, bulk writes, cargo gates, xtask gates, and document audits
use typed tools first. shell.run runs only when active graph policy allows it
and typed tools cannot cover the operation.

## Parameters

`command` is required. `timeout` is optional, default `60`, max `600`.

## Execution

The command runs as `/bin/sh -lc` with the workspace as current directory,
inside the sandbox boundary in [../sandbox/safety.md](../sandbox/safety.md).
Timeouts kill the process and return captured output. Because the shell is
`/bin/sh`, commands must not depend on Bash-only brace expansion.

## Graph Policy

Planning and intake nodes block shell.run. Verification and recovery nodes may
allow it when typed tools cannot express the check. A shell refusal is an
observation, not a panic, and names the active graph node.

## Observation

The observation opens with an exit-code line, then bounded combined stdout and
stderr. Non-zero exits, signal exits, and timeouts return status error.

## Payload Escape Hatch

A payload containing a line that is exactly a closing parameter tag cannot
travel as a parameter value per
[../protocol/action-format.md](../protocol/action-format.md). Such payloads
may route through shell.run with a heredoc only when graph policy allows shell
and fs.batch_write cannot represent the payload.

## Maintenance

During explicit maintenance, shell.run is blocked by the maintenance gate.

## Status

implemented.

# Shell Tool

## Purpose

The contract for shell.run, the one general-purpose tool. Everything the
fixed toolset does not cover, listing, searching, building, testing,
fetching, runs through it, and skills are instructions for driving it.
Canonical parameter table: [registry.md](registry.md).

## Parameters

| Parameter | Rule |
| --- | --- |
| command | required |
| timeout | optional seconds, default 60, max 600 |

## Execution Environment

The command runs as /bin/sh -lc with the workspace directory as the
working directory, inside the container boundary specified by
[../sandbox/safety.md](../sandbox/safety.md). A command that exceeds its
timeout is killed, and the observation reports the timeout alongside the
output captured before the kill. Because the shell is /bin/sh, commands
must not depend on Bash-only expansion such as `{a,b}` brace expansion.

## Observation

The observation opens with an exit code line, then the combined stdout and
stderr, captured head-and-tail within the 2,048-token observation cap of
[../context/budgets.md](../context/budgets.md). Exit code 0 returns status
ok. Non-zero exits, signal exits, and timeouts return status error with the
same captured content. When the middle is elided, the truncation notice
names a narrower command as the retrieval path.

## Narrow Beats Raw

Hygiene pressure ([../context/hygiene.md](../context/hygiene.md)) makes
filtered commands the cheap path: grep with context beats a full dump,
head or tail beats an unbounded read, and a test filter beats the whole
suite. A raw dump spends the cap on bytes the truncation rule cuts anyway.

## Heredoc Escape Hatch

A payload containing a line that is exactly a closing parameter tag cannot
travel as a parameter value per
[../protocol/action-format.md](../protocol/action-format.md). Such
payloads route through shell.run with a heredoc and a custom delimiter:

```
<act>
<tool>shell.run</tool>
<command>
cat > notes/protocol-sample.md <<'LKJ_EOF'
<content>
a sample line showing the protocol shape
</content>
LKJ_EOF
</command>
</act>
```

## Maintenance Behavior

During an explicit maintenance cycle, shell.run has the same workspace,
network, and credential authority it has during task work. The boundary is
the container blast radius in [../sandbox/safety.md](../sandbox/safety.md),
not a maintenance-specific restriction.

## Status

implemented.

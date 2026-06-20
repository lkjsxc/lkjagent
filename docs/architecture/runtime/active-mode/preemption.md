# Preemption

## Purpose

Define how owner work interrupts background work.

## Rule

Owner work wins over maintenance. A pending owner queue row, active owner
case, or recoverable owner case prevents maintenance from starting and stops
maintenance before the next endpoint turn.

## Runtime Actions

When owner work appears during maintenance, the daemon closes or defers the
maintenance cycle without sending another maintenance prompt, delivers owner
work, and renders owner or recovery policy on the next model turn.

## Evidence

Tests must prove a queued owner row preempts active maintenance before any
maintenance action runs, and that recovery for owner work blocks maintenance.

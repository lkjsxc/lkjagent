# Admission Flow

## Purpose

Define the exact path from a parsed model action to accepted dispatch or a
recorded refusal under one persisted runtime decision.

## Contract

Admission is a pure check over one immutable decision-derived view and one
parsed action. It runs before dispatch. It records an admission row for both
accepted and refused actions. Refused actions never reach tool adapters.

## Required Inputs

- current decision id and case id;
- current prompt frame id for model-authored actions;
- authority and staleness fingerprints;
- admitted and blocked tool sets;
- forced next action class, route, and registry example;
- missing and existing evidence;
- retry budget, fault class, action fingerprint, and parameter-shape facts;
- completion, shell, owner-question, and maintenance allowances.

## Refusal Reasons

Admission refuses when:

- decision id is absent or not current;
- prompt frame id is absent or not current for a model action;
- staleness fingerprint changed;
- requested tool is not admitted;
- requested tool is blocked;
- completion is requested while evidence gaps remain;
- repeated action fingerprint is exhausted;
- schema is invalid or unsafe;
- action shape contradicts a forced recovery route;
- shell is requested outside a shell-admitted route;
- owner question is requested for internal inspection work.

## Accepted Path

An accepted action records the admission id and dispatch plan, then becomes a
`RuntimeEffectCommand`. Dispatch receives only that command and the immutable
view metadata needed for audit. Tool output becomes an observation event, not a
side-channel policy update.

## Refusal Output

A refusal records:

- decision id, prompt frame id, and requested tool;
- action and parameter-shape fingerprints;
- active mission, active mode, node, and phase;
- admitted and blocked tools;
- failed gate or stale fact;
- exact next executable action or deterministic effect command;
- registry-rendered example that parses and validates under the same view.

## Invariants

- Graph fallback text cannot admit a tool after authority refuses it.
- Maintenance cannot admit owner graph or artifact tools unless the decision
  explicitly includes them.
- Owner execution cannot display or admit a broader tool surface than the
  persisted decision names.
- Refusal examples never use invalid history, top-level JSON, `<think>`, or
  child `<file>` tags for `fs.batch_write`.

## Status

specified. Pure admission covers several stale, blocked, not-admitted,
completion, and repeat cases. Daemon-wide route coverage remains open.

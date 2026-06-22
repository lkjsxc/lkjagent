# Growth Stages

## Purpose

Growth stages own small-to-large documentation expansion. The controller
selects the smallest next action that increases objective coverage and relation
quality without increasing scaffold risk.

## Stages

1. Intake.
2. Objective contract.
3. Topic contract.
4. Semantic seed.
5. Seed audit.
6. Controlled expansion.
7. Relation pass.
8. Semantic audit.
9. Repair.
10. Integration audit.
11. Verification.
12. Complete.

## Scoring

Candidate expansion score is objective coverage gain plus relation gain,
implementation contract gain, prompt guidance gain, and verification gain,
minus duplication risk, generic scaffold risk, unsupported claim risk, line
limit risk, and context cost.

## Rule

A large directory archetype is never the first action. A new directory needs a
contract, a README, and at least two meaningful children before audit.

## Status

implemented

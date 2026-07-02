# Generic Template

## Purpose

Define the fallback for objectives that do not match a specific template.

## Plan

The generic template creates one explore step and one respond step. The explore
budget is `template.generic.explore-budget=20`. The explore goal restates the
objective and asks the model to gather enough bounded evidence to answer or
propose plan notes.

## Finish

The explore step may finish with a summary. The respond step turns the gathered
facts into an owner-facing message. If exploration discovers measurable file
work, `plan.note` can propose steps that the engine validates before adding
anything to the plan.

## Checks

Generic tasks attach checks from [../completion.md](../completion.md) only when
the objective contains explicit measurable criteria.

## Failure This Prevents

Unknown work receives bounded exploration instead of falling into unrestricted
tool choice or silently pretending to match a specialized template.

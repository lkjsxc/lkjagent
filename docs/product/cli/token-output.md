# Token Output

## Purpose

Define how CLI surfaces show endpoint token usage and context accounting.

## Fields

Every token scope has these fields:

| Field | Meaning |
| --- | --- |
| `input` | prompt or input tokens reported by the endpoint. |
| `output` | completion tokens reported by the endpoint. |
| `cached_input` | input tokens served from provider cache when reported. |
| `total` | endpoint total when reported or safely derivable. |
| `unknown` | count of rows in the scope with at least one omitted field. |
| `cache_ratio` | cached input divided by input when both are known and input is nonzero. |

Unknown endpoint fields render as `unknown`. They do not become zero and do not
participate in sums or ratios.

## Scopes

| Scope | Meaning |
| --- | --- |
| `latest` | newest provider exchange with usage data. |
| `task` | exchanges tied to the active or selected task case. |
| `session` | exchanges since the current daemon session started. |
| `all` | every usage row in the selected data directory. |

`status` and `watch` show the four scopes. `model-log` may show usage for one
exchange, one case, or one export, using the same field names.

## Formatting

- Counts use the compact formatter: `999`, `1.00K`, `1.23M`, `2.00B`.
- Ratios use two decimal places.
- Missing rows render `none` for the scope.
- Missing fields inside present rows render `unknown`.

## Example

```text
tokens.latest.input=1.20K
tokens.latest.output=512
tokens.latest.cached_input=900
tokens.latest.total=1.71K
tokens.latest.unknown=0
tokens.latest.cache_ratio=0.75
tokens.task.input=8.41K
tokens.task.output=2.10K
tokens.task.cached_input=unknown
tokens.task.total=10.51K
tokens.task.unknown=3
tokens.task.cache_ratio=unknown
```

## Acceptance

Focused CLI and store tests must prove multiple rows, missing fields, task
filters, session filters, and status-console agreement.

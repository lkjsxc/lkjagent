# Scaffold Findings

## Purpose

Record why the previous structured-output workspace is retained only as failed
diagnostic evidence.

## Findings

- Many leaves used the same generic role prose instead of objective-specific
  content.
- Files under country, platform, model, and story roots shared the same
  document shape even though those objectives need different readiness
  profiles.
- Several leaves named verification commands but did not include tool evidence
  from those commands.
- README and catalog topology existed, but topology alone cannot prove artifact
  readiness.
- The mixed root shape allowed objective drift across model, country, platform,
  and story topics.

## Next Repair Action

Keep this fixture as a negative sample. A future generated artifact should use a
single owner objective, concrete leaf content, an audit-owned readiness result,
and a bounded repair cursor for weak paths.

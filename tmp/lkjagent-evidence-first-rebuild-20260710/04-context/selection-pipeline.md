# Selection Pipeline

## Stages

1. Derive information needs from selected state and operation.
2. Discover candidates through routed indexes and exact paths.
3. Validate source fingerprints and contamination.
4. Normalize claims and semantic identity.
5. Resolve or block material contradictions.
6. Remove exact and near duplicates across every prompt region.
7. Score utility, trust, freshness, novelty, and dependency value.
8. Select under per-lane and total budgets.
9. Compress only when evidence-preserving compression is beneficial.
10. Render source-linked XML-like cards.
11. Validate budget, uniqueness, and decision binding.

## Budgeted Selection

Use deterministic cost-aware selection, not input order. A practical score is:

    utility = state relevance + obligation relevance + trust + freshness
              + novelty + dependency value - token cost - redundancy

Select mandatory safety and owner constraints first, then maximize remaining
utility under lane caps.

## Cross-Region Validation

The objective, matter brief, state cards, context lanes, recovery card, tool
observations, and user message share one semantic-fingerprint set. A fact
rendered in one region cannot be repeated in another unless the duplicate is an
explicit short reference.

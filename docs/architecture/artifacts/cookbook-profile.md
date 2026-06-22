# Cookbook Profile

## Purpose

Define the default semantic tree for cookbook artifacts and the specialized
bread shape used only when the owner asks for bread.

## Default Example

```text
cookbooks/<semantic-title>/
  README.md
  .lkj-doc-graph.md
  foundations/
    README.md
    japanese-pantry.md
    rice-fundamentals.md
    dashi-and-stocks.md
    knife-work.md
    seasoning-balance.md
  mains/
    README.md
    sushi-rice.md
    ramen-noodles.md
    miso-soup.md
    yakitori-grilling.md
    nikujaga-simmering.md
  sides/
    README.md
    tsukemono-pickles.md
    tempura-frying.md
    nabe-hot-pots.md
    okonomiyaki.md
  regional/
    README.md
    kansai-dishes.md
    hokkaido-dishes.md
    okinawan-dishes.md
  sweets/
    README.md
    mochi.md
    dorayaki.md
  reference/
    README.md
    menus.md
    sauces-and-seasonings.md
    equipment.md
    timelines.md
    troubleshooting.md
```

## Bread-Specific Shape

A bread cookbook keeps the bread profile with flour, kneading, fermentation,
shaping, baking, sourdough, ciabatta, focaccia, rye bread, and milk bread
paths. That shape is selected only when the owner objective names bread or a
specific bread family.

## Contract

A cookbook audit requires recipe or dish content, foundations or technique
content, troubleshooting or equipment coverage, README links, and a graph
manifest. Structural audit and content readiness are separate. A scaffold with
many files can be well shaped and still fail readiness when leaves lack
ingredients, procedure steps, timing, yield, signals, and corrective action.

## Invariants

- The generic cookbook shape must not drift into bread paths.
- A Japanese-food cookbook must include Japanese cuisine topic areas.
- Bread paths are allowed only for explicit bread objectives.
- Empty leaves, heading-only files, and generic status prose fail readiness.
- Completion cannot rely on file counts or manifests alone.

## Failure Cases

- A Japanese-food cookbook generates ciabatta or focaccia paths.
- A 100-file cookbook scaffold is marked ready while leaves are empty.
- A bread cookbook has generic project documentation instead of recipes.
- Audit claims recipe coverage without observing ingredients or methods.

## Verification

Cookbook readiness tests include scaffold-only output as bad, meaningful bread
recipe content as good for explicit bread tasks, and Japanese cookbook drift as
bad when bread paths or phrases appear under a Japanese-food objective.

## Status

partially implemented through cookbook scaffold selection, drift audit, and
content audit checks.

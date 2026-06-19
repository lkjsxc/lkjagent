pub type Readme = (&'static str, &'static str, &'static str, &'static str);

pub const README_DATA: &[Readme] = &[
    (
        "docs/README.md",
        "Encyclopedia Nucleus",
        "Small starting map for an encyclopedia that grows by reviewed batches.",
        "- [current-state.md](current-state.md): durable state ledger.\n- [maps/](maps/README.md): network maps.\n- [domains/](domains/README.md): topic branches.\n- [reference/](reference/README.md): shared definitions.\n- [curation/](curation/README.md): quality rules.\n- [execution/](execution/README.md): expansion queue.",
    ),
    (
        "docs/maps/README.md",
        "Maps",
        "Navigation maps for the current nucleus and planned expansion.",
        "- [concept-network.md](concept-network.md): current concept graph.\n- [growth-map.md](growth-map.md): next branch choices.",
    ),
    (
        "docs/domains/README.md",
        "Domains",
        "Topic branches that expand only after the nucleus is coherent.",
        "- [core/](core/README.md): starter domain.\n- [expansion-backlog.md](expansion-backlog.md): candidate branches.",
    ),
    (
        "docs/domains/core/README.md",
        "Core Domain",
        "Starter domain used to prove shape before broad coverage.",
        "- [foundations/](foundations/README.md): first topic cluster.\n- [overview.md](overview.md): domain summary.",
    ),
    (
        "docs/domains/core/foundations/README.md",
        "Foundations",
        "First cluster for testing links, tone, and depth.",
        "- [seed-topic.md](seed-topic.md): first article stub.\n- [open-questions.md](open-questions.md): unresolved points.",
    ),
    (
        "docs/reference/README.md",
        "Reference",
        "Shared lookup material kept smaller than topic branches.",
        "- [glossary.md](glossary.md): terms.\n- [ontology.md](ontology.md): concept classes.",
    ),
    (
        "docs/curation/README.md",
        "Curation",
        "Rules that keep growth incremental and balanced.",
        "- [workflow.md](workflow.md): page lifecycle.\n- [rebalance.md](rebalance.md): branch repair rules.",
    ),
    (
        "docs/execution/README.md",
        "Execution",
        "Durable work state for expansion and rebalance passes.",
        "- [active-session.md](active-session.md): current work.\n- [expansion-queue.md](expansion-queue.md): next batches.\n- [rebalance-plan.md](rebalance-plan.md): balance triggers.\n- [current-blockers.md](current-blockers.md): blockers.",
    ),
];

pub type Topic = (&'static str, &'static str, &'static str);

const ROOT: &[Topic] = &[(
    "current-state",
    "Current State",
    "Honest ledger for the encyclopedia nucleus.",
)];

const MAPS: &[Topic] = &[
    (
        "concept-network",
        "Concept Network",
        "Map the pages that exist now before adding more.",
    ),
    (
        "growth-map",
        "Growth Map",
        "Compare candidate branches before expanding.",
    ),
];

const DOMAINS: &[Topic] = &[(
    "expansion-backlog",
    "Expansion Backlog",
    "Candidate domains waiting for a small batch.",
)];

const CORE: &[Topic] = &[(
    "overview",
    "Core Overview",
    "Summarize the starter domain and its boundaries.",
)];

const FOUNDATIONS: &[Topic] = &[
    (
        "seed-topic",
        "Seed Topic",
        "First article stub used to validate article shape.",
    ),
    (
        "open-questions",
        "Open Questions",
        "Questions that must be settled before broad expansion.",
    ),
];

const REFERENCE: &[Topic] = &[
    ("glossary", "Glossary", "Terms used by the nucleus."),
    (
        "ontology",
        "Ontology",
        "Concept classes allowed in the current map.",
    ),
];

const CURATION: &[Topic] = &[
    (
        "workflow",
        "Workflow",
        "Move one small topic batch from draft to linked.",
    ),
    (
        "rebalance",
        "Rebalance",
        "Repair branches that grow wider than their neighbors.",
    ),
];

const EXECUTION: &[Topic] = &[
    (
        "active-session",
        "Active Session",
        "Current expansion pass and stop point.",
    ),
    (
        "expansion-queue",
        "Expansion Queue",
        "Next one to three pages to add.",
    ),
    (
        "rebalance-plan",
        "Rebalance Plan",
        "Triggers for splitting, merging, or moving branches.",
    ),
    (
        "current-blockers",
        "Current Blockers",
        "Blockers that prevent the next small batch.",
    ),
];

pub const GROUPS: &[(&str, &[Topic])] = &[
    ("", ROOT),
    ("maps", MAPS),
    ("domains", DOMAINS),
    ("domains/core", CORE),
    ("domains/core/foundations", FOUNDATIONS),
    ("reference", REFERENCE),
    ("curation", CURATION),
    ("execution", EXECUTION),
];

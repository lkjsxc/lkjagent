pub(crate) struct Requirement {
    pub label: &'static str,
    pub paths: &'static [&'static str],
    pub signals: &'static [&'static str],
}

pub(crate) const STORY_REQUIREMENTS: &[Requirement] = &[
    req(
        "premise",
        &["premise", "project/"],
        &["premise", "stakes", "inciting", "story"],
    ),
    req(
        "timeline",
        &["timeline", "continuity/"],
        &["timeline", "sequence", "past", "future"],
    ),
    req(
        "cosmology",
        &["cosmology", "setting/"],
        &["cosmology", "universe", "physics", "rule"],
    ),
    req(
        "technology-rules",
        &["technology"],
        &["technology", "rule", "limit", "cost"],
    ),
    req(
        "locations",
        &["locations", "setting/"],
        &["location", "place", "district", "route"],
    ),
    req(
        "society",
        &["society", "setting/"],
        &["society", "culture", "law", "class"],
    ),
    req(
        "factions",
        &["factions", "setting/"],
        &["faction", "agenda", "rival", "alliance"],
    ),
    req(
        "protagonist",
        &["protagonist", "characters/"],
        &["protagonist", "goal", "flaw", "choice"],
    ),
    req(
        "antagonist",
        &["antagonist", "characters/"],
        &["antagonist", "pressure", "motive", "threat"],
    ),
    req(
        "supporting-cast",
        &["supporting", "characters/"],
        &["supporting", "ally", "mentor", "rival"],
    ),
    req(
        "relationship-matrix",
        &["relationships"],
        &["relationship", "trust", "conflict", "bond"],
    ),
    req(
        "logline",
        &["logline", "premise"],
        &["logline", "must", "before", "stakes"],
    ),
    req(
        "themes",
        &["themes", "project/"],
        &["theme", "cost", "memory", "identity"],
    ),
    req(
        "conflict-lattice",
        &["conflict"],
        &["conflict", "escalation", "pressure", "choice"],
    ),
    req(
        "act-structure",
        &["act-structure", "plot/"],
        &["act", "turning point", "reversal", "climax"],
    ),
    req(
        "chapter-spine",
        &["chapter-spine", "plot/"],
        &["chapter", "scene", "reveal", "consequence"],
    ),
    req(
        "continuity-rules",
        &["rules", "continuity/"],
        &["continuity", "rule", "contradiction", "check"],
    ),
    req(
        "completion-evidence",
        &["readiness", "checks/"],
        &["completion", "evidence", "verified", "audit"],
    ),
];

const fn req(
    label: &'static str,
    paths: &'static [&'static str],
    signals: &'static [&'static str],
) -> Requirement {
    Requirement {
        label,
        paths,
        signals,
    }
}

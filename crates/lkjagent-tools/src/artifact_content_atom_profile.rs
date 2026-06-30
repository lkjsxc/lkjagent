pub(crate) struct AtomSpec {
    pub path: &'static str,
    pub label: &'static str,
}

pub(crate) fn profile(root: &str, kind: &str) -> Option<&'static str> {
    let lower = kind.to_ascii_lowercase();
    let clean_root = root.trim_start_matches("./");
    if clean_root.starts_with("stories/") || matches!(lower.as_str(), "story" | "novel") {
        return Some("story");
    }
    if lower.contains("cookbook") || lower.contains("dictionary") {
        return None;
    }
    if clean_root.starts_with("reports/") || lower.contains("report") {
        return Some("report");
    }
    if clean_root.starts_with("docs/") || clean_root.starts_with("guides/") {
        return Some("documentation");
    }
    if lower.contains("documentation") || lower.contains("guide") {
        return Some("documentation");
    }
    Some("generic")
}

pub(crate) fn atoms(profile: &str) -> &'static [AtomSpec] {
    match profile {
        "story" => &STORY_ATOMS,
        "report" => &REPORT_ATOMS,
        "documentation" => &DOC_ATOMS,
        _ => &GENERIC_ATOMS,
    }
}

static STORY_ATOMS: [AtomSpec; 5] = [
    AtomSpec {
        path: "premise.md",
        label: "premise",
    },
    AtomSpec {
        path: "setting.md",
        label: "setting",
    },
    AtomSpec {
        path: "characters.md",
        label: "characters",
    },
    AtomSpec {
        path: "plot.md",
        label: "plot",
    },
    AtomSpec {
        path: "completion-evidence.md",
        label: "completion-evidence",
    },
];

static REPORT_ATOMS: [AtomSpec; 5] = [
    AtomSpec {
        path: "executive-summary.md",
        label: "summary",
    },
    AtomSpec {
        path: "evidence.md",
        label: "evidence",
    },
    AtomSpec {
        path: "analysis.md",
        label: "analysis",
    },
    AtomSpec {
        path: "recommendations.md",
        label: "recommendations",
    },
    AtomSpec {
        path: "risks.md",
        label: "risks",
    },
];

static DOC_ATOMS: [AtomSpec; 5] = [
    AtomSpec {
        path: "overview.md",
        label: "overview",
    },
    AtomSpec {
        path: "usage.md",
        label: "usage",
    },
    AtomSpec {
        path: "architecture.md",
        label: "architecture",
    },
    AtomSpec {
        path: "operations.md",
        label: "operations",
    },
    AtomSpec {
        path: "verification.md",
        label: "verification",
    },
];

static GENERIC_ATOMS: [AtomSpec; 5] = [
    AtomSpec {
        path: "objective.md",
        label: "objective",
    },
    AtomSpec {
        path: "structure.md",
        label: "structure",
    },
    AtomSpec {
        path: "content.md",
        label: "content",
    },
    AtomSpec {
        path: "evidence.md",
        label: "evidence",
    },
    AtomSpec {
        path: "completion-evidence.md",
        label: "completion-evidence",
    },
];

use std::collections::BTreeSet;

use crate::model::RepoFile;

const ACTIVE_PAGES: &[&str] = &[
    "docs/README.md",
    "docs/agent/README.md",
    "docs/agent/handoff.md",
    "docs/agent/working-here.md",
    "docs/context/README.md",
    "docs/context/assembly.md",
    "docs/context/sources.md",
    "docs/current-state.md",
    "docs/evaluation/README.md",
    "docs/evaluation/experiments.md",
    "docs/evaluation/live-proof.md",
    "docs/evaluation/scenarios.md",
    "docs/operations/README.md",
    "docs/operations/running.md",
    "docs/operations/verification.md",
    "docs/product/README.md",
    "docs/product/cli.md",
    "docs/product/configuration.md",
    "docs/product/daemon.md",
    "docs/product/tui.md",
    "docs/protocol/README.md",
    "docs/protocol/envelopes.md",
    "docs/protocol/faults.md",
    "docs/repository/README.md",
    "docs/repository/documentation-standards.md",
    "docs/repository/functional-style.md",
    "docs/runtime/README.md",
    "docs/runtime/completion.md",
    "docs/runtime/loop.md",
    "docs/runtime/recovery.md",
    "docs/state/README.md",
    "docs/state/cells.md",
    "docs/state/transitions.md",
    "docs/store/README.md",
    "docs/store/schema.md",
    "docs/store/transactions.md",
    "docs/tools/README.md",
    "docs/tools/admission.md",
    "docs/tools/registry.md",
    "docs/vision/README.md",
    "docs/vision/north-star.md",
    "docs/vision/principles.md",
    "docs/vision/scope.md",
    "docs/workspace/README.md",
    "docs/workspace/effects.md",
    "docs/workspace/layout.md",
    "docs/workspace/records.md",
];
const _: [(); 47] = [(); ACTIVE_PAGES.len()];

const RETIRED_NAMES: &[&str] = &[
    "TaskSnapshot",
    "StepKind",
    "plan-family",
    "matter bridge",
    "bridge projection",
    "bridge cells",
    "fixed template",
];

#[rustfmt::skip]
const REQUIREMENTS: &[(&str, &[&str])] = &[
    ("AGENTS.md", &["Durable state rows and persisted `RuntimeDecision` rows are the single control", "Completion is reducer-computed through fresh checks"]),
    ("docs/vision/principles.md", &["Durable authority: state cells and persisted decisions own runtime behavior"]),
    ("docs/context/assembly.md", &["selector first persists immutable operation", "atomically attaches rendered cards and fingerprints", "It cannot change the operation"]),
    ("docs/tools/registry.md", &["## Initial Catalog", "`list_directory`", "`search_text`", "`read_file`", "`edit_file`", "`create_file`"]),
    ("docs/protocol/envelopes.md", &["does not echo decision IDs, context fingerprints, tool fingerprints", "or JSON arguments", "or JSON argument object"]),
    ("docs/product/daemon.md", &["Runtime data and visible workspace are separate capabilities", "host data at `/data`", "host workspace at `/workspace`"]),
    ("docs/workspace/effects.md", &["exact prior/intended bytes", "expected/intended mode", "target and stage `(bytes, mode)` pairs"]),
    ("docs/product/tui.md", &["The TUI reads `conversation_messages`", "ordered by monotonic sequence and logical ID"]),
    ("docs/runtime/completion.md", &["schedules native checks automatically", "harness-rendered factual receipt", "receipt alone is persisted"]),
    ("docs/current-state.md", &["`5604ec89af3ba9dbfb287bd869971781fdcf2fad`", "`28bdaacca4a6d7c779057893e3d48bfbd9f2ccea`", "A synthetic 901-second run", "one blocked task, three blocked steps, and zero runtime", "unsupported executor before any model call", "| docs-reset | complete |", "| acceptance-checker | complete |"]),
    ("docs/evaluation/README.md", &["workgraph.tsv", "acceptance.tsv", "experiment-plan.tsv"]),
    ("evaluation/workgraph.tsv", &["id\twave\tdepends", "acceptance-checker\t0\tdocs-reset"]),
    ("evaluation/acceptance.tsv", &["id\tcategory\tpredicate", "A01\tchecker", "D02\tdocs"]),
    ("evaluation/experiment-plan.tsv", &["cell\tstage\tenvelope", "K\tintegrated"]),
    ("docs/evaluation/live-proof.md", &["acceptance verify", "--source SOURCE --evidence evaluation/evidence/SOURCE", "nonzero incomplete mode", "nine negative fixtures", "every required row as missing"]),
];

pub(crate) fn check(files: &[RepoFile], failures: &mut Vec<String>) {
    check_page_map(files, failures);
    check_retired_names(files, failures);
    for (path, tokens) in REQUIREMENTS {
        require_all(files, path, tokens, failures);
    }
}

fn check_page_map(files: &[RepoFile], failures: &mut Vec<String>) {
    let expected = ACTIVE_PAGES.iter().copied().collect::<BTreeSet<_>>();
    let actual = files
        .iter()
        .filter(|file| file.path.starts_with("docs/") && file.path.ends_with(".md"))
        .map(|file| file.path.as_str())
        .collect::<BTreeSet<_>>();
    for path in expected.difference(&actual) {
        failures.push(format!("compact authority page is missing: {path}"));
    }
    for path in actual.difference(&expected) {
        failures.push(format!(
            "page is outside the compact 47-page authority map: {path}"
        ));
    }
}

fn check_retired_names(files: &[RepoFile], failures: &mut Vec<String>) {
    for file in files.iter().filter(|file| authority_page(&file.path)) {
        let lower = file.text.to_ascii_lowercase();
        for name in RETIRED_NAMES {
            if lower.contains(&name.to_ascii_lowercase()) {
                failures.push(format!(
                    "retired authority name remains in {}: {name}",
                    file.path
                ));
            }
        }
    }
}

fn authority_page(path: &str) -> bool {
    path.starts_with("docs/") && !matches!(path, "docs/current-state.md" | "docs/store/schema.md")
}

fn require_all(files: &[RepoFile], path: &str, tokens: &[&str], failures: &mut Vec<String>) {
    let Some(text) = find(files, path) else {
        failures.push(format!("required contract input is missing: {path}"));
        return;
    };
    let normalized = normalize(text);
    for token in tokens {
        if !normalized.contains(&normalize(token)) {
            failures.push(format!("{path} is missing focused contract text: {token}"));
        }
    }
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn find<'a>(files: &'a [RepoFile], path: &str) -> Option<&'a str> {
    files
        .iter()
        .find(|file| file.path == path)
        .map(|file| file.text.as_str())
}

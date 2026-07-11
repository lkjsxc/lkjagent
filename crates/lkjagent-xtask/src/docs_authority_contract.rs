use crate::model::RepoFile;

const REQUIRED_PAGES: &[&str] = &[
    "docs/current-state.md",
    "docs/product/authority-boundaries.md",
    "docs/runtime/authority-model.md",
    "docs/runtime/matters-and-obligations.md",
    "docs/runtime/operation-graph.md",
    "docs/runtime/waiting-and-quiescence.md",
    "docs/state/reducer-and-selectors.md",
    "docs/store/schema.md",
    "docs/store/schema-constraints.md",
    "docs/store/effect-journal.md",
    "docs/workspace/transaction-protocol.md",
    "docs/tui/README.md",
    "docs/tui/transcript-model.md",
    "docs/tui/scrolling.md",
];

const RETIRED_PAGES: &[&str] = &[
    "docs/decisions/personal-as-templates.md",
    "docs/engine/README.md",
    "docs/engine/completion.md",
    "docs/engine/matter-bridge.md",
    "docs/engine/plan-and-steps.md",
    "docs/engine/retry-and-escalation.md",
    "docs/engine/step-kinds.md",
    "docs/engine/templates/README.md",
    "docs/engine/templates/docs-tree.md",
    "docs/engine/templates/file-work.md",
    "docs/engine/templates/generic.md",
    "docs/engine/templates/journal.md",
    "docs/engine/templates/question.md",
    "docs/engine/turn-cycle.md",
    "docs/evaluation/prose-trial.md",
    "docs/protocol/plan-line-grammar.md",
];

const SCHEMA_TABLES: &str = concat!(
    "matters obligations operations operation_edges runs runtime_events state_cells ",
    "state_cell_history state_edges runtime_decisions context_frames prompt_cards ",
    "provider_exchanges failure_lineages tool_admissions effect_journal observations ",
    "checks conversation_messages owner_turns commands diagnostics outbox_messages ",
    "config daemon_leases workspace_documents workspace_revisions content_blobs ",
    "workspace_aliases workspace_tombstones workspace_relations workspace_search_rows ",
    "workspace_index_debt",
);

pub(crate) fn check(files: &[RepoFile], failures: &mut Vec<String>) {
    check_pages(files, failures);
    check_current_state(files, failures);
    check_authority(files, failures);
    check_schema(files, failures);
}

fn check_pages(files: &[RepoFile], failures: &mut Vec<String>) {
    for path in REQUIRED_PAGES {
        if find(files, path).is_none() {
            failures.push(format!("required authority page is missing: {path}"));
        }
    }
    for path in RETIRED_PAGES {
        if find(files, path).is_some() {
            failures.push(format!("retired page remains: {path}"));
        }
    }
}

fn check_current_state(files: &[RepoFile], failures: &mut Vec<String>) {
    let Some(text) = find(files, "docs/current-state.md") else {
        return;
    };
    require_all(
        text,
        "current-state",
        &[
            "three tasks, thirteen steps, ten decisions",
            "zero tool",
            "admissions, observations, artifacts, and workspace records",
            "raw/12-sqlite-facts.tsv",
            "raw/31-clean-checkout.log",
            "raw/17-diary-run-once.log",
            "raw/14-live-summary-facts.tsv",
            "raw/19-relative-root-historical.log",
            "reproduced",
            "bounded",
            "Production still hydrates",
            "Context may be prepared before the final decision",
            "Recovery tuples and progress windows persist",
            "final live and PTY evidence",
        ],
        failures,
    );
    let lower = text.to_ascii_lowercase();
    for claim in [
        "all four live profiles ran and closed successfully",
        "proven in current checkout",
        "interactive behavior is proven",
    ] {
        if lower.contains(claim) {
            failures.push(format!("false live claim remains: {claim}"));
        }
    }
}

fn check_authority(files: &[RepoFile], failures: &mut Vec<String>) {
    let Some(text) = find(files, "docs/runtime/authority-model.md") else {
        return;
    };
    require_all(
        text,
        "authority-model",
        &[
            "RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState",
            "RuntimeState + Policy + CurrentTime -> RuntimeDecision",
            "RuntimeDecision -> EffectResult",
            "EffectResult -> RuntimeEvent",
            "commits before prompt compilation or effect execution",
            "TaskSnapshot",
            "plan-family",
        ],
        failures,
    );
}

fn check_schema(files: &[RepoFile], failures: &mut Vec<String>) {
    let Some(text) = find(files, "docs/store/schema.md") else {
        return;
    };
    for table in SCHEMA_TABLES.split_ascii_whitespace() {
        if !text.contains(table) {
            failures.push(format!("store schema is missing table {table}"));
        }
    }
    require_all(
        text,
        "store-schema",
        &[
            "predicate kind and payload",
            "unique typed acyclic edges",
            "selected cells and",
            "required observations",
            "unique non-null admission",
            "unique logical ID and monotonic sequence",
            "provider tokenizer and count",
            "`selected_monotonic_ms`",
            "`tool_count`",
            "`prompt_tokens`",
            "`prompt_token_cap`",
            "`semantic_duplicate_count`",
            "`harness_json_count`",
            "`unresolved_material_conflict_count`",
            "`useful` and `progressed` booleans",
            "effect reference, status",
            "A fresh store creates no task, step, template, plan-family, bridge",
        ],
        failures,
    );
    let Some(constraints) = find(files, "docs/store/schema-constraints.md") else {
        return;
    };
    require_all(
        constraints,
        "schema-constraints",
        &[
            "rejects any second run",
            "check constraint",
            "unique selection sequence",
            "partial unique index permits only one current terminal outcome",
            "`active`",
            "`invalid`",
            "`archived`",
            "`tombstoned`",
            "only non-tombstoned rows require current",
            "`managed` controls header and token admission",
            "Exactly one active index-debt row exists",
        ],
        failures,
    );
}

fn require_all(text: &str, owner: &str, tokens: &[&str], failures: &mut Vec<String>) {
    for token in tokens {
        if !text.contains(token) {
            failures.push(format!("{owner} is missing contract text: {token}"));
        }
    }
}

fn find<'a>(files: &'a [RepoFile], path: &str) -> Option<&'a str> {
    files
        .iter()
        .find(|file| file.path == path)
        .map(|file| file.text.as_str())
}

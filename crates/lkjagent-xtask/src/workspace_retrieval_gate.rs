use std::path::Path;
use std::process::Command;

use crate::node_suites::{check as check_suites, Suite};

const SUITES: &[Suite] = &[
    Suite {
        package: "lkjagent-app",
        target: "explore",
        minimum_tests: 3,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_evidence",
        minimum_tests: 3,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_rebalance",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "record_wrappers",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "cli_rows",
        minimum_tests: 4,
    },
    Suite {
        package: "lkjagent-app",
        target: "archive_compensation",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "archive_recovery",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "archive_link_recovery",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "archive_partial_settlement",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "prepared_operation_startup",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "archive_settled_integrity",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-store",
        target: "workspace_rows",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-store",
        target: "record_rows",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_search",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_index_predicates",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_rebalance_compensation",
        minimum_tests: 1,
    },
];

const REQUIRED: &[(&str, &str)] = &[
    (
        "workspace_search",
        "workspace_search_discovers_body_filters_and_rejects_drift",
    ),
    (
        "workspace_search",
        "search_uses_trigram_project_date_and_stable_rebuild",
    ),
    (
        "archive_compensation",
        "archive_restores_file_and_row_when_audit_fails",
    ),
    (
        "archive_compensation",
        "archive_restores_indexes_when_state_suppression_fails",
    ),
    (
        "archive_recovery",
        "archive_resumes_prepared_operation_after_file_move",
    ),
    (
        "archive_recovery",
        "archive_startup_preserves_conflicting_target",
    ),
    (
        "archive_link_recovery",
        "archive_startup_preserves_linked_duplicate",
    ),
    (
        "archive_partial_settlement",
        "archive_recovery_restores_preimage_alias_and_cells",
    ),
    (
        "prepared_operation_startup",
        "startup_skips_non_archive_prepared_operation",
    ),
    (
        "archive_settled_integrity",
        "archive_rejects_drifted_settled_target",
    ),
    (
        "archive_settled_integrity",
        "archive_rejects_reoccupied_settled_prior_path",
    ),
    (
        "workspace_index_predicates",
        "open_todos_excludes_closed_rows_and_rebuild_bytes_are_stable",
    ),
    (
        "workspace_rebalance_compensation",
        "rebalance_restores_files_rows_search_and_indexes_when_rebuild_fails",
    ),
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let mut failures = check_suites(root, "workspace-retrieval-maintenance", SUITES)
        .err()
        .unwrap_or_default();
    failures.extend(
        REQUIRED
            .iter()
            .filter_map(|(target, test)| run(root, target, test)),
    );
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn run(root: &Path, target: &str, test: &str) -> Option<String> {
    let output = match Command::new("cargo")
        .args([
            "test",
            "--locked",
            "-p",
            "lkjagent-app",
            "--test",
            target,
            "--",
            test,
        ])
        .current_dir(root)
        .output()
    {
        Ok(output) => output,
        Err(error) => return Some(format!("required workspace test could not start: {error}")),
    };
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() && text.contains("test result: ok. 1 passed;") {
        None
    } else {
        Some(format!(
            "required workspace test {target}:{test} did not pass"
        ))
    }
}

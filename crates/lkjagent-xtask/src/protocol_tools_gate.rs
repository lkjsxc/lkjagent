use crate::node_suites::{check as check_suites, Suite};
use std::path::Path;
use std::process::Command;

#[rustfmt::skip]
const SUITES: &[Suite] = &[
    Suite { package: "lkjagent-core", target: "parse_contract", minimum_tests: 6 },
    Suite { package: "lkjagent-core", target: "direct_action_grammar", minimum_tests: 4 },
    Suite { package: "lkjagent-core", target: "parse_diagnosis", minimum_tests: 2 },
    Suite { package: "lkjagent-core", target: "default_tool_view", minimum_tests: 1 },
    Suite { package: "lkjagent-core", target: "tool_call", minimum_tests: 7 },
    Suite {
        package: "lkjagent-core",
        target: "tool_call_edges",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-core",
        target: "admission",
        minimum_tests: 5,
    },
    Suite {
        package: "lkjagent-core",
        target: "generic_flow",
        minimum_tests: 9,
    },
    Suite {
        package: "lkjagent-core",
        target: "persisted_tool_view",
        minimum_tests: 1,
    },
    Suite { package: "lkjagent-app", target: "admission_rejection", minimum_tests: 1 },
    Suite { package: "lkjagent-app", target: "admission_settlement", minimum_tests: 1 },
    Suite {
        package: "lkjagent-app",
        target: "app",
        minimum_tests: 6,
    },
    Suite {
        package: "lkjagent-app",
        target: "contamination",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "native_append_effect",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "recovery_ladder",
        minimum_tests: 3,
    },
    Suite {
        package: "lkjagent-app",
        target: "tool_views",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "effect_journal",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "effect_recovery",
        minimum_tests: 1,
    },
    Suite { package: "lkjagent-app", target: "shell_check_journal", minimum_tests: 3 },
    Suite { package: "lkjagent-app", target: "shell_check_preflight", minimum_tests: 2 },
    Suite { package: "lkjagent-app", target: "artifact_bundle_recovery", minimum_tests: 2 },
    Suite { package: "lkjagent-app", target: "artifact_settlement", minimum_tests: 3 },
    Suite { package: "lkjagent-app", target: "artifact_identity", minimum_tests: 3 },
    Suite { package: "lkjagent-app", target: "effect_dispatch", minimum_tests: 2 },
    Suite { package: "lkjagent-effects", target: "effects", minimum_tests: 5 },
];

#[rustfmt::skip]
const REQUIRED: &[(&str, &str, &str)] = &[
    ("lkjagent-app", "admission_settlement", "rejected_admission_rolls_back_with_late_failure"),
    ("lkjagent-app", "artifact_bundle_recovery", "complete_bundle_recovery_settles_artifacts_and_refs"),
    ("lkjagent-app", "artifact_bundle_recovery", "partial_and_conflicting_bundles_fail_without_artifacts"),
    ("lkjagent-app", "artifact_settlement", "mismatched_observation_refs_roll_back_artifact_settlement"),
    ("lkjagent-app", "artifact_settlement", "orphan_artifact_intent_rolls_back_settlement"),
    ("lkjagent-app", "artifact_settlement", "late_decision_failure_rolls_back_turn_settlement"),
    ("lkjagent-app", "artifact_identity", "equal_content_at_different_paths_has_distinct_artifact_identity"),
    ("lkjagent-app", "artifact_identity", "overlapping_bundle_targets_are_rejected_before_admission"),
    ("lkjagent-app", "artifact_identity", "later_planning_failure_rolls_back_turn_admissions"),
    ("lkjagent-app", "effect_dispatch", "partial_bundle_failure_restores_prior_targets"),
    ("lkjagent-app", "native_append_effect", "multipart_append_reassembles_owned_parts_before_appending"),
    ("lkjagent-core", "generic_flow", "duplicate_command_checks_use_ordinal_outcomes"),
    ("lkjagent-core", "tool_call_edges", "count_values_are_canonical_and_within_persisted_bounds"),
    ("lkjagent-app", "recovery", "pending_decision_with_stale_field_bounds_fails_closed"),
    ("lkjagent-effects", "effects", "shell_bounds_only_its_background_and_detached_descendants"),
    (
        "lkjagent-core",
        "generic_flow",
        "generic_explore_requires_a_persisted_decision",
    ),
    (
        "lkjagent-core",
        "parse_contract",
        "generic_explore_rejects_every_action_shape",
    ),
    (
        "lkjagent-app",
        "admission_bridge",
        "native_workspace_effect_has_harness_admission_and_prepared_journal",
    ),
    (
        "lkjagent-app",
        "admission_bridge",
        "model_workspace_write_prepares_target_fingerprints",
    ),
    (
        "lkjagent-app",
        "native_append_effect",
        "payload_workspace_append_effect_appends_file_and_artifact",
    ),
    (
        "lkjagent-app",
        "effect_dispatch",
        "prior_failure_reports_completed_effects",
    ),
    (
        "lkjagent-app",
        "effect_journal",
        "accepted_explore_effect_has_prepared_journal_and_linked_observation",
    ),
    (
        "lkjagent-app",
        "effect_journal",
        "settlement_binds_one_immutable_observation",
    ),
    (
        "lkjagent-app",
        "effect_journal",
        "startup_settles_unresolved_effects_once_without_replay",
    ),
    (
        "lkjagent-app",
        "effect_recovery",
        "startup_recovers_applying_write_when_target_matches_intended_bytes",
    ),
    ("lkjagent-app", "shell_check_journal", "shell_checks_have_prepared_journals_and_bounded_observations"),
    ("lkjagent-app", "shell_check_journal", "invalid_shell_check_records_failed_fact_and_journal"),
    ("lkjagent-app", "shell_check_journal", "late_turn_failure_rolls_back_shell_observation_and_check"),
    ("lkjagent-app", "shell_check_preflight", "file_preflight_failure_prevents_earlier_shell_execution"),
    ("lkjagent-app", "shell_check_preflight", "direct_multi_check_settlement_rolls_back_earlier_rows"),
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let mut failures = check_suites(root, "protocol-tools", SUITES)
        .err()
        .unwrap_or_default();
    failures.extend(
        REQUIRED
            .iter()
            .filter_map(|(package, target, test)| run(root, package, target, test)),
    );
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn run(root: &Path, package: &str, target: &str, test: &str) -> Option<String> {
    let mut args = vec!["test", "--locked", "-p", package];
    if target == "lib" {
        args.push("--lib");
    } else {
        args.extend(["--test", target]);
    }
    args.extend(["--", test]);
    let output = match Command::new("cargo").args(args).current_dir(root).output() {
        Ok(output) => output,
        Err(error) => return Some(format!("required protocol test could not start: {error}")),
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
            "required protocol test {package}:{target}:{test} did not pass"
        ))
    }
}

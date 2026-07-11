use std::path::Path;
use std::process::Command;

use crate::node_suites::{check as check_suites, Suite};

const SUITES: &[Suite] = &[
    Suite {
        package: "lkjagent-core",
        target: "parse_contract",
        minimum_tests: 6,
    },
    Suite {
        package: "lkjagent-core",
        target: "direct_action_grammar",
        minimum_tests: 4,
    },
    Suite {
        package: "lkjagent-core",
        target: "parse_diagnosis",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-core",
        target: "default_tool_view",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-core",
        target: "tool_call",
        minimum_tests: 7,
    },
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
    Suite {
        package: "lkjagent-app",
        target: "admission_rejection",
        minimum_tests: 1,
    },
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
        minimum_tests: 1,
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
];

const REQUIRED: &[(&str, &str, &str)] = &[
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
        "lib",
        "effect_dispatch::tests::prior_failure_reports_completed_effects",
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

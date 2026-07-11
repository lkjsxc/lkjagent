use std::path::Path;
use std::process::Command;

use crate::node_suites::{check as check_suites, Suite};

#[rustfmt::skip]
const SUITES: &[Suite] = &[
    Suite { package: "lkjagent-core", target: "runtime_recovery", minimum_tests: 3 },
    Suite { package: "lkjagent-core", target: "runtime_eligibility", minimum_tests: 4 },
    Suite { package: "lkjagent-app", target: "recovery_ladder", minimum_tests: 3 },
    Suite { package: "lkjagent-app", target: "runtime_wait", minimum_tests: 1 },
    Suite { package: "lkjagent-app", target: "endpoint_wait", minimum_tests: 1 },
    Suite { package: "lkjagent-app", target: "recovery", minimum_tests: 2 },
    Suite { package: "lkjagent-app", target: "effect_recovery", minimum_tests: 1 },
    Suite { package: "lkjagent-app", target: "prepared_operation_startup", minimum_tests: 1 },
    Suite { package: "lkjagent-app", target: "matter_continuation", minimum_tests: 1 },
    Suite { package: "lkjagent-app", target: "resume", minimum_tests: 2 },
    Suite { package: "lkjagent-llm", target: "backoff", minimum_tests: 1 },
];

#[rustfmt::skip]
const REQUIRED: &[(&str, &str, &str)] = &[
    ("lkjagent-core", "runtime_recovery", "failure_tuple_binds_every_no_repeat_dimension"),
    ("lkjagent-core", "runtime_recovery", "signatures_are_normalized_and_diagnostics_are_bounded"),
    ("lkjagent-core", "runtime_eligibility", "future_cell_waits_without_consuming_it_and_due_cell_runs"),
    ("lkjagent-core", "runtime_eligibility", "entirely_edge_blocked_state_selects_visible_blocker"),
    ("lkjagent-core", "runtime_eligibility", "malformed_cooldown_fails_closed_as_visible_blocker"),
    ("lkjagent-app", "recovery_ladder", "repeated_parse_failure_advances_without_premature_block"),
    ("lkjagent-app", "endpoint_wait", "endpoint_retries_wait_until_due_then_stop_at_configured_limit"),
    ("lkjagent-app", "runtime_wait", "future_recovery_wait_makes_no_decision_or_model_call_until_due"),
    ("lkjagent-app", "recovery", "unfinished_decision_with_exchange_blocks_without_provider_replay"),
    ("lkjagent-app", "effect_recovery", "startup_recovers_applying_write_when_target_matches_intended_bytes"),
    ("lkjagent-llm", "backoff", "exponential_backoff_caps_at_fifteen_minutes"),
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let mut failures = check_suites(root, "recovery-continuity", SUITES)
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
    let output = match Command::new("cargo")
        .args([
            "test", "--locked", "-p", package, "--test", target, "--", test,
        ])
        .current_dir(root)
        .output()
    {
        Ok(output) => output,
        Err(error) => return Some(format!("required recovery test could not start: {error}")),
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
            "required recovery test {package}:{target}:{test} did not pass"
        ))
    }
}

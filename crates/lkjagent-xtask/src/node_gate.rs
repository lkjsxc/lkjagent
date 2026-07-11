use std::path::Path;

use crate::facts::{expect, pairs, read};

const BASE: &str = "ae5ff551457adce869dee6159200c85a63aab3de";
const RAW_PATH: &str = "tmp/lkjagent-progress/nodes/baseline-capture/raw";

pub fn check(root: &Path, identifier: &str) -> Result<(), Vec<String>> {
    match identifier {
        "baseline-capture" => check_baseline(&root.join(RAW_PATH)),
        "docs-authority" => crate::docs_authority_gate::check(root),
        "repository-determinism" => crate::repository_determinism_gate::check(root),
        "evaluation-harness" => crate::evaluation_harness::check(root),
        "protocol-tools" => crate::protocol_tools_gate::check(root),
        "recovery-continuity" => crate::recovery_continuity_gate::check(root),
        "workspace-retrieval-maintenance" => crate::workspace_retrieval_gate::check(root),
        _ => Err(vec![format!("unknown node gate: {identifier}")]),
    }
}

fn check_baseline(raw: &Path) -> Result<(), Vec<String>> {
    if !raw.is_dir() {
        return Err(vec![format!(
            "raw evidence directory is missing: {}",
            raw.display()
        )]);
    }
    let mut failures = Vec::new();
    check_exit(raw, "01-gate-red.tsv", "2", &mut failures);
    check_exit(raw, "20-docker-lint.tsv", "0", &mut failures);
    check_exit(raw, "21-docker-test.tsv", "0", &mut failures);
    check_exit(raw, "22-docker-verify.tsv", "0", &mut failures);
    check_exit(raw, "31-clean-checkout.tsv", "1", &mut failures);
    check_database(raw, &mut failures);
    check_requests(raw, &mut failures);
    check_inventory(raw, &mut failures);
    check_bounded_facts(raw, &mut failures);
    check_diary(raw, &mut failures);
    check_failures(raw, &mut failures);
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn check_database(raw: &Path, failures: &mut Vec<String>) {
    let pairs = pairs(raw.join("12-sqlite-facts.tsv"), failures);
    for (key, value) in [
        ("count.tool_admissions", "0"),
        ("count.observations", "0"),
        ("count.artifacts", "0"),
        ("count.workspace_records", "0"),
        ("case2.state", "blocked"),
        ("case2.active_write_attempts", "2"),
        ("case2.output_cap_failures", "2"),
        ("case2.same_output_cap_diagnosis", "true"),
    ] {
        expect(&pairs, key, value, failures);
    }
    let sequence = pairs.get("case2.decision_sequence").map(String::as_str);
    if sequence.is_none_or(|value| value.matches("model.call/2004:768").count() != 2) {
        failures.push("case 2 does not preserve two identical capped decisions".into());
    }
}

fn check_requests(raw: &Path, failures: &mut Vec<String>) {
    let pairs = pairs(raw.join("13-request-facts.tsv"), failures);
    for attempt in ["attempt1", "attempt2"] {
        expect(&pairs, &format!("{attempt}.max_tokens"), "768", failures);
        expect(
            &pairs,
            &format!("{attempt}.requires_1500_words"),
            "true",
            failures,
        );
        expect(
            &pairs,
            &format!("{attempt}.output_path_same"),
            "true",
            failures,
        );
    }
    expect(&pairs, "identical_max_tokens", "true", failures);
    expect(&pairs, "identical_stop", "true", failures);
}

fn check_inventory(raw: &Path, failures: &mut Vec<String>) {
    let text = read(raw.join("10-repository-inventory.log"), failures);
    for required in [
        format!("product_base_commit\t{BASE}"),
        "packet_anchor\t1b615a76c03dfd58dfd2986f017563bd6789e832".to_string(),
        "tracked_inputs_begin\ndata/README.md\ntracked_inputs_end".to_string(),
    ] {
        if !text.contains(&required) {
            failures.push(format!("repository inventory is missing: {required}"));
        }
    }
    let clean = read(raw.join("31-clean-checkout.log"), failures);
    if !clean.contains("pathspec 'Cargo.lock' did not match") {
        failures.push("clean checkout did not expose the missing lockfile".into());
    }
}

fn check_bounded_facts(raw: &Path, failures: &mut Vec<String>) {
    let source = read(raw.join("11-source-facts.tsv"), failures);
    for fact in [
        "diary-canned",
        "relative-root-strip",
        "synthetic-idle-snapshot",
        "live-idle-loop",
        "readiness-message-close",
    ] {
        if !source.lines().any(|line| line.starts_with(fact)) {
            failures.push(format!("source fact is missing: {fact}"));
        }
    }
    let live = read(raw.join("14-live-summary-facts.tsv"), failures);
    let rows = live
        .lines()
        .skip(1)
        .filter(|line| line.contains("\tran\tclosed\t900\t"))
        .count();
    if rows != 4 {
        failures.push(format!(
            "expected four bounded live summaries, found {rows}"
        ));
    }
    let relative = read(raw.join("19-relative-root-historical.log"), failures);
    if !relative.contains("fs.tree") || !relative.contains("io: prefix not found") {
        failures.push("relative-root observation is missing its tool error".into());
    }
}

fn check_diary(raw: &Path, failures: &mut Vec<String>) {
    check_exit(raw, "17-diary-run-once.tsv", "0", failures);
    let manifest = read(raw.join("18-diary-after-run-manifest.tsv"), failures);
    if manifest
        .lines()
        .filter(|line| line.ends_with("\ttrue"))
        .count()
        != 1
    {
        failures.push("diary fixture must contain exactly one canned record".into());
    }
}

fn check_failures(raw: &Path, failures: &mut Vec<String>) {
    let text = read(raw.join("40-critical-failures.tsv"), failures);
    let rows = text.lines().skip(1).collect::<Vec<_>>();
    if rows.len() != 6 {
        failures.push(format!(
            "expected six critical failure rows, found {}",
            rows.len()
        ));
    }
    let reproduced = rows
        .iter()
        .filter(|row| row.contains("\treproduced\t"))
        .count();
    let bounded = rows
        .iter()
        .filter(|row| row.contains("\tbounded\t"))
        .count();
    if reproduced != 3 || bounded != 3 {
        failures
            .push("critical failures must distinguish three reproduced and three bounded".into());
    }
    for row in rows {
        let Some(references) = row.split('\t').nth(2) else {
            failures.push("critical failure row has no evidence refs".into());
            continue;
        };
        for reference in references.split(',') {
            let Some(name) = reference.strip_prefix("raw/") else {
                failures.push(format!("critical failure ref escapes raw: {reference}"));
                continue;
            };
            read(raw.join(name), failures);
        }
    }
}

fn check_exit(raw: &Path, name: &str, expected: &str, failures: &mut Vec<String>) {
    let values = pairs(raw.join(name), failures);
    expect(&values, "exit_code", expected, failures);
}

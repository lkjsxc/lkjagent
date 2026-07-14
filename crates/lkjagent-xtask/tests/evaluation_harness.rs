use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

use lkjagent_xtask::evaluation_harness::{
    endpoint_file, fixture_errors, sha256, validate_cast, FakeClock, FaultInjector,
};
use lkjagent_xtask::{node_gate, run};

#[test]
fn repository_satisfies_evaluation_runner_contract() {
    let root = repository_root();
    assert_eq!(node_gate::check(&root, "evaluation-harness"), Ok(()));
}

#[test]
fn tracked_blocked_baseline_evidence_validates_honestly() {
    let root = repository_root();
    let source = "97e00698f348fc2435d47a107b5b8453c98b9d1f";
    let args = words(&[
        "evidence",
        "check",
        "--campaign",
        "baseline",
        "--source",
        source,
    ]);
    let reachable = Command::new("git")
        .args(["cat-file", "-e", &format!("{source}^{{commit}}")])
        .current_dir(&root)
        .status()
        .is_ok_and(|status| status.success());
    assert_eq!(run(&args, &root), i32::from(!reachable));
}

#[test]
fn absent_baseline_evidence_fails_honestly() {
    let root = repository_root();
    let args = words(&[
        "evidence",
        "check",
        "--campaign",
        "baseline",
        "--source",
        "0000000000000000000000000000000000000000",
    ]);
    assert_eq!(run(&args, &root), 1);
}

#[test]
fn acceptance_negative_rejects_unsupported_and_unconfined_commands() {
    let root = repository_root();
    assert_eq!(run(&words(&["benchmark", "live"]), &root), 1);
    assert_eq!(run(&words(&["experiment", "run"]), &root), 2);
    assert_eq!(run(&words(&["proof"]), &root), 1);
    assert_eq!(run(&words(&["campaign", "run", "/bin/sh"]), &root), 2);
}

#[test]
fn campaign_parser_rejects_commands_and_owner_text() {
    let root = repository_root();
    assert_eq!(run(&words(&["campaign", "run", "/bin/sh"]), &root), 2);
    assert_eq!(
        run(
            &words(&[
                "campaign",
                "run",
                "daily-life-recall",
                "--owner-text",
                "fake"
            ]),
            &root
        ),
        2
    );
}

#[test]
fn sha256_matches_a_published_empty_vector() {
    assert_eq!(
        sha256(b""),
        "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn fake_clock_rejects_a_monotonic_regression() {
    let mut clock = FakeClock::default();
    assert_eq!(clock.advance_to(20), Ok(()));
    assert_eq!(
        clock.advance_to(19),
        Err("fake clock monotonic regression".into())
    );
}

#[test]
fn fault_injector_rejects_reordering_and_incomplete_replay() -> Result<(), String> {
    let root = repository_root();
    let mut injector = FaultInjector::from_path(&root.join("evaluation/fault-schedule.tsv"))?;
    let second = injector
        .faults()
        .get(1)
        .cloned()
        .ok_or("second fault is missing")?;
    let error = injector
        .consume(
            &second.injection_id,
            &second.boundary,
            &mut FakeClock::default(),
        )
        .err()
        .ok_or("out-of-order fault was accepted")?;
    assert!(error.contains("fault order mismatch"));
    assert!(injector.finish().is_err());
    Ok(())
}

#[test]
fn every_false_positive_fixture_has_one_mechanical_failure() -> Result<(), String> {
    let directory = repository_root().join("evaluation/false-positive-fixtures");
    for (name, expected) in [
        ("idle-as-complete.tsv", "useful decision floor not met"),
        (
            "blocked-as-complete.tsv",
            "claimed terminal contradicts raw terminal",
        ),
        ("skipped-command.tsv", "required command was skipped"),
        ("zero-test-filter.tsv", "test count must be positive"),
        (
            "generated-placeholder.tsv",
            "generated placeholder is not evidence",
        ),
    ] {
        assert_eq!(fixture_errors(&directory.join(name))?, [expected]);
    }
    Ok(())
}

#[test]
fn endpoint_file_is_shell_free_bounded_and_secret_opaque() -> Result<(), String> {
    let path = temporary("endpoint.env");
    fs::write(&path, "LKJAGENT_ENDPOINT_URL=http://127.0.0.1:9\nLKJAGENT_MODEL=local\nLKJAGENT_API_KEY=private-test-value\n")
        .map_err(|error| error.to_string())?;
    let values = endpoint_file(&path)?;
    assert_eq!(values.len(), 3);
    fs::write(&path, "LKJAGENT_MODEL=$(printf bad)\n").map_err(|error| error.to_string())?;
    let error = endpoint_file(&path)
        .err()
        .ok_or("unsafe endpoint value was accepted")?;
    assert!(error.contains("unsafe value syntax"));
    fs::remove_file(path).map_err(|error| error.to_string())
}

#[test]
fn endpoint_file_rejects_unknown_duplicate_and_symlink() -> Result<(), String> {
    let path = temporary("bad.env");
    fs::write(&path, "OTHER=value\n").map_err(|error| error.to_string())?;
    assert!(endpoint_file(&path).is_err());
    fs::write(&path, "LKJAGENT_MODEL=a\nLKJAGENT_MODEL=b\n").map_err(|error| error.to_string())?;
    assert!(endpoint_file(&path).is_err());
    let link = temporary("link.env");
    symlink(&path, &link).map_err(|error| error.to_string())?;
    assert!(endpoint_file(&link).is_err());
    fs::remove_file(link).map_err(|error| error.to_string())?;
    fs::remove_file(path).map_err(|error| error.to_string())
}

#[test]
fn generic_pty_fixture_is_rejected() -> Result<(), String> {
    let path = temporary("empty.cast");
    fs::write(&path, "{\"version\":2}\n").map_err(|error| error.to_string())?;
    let error = validate_cast(&path)
        .err()
        .ok_or("generic PTY fixture was accepted")?;
    assert!(error.contains("incomplete"));
    fs::remove_file(path).map_err(|error| error.to_string())
}

#[test]
fn ctrl_f_is_measured_as_search_input() -> Result<(), String> {
    let path = temporary("search.cast");
    let cast = "{\"version\":2}\n[0.0,\"m\",\"slow-start\"]\n[0.1,\"i\",\"\\u0006日本語\"]\n[1.2,\"m\",\"slow-end\"]\n";
    fs::write(&path, cast).map_err(|error| error.to_string())?;
    assert_eq!(validate_cast(&path)?.search_inputs, 1);
    fs::remove_file(path).map_err(|error| error.to_string())
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn temporary(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "lkjagent-evaluation-test-{}-{name}",
        std::process::id()
    ))
}
fn words(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

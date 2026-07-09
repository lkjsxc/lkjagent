use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_xtask::evaluation_harness::{
    fixture_errors, sha256, validate, validate_cast, Facts, FakeClock, FaultInjector,
};
use lkjagent_xtask::node_gate;

#[test]
fn repository_satisfies_evaluation_harness_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert_eq!(node_gate::check(&root, "evaluation-harness"), Ok(()));
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
        .ok_or_else(|| "second fault is missing".to_string())?;
    let error = injector
        .consume(
            &second.injection_id,
            &second.boundary,
            &mut FakeClock::default(),
        )
        .expect_err("out-of-order fault must fail");
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
fn computed_success_is_not_an_editable_pass_label() {
    let mut facts = Facts::computed(
        "sha256:1111111111111111111111111111111111111111111111111111111111111111",
        7,
    );
    assert_eq!(validate(&facts), Vec::<String>::new());
    facts.set("skipped", "true");
    assert_eq!(validate(&facts), ["required command was skipped"]);
}

#[test]
fn trace_only_pty_fixture_is_rejected() -> Result<(), String> {
    let path = temporary("empty.cast");
    fs::write(&path, "{\"version\":2,\"width\":80,\"height\":24}\n")
        .map_err(|error| error.to_string())?;
    let error = match validate_cast(&path) {
        Err(error) => error,
        Ok(_) => return Err("empty cast was accepted".into()),
    };
    fs::remove_file(path).map_err(|error| error.to_string())?;
    assert!(error.contains("raw owner and Japanese input"));
    Ok(())
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

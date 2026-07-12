use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_xtask::evaluation_harness::{validate_corpus, validate_scenario};

#[test]
fn parser_covers_four_tracked_aliases() {
    assert_eq!(validate_corpus(&repository_root()), Ok(4));
}

#[test]
fn parser_rejects_changed_owner_and_seed_hashes() -> Result<(), String> {
    let root = temporary_root();
    let source = repository_root().join("evaluation/scenarios/exact-file-edit");
    let scenario = root.join("evaluation/scenarios/exact-file-edit");
    fs::create_dir_all(scenario.join("seed/notes")).map_err(|error| error.to_string())?;
    for name in [
        "scenario.tsv",
        "matters.tsv",
        "owner-schedule.tsv",
        "seed-manifest.tsv",
        "checks.tsv",
    ] {
        fs::copy(source.join(name), scenario.join(name)).map_err(|error| error.to_string())?;
    }
    fs::copy(
        source.join("seed/notes/exact-base.txt"),
        scenario.join("seed/notes/exact-base.txt"),
    )
    .map_err(|error| error.to_string())?;

    let schedule = fs::read_to_string(scenario.join("owner-schedule.tsv"))
        .map_err(|error| error.to_string())?;
    fs::write(
        scenario.join("owner-schedule.tsv"),
        schedule.replacen("sha256:f28c1", "sha256:028c1", 1),
    )
    .map_err(|error| error.to_string())?;
    assert!(validate_scenario(&root, "exact-file-edit").is_err());

    fs::copy(
        source.join("owner-schedule.tsv"),
        scenario.join("owner-schedule.tsv"),
    )
    .map_err(|error| error.to_string())?;
    fs::write(scenario.join("seed/notes/exact-base.txt"), "changed\n")
        .map_err(|error| error.to_string())?;
    assert!(validate_scenario(&root, "exact-file-edit").is_err());
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temporary_root() -> PathBuf {
    std::env::temp_dir().join(format!(
        "lkjagent-scenario-parser-test-{}",
        std::process::id()
    ))
}

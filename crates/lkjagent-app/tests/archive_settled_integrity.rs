use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use lkjagent_app::cli;
use lkjagent_core::workspace_record::archive_path;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

#[test]
fn archive_rejects_drifted_settled_target() -> TestResult<()> {
    let data = fixture_root()?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Settled",
        "Drift",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])?;
    let target = data.join("workspace").join(archive_path("custom", &id)?);
    fs::write(&target, "owner replacement")?;

    let error = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])
        .err()
        .ok_or_else(|| std::io::Error::other("settled drift unexpectedly succeeded"))?;
    assert!(error.contains("settled archive target changed"));
    assert_eq!(fs::read_to_string(target)?, "owner replacement");
    Ok(())
}

#[test]
fn archive_rejects_reoccupied_settled_prior_path() -> TestResult<()> {
    let data = fixture_root()?;
    let data_arg = data.to_string_lossy();
    let added = cli::run([
        "--data",
        data_arg.as_ref(),
        "record",
        "add",
        "custom",
        "Settled",
        "Prior",
        "--body",
        "body",
    ])?;
    let id = field(&added, "record: ")?;
    let prior = field(&added, "path=")?;
    cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])?;
    let owner_path = data.join("workspace").join(prior);
    fs::create_dir_all(
        owner_path
            .parent()
            .ok_or_else(|| std::io::Error::other("missing parent"))?,
    )?;
    fs::write(&owner_path, "owner replacement")?;

    let error = cli::run(["--data", data_arg.as_ref(), "record", "archive", &id])
        .err()
        .ok_or_else(|| std::io::Error::other("settled prior unexpectedly succeeded"))?;
    assert!(error.contains("settled archive prior path reoccupied"));
    assert_eq!(fs::read_to_string(owner_path)?, "owner replacement");
    Ok(())
}

fn field(output: &str, marker: &str) -> Result<String, String> {
    output
        .split(marker)
        .nth(1)
        .and_then(|value| value.split_whitespace().next())
        .map(str::to_string)
        .ok_or_else(|| format!("missing {marker} in {output}"))
}

fn fixture_root() -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "lkjagent-archive-settled-integrity-{}-{}",
        std::process::id(),
        NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed),
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}

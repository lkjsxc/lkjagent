use std::fs;
use std::path::PathBuf;

use lkjagent_xtask::structure::audit;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn structure_audit_accepts_current_crates_and_rejects_removed_crates() -> TestResult<()> {
    let root = fixture_root("structure")?;
    let crates = root.join("crates");
    for name in [
        "lkjagent-core",
        "lkjagent-store",
        "lkjagent-llm",
        "lkjagent-effects",
        "lkjagent-app",
        "lkjagent-xtask",
    ] {
        fs::create_dir_all(crates.join(name))?;
    }
    assert!(audit(&root).is_ok());
    fs::create_dir_all(crates.join("lkjagent-runtime"))?;
    assert!(audit(&root).is_err_and(|error| error.contains("removed crate")));
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-xtask-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_xtask::facts::collect_files;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn collect_files_without_git_skips_build_and_state_dirs() -> TestResult<()> {
    let root = temp_root("facts")?;
    fs::create_dir_all(root.join("docs"))?;
    fs::create_dir_all(root.join("target/debug"))?;
    fs::create_dir_all(root.join(".lkjagent-workspace"))?;
    fs::write(root.join("docs/ok.md"), "# Ok\n\n## Purpose\n\nok\n")?;
    fs::write(root.join("target/debug/build.log"), "generated\n")?;
    fs::write(root.join(".lkjagent-workspace/file.md"), "owner\n")?;
    fs::write(root.join("Cargo.lock"), "generated lock\n")?;
    fs::write(root.join("state.sqlite3"), "db\n")?;

    let files = collect_files(&root)?;
    let paths = files.into_iter().map(|file| file.path).collect::<Vec<_>>();
    assert_eq!(paths, vec!["docs/ok.md"]);
    fs::remove_dir_all(root)?;
    Ok(())
}

fn temp_root(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-xtask-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

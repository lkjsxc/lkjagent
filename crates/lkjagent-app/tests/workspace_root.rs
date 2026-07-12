use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use lkjagent_app::{cli, config, workspace_root};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;
static NEXT: AtomicUsize = AtomicUsize::new(0);

#[test]
fn resolves_default_relative_absolute_and_rejects_internal_roots() -> TestResult<()> {
    assert_eq!(
        workspace_root::resolve(Path::new("/data"), "../workspace")?,
        PathBuf::from("/workspace")
    );
    assert_eq!(
        workspace_root::resolve(Path::new("/srv/state"), "../../owner")?,
        PathBuf::from("/owner")
    );
    assert_eq!(
        workspace_root::resolve(Path::new("/data"), "/owner/files")?,
        PathBuf::from("/owner/files")
    );
    for root in ["", " \n", "logs", "tmp/cache", "lkjagent.sqlite3"] {
        assert!(workspace_root::resolve(Path::new("/data"), root).is_err());
    }
    Ok(())
}

#[test]
fn help_invalid_and_native_status_leave_workspace_absent() -> TestResult<()> {
    let (parent, data, workspace) = fixture("lazy")?;
    let help = cli::run(["help"])?;
    assert!(help.contains("lkjagent commands"));
    assert!(!workspace.exists());
    assert!(cli::run(["--data", text(&data), "unknown"]).is_err());
    assert!(!workspace.exists());
    let status = cli::run(["--data", text(&data), "status"])?;
    assert!(status.contains(&format!("workspace={}", workspace.display())));
    assert!(!workspace.exists());
    fs::remove_dir_all(parent)?;
    Ok(())
}

#[test]
fn environment_override_wins_in_an_isolated_process() -> TestResult<()> {
    let (parent, data, workspace) = fixture("env")?;
    let override_root = parent.join("override");
    let output = Command::new(env!("CARGO_BIN_EXE_lkjagent"))
        .args(["--data", text(&data), "doctor", "--json"])
        .env("LKJAGENT_WORKSPACE_ROOT", &override_root)
        .output()?;
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(json["workspace_root"], text(&override_root));
    assert!(!workspace.exists());
    assert!(!override_root.exists());
    fs::remove_dir_all(parent)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn rejects_a_symlink_workspace_root() -> TestResult<()> {
    use std::os::unix::fs::symlink;
    let (parent, _data, workspace) = fixture("symlink")?;
    let outside = parent.join("outside");
    fs::create_dir(&outside)?;
    symlink(&outside, &workspace)?;
    let result = workspace_root::open(&workspace);
    assert!(result.is_err(), "symlink root was accepted");
    fs::remove_dir_all(parent)?;
    Ok(())
}

#[test]
fn config_validation_rejects_control_characters_without_creating_paths() {
    assert!(config::validate_document("{\"workspace_root\":\"bad\\nroot\"}").is_err());
}

fn fixture(name: &str) -> TestResult<(PathBuf, PathBuf, PathBuf)> {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let parent = std::env::temp_dir().join(format!(
        "lkjagent-workspace-root-{name}-{}-{id}",
        std::process::id()
    ));
    if parent.exists() {
        fs::remove_dir_all(&parent)?;
    }
    let data = parent.join("data");
    let workspace = parent.join("workspace");
    fs::create_dir_all(&data)?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"workspace_root\":\"../workspace\"}",
    )?;
    Ok((parent, data, workspace))
}

fn text(path: &Path) -> &str {
    path.to_str().unwrap_or("")
}

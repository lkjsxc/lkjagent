use std::fs;
use std::path::PathBuf;

use lkjagent_app::cli;
use serde_json::Value;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn doctor_reports_schema_and_safe_warnings() -> TestResult<()> {
    let data = fixture_root("doctor")?;

    let output = cli::run(["--data", data.to_string_lossy().as_ref(), "doctor"])?;

    assert!(output.contains("doctor: warn"));
    assert!(output.contains("schema: tables=25 missing=none"));
    assert!(output.contains("endpoint: url="));
    assert!(output.contains("workspace: root="));
    assert!(output.contains("prompt_refs: orphan=0"));
    assert!(output.contains("warnings: missing-workspace-dirs"));
    Ok(())
}

#[test]
fn workspace_reports_rows_and_json_shape() -> TestResult<()> {
    let data = fixture_root("workspace")?;
    cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "todo",
        "Buy",
        "milk",
    ])?;

    let text = cli::run(["--data", data.to_string_lossy().as_ref(), "workspace"])?;
    assert!(text.contains("records: total=1 archived=0"));
    assert!(text.contains("readmes: workspace=true,records=true"));

    let json = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "workspace",
        "--json",
    ])?;
    let value: Value = serde_json::from_str(&json)?;
    assert_eq!(value["records"]["total"], 1);
    assert_eq!(value["artifacts"], 0);
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_xtask::repository_determinism_gate::{
    check_configuration, check_docker, check_inputs, check_workflow,
};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn canonical_inputs_and_configuration_pass() -> TestResult<()> {
    let root = fixture("canonical")?;
    assert!(check_inputs(&root).is_empty());
    assert!(check_configuration(&root).is_empty());
    assert!(check_workflow(&root).is_empty());
    Ok(())
}

#[test]
fn missing_and_ignored_lockfile_fail() -> TestResult<()> {
    let root = fixture("lockfile")?;
    fs::remove_file(root.join("Cargo.lock"))?;
    assert!(check_inputs(&root)
        .iter()
        .any(|line| line.contains("Cargo.lock")));
    fs::copy(repo().join("Cargo.lock"), root.join("Cargo.lock"))?;
    fs::write(root.join(".gitignore"), "/Cargo.lock\n")?;
    assert!(check_inputs(&root)
        .iter()
        .any(|line| line == "Cargo.lock remains ignored"));
    Ok(())
}

#[test]
fn unknown_and_composite_configuration_fail() -> TestResult<()> {
    for (name, value) in [
        ("unknown", serde_json::json!({"unexpected": true})),
        (
            "array",
            serde_json::json!({"context_retrieval_limit": [12]}),
        ),
    ] {
        let root = fixture(name)?;
        let path = root.join("data/lkjagent.json");
        let mut config: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let object = config
            .as_object_mut()
            .ok_or("configuration is not an object")?;
        object.extend(value.as_object().ok_or("fault is not an object")?.clone());
        fs::write(path, serde_json::to_string_pretty(&config)?)?;
        assert!(!check_configuration(&root).is_empty(), "accepted {name}");
    }
    Ok(())
}

#[test]
fn missing_docker_copy_source_fails() -> TestResult<()> {
    let root = fixture("docker-source")?;
    let path = root.join("Dockerfile");
    let mut dockerfile = fs::read_to_string(&path)?;
    dockerfile.push_str("\nCOPY absent-input /tmp/absent\n");
    fs::write(path, dockerfile)?;
    assert!(check_docker(&root)
        .iter()
        .any(|line| line.contains("absent-input")));
    Ok(())
}

#[test]
fn workflow_without_anchored_clean_gate_fails() -> TestResult<()> {
    let root = fixture("workflow")?;
    fs::write(
        root.join(".github/workflows/verify.yml"),
        "steps:\n  - uses: actions/checkout@v4\n  - run: docker compose run verify\n",
    )?;
    assert!(!check_workflow(&root).is_empty());
    Ok(())
}

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> TestResult<PathBuf> {
    let root = std::env::temp_dir().join(format!(
        "lkjagent-repository-determinism-{name}-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    for relative in [
        "Cargo.lock",
        ".gitignore",
        ".dockerignore",
        "Dockerfile",
        "docker-compose.yml",
        ".github/workflows/verify.yml",
        "data/lkjagent.json",
        "docs/product/configuration-registry.md",
    ] {
        let target = root.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(repo().join(relative), target)?;
    }
    Ok(root)
}

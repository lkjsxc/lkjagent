use std::fs;
use std::path::PathBuf;

use lkjagent_xtask::gate::{parse_gate, Gate};
use lkjagent_xtask::node_gate;

#[test]
fn baseline_gate_has_a_named_command_surface() {
    let args = vec!["gate".to_string(), "baseline-capture".to_string()];
    assert!(matches!(
        parse_gate(&args),
        Ok(Gate::Node(name)) if name == "baseline-capture"
    ));
}

#[test]
fn protocol_and_retrieval_nodes_route_to_named_suites() -> Result<(), String> {
    let root = fixture_root("named")?;
    for identifier in ["protocol-tools", "workspace-retrieval-maintenance"] {
        let failures = node_gate::check(&root, identifier)
            .expect_err("an empty fixture cannot execute a node suite");
        if failures
            .iter()
            .any(|line| line == &format!("unknown node gate: {identifier}"))
        {
            return Err(format!("node did not route to a suite: {identifier}"));
        }
    }
    fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn baseline_gate_rejects_missing_raw_evidence() -> Result<(), String> {
    let root = fixture_root("missing")?;
    let failures = match node_gate::check(&root, "baseline-capture") {
        Err(failures) => failures,
        Ok(()) => return Err("gate unexpectedly passed".to_string()),
    };
    assert!(failures
        .iter()
        .any(|line| line.contains("raw evidence directory")));
    fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    Ok(())
}

fn fixture_root(name: &str) -> Result<PathBuf, String> {
    let root =
        std::env::temp_dir().join(format!("lkjagent-node-gate-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(root)
}

use std::fs;

use lkjagent_xtask::run;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn protocol_experiment_writes_decision_backed_matrix() -> TestResult<()> {
    let root = std::env::current_dir()?;
    let out = std::env::temp_dir().join(format!(
        "lkjagent-protocol-{}-matrix.md",
        std::process::id()
    ));
    if out.exists() {
        fs::remove_file(&out)?;
    }

    let code = run(
        &[
            "experiment".to_string(),
            "protocol".to_string(),
            "--out".to_string(),
            out.to_string_lossy().to_string(),
        ],
        &root,
    );

    assert_eq!(code, 0);
    let text = fs::read_to_string(&out)?;
    assert!(text.contains("decision=experiment-decision"));
    assert!(text.contains("tool_fp=fnv1a64:"));
    assert!(text.contains("tool-name-second parse=reject:BadParams"));
    assert!(text.contains("placeholder-path parse=accept admission=rejected result=pass"));
    assert!(text.contains("workspace-escape parse=accept admission=rejected result=pass"));
    assert!(!text.contains("result=fail"));
    Ok(())
}

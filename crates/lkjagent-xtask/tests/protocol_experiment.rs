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
    assert!(text.contains("legacy-v1-tool-call parse=reject:ActionV2(NoActionFound)"));
    assert!(text.contains("safe-v2-fs-read parse=accept admission=admitted result=pass"));
    assert!(text.contains("invalid-count parse=reject:ActionV2"));
    assert!(text.contains("placeholder-path parse=accept admission=rejected result=pass"));
    assert!(text.contains("workspace-escape parse=accept admission=rejected result=pass"));
    assert!(!text.contains("result=fail"));
    Ok(())
}

#[test]
fn protocol_experiment_writes_multi_profile_set() -> TestResult<()> {
    let root = std::env::current_dir()?;
    let out = std::env::temp_dir().join(format!("lkjagent-protocol-{}-all", std::process::id()));
    if out.exists() {
        fs::remove_dir_all(&out)?;
    }

    let code = run(
        &[
            "experiment".to_string(),
            "protocol".to_string(),
            "--all".to_string(),
            "--out-dir".to_string(),
            out.to_string_lossy().to_string(),
        ],
        &root,
    );

    assert_eq!(code, 0);
    for name in [
        "baseline",
        "protocol-safe",
        "context-kernel",
        "personal-workspace",
        "software-project",
        "artifact-manifest",
        "protocol-stress",
    ] {
        let text = fs::read_to_string(out.join(format!("{name}.md")))?;
        assert!(text.contains(&format!("profile={name}")));
        assert!(text.contains("result=pass"));
        assert!(!text.contains("result=fail"));
    }
    let adoption = fs::read_to_string(out.join("adoption.md"))?;
    assert!(adoption.contains("idea=artifact-manifest status=deferred"));
    Ok(())
}

#[test]
fn live_profiles_write_honest_skip_when_endpoint_env_absent() -> TestResult<()> {
    let root = std::env::current_dir()?;
    let out = std::env::temp_dir().join(format!("lkjagent-live-{}-profiles", std::process::id()));
    if out.exists() {
        fs::remove_dir_all(&out)?;
    }

    let code = run(
        &[
            "experiment".to_string(),
            "live-profiles".to_string(),
            "--duration-seconds".to_string(),
            "1".to_string(),
            "--out-dir".to_string(),
            out.to_string_lossy().to_string(),
            "--skip-endpoint".to_string(),
        ],
        &root,
    );

    assert_eq!(code, 0);
    for name in [
        "personal-workspace",
        "software-project",
        "structured-artifact",
        "protocol-stress",
    ] {
        let text = fs::read_to_string(out.join(name).join("summary.md"))?;
        assert!(text.contains("status=skipped"));
        assert!(text.contains("missing_env=LKJAGENT_ENDPOINT_URL,LKJAGENT_MODEL"));
    }
    let adoption = fs::read_to_string(out.join("adoption.md"))?;
    assert!(adoption.contains("status=skipped"));
    Ok(())
}

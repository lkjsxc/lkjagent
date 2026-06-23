mod support;

use std::fs;
use std::path::Path;

use lkjagent_protocol::parse_completion;
use lkjagent_tools::dispatch::{dispatch, validate_action};
use support::{action, dispatch_state, store, temp_workspace, tool_runtime, TestResult};

#[test]
fn file_root_refusal_renders_admitted_next_action() -> TestResult<()> {
    let workspace = temp_workspace("runtime-address-file-root")?;
    seed_artifact_file(&workspace)?;
    let output = dispatch_action(
        &workspace,
        action(
            "artifact.audit",
            &[
                ("root", "stories/root/topics/background.md"),
                ("kind", "story"),
            ],
        ),
    )?;

    assert!(output.contains("address_status=root_is_file"));
    assert!(output.contains("valid_example:"));
    validate_example(&output)?;
    Ok(())
}

#[test]
fn completion_refusal_does_not_render_file_root_audit() -> TestResult<()> {
    let workspace = temp_workspace("runtime-address-completion")?;
    let runtime = tool_runtime(workspace)?;
    let mut conn = store()?;
    let mut state = dispatch_state();
    state.graph_state = Some("case=1".to_string());
    state.graph_completion_ready = false;
    state.graph_missing = vec!["artifact-readiness".to_string()];

    let output = dispatch(
        &action("agent.done", &[("summary", "complete")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(output.content.contains("graph completion refused"));
    assert!(!output.content.contains(".md</root>"));
    validate_example(&output.content)?;
    Ok(())
}

#[test]
fn address_refusal_records_runtime_event() -> TestResult<()> {
    let workspace = temp_workspace("runtime-address-event")?;
    seed_artifact_file(&workspace)?;
    let runtime = tool_runtime(workspace)?;
    let mut conn = store()?;
    let mut state = dispatch_state();

    dispatch(
        &action(
            "artifact.audit",
            &[
                ("root", "stories/root/topics/background.md"),
                ("kind", "story"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(state
        .graph_evidence
        .iter()
        .any(|event| event.kind == "tool-address-refusal"
            && event.summary.contains("address_status=root_is_file")));
    Ok(())
}

#[test]
fn stale_cached_action_after_address_change_refused() -> TestResult<()> {
    let workspace = temp_workspace("runtime-address-stale")?;
    seed_artifact_file(&workspace)?;
    let output = dispatch_action(
        &workspace,
        action(
            "artifact.next",
            &[
                ("root", "stories/root/topics/background.md"),
                ("kind", "story"),
            ],
        ),
    )?;

    assert!(output.contains("address_status=root_is_file"));
    assert!(output.contains("normalized_root=stories/root"));
    assert!(!output.contains("next_action=artifact.audit"));
    validate_example(&output)?;
    Ok(())
}

fn seed_artifact_file(workspace: &Path) -> TestResult<()> {
    fs::create_dir_all(workspace.join("stories/root/topics"))?;
    fs::write(
        workspace.join("stories/root/catalog.toml"),
        "kind = \"story\"\n",
    )?;
    fs::write(
        workspace.join("stories/root/topics/background.md"),
        "# Background\n\n## Purpose\n\ncontent_state=structure-only\n",
    )?;
    Ok(())
}

fn dispatch_action(workspace: &Path, action: lkjagent_protocol::Action) -> TestResult<String> {
    let runtime = tool_runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut state = dispatch_state();
    Ok(dispatch(&action, &runtime, &mut conn, &mut state).content)
}

fn validate_example(output: &str) -> TestResult<()> {
    let example = output
        .split_once("valid_example:\n")
        .map(|(_, example)| example)
        .ok_or_else(|| "missing valid example".to_string())?;
    let parsed = parse_completion(example).map_err(|err| format!("parse failed: {err:?}"))?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    Ok(())
}

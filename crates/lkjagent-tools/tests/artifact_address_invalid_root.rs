mod support;

use std::fs;
use std::path::Path;

use lkjagent_protocol::parse_completion;
use lkjagent_tools::dispatch::{dispatch, validate_action};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn existing_markdown_directory_reports_invalid_root_repair() -> TestResult<()> {
    let workspace = temp_workspace("artifact-invalid-md-dir")?;
    fs::create_dir_all(workspace.join("stories/foo.md"))?;
    fs::write(workspace.join("stories/foo.md/README.md"), "# Bad Root\n")?;

    let output = run(
        &workspace,
        action(
            "artifact.audit",
            &[("root", "stories/foo.md"), ("kind", "story")],
        ),
    )?;

    assert!(output.contains("address_status=root_ends_with_markdown_suffix"));
    assert!(output.contains("detected_path_kind=directory"));
    assert!(output.contains("invalid_root_marker=required"));
    assert!(output.contains("repair_outcome=choose_catalog_root_or_repair_markdown_directory"));
    validate_example(&workspace, &output)?;
    Ok(())
}

fn run(workspace: &Path, action: lkjagent_protocol::Action) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(&action, &runtime, &mut conn, &mut dispatch_state).content)
}

fn validate_example(workspace: &Path, output: &str) -> TestResult<()> {
    let example = output
        .split_once("valid_example:\n")
        .map(|(_, example)| example)
        .ok_or_else(|| "missing valid example".to_string())?;
    let example = exact_action(example).ok_or("missing action close")?;
    let parsed = parse_completion(example).map_err(|err| format!("parse failed: {err:?}"))?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    let observation = run(workspace, parsed)?;
    assert!(!observation.contains("unknown tool"));
    assert!(!observation.is_empty());
    Ok(())
}

fn exact_action(text: &str) -> Option<&str> {
    let end = text.find("</action>")?.saturating_add("</action>".len());
    text.get(..end)
}

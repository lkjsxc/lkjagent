mod support;

use std::fs;
use std::path::Path;

use lkjagent_protocol::parse_completion;
use lkjagent_store::artifact_ledger::latest_for_case;
use lkjagent_tools::dispatch::{dispatch, validate_action};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_audit_refuses_existing_markdown_suffix_directory() -> TestResult<()> {
    let workspace = temp_workspace("md-dir-artifact-audit")?;
    seed_bad_root(&workspace)?;

    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/characters.md"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    let ledger = latest_for_case(&conn, 0)?.ok_or("missing invalid root marker")?;
    assert_eq!(ledger.lifecycle_state, "invalid-root");
    assert_eq!(ledger.readiness_status, "invalid");
    assert!(output.contains("address_status=root_ends_with_markdown_suffix"));
    assert!(!output.contains("Not a directory"));
    assert!(!output.contains("artifact audit passed"));
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn doc_audit_refuses_existing_markdown_suffix_directory_root() -> TestResult<()> {
    let workspace = temp_workspace("md-dir-doc-audit")?;
    seed_bad_root(&workspace)?;

    let output = run(
        &workspace,
        action("doc.audit", &[("root", "stories/characters.md")]),
    )?;

    assert!(output.contains("address_status=root_ends_with_markdown_suffix"));
    assert!(!output.contains("Not a directory"));
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn artifact_next_file_under_bad_markdown_directory_refuses_bad_root() -> TestResult<()> {
    let workspace = temp_workspace("md-dir-artifact-next")?;
    seed_bad_root(&workspace)?;

    let output = run(
        &workspace,
        action(
            "artifact.next",
            &[
                ("root", "stories/characters.md/topics/a.md"),
                ("kind", "story"),
            ],
        ),
    )?;

    assert!(output.contains("address_status=root_ends_with_markdown_suffix"));
    assert!(!output.contains("missing=0"));
    assert!(!output.contains("next_action=artifact.audit"));
    validate_example(&workspace, &output)?;
    Ok(())
}

fn seed_bad_root(workspace: &Path) -> TestResult<()> {
    fs::create_dir_all(workspace.join("stories/characters.md/topics"))?;
    fs::write(
        workspace.join("stories/characters.md/catalog.toml"),
        "kind = \"story\"\n",
    )?;
    fs::write(
        workspace.join("stories/characters.md/README.md"),
        "# Characters\n\n## Purpose\n\nBad root.\n",
    )?;
    fs::write(workspace.join("stories/characters.md/topics/a.md"), "# A\n")?;
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

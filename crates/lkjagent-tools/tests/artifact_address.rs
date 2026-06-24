mod support;

use std::fs;
use std::path::Path;

use lkjagent_protocol::parse_completion;
use lkjagent_tools::dispatch::{dispatch, validate_action};
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_next_on_file_root_does_not_render_file_audit() -> TestResult<()> {
    let workspace = temp_workspace("artifact-address-next-file")?;
    seed_artifact_file(&workspace)?;

    let output = run(
        &workspace,
        action(
            "artifact.next",
            &[("root", "stories/root/topics/background.md")],
        ),
    )?;

    assert!(output.contains("address_status=root_is_file"));
    assert!(output.contains("normalized_root=stories/root"));
    assert!(output.contains("weak_path=topics/background.md"));
    assert!(output.contains("candidate_action=fs.batch_write"));
    assert!(!output
        .contains("<tool>artifact.audit</tool>\n<root>stories/root/topics/background.md</root>"));
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn artifact_audit_on_file_root_returns_semantic_refusal() -> TestResult<()> {
    let workspace = temp_workspace("artifact-address-audit-file")?;
    seed_artifact_file(&workspace)?;

    let output = run(
        &workspace,
        action(
            "artifact.audit",
            &[
                ("root", "stories/root/topics/background.md"),
                ("kind", "story"),
            ],
        ),
    )?;

    assert!(output.contains("artifact address refused"));
    assert!(output.contains("address_status=root_is_file"));
    assert!(output.contains("<tool>artifact.audit</tool>"));
    assert!(output.contains("<root>stories/root</root>"));
    assert!(!output.contains("Not a directory"));
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn doc_audit_on_file_root_returns_semantic_refusal() -> TestResult<()> {
    let workspace = temp_workspace("artifact-address-doc-file")?;
    fs::create_dir_all(workspace.join("docs"))?;
    fs::write(
        workspace.join("docs/page.md"),
        "# Page\n\n## Purpose\n\nText.\n",
    )?;

    let output = run(&workspace, action("doc.audit", &[("root", "docs/page.md")]))?;

    assert!(output.contains("address_status=root_is_file"));
    assert!(output.contains("root_is_file: docs/page.md"));
    assert!(output.contains("<tool>fs.read</tool>"));
    assert!(!output.contains("Not a directory"));
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn artifact_apply_refuses_missing_markdown_root() -> TestResult<()> {
    let workspace = temp_workspace("artifact-address-apply-md")?;
    let output = run(
        &workspace,
        action(
            "artifact.apply",
            &[
                ("root", "stories/x/characters.md"),
                ("title", "Characters"),
                ("kind", "story"),
            ],
        ),
    )?;

    assert!(output.contains("root_ends_with_markdown_suffix"));
    assert!(!workspace.join("stories/x/characters.md").exists());
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn doc_scaffold_refuses_markdown_suffix_root() -> TestResult<()> {
    let workspace = temp_workspace("artifact-address-doc-scaffold-md")?;
    let output = run(
        &workspace,
        action(
            "doc.scaffold",
            &[("root", "docs/page.md"), ("title", "Page")],
        ),
    )?;

    assert!(output.contains("root_ends_with_markdown_suffix"));
    assert!(!workspace.join("docs/page.md").exists());
    validate_example(&workspace, &output)?;
    Ok(())
}

#[test]
fn artifact_next_missing_directory_root_still_suggests_artifact_apply() -> TestResult<()> {
    let workspace = temp_workspace("artifact-address-missing-root")?;
    let output = run(
        &workspace,
        action("artifact.next", &[("root", "stories/new")]),
    )?;

    assert!(output.contains("missing=root"));
    assert!(output.contains("candidate_action=artifact.apply"));
    assert!(output.contains("<root>stories/new</root>"));
    validate_example(&workspace, &output)?;
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

fn run(workspace: &Path, action: lkjagent_protocol::Action) -> TestResult<String> {
    let runtime = runtime(workspace.to_path_buf())?;
    let mut conn = store()?;
    let mut dispatch_state = state();
    Ok(dispatch(&action, &runtime, &mut conn, &mut dispatch_state).content)
}

fn validate_example(workspace: &Path, output: &str) -> TestResult<()> {
    let example = output
        .split_once("candidate_example:\n")
        .or_else(|| output.split_once("valid_example:\n"))
        .map(|(_, example)| example)
        .ok_or_else(|| "missing valid example".to_string())?;
    let parsed = parse_completion(example).map_err(|err| format!("parse failed: {err:?}"))?;
    validate_action(&parsed).map_err(|err| format!("validation failed: {err}"))?;
    let observation = run(workspace, parsed)?;
    assert!(!observation.contains("unknown tool"));
    assert!(!observation.contains("parameter validation failed"));
    assert!(!observation.contains("effective policy refused"));
    assert!(!observation.is_empty());
    Ok(())
}

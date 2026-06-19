mod support;

use lkjagent_tools::dispatch::{dispatch, DispatchOutput};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn recursive_knowledge_refuses_duplicate_docs_roots() -> TestResult<()> {
    let workspace = temp_workspace("knowledge-path-guard")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state
        .control
        .start_task("百科事典を高度に再帰的な構造でdocsに作ってください");

    let survey = dispatch(
        &action("shell.run", &[("command", git_probe())]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(!is_error(&survey));
    assert!(survey.content.contains("git=absent"));

    let write = dispatch(
        &action(
            "fs.write",
            &[("path", "docs/ontology/README.md"), ("content", "# Bad\n")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&write));
    assert!(write.content.contains("seeded top-level docs map"));

    let mkdir = dispatch(
        &action(
            "shell.run",
            &[("command", "mkdir -p docs/ontology/docs/agents")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&mkdir));
    assert!(mkdir.content.contains("shell.run is read-only"));
    Ok(())
}

#[test]
fn counted_recursive_knowledge_allows_batch_writes() -> TestResult<()> {
    let workspace = temp_workspace("knowledge-count-guard")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    state
        .control
        .start_task("百科事典を再帰的な構造で合計100ファイル作ってください");

    let mkdir = dispatch(
        &action("shell.run", &[("command", "mkdir -p docs/ontology")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(!is_error(&mkdir));

    let write = dispatch(
        &action(
            "fs.write",
            &[("path", "docs/ontology/README.md"), ("content", "# Ok\n")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(!is_error(&write));
    Ok(())
}

fn git_probe() -> &'static str {
    "git rev-parse --is-inside-work-tree >/dev/null 2>&1 && git status --short || printf 'git=absent\\n'"
}

fn is_error(output: &DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}

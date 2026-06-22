mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn fs_list_search_stat_and_batch_write_are_bounded() -> TestResult<()> {
    let workspace = temp_workspace("typed-fs")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    fs::create_dir_all(workspace.join("docs"))?;
    fs::write(workspace.join("docs/a.md"), "Alpha\nBeta\n")?;
    fs::write(workspace.join("docs/b.txt"), "beta\n")?;

    let listed = dispatch(
        &action("fs.list", &[("path", "docs"), ("depth", "1")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(listed.content.contains("file docs/a.md"));
    assert!(listed.content.contains("lines=2"));

    let found = dispatch(
        &action("fs.search", &[("query", "alpha"), ("path", "docs")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(found.content.contains("docs/a.md:1"));

    let stat = dispatch(
        &action("fs.stat", &[("path", "docs/a.md")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(stat.content.contains("kind=file"));
    assert!(stat.content.contains("lines=2"));
    assert!(stat.content.contains("checksum="));

    let files = "path: out/one.md\ncontent:\n# One\n-- lkjagent-next-file --\npath: out/two.md\ncontent:\n# Two\n";
    let batch = dispatch(
        &action("fs.batch_write", &[("files", files)]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(batch.content.contains("files_written=2"));
    assert!(workspace.join("out/one.md").is_file());
    Ok(())
}

#[test]
fn fs_batch_write_normalizes_common_path_variants() -> TestResult<()> {
    let workspace = temp_workspace("typed-fs-normalize")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    let files = "\n\npath:out/no-space.md\ncontent:\n# No Space\n\
-- lkjagent-next-file --\n<path>out/xml.md</path>\ncontent:\n# Xml\n\
-- lkjagent-next-file --\n<path:out/angled.md>\ncontent:\n# Angled\n";
    let batch = dispatch(
        &action("fs.batch_write", &[("files", files)]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(batch.content.contains("files_written=3"));
    assert!(workspace.join("out/no-space.md").is_file());
    assert!(workspace.join("out/xml.md").is_file());
    assert!(workspace.join("out/angled.md").is_file());
    Ok(())
}

#[test]
fn fs_batch_write_rejects_duplicate_and_escape() -> TestResult<()> {
    let workspace = temp_workspace("typed-fs-reject")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let dup = "path: a.md\ncontent:\nA\n-- lkjagent-next-file --\npath: a.md\ncontent:\nB\n";
    let duplicate = dispatch(
        &action("fs.batch_write", &[("files", dup)]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&duplicate));
    assert!(duplicate.content.contains("duplicate path"));

    let escaped = dispatch(
        &action("fs.mkdir", &[("path", "../outside")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&escaped));
    Ok(())
}

#[test]
fn workspace_doc_and_verify_tools_route_without_shell() -> TestResult<()> {
    let workspace = temp_workspace("typed-doc")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();
    fs::write(workspace.join("Cargo.toml"), "[workspace]\nmembers=[]\n")?;

    let summary = dispatch(
        &action("workspace.summary", &[("path", "."), ("depth", "1")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(summary.content.contains("cargo_workspace=present"));

    let scaffold = dispatch(
        &action(
            "doc.scaffold",
            &[
                ("root", "guide"),
                ("title", "Guide"),
                ("count", "3"),
                ("mode", "exact"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(scaffold.content.contains("files=3"));

    let audit = dispatch(
        &action(
            "doc.audit",
            &[("root", "guide"), ("count", "3"), ("mode", "exact")],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(audit.content.contains("document audit failed"));
    assert!(audit.content.contains("scaffold_only_content"));

    let bad_gate = dispatch(
        &action("verify.xtask", &[("gate", "unknown")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&bad_gate));
    assert!(bad_gate.content.contains("unknown xtask gate"));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}

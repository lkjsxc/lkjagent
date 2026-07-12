use std::path::PathBuf;
use std::{fs, vec};

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::model::TaskState;
use lkjagent_effects::checks::run_check;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

mod support;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn docs_tree_fake_endpoint_closes_and_effect_checks_pass() -> TestResult<()> {
    let data = run_docs(
        "docs-ok",
        vec![
            plan(&[
                "docs/daemon/README.md",
                "docs/daemon/setup.md",
                "docs/daemon/run.md",
            ]),
            "<content># Daemon Docs\n\n- [Setup](setup.md)\n- [Run](run.md)</content>".to_string(),
            "<content># Setup\n\nReturn to [index](README.md).</content>".to_string(),
            "<content># Run\n\nReturn to [index](README.md).</content>".to_string(),
            "<final><message>docs tree complete</message></final>".to_string(),
        ],
    )?;
    let root = support::workspace(&data);
    let readme = lkjagent_core::model::CheckSpec::ReadmeCoverage {
        root: "docs/daemon".to_string(),
    };
    let links = lkjagent_core::model::CheckSpec::LinksResolve {
        root: "docs/daemon".to_string(),
    };
    assert!(run_check(&root, &readme)?.passed);
    assert!(run_check(&root, &links)?.passed);
    Ok(())
}

#[test]
fn docs_tree_dangling_link_materializes_revise_then_closes() -> TestResult<()> {
    let data = run_docs(
        "docs-revise",
        vec![
            plan(&[
                "docs/daemon/README.md",
                "docs/daemon/setup.md",
                "docs/daemon/run.md",
            ]),
            "<content># Daemon Docs\n\n- [Missing](missing.md)</content>".to_string(),
            "<content># Setup\n\nReturn to [index](README.md).</content>".to_string(),
            "<content># Run\n\nReturn to [index](README.md).</content>".to_string(),
            "<content># Daemon Docs\n\n- [Setup](setup.md)\n- [Run](run.md)</content>".to_string(),
            "<final><message>docs tree repaired</message></final>".to_string(),
        ],
    )?;
    let readme = fs::read_to_string(support::workspace(&data).join("docs/daemon/README.md"))?;
    assert!(readme.contains("setup.md"));
    assert!(!readme.contains("missing.md"));
    Ok(())
}

fn run_docs(name: &str, outputs: Vec<String>) -> TestResult<PathBuf> {
    let data = fixture_root(name)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(
        &conn,
        "Create 2 pages of documentation in docs/daemon.",
        "now",
    )?;
    drop(conn);
    let mut endpoint = ScriptedEndpoint { outputs, index: 0 };
    let snapshot = run_until_idle(&data, &mut endpoint, 20)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    Ok(data)
}

fn plan(paths: &[&str]) -> String {
    let body = paths
        .iter()
        .map(|path| format!("write {path} | page | words=20"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("<plan>{body}</plan>")
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    support::isolate_workspace(&path)?;
    Ok(path)
}

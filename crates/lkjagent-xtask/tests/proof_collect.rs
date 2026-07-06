use std::fs;
use std::path::PathBuf;

use lkjagent_store::plan_schema::setup;
use lkjagent_xtask::run;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn proof_collect_writes_bounded_checks_file() -> TestResult<()> {
    let root = fixture_root("proof-collect")?;
    let data = root.join("data");
    let out = root.join("proof");
    fs::create_dir_all(data.join("workspace"))?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    seed_check(&conn)?;
    drop(conn);

    let code = run(
        &[
            "proof".to_string(),
            "collect".to_string(),
            "--data".to_string(),
            data.to_string_lossy().to_string(),
            "--out".to_string(),
            out.to_string_lossy().to_string(),
        ],
        &root,
    );

    assert_eq!(code, 0);
    let checks = fs::read_to_string(out.join("checks.md"))?;
    assert!(checks.contains("# Checks"));
    assert!(checks.contains("step=1 name=min_words passed=true"));
    assert!(checks.contains("params={\"min\":350}"));
    assert!(checks.contains("measured=words=370"));
    Ok(())
}

fn seed_check(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO tasks (id, objective, template, state, brief, budget_used,
         budget, summary, created_at, updated_at)
         VALUES (1, 'write', 'generic', 'Open', 'brief', 0, 1, '', 'now', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO steps (id, task_id, ordinal, kind, title, instruction,
         inputs_json, checks_json, state, attempts_used, created_at, updated_at)
         VALUES (1, 1, 1, 'Write', 'draft', 'write', '{}', '[]', 'Open', 0,
         'now', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO check_results (step_id, name, params_json, passed,
         measured, created_at) VALUES (1, 'min_words', '{\"min\":350}', 1,
         'words=370', 'now')",
        [],
    )?;
    Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::{insert_step_tx, insert_task, set_task_state};
use lkjagent_store::plan_schema::setup;
use lkjagent_store::plan_turn::commit_commands;
use rusqlite::Connection;

pub fn run(args: &[String], root: &Path) -> i32 {
    match args {
        [] => replay(root),
        [cmd] if cmd == "replay" => replay(root),
        [cmd] if cmd == "live" => live(root),
        _ => fail("smoke", "use: smoke replay | smoke live"),
    }
}

fn replay(root: &Path) -> i32 {
    match run_replay(root) {
        Ok(path) => {
            println!("ok smoke replay data={}", path.display());
            0
        }
        Err(error) => fail("smoke replay", &error),
    }
}

pub fn run_replay(root: &Path) -> Result<PathBuf, String> {
    let data = root.join("tmp/smoke-replay-data");
    let _ = fs::remove_dir_all(&data);
    fs::create_dir_all(data.join("workspace")).map_err(|e| e.to_string())?;
    let mut conn = Connection::open(data.join("lkjagent.sqlite3")).map_err(|e| e.to_string())?;
    setup(&conn).map_err(|e| e.to_string())?;
    replay_manuscript(&mut conn, &data)?;
    replay_question(&mut conn)?;
    fs::write(
        data.join("summary.md"),
        "# Smoke Replay\n\nstatus: closed\n",
    )
    .map_err(|e| e.to_string())?;
    Ok(data)
}

fn replay_manuscript(conn: &mut Connection, data: &Path) -> Result<(), String> {
    let objective = "Write the Aurora Ledger manuscript at stories/aurora-ledger as 3 chapters totaling 1500 words.";
    let snapshot = instantiate(1, objective);
    persist_snapshot(conn, &snapshot)?;
    let dir = data.join("workspace/stories/aurora-ledger/manuscript");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    for index in 1..=3 {
        fs::write(
            dir.join(format!("chapter-{index:02}.md")),
            prose(index, 500),
        )
        .map_err(|e| e.to_string())?;
    }
    let results = snapshot
        .task
        .checks
        .iter()
        .map(|check| lkjagent_effects::checks::run_check(&data.join("workspace"), check))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    commit_commands(conn, 1, &[Command::RecordChecks(results)], "now")
        .map_err(|e| e.to_string())?;
    set_task_state(conn, 1, TaskState::Closed, "now").map_err(|e| e.to_string())
}

fn replay_question(conn: &mut Connection) -> Result<(), String> {
    let mut snapshot = instantiate(2, "What is a plan-driven agent?");
    for step in &mut snapshot.steps {
        step.id = step.id.saturating_add(200);
    }
    persist_snapshot(conn, &snapshot)?;
    set_task_state(conn, 2, TaskState::Closed, "now").map_err(|e| e.to_string())
}

fn persist_snapshot(
    conn: &mut Connection,
    snapshot: &lkjagent_core::model::TaskSnapshot,
) -> Result<(), String> {
    insert_task(conn, &snapshot.task, None, "now").map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    for step in &snapshot.steps {
        insert_step_tx(&tx, step, "now").map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}

fn live(root: &Path) -> i32 {
    let configured = (env_present("LKJAGENT_ENDPOINT_URL") && env_present("LKJAGENT_MODEL"))
        || config_present(root);
    if configured {
        println!("ok smoke live status=skipped reason=operator-command-required");
    } else {
        println!("ok smoke live status=skipped reason=endpoint-config-absent");
    }
    0
}

fn env_present(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

fn config_present(root: &Path) -> bool {
    let Ok(text) = fs::read_to_string(root.join("data/lkjagent.json")) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return false;
    };
    let endpoint = value.get("endpoint").unwrap_or(&serde_json::Value::Null);
    endpoint
        .get("url")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|url| !url.is_empty())
        && endpoint
            .get("model")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|model| !model.is_empty())
}

fn prose(index: usize, words: usize) -> String {
    (0..words)
        .map(|n| format!("aurora{index}_{n}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn fail(name: &str, message: &str) -> i32 {
    eprintln!("{name} failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}

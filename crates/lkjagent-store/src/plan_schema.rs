use rusqlite::Connection;

use crate::error::StoreResult;

pub const APPLICATION_TABLES: &[&str] = &[
    "queue",
    "tasks",
    "steps",
    "attempts",
    "check_results",
    "events",
    "memory",
    "memory_fts",
    "token_usage",
    "config",
];

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;

        CREATE TABLE IF NOT EXISTS queue (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at TEXT NOT NULL,
            delivered_at TEXT,
            task_id INTEGER
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            queue_id INTEGER,
            objective TEXT NOT NULL,
            template TEXT NOT NULL,
            state TEXT NOT NULL,
            brief TEXT NOT NULL,
            budget_used INTEGER NOT NULL,
            budget INTEGER NOT NULL,
            summary TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS steps (
            id INTEGER PRIMARY KEY,
            task_id INTEGER NOT NULL,
            ordinal INTEGER NOT NULL,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            instruction TEXT NOT NULL,
            inputs_json TEXT NOT NULL,
            output_path TEXT,
            checks_json TEXT NOT NULL,
            state TEXT NOT NULL,
            attempts_used INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY(task_id) REFERENCES tasks(id)
        );

        CREATE TABLE IF NOT EXISTS attempts (
            id INTEGER PRIMARY KEY,
            step_id INTEGER NOT NULL,
            ordinal INTEGER NOT NULL,
            prompt_fingerprint TEXT NOT NULL,
            exchange_ref TEXT NOT NULL,
            outcome TEXT NOT NULL,
            diagnosis TEXT NOT NULL,
            tokens_in INTEGER NOT NULL,
            tokens_out INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(step_id) REFERENCES steps(id)
        );
        ",
    )?;
    setup_tail(conn)
}

fn setup_tail(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS check_results (
            id INTEGER PRIMARY KEY,
            step_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            params_json TEXT NOT NULL,
            passed INTEGER NOT NULL,
            measured TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            task_id INTEGER,
            kind TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS memory (
            id INTEGER PRIMARY KEY,
            topic TEXT NOT NULL,
            content TEXT NOT NULL,
            task_id INTEGER,
            created_at TEXT NOT NULL
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts USING fts5(topic, content);

        CREATE TABLE IF NOT EXISTS token_usage (
            id INTEGER PRIMARY KEY,
            task_id INTEGER NOT NULL,
            attempt_id INTEGER,
            prompt_tokens INTEGER NOT NULL,
            completion_tokens INTEGER NOT NULL,
            cached_tokens INTEGER NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

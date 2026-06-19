use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;

        CREATE TABLE IF NOT EXISTS queue (
            id INTEGER PRIMARY KEY,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            source_queue_id INTEGER,
            content TEXT NOT NULL,
            status TEXT NOT NULL,
            delivered_turn INTEGER
        );

        CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            turn INTEGER,
            kind TEXT NOT NULL,
            content TEXT NOT NULL,
            tokens INTEGER NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS memory (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            tags TEXT NOT NULL,
            content TEXT NOT NULL,
            tokens INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts
        USING fts5(title, tags, content, content='memory', content_rowid='id');

        CREATE TABLE IF NOT EXISTS state (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS graph_cases (
            id INTEGER PRIMARY KEY,
            objective TEXT NOT NULL,
            family TEXT NOT NULL,
            phase TEXT NOT NULL,
            active_node TEXT NOT NULL,
            status TEXT NOT NULL,
            plan TEXT NOT NULL,
            evidence_requirements TEXT NOT NULL,
            selected_packages TEXT NOT NULL,
            pending_checks TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS graph_events (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            kind TEXT NOT NULL,
            node TEXT NOT NULL,
            phase TEXT NOT NULL,
            summary TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(case_id) REFERENCES graph_cases(id)
        );

        CREATE TABLE IF NOT EXISTS graph_evidence (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            requirement TEXT NOT NULL,
            kind TEXT NOT NULL,
            summary TEXT NOT NULL,
            path TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY(case_id) REFERENCES graph_cases(id)
        );

        CREATE TABLE IF NOT EXISTS graph_memory_links (
            case_id INTEGER NOT NULL,
            memory_id INTEGER NOT NULL,
            node TEXT NOT NULL,
            reason TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY(case_id, memory_id, node),
            FOREIGN KEY(case_id) REFERENCES graph_cases(id),
            FOREIGN KEY(memory_id) REFERENCES memory(id)
        );
        ",
    )?;
    Ok(())
}

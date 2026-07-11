use crate::error::StoreResult;
use crate::state_schema;
use rusqlite::Connection;
#[rustfmt::skip]
pub const APPLICATION_TABLES: &[&str] = &[
    "queue", "tasks", "steps", "attempts", "check_results", "events", "memory", "memory_fts",
    "token_usage", "config", "cases", "runtime_events", "state_cells", "state_history",
    "runtime_decisions", "prompt_frames", "prompt_cards", "tool_admissions", "observations",
    "context_items", "context_edges", "state_edges", "workspace_records", "workspace_record_history",
    "workspace_manifest", "workspace_path_aliases", "workspace_rebalance_audit", "artifacts",
    "provider_exchanges", "effect_journal", "workspace_operations", "workspace_search_chunks", "workspace_search_lexical", "workspace_search_trigram",
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
            force_new INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            delivered_at TEXT,
            task_id INTEGER,
            route_lane TEXT,
            route_durability TEXT,
            route_title_seed TEXT,
            route_transform_allowed INTEGER
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
            actions_used INTEGER NOT NULL DEFAULT 0,
            action_budget INTEGER NOT NULL DEFAULT 0,
            split_used INTEGER NOT NULL DEFAULT 0,
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
    crate::plan_migrations::migrate(conn)?;
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
            decision_id TEXT,
            evidence_fingerprint TEXT,
            artifact_refs_json TEXT NOT NULL DEFAULT '[]',
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

        CREATE TRIGGER IF NOT EXISTS memory_ai AFTER INSERT ON memory BEGIN
            INSERT INTO memory_fts(rowid, topic, content)
            VALUES (new.id, new.topic, new.content);
        END;

        INSERT INTO memory_fts(rowid, topic, content)
        SELECT id, topic, content FROM memory
        WHERE id NOT IN (SELECT rowid FROM memory_fts);

        CREATE TABLE IF NOT EXISTS token_usage (
            id INTEGER PRIMARY KEY,
            task_id INTEGER NOT NULL,
            attempt_id INTEGER,
            prompt_tokens INTEGER,
            completion_tokens INTEGER,
            cached_tokens INTEGER,
            input_total_tokens INTEGER,
            input_cached_tokens INTEGER,
            input_uncached_tokens INTEGER,
            output_tokens INTEGER,
            cache_status TEXT NOT NULL DEFAULT 'unknown',
            raw_usage_json TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS effect_journal (
            id TEXT PRIMARY KEY, admission_id TEXT NOT NULL UNIQUE REFERENCES tool_admissions(id), decision_id TEXT NOT NULL,
            idempotency_key TEXT NOT NULL UNIQUE, command_ordinal INTEGER NOT NULL, target_path TEXT, effect_name TEXT NOT NULL, state TEXT NOT NULL,
            prior_fingerprint TEXT NOT NULL, intended_fingerprint TEXT NOT NULL, observation_id TEXT UNIQUE REFERENCES observations(id),
            outcome_fingerprint TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL
        );
        ",
    )?;
    ensure_token_usage_columns(conn)?;
    crate::plan_migrations::migrate_effect_journal(conn)?;
    crate::plan_migrations::migrate_checks(conn)?;
    state_schema::setup(conn)
}
fn ensure_token_usage_columns(conn: &Connection) -> StoreResult<()> {
    for (name, spec) in [
        ("input_total_tokens", "INTEGER"),
        ("input_cached_tokens", "INTEGER"),
        ("input_uncached_tokens", "INTEGER"),
        ("output_tokens", "INTEGER"),
        ("cache_status", "TEXT NOT NULL DEFAULT 'unknown'"),
        ("raw_usage_json", "TEXT NOT NULL DEFAULT '{}'"),
    ] {
        if !has_column(conn, "token_usage", name)? {
            conn.execute(
                &format!("ALTER TABLE token_usage ADD COLUMN {name} {spec}"),
                [],
            )?;
        }
    }
    Ok(())
}

fn has_column(conn: &Connection, table: &str, name: &str) -> StoreResult<bool> {
    let mut statement = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let rows = statement.query_map([], |row| row.get::<_, String>(1))?;
    for row in rows {
        if row? == name {
            return Ok(true);
        }
    }
    Ok(false)
}

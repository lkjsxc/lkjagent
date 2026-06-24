use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS personal_records (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            status TEXT NOT NULL,
            tags TEXT NOT NULL,
            timezone TEXT,
            start_at TEXT,
            end_at TEXT,
            due_at TEXT,
            recurrence TEXT,
            priority TEXT,
            project TEXT,
            source_case_id INTEGER,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            closed_at TEXT
        );

        CREATE TABLE IF NOT EXISTS personal_record_events (
            id INTEGER PRIMARY KEY,
            record_id INTEGER NOT NULL,
            event_kind TEXT NOT NULL,
            summary TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(record_id) REFERENCES personal_records(id)
        );

        CREATE TABLE IF NOT EXISTS personal_record_links (
            id INTEGER PRIMARY KEY,
            source_record_id INTEGER NOT NULL,
            relation TEXT NOT NULL,
            target_record_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(source_record_id) REFERENCES personal_records(id),
            FOREIGN KEY(target_record_id) REFERENCES personal_records(id)
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS personal_records_fts
        USING fts5(title, body, tags, project, content='personal_records', content_rowid='id');

        CREATE INDEX IF NOT EXISTS idx_personal_kind_status
        ON personal_records(kind, status);
        CREATE INDEX IF NOT EXISTS idx_personal_start
        ON personal_records(start_at);
        CREATE INDEX IF NOT EXISTS idx_personal_due
        ON personal_records(due_at);
        CREATE INDEX IF NOT EXISTS idx_personal_project
        ON personal_records(project);
        ",
    )?;
    Ok(())
}

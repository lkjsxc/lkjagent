use rusqlite::Connection;

use crate::error::StoreResult;

pub fn setup(conn: &Connection) -> StoreResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS artifact_ledger (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            artifact_id TEXT NOT NULL UNIQUE,
            root TEXT NOT NULL,
            kind TEXT NOT NULL,
            normalized_topic TEXT NOT NULL,
            requested_scale TEXT NOT NULL,
            profile TEXT NOT NULL,
            lifecycle_state TEXT NOT NULL,
            topology_status TEXT NOT NULL,
            readiness_status TEXT NOT NULL,
            objective_match_status TEXT NOT NULL,
            latest_audit_id TEXT,
            weak_path_count INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_weak_paths (
            id INTEGER PRIMARY KEY,
            artifact_ledger_id INTEGER NOT NULL,
            path TEXT NOT NULL,
            role TEXT NOT NULL,
            missing_requirements TEXT NOT NULL,
            weak_signals TEXT NOT NULL,
            semantic_mismatch TEXT NOT NULL,
            retry_count INTEGER NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_batch_cursors (
            id INTEGER PRIMARY KEY,
            artifact_ledger_id INTEGER NOT NULL,
            root TEXT NOT NULL,
            planned_paths TEXT NOT NULL,
            completed_paths TEXT NOT NULL,
            failed_paths TEXT NOT NULL,
            current_index INTEGER NOT NULL,
            last_valid_example TEXT NOT NULL,
            retry_counts TEXT NOT NULL,
            fallback_mode TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_plans (
            id INTEGER PRIMARY KEY,
            case_id INTEGER NOT NULL,
            artifact_id TEXT NOT NULL UNIQUE,
            owner_objective TEXT NOT NULL,
            artifact_kind TEXT NOT NULL,
            root TEXT NOT NULL,
            profile TEXT NOT NULL,
            normalized_title TEXT NOT NULL,
            measurement_kind TEXT NOT NULL,
            requested_total INTEGER NOT NULL,
            accepted_floor INTEGER NOT NULL,
            section_count INTEGER NOT NULL,
            language_hint TEXT NOT NULL,
            forbidden_roots TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_atoms (
            id INTEGER PRIMARY KEY,
            plan_id INTEGER NOT NULL,
            atom_id TEXT NOT NULL UNIQUE,
            sequence INTEGER NOT NULL,
            role TEXT NOT NULL,
            path TEXT NOT NULL,
            status TEXT NOT NULL,
            measurement_kind TEXT NOT NULL,
            target_count INTEGER NOT NULL,
            count_floor INTEGER NOT NULL,
            measured_count INTEGER NOT NULL,
            byte_budget INTEGER NOT NULL,
            required_sections TEXT NOT NULL,
            weak_classes TEXT NOT NULL,
            assembly_target TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_atom_edges (
            id INTEGER PRIMARY KEY,
            plan_id INTEGER NOT NULL,
            from_atom_id TEXT NOT NULL,
            to_atom_id TEXT NOT NULL,
            relation TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_write_contracts (
            id INTEGER PRIMARY KEY,
            contract_id TEXT NOT NULL UNIQUE,
            plan_id INTEGER NOT NULL,
            atom_ids TEXT NOT NULL,
            exact_paths TEXT NOT NULL,
            max_files INTEGER NOT NULL,
            max_file_bytes INTEGER NOT NULL,
            max_batch_bytes INTEGER NOT NULL,
            target_count INTEGER NOT NULL,
            count_floor INTEGER NOT NULL,
            required_sections TEXT NOT NULL,
            continuity_digest TEXT NOT NULL,
            forbidden_weak_classes TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_atom_events (
            id INTEGER PRIMARY KEY,
            plan_id INTEGER NOT NULL,
            atom_id TEXT NOT NULL,
            event_kind TEXT NOT NULL,
            summary TEXT NOT NULL,
            measured_count INTEGER NOT NULL,
            weak_classes TEXT NOT NULL,
            contract_id TEXT,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_assembly_runs (
            id INTEGER PRIMARY KEY,
            plan_id INTEGER NOT NULL,
            run_id TEXT NOT NULL UNIQUE,
            source_atom_ids TEXT NOT NULL,
            target_paths TEXT NOT NULL,
            status TEXT NOT NULL,
            measured_count INTEGER NOT NULL,
            summary TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artifact_readiness (
            id INTEGER PRIMARY KEY,
            plan_id INTEGER NOT NULL UNIQUE,
            status TEXT NOT NULL,
            atom_total INTEGER NOT NULL,
            atom_ready INTEGER NOT NULL,
            atom_missing INTEGER NOT NULL,
            next_atom_id TEXT NOT NULL,
            next_path TEXT NOT NULL,
            active_contract_id TEXT NOT NULL,
            measured_total INTEGER NOT NULL,
            accepted_floor INTEGER NOT NULL,
            assembly_pending TEXT NOT NULL,
            completion_blockers TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

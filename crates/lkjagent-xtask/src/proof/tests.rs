use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

use super::collect::collect;
use super::model::CollectOptions;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn proof_collect_empty_store_writes_warnings() -> TestResult {
    let root = temp_root("empty")?;
    let data = root.join("data");
    fs::create_dir_all(&data)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    lkjagent_store::schema::setup(&conn)?;

    let out = root.join("proof");
    let summary = collect(&CollectOptions {
        data_dir: data,
        out_dir: out.clone(),
    })?;
    let warnings = fs::read_to_string(out.join("warnings.md"))?;

    assert!(summary.exists());
    assert!(warnings.contains("no graph case rows"), "{warnings}");
    cleanup(root);
    Ok(())
}

#[test]
fn proof_collect_seeded_artifact_reports_word_count() -> TestResult {
    let root = temp_root("seeded")?;
    let data = root.join("data");
    seed_store(&data)?;
    let manuscript = data.join("workspace/stories/aurora-ledger/manuscript");
    fs::create_dir_all(&manuscript)?;
    fs::write(
        manuscript.join("chapter-01.md"),
        "# Chapter\n\nOne two three four.",
    )?;
    fs::create_dir_all(data.join("logs"))?;
    fs::write(data.join("logs/index.ndjson"), "{}\n")?;

    let out = root.join("proof");
    collect(&CollectOptions {
        data_dir: data,
        out_dir: out.clone(),
    })?;
    let counts = fs::read_to_string(out.join("word-counts.md"))?;
    let status = fs::read_to_string(out.join("status.md"))?;

    assert!(counts.contains("stories/aurora-ledger"), "{counts}");
    assert!(counts.contains("5"), "{counts}");
    assert!(status.contains("contract-1"), "{status}");
    cleanup(root);
    Ok(())
}

fn seed_store(data: &PathBuf) -> TestResult {
    fs::create_dir_all(data)?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    lkjagent_store::schema::setup(&conn)?;
    conn.execute(
        "INSERT INTO graph_cases
         (objective, family, phase, active_node, status, plan, evidence_requirements,
          selected_packages, pending_checks, next_action_class, created_at, updated_at)
         VALUES ('secret omitted', 'story', 'execute', 'artifact-write', 'open', '', '', '', '',
          'fs.batch_write', 'now', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO artifact_plans
         (case_id, artifact_id, owner_objective, artifact_kind, root, profile,
          normalized_title, measurement_kind, requested_total, accepted_floor, section_count,
          language_hint, forbidden_roots, status, created_at, updated_at)
         VALUES (1, 'artifact-1', 'secret omitted', 'story', 'stories/aurora-ledger',
          'manuscript', 'Aurora Ledger', 'words', 10000, 10000, 10, 'en', '',
          'active', 'now', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO artifact_readiness
         (plan_id, status, atom_total, atom_ready, atom_missing, next_atom_id, next_path,
          active_contract_id, measured_total, accepted_floor, assembly_pending,
          completion_blockers, updated_at)
         VALUES (1, 'blocked', 10, 1, 9, 'atom-2',
          'stories/aurora-ledger/manuscript/chapter-02.md', 'contract-1', 5, 10000,
          '', 'below_floor', 'now')",
        [],
    )?;
    conn.execute(
        "INSERT INTO artifact_write_contracts
         (contract_id, plan_id, atom_ids, exact_paths, max_files, max_file_bytes,
          max_batch_bytes, target_count, count_floor, required_sections, continuity_digest,
          forbidden_weak_classes, status, created_at, updated_at)
         VALUES (?1, 1, 'atom-2', ?2, 1, 1800, 1800, 1000, 900, '', '', '', 'active', 'now', 'now')",
        params!["contract-1", "stories/aurora-ledger/manuscript/chapter-02.md"],
    )?;
    conn.execute(
        "INSERT INTO events (turn, kind, content, tokens, created_at)
         VALUES (1, 'tool', 'not exported', 3, 'now')",
        [],
    )?;
    Ok(())
}

fn temp_root(label: &str) -> Result<PathBuf, std::io::Error> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let path = std::env::temp_dir().join(format!("lkjagent-proof-{label}-{nonce}"));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn cleanup(path: PathBuf) {
    let _ = fs::remove_dir_all(path);
}

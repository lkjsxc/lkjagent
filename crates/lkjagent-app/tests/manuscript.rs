use std::fs;
use std::path::PathBuf;

use lkjagent_app::daemon::{run_until_idle, ScriptedEndpoint};
use lkjagent_core::manuscript::{chapter_plan, extract};
use lkjagent_core::model::TaskState;
use lkjagent_store::plan_access::enqueue;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn manuscript_fake_endpoint_closes_with_ten_chapters_and_word_count() -> TestResult<()> {
    let objective = "Write the Aurora Ledger manuscript at stories/aurora-ledger as 10 chapters totaling 10000 words.";
    let data = fixture_root("manuscript")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    drop(conn);
    let fields = extract(objective);
    let mut outputs = vec![format!("<plan>{}</plan>", chapter_plan(&fields).join("\n"))];
    for index in 1..=10 {
        outputs.push(format!("<content>{}</content>", prose(index, 1_000)));
    }
    outputs.push("<content># Settings\n\nAurora Ledger continuity facts.</content>".to_string());
    outputs
        .push("<message>Manuscript complete: 10 chapters and 10000 words.</message>".to_string());
    let mut endpoint = ScriptedEndpoint { outputs, index: 0 };
    let snapshot = run_until_idle(&data, &mut endpoint, 30)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    let chapter_dir = data.join("workspace/stories/aurora-ledger/manuscript");
    let chapters = fs::read_dir(&chapter_dir)?.filter_map(Result::ok).count();
    assert_eq!(chapters, 10);
    let total = total_words(&chapter_dir)?;
    assert!(total >= 10_000, "{total}");
    assert!(!fs::read_to_string(chapter_dir.join("chapter-01.md"))?.contains("TODO"));
    Ok(())
}

#[test]
fn manuscript_shortfall_extends_and_write_fault_splits() -> TestResult<()> {
    let objective = "Write the Aurora Ledger manuscript at stories/aurora-ledger as 10 chapters totaling 10000 words.";
    let data = fixture_root("manuscript-fault")?;
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    enqueue(&conn, objective, "now")?;
    drop(conn);
    let fields = extract(objective);
    let mut outputs = vec![format!("<plan>{}</plan>", chapter_plan(&fields).join("\n"))];
    outputs.extend(
        [
            "<message>bad</message>",
            "<message>bad</message>",
            "<message>bad</message>",
        ]
        .map(str::to_string),
    );
    outputs.push(format!("<content>{}</content>", prose(1, 900)));
    for index in 2..=10 {
        outputs.push(format!("<content>{}</content>", prose(index, 900)));
    }
    outputs.push("<content># Settings\n\nAurora Ledger continuity facts.</content>".to_string());
    outputs.push(format!("<content>{}</content>", prose(11, 1_000)));
    outputs.push("<message>Manuscript extended after split.</message>".to_string());
    let mut endpoint = ScriptedEndpoint { outputs, index: 0 };
    let snapshot = run_until_idle(&data, &mut endpoint, 40)?;
    assert_eq!(snapshot.task.state, TaskState::Closed);
    let total = total_words(&data.join("workspace/stories/aurora-ledger/manuscript"))?;
    assert!(total >= 10_000, "{total}");
    assert!(snapshot
        .steps
        .iter()
        .any(|step| step.title.contains("extension")));
    assert!(
        snapshot
            .attempts
            .iter()
            .filter(|attempt| attempt.diagnosis.contains("WrongBlock"))
            .count()
            >= 3
    );
    Ok(())
}

fn prose(index: usize, words: usize) -> String {
    (0..words)
        .map(|word| format!("aurora{index}_{word}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn total_words(dir: &std::path::Path) -> TestResult<usize> {
    let mut total = 0;
    for entry in fs::read_dir(dir)?.filter_map(Result::ok) {
        total += fs::read_to_string(entry.path())?.split_whitespace().count();
    }
    Ok(total)
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

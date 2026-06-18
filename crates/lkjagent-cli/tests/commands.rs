mod support;

use std::fs;
use std::io::Cursor;

use lkjagent_cli::console::run_console;
use lkjagent_cli::run_cli;
use lkjagent_store::events::read_events;
use lkjagent_store::memory::{save, MemoryKind};
use support::{open_store, temp_data, valid_skill, write_config, TestResult};

#[test]
fn send_persists_and_status_log_render_store_facts() -> TestResult<()> {
    let data = temp_data("send")?;
    let sent = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "send",
        "hello",
        "agent",
    ]);
    assert_eq!(sent.code, 0);
    assert_eq!(sent.stdout, "queue_id=1");

    let conn = open_store(&data)?;
    let events = read_events(&conn)?;
    assert!(events
        .iter()
        .any(|event| event.content.contains("reason=owner-send")));

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);
    assert!(status.stdout.contains("queue_depth=1"));
    assert!(status.stdout.contains("context_compaction_trigger=28672"));

    let log = run_cli(["--data", data.to_string_lossy().as_ref(), "log"]);
    assert!(log.stdout.contains("kind=queue_mutation"));
    assert!(log.stdout.contains("preview=operation=enqueue"));

    let full = run_cli(["--data", data.to_string_lossy().as_ref(), "log", "--full"]);
    assert!(full.stdout.contains("reason=owner-send"));
    Ok(())
}

#[test]
fn console_renders_status_and_sends_owner_messages() -> TestResult<()> {
    let data = temp_data("console")?;
    let input = b"hello from console\n/quit\n";
    let reader = Cursor::new(input.to_vec());
    let mut output = Vec::new();

    run_console(&data, reader, &mut output)?;
    let text = String::from_utf8(output)?;
    assert!(text.contains("lkjagent console"));
    assert!(text.contains("queued id=1"));
    assert!(text.contains("console closed"));

    let conn = open_store(&data)?;
    let rows = lkjagent_store::queue::list(&conn)?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].content, "hello from console");
    let events = read_events(&conn)?;
    assert!(events
        .iter()
        .any(|event| event.content.contains("reason=console-send")));
    Ok(())
}

#[test]
fn console_replaces_invalid_utf8_input_instead_of_failing() -> TestResult<()> {
    let data = temp_data("console-utf8")?;
    let input = b"bad\xffline\n/quit\n";
    let reader = Cursor::new(input.to_vec());
    let mut output = Vec::new();

    run_console(&data, reader, &mut output)?;
    let text = String::from_utf8(output)?;
    assert!(text.contains("queued id=1"));

    let conn = open_store(&data)?;
    let rows = lkjagent_store::queue::list(&conn)?;
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].content,
        format!("bad{}line", char::REPLACEMENT_CHARACTER)
    );
    Ok(())
}

#[test]
fn console_uses_answer_prompt_when_daemon_waits() -> TestResult<()> {
    let data = temp_data("console-waiting")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "daemon state", "waiting")?;
    lkjagent_store::state::set(&conn, "daemon question", "Need an answer?")?;
    let input = b"/quit\n";
    let reader = Cursor::new(input.to_vec());
    let mut output = Vec::new();

    run_console(&data, reader, &mut output)?;
    let text = String::from_utf8(output)?;
    assert!(text.contains("WAITING"));
    assert!(text.contains("question Need an answer?"));
    assert!(text.contains("answer>"));
    Ok(())
}

#[test]
fn memory_and_skills_commands_read_existing_store_and_library() -> TestResult<()> {
    let data = temp_data("read")?;
    let mut conn = open_store(&data)?;
    save(
        &mut conn,
        MemoryKind::Fact,
        "runtime cli",
        "cli",
        "runtime cli fact",
        4,
        "2026-01-01T00:00:00Z",
    )?;
    let memory = run_cli([
        "--data",
        data.to_string_lossy().as_ref(),
        "memory",
        "runtime",
    ]);
    assert!(memory.stdout.contains("kind=fact"));
    assert!(memory.stdout.contains("title=runtime cli"));

    fs::create_dir_all(data.join("skills"))?;
    fs::write(data.join("skills/demo-skill.md"), valid_skill())?;
    let skills = run_cli(["--data", data.to_string_lossy().as_ref(), "skills"]);
    assert!(skills.stdout.contains("name=demo-skill"));
    assert!(skills
        .stdout
        .contains("trigger=A CLI test needs a listed skill."));
    Ok(())
}

#[test]
fn run_writes_first_config_and_refuses_existing_lock() -> TestResult<()> {
    let first = temp_data("first")?;
    let first_start = run_cli(["--data", first.to_string_lossy().as_ref(), "run"]);
    assert_eq!(first_start.code, 1);
    assert!(first_start.stderr.contains("config_written="));
    assert!(first_start.stderr.contains("missing=endpoint.model"));
    assert!(first.join("lkjagent.json").exists());
    assert!(first.join("workspace").is_dir());

    let locked = temp_data("locked")?;
    write_config(&locked)?;
    let conn = open_store(&locked)?;
    lkjagent_store::state::take_lock(&conn, "other", "9999999999", "0")?;
    let refused = run_cli(["--data", locked.to_string_lossy().as_ref(), "run"]);
    assert_eq!(refused.code, 1);
    assert!(refused.stderr.contains("daemon_refused=other"));
    Ok(())
}

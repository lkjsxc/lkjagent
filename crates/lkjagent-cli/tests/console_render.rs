mod support;

use lkjagent_cli::console::render_snapshot;
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::token_usage::{record, TokenUsageEvent};
use support::{open_store, temp_data, TestResult};

#[test]
fn console_pads_sparse_transcript_above_bottom_deck() -> TestResult<()> {
    let data = temp_data("console-sparse")?;

    let screen = render_snapshot(&data, "ready", 56, 16)?;
    let lines = screen.lines().collect::<Vec<_>>();
    let rule = bottom_rule_index(&lines);

    assert_eq!(lines.len(), 15);
    assert_eq!(rule, 7);
    assert!(lines[2..rule]
        .iter()
        .all(|line| strip_ansi(line).is_empty()));
    assert!(strip_ansi(lines[14]).contains("/refresh"));
    Ok(())
}

#[test]
fn console_puts_operational_state_in_bottom_deck() -> TestResult<()> {
    let data = temp_data("console-bottom")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "daemon state", "waiting")?;
    lkjagent_store::state::set(&conn, "daemon question", "Need owner guidance?")?;
    lkjagent_store::state::set(&conn, "open task", "write docs")?;
    lkjagent_store::state::set(&conn, "context used tokens", "1234")?;
    record(
        &conn,
        &TokenUsageEvent {
            task_id: None,
            turn: 1,
            input_tokens: Some(8_120),
            output_tokens: Some(1_040),
            cached_input_tokens: Some(6_880),
            total_tokens: Some(9_160),
            context_window: Some(24_576),
            context_used_estimate: Some(1_234),
            source: "endpoint".to_string(),
        },
        "2026-06-20T00:00:00Z",
    )?;

    let screen = render_snapshot(&data, "ready", 56, 16)?;
    let lines = screen.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 15);
    let state_line = lines
        .iter()
        .position(|line| strip_ansi(line).contains("state WAITING"))
        .unwrap_or(0);
    assert!(state_line >= lines.len().saturating_sub(10));
    assert!(screen.contains("ctx=1.23K/24.58K 5.02%"));
    assert!(screen.contains("in=8.12K out=1.04K cache=6.88K total=9.16K"));
    assert!(screen.contains("gpt_log"));
    assert!(screen.contains("question Need owner guidance?"));
    Ok(())
}

#[test]
fn console_uses_one_less_body_line_than_terminal_rows() -> TestResult<()> {
    let data = temp_data("console-body-lines")?;

    let screen = render_snapshot(&data, "ready", 80, 24)?;

    assert_eq!(screen.lines().count(), 23);
    Ok(())
}

#[test]
fn console_keeps_bottom_deck_visible_on_minimum_screen() -> TestResult<()> {
    let data = temp_data("console-narrow")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "daemon state", "waiting")?;

    let screen = render_snapshot(&data, "ready", 40, 12)?;
    let lines = screen.lines().collect::<Vec<_>>();
    let rule = bottom_rule_index(&lines);

    assert_eq!(lines.len(), 11);
    assert_eq!(rule, 2);
    assert!(screen.contains("WAITING"));
    for line in lines {
        assert!(visible_width(&strip_ansi(line)) <= 40);
    }
    Ok(())
}

#[test]
fn console_wraps_wide_text_to_terminal_width() -> TestResult<()> {
    let data = temp_data("console-wide")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "daemon state", "working")?;
    lkjagent_store::state::set(&conn, "open task", "日本語ドキュメント整備")?;
    append_event(
        &conn,
        Some(7),
        EventKind::Observation,
        "<observation>
<status>ok</status>
<content>日本語の長い文章を含む進捗報告を折り返して表示する</content>
</observation>",
        12,
        "2026-06-19T00:00:00Z",
    )?;

    let screen = render_snapshot(&data, "日本語の通知も崩さない", 44, 14)?;
    assert_eq!(screen.lines().count(), 13);
    for line in screen.lines() {
        assert!(
            visible_width(&strip_ansi(line)) <= 44,
            "line exceeded width: {line}"
        );
    }
    assert!(screen.contains("WORKING"));
    Ok(())
}

fn bottom_rule_index(lines: &[&str]) -> usize {
    lines
        .iter()
        .position(|line| strip_ansi(line).starts_with("---"))
        .unwrap_or(usize::MAX)
}

fn strip_ansi(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            for next in chars.by_ref() {
                if next == 'm' {
                    break;
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn visible_width(text: &str) -> usize {
    text.chars().map(char_width).sum()
}

fn char_width(ch: char) -> usize {
    let code = ch as u32;
    if ch.is_control() {
        0
    } else if matches!(
        code,
        0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 | 0xFF00..=0xFF60
    ) {
        2
    } else {
        1
    }
}

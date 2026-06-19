mod support;

use lkjagent_cli::console::render_snapshot;
use lkjagent_store::events::{append_event, EventKind};
use support::{open_store, temp_data, TestResult};

#[test]
fn console_puts_operational_state_in_bottom_deck() -> TestResult<()> {
    let data = temp_data("console-bottom")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "daemon state", "waiting")?;
    lkjagent_store::state::set(&conn, "daemon question", "Need owner guidance?")?;
    lkjagent_store::state::set(&conn, "open task", "write docs")?;

    let screen = render_snapshot(&data, "ready", 56, 16)?;
    let lines = screen.lines().collect::<Vec<_>>();
    let state_line = lines
        .iter()
        .position(|line| strip_ansi(line).contains("state WAITING"))
        .unwrap_or(0);
    assert!(state_line >= lines.len().saturating_sub(6));
    assert!(screen.contains("question Need owner guidance?"));
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
    assert!(screen.lines().count() <= 13);
    for line in screen.lines() {
        assert!(
            visible_width(&strip_ansi(line)) <= 44,
            "line exceeded width: {line}"
        );
    }
    assert!(screen.contains("WORKING"));
    Ok(())
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

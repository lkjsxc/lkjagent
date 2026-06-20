use std::fs::File;
use std::process::{Command, Stdio};

const DEFAULT_COLUMNS: usize = 80;
const DEFAULT_ROWS: usize = 24;
const MIN_COLUMNS: usize = 40;
const MIN_ROWS: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenSize {
    pub columns: usize,
    pub rows: usize,
}

impl ScreenSize {
    pub fn current() -> Self {
        let terminal = terminal_size_text();
        let columns = std::env::var("COLUMNS").ok();
        let rows = std::env::var("LINES").ok();
        size_from_sources(terminal.as_deref(), columns.as_deref(), rows.as_deref())
    }

    pub fn clamp(self) -> Self {
        Self {
            columns: self.columns.max(MIN_COLUMNS),
            rows: self.rows.max(MIN_ROWS),
        }
    }
}

fn terminal_size_text() -> Option<String> {
    let output = Command::new("stty")
        .arg("size")
        .stdin(Stdio::from(File::open("/dev/tty").ok()?))
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).to_string())
}

fn size_from_sources(
    terminal: Option<&str>,
    columns: Option<&str>,
    rows: Option<&str>,
) -> ScreenSize {
    terminal
        .and_then(parse_stty_size)
        .unwrap_or_else(|| env_size(columns, rows))
}

fn parse_stty_size(text: &str) -> Option<ScreenSize> {
    let mut parts = text.split_whitespace();
    let rows = parse_usize(parts.next())?;
    let columns = parse_usize(parts.next())?;
    if parts.next().is_some() {
        return None;
    }
    Some(ScreenSize { columns, rows })
}

fn env_size(columns: Option<&str>, rows: Option<&str>) -> ScreenSize {
    ScreenSize {
        columns: parse_usize(columns).unwrap_or(DEFAULT_COLUMNS),
        rows: parse_usize(rows).unwrap_or(DEFAULT_ROWS),
    }
}

fn parse_usize(value: Option<&str>) -> Option<usize> {
    value.and_then(|text| text.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::{size_from_sources, ScreenSize, DEFAULT_ROWS};

    #[test]
    fn parses_stty_size_as_rows_then_columns() {
        assert_eq!(
            size_from_sources(Some("33 101\n"), Some("80"), Some("24")),
            ScreenSize {
                columns: 101,
                rows: 33
            }
        );
    }

    #[test]
    fn invalid_terminal_size_falls_back_to_env_then_defaults() {
        assert_eq!(
            size_from_sources(Some("not a size"), Some("132"), Some("43")),
            ScreenSize {
                columns: 132,
                rows: 43
            }
        );
        assert_eq!(
            size_from_sources(Some("1 2 3"), Some("100"), Some("bad")),
            ScreenSize {
                columns: 100,
                rows: DEFAULT_ROWS
            }
        );
    }
}

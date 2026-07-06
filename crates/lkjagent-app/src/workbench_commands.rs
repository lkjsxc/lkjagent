use crate::workbench_state::{reduce, UiEvent, UiState, WorkbenchMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkbenchCommand {
    Mode(WorkbenchMode),
    Scroll(isize),
    Top,
}

pub fn apply(state: &mut UiState, command: WorkbenchCommand) -> String {
    match command {
        WorkbenchCommand::Mode(mode) => {
            *state = reduce(state.clone(), UiEvent::Mode(mode));
            format!("workbench: mode={}", mode.as_str())
        }
        WorkbenchCommand::Scroll(delta) => {
            *state = reduce(state.clone(), UiEvent::Scroll(delta));
            format!("workbench: scroll={}", state.scroll)
        }
        WorkbenchCommand::Top => {
            *state = reduce(state.clone(), UiEvent::Top);
            "workbench: scroll=0".to_string()
        }
    }
}

pub fn parse(line: &str) -> Result<Option<WorkbenchCommand>, String> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("/mode") {
        return mode(rest.trim()).map(Some);
    }
    if let Some(rest) = trimmed.strip_prefix("/scroll") {
        return scroll(rest.trim()).map(Some);
    }
    if let Some(rest) = trimmed.strip_prefix("/page") {
        return page(rest.trim()).map(Some);
    }
    Ok(None)
}

fn mode(value: &str) -> Result<WorkbenchCommand, String> {
    if value.is_empty() {
        return Err("/mode requires append or pane".to_string());
    }
    WorkbenchMode::parse(value).map(WorkbenchCommand::Mode)
}

fn scroll(value: &str) -> Result<WorkbenchCommand, String> {
    match value {
        "up" => Ok(WorkbenchCommand::Scroll(-1)),
        "down" => Ok(WorkbenchCommand::Scroll(1)),
        "top" => Ok(WorkbenchCommand::Top),
        _ => Err("/scroll requires up, down, or top".to_string()),
    }
}

fn page(value: &str) -> Result<WorkbenchCommand, String> {
    match value {
        "up" => Ok(WorkbenchCommand::Scroll(-10)),
        "down" => Ok(WorkbenchCommand::Scroll(10)),
        _ => Err("/page requires up or down".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mode_and_scroll_commands() {
        assert_eq!(
            parse("/mode pane"),
            Ok(Some(WorkbenchCommand::Mode(WorkbenchMode::Pane)))
        );
        assert_eq!(parse("/scroll down"), Ok(Some(WorkbenchCommand::Scroll(1))));
        assert_eq!(parse("/page up"), Ok(Some(WorkbenchCommand::Scroll(-10))));
        assert_eq!(parse("hello"), Ok(None));
    }
}

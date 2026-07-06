#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkbenchMode {
    Append,
    Pane,
}

impl WorkbenchMode {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "append" => Ok(Self::Append),
            "pane" => Ok(Self::Pane),
            other => Err(format!("unknown workbench mode: {other}")),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Append => "append",
            Self::Pane => "pane",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiState {
    pub mode: WorkbenchMode,
    pub refreshes: u64,
    pub scroll: usize,
    pub width: u16,
    pub height: u16,
    pub latest: String,
}

impl UiState {
    pub fn new(mode: WorkbenchMode) -> Self {
        Self {
            mode,
            refreshes: 0,
            scroll: 0,
            width: 100,
            height: 30,
            latest: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent {
    Refresh(String),
    Mode(WorkbenchMode),
    Scroll(isize),
    Resize { width: u16, height: u16 },
}

pub fn reduce(mut state: UiState, event: UiEvent) -> UiState {
    match event {
        UiEvent::Refresh(body) => {
            state.latest = body;
            state.refreshes = state.refreshes.saturating_add(1);
        }
        UiEvent::Mode(mode) => {
            state.mode = mode;
        }
        UiEvent::Scroll(delta) => {
            state.scroll = scroll(state.scroll, delta);
        }
        UiEvent::Resize { width, height } => {
            state.width = width.max(40);
            state.height = height.max(10);
        }
    }
    state
}

fn scroll(current: usize, delta: isize) -> usize {
    if delta.is_negative() {
        current.saturating_sub(delta.unsigned_abs())
    } else {
        current.saturating_add(delta as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reducer_tracks_mode_refresh_and_scroll() {
        let state = UiState::new(WorkbenchMode::Append);
        let state = reduce(state, UiEvent::Refresh("body".to_string()));
        let state = reduce(state, UiEvent::Mode(WorkbenchMode::Pane));
        let state = reduce(state, UiEvent::Scroll(4));
        let state = reduce(state, UiEvent::Scroll(-1));

        assert_eq!(state.mode, WorkbenchMode::Pane);
        assert_eq!(state.refreshes, 1);
        assert_eq!(state.scroll, 3);
        assert_eq!(state.latest, "body");
    }
}

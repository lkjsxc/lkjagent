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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Viewport {
    Follow,
    Manual { top_line: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiState {
    pub mode: WorkbenchMode,
    pub refreshes: u64,
    pub scroll: usize,
    pub follow: bool,
    pub viewport: Viewport,
    pub width: u16,
    pub height: u16,
    pub latest: String,
    pub search: String,
}

impl UiState {
    pub fn new(mode: WorkbenchMode) -> Self {
        Self {
            mode,
            refreshes: 0,
            scroll: 0,
            follow: true,
            viewport: Viewport::Follow,
            width: 100,
            height: 30,
            latest: String::new(),
            search: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent {
    Refresh(String),
    Mode(WorkbenchMode),
    Scroll(isize),
    Top,
    Follow(bool),
    Search(String),
    Resize { width: u16, height: u16 },
}

pub fn reduce(mut state: UiState, event: UiEvent) -> UiState {
    match event {
        UiEvent::Refresh(body) => {
            state.latest = body;
            state.refreshes = state.refreshes.saturating_add(1);
            normalize_viewport(&mut state);
        }
        UiEvent::Mode(mode) => state.mode = mode,
        UiEvent::Scroll(delta) => scroll_viewport(&mut state, delta),
        UiEvent::Top => set_viewport(&mut state, Viewport::Manual { top_line: 0 }),
        UiEvent::Follow(enabled) => {
            let viewport = if enabled {
                Viewport::Follow
            } else {
                Viewport::Manual {
                    top_line: state.scroll,
                }
            };
            set_viewport(&mut state, viewport);
        }
        UiEvent::Search(query) => {
            state.search = query;
            let top_line = state.scroll;
            set_viewport(&mut state, Viewport::Manual { top_line });
        }
        UiEvent::Resize { width, height } => {
            state.width = width.max(40);
            state.height = height.max(10);
            normalize_viewport(&mut state);
        }
    }
    state
}

pub fn visible_height(state: &UiState) -> usize {
    (state.height.saturating_sub(12) as usize).max(1)
}

fn scroll_viewport(state: &mut UiState, delta: isize) {
    let max_top = max_top_line(state);
    let current = match state.viewport {
        Viewport::Follow => max_top,
        Viewport::Manual { top_line } => top_line.min(max_top),
    };
    let next = if delta.is_negative() {
        current.saturating_sub(delta.unsigned_abs())
    } else {
        current.saturating_add(delta as usize)
    };
    if !delta.is_negative() && next >= max_top {
        set_viewport(state, Viewport::Follow);
    } else {
        set_viewport(
            state,
            Viewport::Manual {
                top_line: next.min(max_top),
            },
        );
    }
}

fn normalize_viewport(state: &mut UiState) {
    let max_top = max_top_line(state);
    let viewport = match state.viewport {
        Viewport::Follow => Viewport::Follow,
        Viewport::Manual { top_line } => Viewport::Manual {
            top_line: top_line.min(max_top),
        },
    };
    set_viewport(state, viewport);
}

fn set_viewport(state: &mut UiState, viewport: Viewport) {
    state.viewport = viewport;
    match viewport {
        Viewport::Follow => {
            state.follow = true;
            state.scroll = 0;
        }
        Viewport::Manual { top_line } => {
            state.follow = false;
            state.scroll = top_line;
        }
    }
}

fn max_top_line(state: &UiState) -> usize {
    state
        .latest
        .lines()
        .count()
        .saturating_sub(visible_height(state))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiEvent {
    UserInputChanged(String),
    UserInsertChar(char),
    UserComposerNewline,
    UserBackspace,
    UserMoveComposer(isize),
    UserSubmit,
    UserInterrupt,
    UserApproveTool,
    UserRejectTool(String),
    UserOpenPalette,
    UserCloseModal,
    UserSearchChanged(String),
    UserScroll(isize),
    UserFollow(bool),
    UserSelectPane(TuiPane),
    AgentTextDelta(String),
    AgentMessageComplete,
    ToolCallProposed { name: String, decision_id: String },
    ToolCallStarted(String),
    ToolCallFinished(String),
    StateTransitionObserved(String),
    ArtifactCreated(String),
    WorkspaceChanged(String),
    TimerTick,
    TerminalResize { width: u16, height: u16 },
    ErrorObserved(String),
    SaveTranscript,
    QuitRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiPane {
    Transcript,
    Tasks,
    Tools,
    StateGraph,
    Workspace,
    Artifacts,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiRunState {
    Idle,
    Running,
    ToolPending,
    ToolRunning,
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptSource {
    Owner,
    Agent,
    Tool,
    State,
    System,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptEntry {
    pub id: String,
    pub source: TranscriptSource,
    pub text: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCard {
    pub name: String,
    pub decision_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiModel {
    pub composer: String,
    pub composer_cursor: usize,
    pub transcript: Vec<TranscriptEntry>,
    pub agent_draft: Option<TranscriptEntry>,
    pub active_pane: TuiPane,
    pub palette_open: bool,
    pub follow: bool,
    pub scroll: usize,
    pub run_state: TuiRunState,
    pub width: u16,
    pub height: u16,
    pub pending_tool: Option<ToolCard>,
    pub last_error: Option<String>,
    pub search: String,
    pub next_entry_seq: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiEffect {
    SubmitOwnerMessage(String),
    InterruptRun,
    ApproveTool(ToolCard),
    RejectTool { card: ToolCard, reason: String },
    SaveTranscript,
    Quit,
    Redraw,
}

impl TuiModel {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            composer: String::new(),
            composer_cursor: 0,
            transcript: Vec::new(),
            agent_draft: None,
            active_pane: TuiPane::Transcript,
            palette_open: false,
            follow: true,
            scroll: 0,
            run_state: TuiRunState::Idle,
            width: 100,
            height: 30,
            pending_tool: None,
            last_error: None,
            search: String::new(),
            next_entry_seq: 1,
        }
    }
}

pub fn source_label(source: TranscriptSource) -> &'static str {
    match source {
        TranscriptSource::Owner => "owner",
        TranscriptSource::Agent => "agent",
        TranscriptSource::Tool => "tool",
        TranscriptSource::State => "state",
        TranscriptSource::System => "system",
        TranscriptSource::Error => "error",
    }
}

pub fn run_state_label(state: TuiRunState) -> &'static str {
    match state {
        TuiRunState::Idle => "idle",
        TuiRunState::Running => "running",
        TuiRunState::ToolPending => "tool-pending",
        TuiRunState::ToolRunning => "tool-running",
        TuiRunState::Interrupted => "interrupted",
    }
}

pub fn pane_label(pane: TuiPane) -> &'static str {
    match pane {
        TuiPane::Transcript => "transcript",
        TuiPane::Tasks => "matters",
        TuiPane::Tools => "tools",
        TuiPane::StateGraph => "state-graph",
        TuiPane::Workspace => "workspace",
        TuiPane::Artifacts => "artifacts",
        TuiPane::Help => "help",
    }
}

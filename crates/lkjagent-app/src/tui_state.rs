#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiEvent {
    UserInputChanged(String),
    UserSubmit,
    UserInterrupt,
    UserApproveTool,
    UserRejectTool(String),
    UserOpenPalette,
    UserCloseModal,
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
    pub source: TranscriptSource,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCard {
    pub name: String,
    pub decision_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiModel {
    pub composer: String,
    pub transcript: Vec<TranscriptEntry>,
    pub active_pane: TuiPane,
    pub palette_open: bool,
    pub follow: bool,
    pub scroll: usize,
    pub run_state: TuiRunState,
    pub width: u16,
    pub height: u16,
    pub pending_tool: Option<ToolCard>,
    pub last_error: Option<String>,
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
            transcript: Vec::new(),
            active_pane: TuiPane::Transcript,
            palette_open: false,
            follow: true,
            scroll: 0,
            run_state: TuiRunState::Idle,
            width: 100,
            height: 30,
            pending_tool: None,
            last_error: None,
        }
    }
}

pub fn reduce(mut model: TuiModel, event: TuiEvent) -> (TuiModel, Vec<TuiEffect>) {
    let mut effects = vec![TuiEffect::Redraw];
    match event {
        TuiEvent::UserInputChanged(text) => model.composer = text,
        TuiEvent::UserSubmit => submit_composer(&mut model, &mut effects),
        TuiEvent::UserInterrupt => {
            model.run_state = TuiRunState::Interrupted;
            push(&mut model, TranscriptSource::System, "interrupt requested");
            effects.push(TuiEffect::InterruptRun);
        }
        TuiEvent::UserApproveTool => approve_tool(&mut model, &mut effects),
        TuiEvent::UserRejectTool(reason) => reject_tool(&mut model, &mut effects, reason),
        TuiEvent::UserOpenPalette => model.palette_open = true,
        TuiEvent::UserCloseModal => model.palette_open = false,
        TuiEvent::AgentTextDelta(text) => {
            model.run_state = TuiRunState::Running;
            push(&mut model, TranscriptSource::Agent, text);
        }
        TuiEvent::AgentMessageComplete => model.run_state = TuiRunState::Idle,
        TuiEvent::ToolCallProposed { name, decision_id } => {
            model.run_state = TuiRunState::ToolPending;
            model.pending_tool = Some(ToolCard { name, decision_id });
        }
        TuiEvent::ToolCallStarted(name) => {
            model.run_state = TuiRunState::ToolRunning;
            push(
                &mut model,
                TranscriptSource::Tool,
                format!("started {name}"),
            );
        }
        TuiEvent::ToolCallFinished(summary) => {
            model.run_state = TuiRunState::Running;
            push(&mut model, TranscriptSource::Tool, summary);
        }
        TuiEvent::StateTransitionObserved(text) => push(&mut model, TranscriptSource::State, text),
        TuiEvent::ArtifactCreated(text) => push(&mut model, TranscriptSource::System, text),
        TuiEvent::WorkspaceChanged(text) => push(&mut model, TranscriptSource::System, text),
        TuiEvent::TimerTick => {}
        TuiEvent::TerminalResize { width, height } => {
            model.width = width.max(40);
            model.height = height.max(10);
        }
        TuiEvent::ErrorObserved(text) => {
            model.last_error = Some(text.clone());
            push(&mut model, TranscriptSource::Error, text);
        }
        TuiEvent::SaveTranscript => effects.push(TuiEffect::SaveTranscript),
        TuiEvent::QuitRequested => effects.push(TuiEffect::Quit),
    }
    (model, effects)
}

fn submit_composer(model: &mut TuiModel, effects: &mut Vec<TuiEffect>) {
    let text = model.composer.trim().to_string();
    if text.is_empty() {
        return;
    }
    model.composer.clear();
    push(model, TranscriptSource::Owner, text.clone());
    effects.push(TuiEffect::SubmitOwnerMessage(text));
}

fn approve_tool(model: &mut TuiModel, effects: &mut Vec<TuiEffect>) {
    if let Some(card) = model.pending_tool.take() {
        model.run_state = TuiRunState::ToolRunning;
        effects.push(TuiEffect::ApproveTool(card));
    }
}

fn reject_tool(model: &mut TuiModel, effects: &mut Vec<TuiEffect>, reason: String) {
    if let Some(card) = model.pending_tool.take() {
        model.run_state = TuiRunState::Running;
        effects.push(TuiEffect::RejectTool { card, reason });
    }
}

fn push(model: &mut TuiModel, source: TranscriptSource, text: impl Into<String>) {
    model.transcript.push(TranscriptEntry {
        source,
        text: text.into(),
    });
    if model.follow {
        model.scroll = 0;
    }
}

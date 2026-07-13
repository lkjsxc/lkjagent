use lkjagent_store::tui_snapshot::TuiSnapshot;

use crate::tui_screen::ScreenModel;
use crate::tui_viewport::Viewport;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComposerState {
    pub text: String,
    pub cursor: usize,
    pub revision: u64,
    pub pending: Option<PendingSubmission>,
    pub last_error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingSubmission {
    pub message_id: String,
    pub body: String,
    pub revision: u64,
    pub durable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposerEvent {
    Replace(String),
    Insert(String),
    Paste(String),
    Backspace,
    Delete,
    MoveLeft,
    MoveRight,
    Home,
    End,
    Submit {
        message_id: String,
    },
    SubmitCommitted {
        request_id: String,
        message_id: String,
    },
    SubmitSucceeded {
        message_id: String,
    },
    SubmitFailed {
        message_id: String,
        error: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TuiEffect {
    CommitOwnerMessage { message_id: String, body: String },
}

#[derive(Clone, PartialEq, Eq)]
pub enum TuiEvent {
    Composer(ComposerEvent),
    Snapshot(TuiSnapshot),
    Resize { width: usize, height: usize },
    Search(String),
    SearchMode(bool),
    Scroll(isize),
    ActivityExpanded(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TuiModel {
    pub composer: ComposerState,
    pub screen: ScreenModel,
    pub viewport: Viewport,
    pub search: String,
    pub search_active: bool,
    pub width: usize,
    pub height: usize,
}

impl TuiModel {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            composer: ComposerState::default(),
            screen: ScreenModel::default(),
            viewport: Viewport::default(),
            search: String::new(),
            search_active: false,
            width,

            height,
        }
    }
}

use crate::tui_types::{
    ToolCard, TranscriptEntry, TranscriptSource, TuiEffect, TuiEvent, TuiModel, TuiRunState,
};

pub fn reduce(mut model: TuiModel, event: TuiEvent) -> (TuiModel, Vec<TuiEffect>) {
    let mut effects = vec![TuiEffect::Redraw];
    match event {
        TuiEvent::UserInputChanged(text) => model.composer = text,
        TuiEvent::UserComposerNewline => model.composer.push('\n'),
        TuiEvent::UserSubmit => submit_composer(&mut model, &mut effects),
        TuiEvent::UserInterrupt => interrupt(&mut model, &mut effects),
        TuiEvent::UserApproveTool => approve_tool(&mut model, &mut effects),
        TuiEvent::UserRejectTool(reason) => reject_tool(&mut model, &mut effects, reason),
        TuiEvent::UserOpenPalette => model.palette_open = true,
        TuiEvent::UserCloseModal => model.palette_open = false,
        TuiEvent::UserSearchChanged(query) => {
            model.search = query;
            model.follow = false;
        }
        TuiEvent::UserScroll(delta) => {
            model.follow = false;
            model.scroll = scroll(model.scroll, delta);
        }
        TuiEvent::UserFollow(enabled) => {
            model.follow = enabled;
            if enabled {
                model.scroll = 0;
            }
        }
        TuiEvent::UserSelectPane(pane) => model.active_pane = pane,
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
    model.palette_open = false;
    push(model, TranscriptSource::Owner, text.clone());
    effects.push(TuiEffect::SubmitOwnerMessage(text));
}

fn interrupt(model: &mut TuiModel, effects: &mut Vec<TuiEffect>) {
    model.run_state = TuiRunState::Interrupted;
    push(model, TranscriptSource::System, "interrupt requested");
    effects.push(TuiEffect::InterruptRun);
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

fn scroll(current: usize, delta: isize) -> usize {
    if delta.is_negative() {
        current.saturating_sub(delta.unsigned_abs())
    } else {
        current.saturating_add(delta as usize)
    }
}

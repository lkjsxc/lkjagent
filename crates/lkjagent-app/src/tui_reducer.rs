use crate::tui_composer;
use crate::tui_model::{ComposerEvent, TuiEffect, TuiEvent, TuiModel};
use crate::tui_screen::ScreenModel;
use crate::tui_viewport;

pub fn reduce(mut model: TuiModel, event: TuiEvent) -> (TuiModel, Vec<TuiEffect>) {
    let mut effects = Vec::new();
    match event {
        TuiEvent::Composer(event) => {
            let (composer, composer_effects) = tui_composer::reduce(model.composer, event);
            model.composer = composer;
            effects = composer_effects;
            refresh_draft(&mut model);
        }
        TuiEvent::Snapshot(snapshot) => {
            acknowledge_snapshot(&mut model, &snapshot);
            let expanded = model.screen.activity.expanded;
            model.screen = ScreenModel::project(&snapshot, model.composer.pending.as_ref());
            model.screen.activity.expanded = expanded;
            reconcile(&mut model);
        }
        TuiEvent::Resize { width, height } => {
            model.width = width.max(1);
            model.height = height.max(1);
            reconcile(&mut model);
        }
        TuiEvent::Search(search) => model.search = search,
        TuiEvent::Scroll(delta) => {
            let rows = model.screen.rows(model.width, &model.search);
            tui_viewport::scroll(&mut model.viewport, &rows, model.height, delta);
        }
        TuiEvent::ActivityExpanded(expanded) => model.screen.activity.expanded = expanded,
    }
    (model, effects)
}

fn acknowledge_snapshot(
    model: &mut TuiModel,
    snapshot: &lkjagent_store::tui_snapshot::TuiSnapshot,
) {
    let Some(message_id) = model
        .composer
        .pending
        .as_ref()
        .map(|pending| pending.message_id.clone())
    else {
        return;
    };
    if snapshot.conversation.iter().any(|row| row.id == message_id) {
        let (mut composer, _) = tui_composer::reduce(
            std::mem::take(&mut model.composer),
            ComposerEvent::SubmitSucceeded {
                message_id: message_id.clone(),
            },
        );
        composer.pending = None;
        model.composer = composer;
    }
}

fn refresh_draft(model: &mut TuiModel) {
    if let Some(draft) = model.composer.pending.as_ref() {
        if let Some(item) = model
            .screen
            .conversation
            .iter_mut()
            .find(|item| item.id == draft.message_id && !item.durable)
        {
            item.body.clone_from(&draft.body);
            return;
        }
        if model
            .screen
            .conversation
            .iter()
            .all(|item| item.id != draft.message_id)
        {
            model
                .screen
                .conversation
                .push(crate::tui_screen::ConversationItem {
                    id: draft.message_id.clone(),
                    sequence: None,
                    role: "owner".to_string(),
                    body: draft.body.clone(),
                    lifecycle: "pending".to_string(),
                    durable: false,
                });
        }
    } else {
        model.screen.conversation.retain(|item| item.durable);
    }
    reconcile(model);
}

fn reconcile(model: &mut TuiModel) {
    let rows = model.screen.rows(model.width, "");
    tui_viewport::reconcile(&mut model.viewport, &rows, model.height);
}

use lkjagent_store::tui_snapshot::{StatusCounts, TuiSnapshot};

use crate::tui_model::PendingSubmission;
use crate::tui_viewport::ViewRow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConversationItem {
    pub id: String,
    pub sequence: Option<i64>,
    pub role: String,
    pub body: String,
    pub lifecycle: String,
    pub durable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityItem {
    pub id: String,
    pub kind: String,
    pub matter_id: String,
    pub status: String,
    pub monotonic_ms: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActivityPanel {
    pub expanded: bool,
    pub items: Vec<ActivityItem>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScreenModel {
    pub conversation: Vec<ConversationItem>,
    pub activity: ActivityPanel,
    pub status: StatusCounts,
}

impl ScreenModel {
    pub fn project(snapshot: &TuiSnapshot, pending: Option<&PendingSubmission>) -> Self {
        let mut screen = Self {
            activity: ActivityPanel {
                expanded: false,
                items: Vec::new(),
            },
            ..Self::default()
        };
        screen.merge(snapshot, pending);
        screen
    }

    pub fn merge(&mut self, snapshot: &TuiSnapshot, pending: Option<&PendingSubmission>) {
        let expanded = self.activity.expanded;
        for row in snapshot
            .conversation
            .iter()
            .filter(|row| row.lifecycle != "active")
        {
            self.conversation.retain(|item| item.id != row.id);
        }
        let mut conversation = snapshot
            .conversation
            .iter()
            .filter(|row| row.lifecycle == "active")
            .map(|row| ConversationItem {
                id: row.id.clone(),
                sequence: Some(row.sequence),
                role: row.role.clone(),
                body: body(row.body.as_slice(), row.body_truncated),
                lifecycle: row.lifecycle.clone(),
                durable: true,
            })
            .collect::<Vec<_>>();
        for item in conversation.drain(..) {
            if let Some(old) = self.conversation.iter_mut().find(|old| old.id == item.id) {
                *old = item;
            } else {
                self.conversation.push(item);
            }
        }
        self.conversation.sort_by(|left, right| {
            left.sequence
                .unwrap_or(i64::MAX)
                .cmp(&right.sequence.unwrap_or(i64::MAX))
                .then_with(|| left.id.cmp(&right.id))
        });
        if let Some(draft) = pending {
            if !self
                .conversation
                .iter()
                .any(|item| item.id == draft.message_id)
            {
                self.conversation.push(ConversationItem {
                    id: draft.message_id.clone(),
                    sequence: None,
                    role: "owner".to_string(),
                    body: draft.body.clone(),
                    lifecycle: "pending".to_string(),
                    durable: false,
                });
            }
        }
        self.activity = ActivityPanel {
            expanded,
            items: snapshot.activity.iter().map(activity_item).collect(),
        };
        self.status = snapshot.status.clone();
    }

    pub fn rows(&self, width: usize, search: &str) -> Vec<ViewRow> {
        let query = search.to_lowercase();
        self.conversation
            .iter()
            .filter(|item| query.is_empty() || item.body.to_lowercase().contains(&query))
            .flat_map(|item| {
                crate::tui_wrap::wrap(&item.body, width)
                    .into_iter()
                    .enumerate()
                    .map(|(wrapped_row, text)| ViewRow {
                        message_id: item.id.clone(),
                        wrapped_row,
                        role: item.role.clone(),
                        text,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn activity_item(row: &lkjagent_store::tui_snapshot::ActivityRow) -> ActivityItem {
    ActivityItem {
        id: row.id.clone(),
        kind: row.kind.clone(),
        matter_id: row.matter_id.clone(),
        status: row.status.clone(),
        monotonic_ms: row.monotonic_ms,
    }
}

fn body(bytes: &[u8], truncated: bool) -> String {
    let mut value = String::from_utf8_lossy(bytes).into_owned();
    if truncated {
        value.push_str(" [truncated]");
    }
    value
}

use crate::kernel::{RuntimeEvent, SnapshotAdapterInput};

#[derive(Debug, Clone)]
pub struct KernelTurnInput {
    pub snapshot: SnapshotAdapterInput,
    pub event: RuntimeEvent,
    pub case_scope: String,
    pub created_at: String,
}

impl KernelTurnInput {
    pub fn new(snapshot: SnapshotAdapterInput, event: RuntimeEvent) -> Self {
        Self {
            snapshot,
            event,
            case_scope: "case".to_string(),
            created_at: "kernel-driver".to_string(),
        }
    }

    pub fn case_id_i64(&self) -> Option<i64> {
        self.snapshot.case_id.as_deref().and_then(parse_case_id)
    }
}

fn parse_case_id(value: &str) -> Option<i64> {
    let digits = value.strip_prefix("case-").unwrap_or(value);
    digits.parse::<i64>().ok()
}

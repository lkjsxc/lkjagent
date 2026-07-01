#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessInput<'a> {
    pub plan_id: i64,
    pub status: &'a str,
    pub atom_total: i64,
    pub atom_ready: i64,
    pub atom_missing: i64,
    pub next_atom_id: &'a str,
    pub next_path: &'a str,
    pub active_contract_id: &'a str,
    pub measured_total: i64,
    pub accepted_floor: i64,
    pub assembly_pending: &'a str,
    pub completion_blockers: &'a [String],
    pub updated_at: &'a str,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReadinessRow {
    pub plan_id: i64,
    pub root: String,
    pub profile: String,
    pub plan_status: String,
    pub status: String,
    pub atom_total: i64,
    pub atom_ready: i64,
    pub atom_missing: i64,
    pub next_atom_id: String,
    pub next_path: String,
    pub active_contract_id: String,
    pub measured_total: i64,
    pub accepted_floor: i64,
    pub assembly_pending: String,
    pub completion_blockers: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventInput<'a> {
    pub plan_id: i64,
    pub atom_id: &'a str,
    pub event_kind: &'a str,
    pub summary: &'a str,
    pub measured_count: i64,
    pub weak_classes: &'a [String],
    pub contract_id: Option<&'a str>,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyRunInput<'a> {
    pub plan_id: i64,
    pub run_id: &'a str,
    pub source_atom_ids: &'a [String],
    pub target_paths: &'a [String],
    pub status: &'a str,
    pub measured_count: i64,
    pub summary: &'a str,
    pub created_at: &'a str,
}

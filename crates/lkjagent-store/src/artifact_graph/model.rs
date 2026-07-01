#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanInput<'a> {
    pub case_id: i64,
    pub artifact_id: &'a str,
    pub owner_objective: &'a str,
    pub artifact_kind: &'a str,
    pub root: &'a str,
    pub profile: &'a str,
    pub normalized_title: &'a str,
    pub measurement_kind: &'a str,
    pub requested_total: i64,
    pub accepted_floor: i64,
    pub section_count: i64,
    pub language_hint: &'a str,
    pub forbidden_roots: &'a [String],
    pub status: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanRow {
    pub id: i64,
    pub case_id: i64,
    pub artifact_id: String,
    pub artifact_kind: String,
    pub root: String,
    pub profile: String,
    pub status: String,
    pub accepted_floor: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomInput<'a> {
    pub plan_id: i64,
    pub atom_id: String,
    pub sequence: i64,
    pub role: &'a str,
    pub path: &'a str,
    pub status: &'a str,
    pub measurement_kind: &'a str,
    pub target_count: i64,
    pub count_floor: i64,
    pub measured_count: i64,
    pub byte_budget: i64,
    pub required_sections: &'a [String],
    pub weak_classes: &'a [String],
    pub assembly_target: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomRow {
    pub id: i64,
    pub atom_id: String,
    pub sequence: i64,
    pub role: String,
    pub path: String,
    pub status: String,
    pub measurement_kind: String,
    pub target_count: i64,
    pub count_floor: i64,
    pub measured_count: i64,
    pub byte_budget: i64,
    pub required_sections: String,
    pub weak_classes: String,
    pub assembly_target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeInput<'a> {
    pub plan_id: i64,
    pub from_atom_id: String,
    pub to_atom_id: String,
    pub relation: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractInput<'a> {
    pub contract_id: &'a str,
    pub plan_id: i64,
    pub atom_ids: &'a [String],
    pub exact_paths: &'a [String],
    pub max_files: i64,
    pub max_file_bytes: i64,
    pub max_batch_bytes: i64,
    pub target_count: i64,
    pub count_floor: i64,
    pub required_sections: &'a [String],
    pub continuity_digest: &'a str,
    pub forbidden_weak_classes: &'a [String],
    pub status: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractRow {
    pub id: i64,
    pub contract_id: String,
    pub plan_id: i64,
    pub atom_ids: String,
    pub exact_paths: String,
    pub max_files: i64,
    pub max_file_bytes: i64,
    pub max_batch_bytes: i64,
    pub target_count: i64,
    pub count_floor: i64,
    pub required_sections: String,
    pub continuity_digest: String,
    pub forbidden_weak_classes: String,
    pub status: String,
}

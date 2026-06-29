#[derive(Debug, Clone, Copy)]
pub struct DenseRuntimeRowInput<'a> {
    pub decision_id: i64,
    pub row_kind: &'a str,
    pub subject: &'a str,
    pub predicate: &'a str,
    pub object: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseRuntimeRow {
    pub id: i64,
    pub decision_id: i64,
    pub row_kind: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenseRuntimePacket {
    pub decision_id: i64,
    pub facts: Vec<DenseRuntimeRow>,
    pub obligations: Vec<DenseRuntimeRow>,
    pub resolver_plans: Vec<DenseRuntimeRow>,
    pub progress: Vec<DenseRuntimeRow>,
    pub completion_inputs: Vec<DenseRuntimeRow>,
}

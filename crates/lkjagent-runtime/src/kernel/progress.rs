use crate::kernel::obligation_facts::root_identity_required;
use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressKey {
    pub target: String,
    pub action_class: String,
    pub fact_digest: String,
}

impl ProgressKey {
    pub fn new(target: String, action_class: String, fact_digest: String) -> Self {
        Self {
            target,
            action_class,
            fact_digest,
        }
    }

    pub fn fingerprint(&self) -> String {
        format!(
            "target={};action_class={};fact_digest={}",
            self.target, self.action_class, self.fact_digest
        )
    }
}

pub fn progress_key_for_snapshot(snapshot: &RuntimeSnapshot) -> ProgressKey {
    let target = snapshot
        .artifact
        .root
        .clone()
        .or_else(|| snapshot.case.case_id.clone())
        .unwrap_or_else(|| "workspace".to_string());
    let action_class = if root_identity_required(snapshot) {
        "identity-write"
    } else if snapshot
        .observation
        .latest
        .as_deref()
        .is_some_and(|text| text.contains("next_decision_required=true"))
    {
        "content-write"
    } else {
        "prior-action"
    };
    ProgressKey::new(target, action_class.to_string(), fact_digest(snapshot))
}

fn fact_digest(snapshot: &RuntimeSnapshot) -> String {
    let mut text = String::new();
    if let Some(observation) = &snapshot.observation.latest {
        text.push_str(observation);
    }
    text.push_str("|missing=");
    text.push_str(&snapshot.evidence.missing.join(","));
    text.push_str("|weak=");
    text.push_str(&snapshot.artifact.weak_paths.join(","));
    if let Some(fault) = snapshot.latest_fault {
        text.push_str("|fault=");
        text.push_str(fault.class().label());
    }
    digest(&text)
}

fn digest(text: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

trait FaultClassName {
    fn label(self) -> &'static str;
}

impl FaultClassName for crate::kernel::fault::FaultClass {
    fn label(self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Parameter => "parameter",
            Self::Schema => "schema",
            Self::Tool => "tool",
            Self::Repeat => "repeat",
            Self::Endpoint => "endpoint",
            Self::Budget => "budget",
            Self::Context => "context",
            Self::Verification => "verification",
            Self::Compaction => "compaction",
            Self::Payload => "payload",
            Self::Completion => "completion",
            Self::MaintenanceConflict => "maintenance_conflict",
        }
    }
}

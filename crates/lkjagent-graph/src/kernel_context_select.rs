use crate::kernel_track_ops::weight;
use crate::kernel_types::*;

pub fn tool_biases_from_tracks(vector: &StateVector) -> Vec<String> {
    let mut tools = Vec::new();
    if weight(vector, TrackLabel::ParseRecovery) >= 0.80 {
        tools.extend(names(&[
            "graph.state",
            "doc.audit",
            "fs.list",
            "fs.tree",
            "fs.write",
        ]));
    }
    if weight(vector, TrackLabel::ArtifactDrift) >= 0.75 {
        tools.extend(names(&["artifact.audit", "fs.read", "fs.tree"]));
    }
    if document_repair_active(vector) {
        tools.extend(names(&["doc.audit", "fs.tree", "fs.list"]));
    }
    if weight(vector, TrackLabel::ModelSpecificNaming) >= 0.60 {
        tools.extend(names(&["fs.read", "fs.edit", "doc.audit"]));
    }
    if weight(vector, TrackLabel::QueueInterruption) >= 0.70 {
        tools.extend(names(&["queue.list", "graph.state"]));
    }
    if weight(vector, TrackLabel::MaintenanceNoopRisk) >= 0.60 {
        tools.extend(names(&["memory.save", "queue.list", "graph.state"]));
    }
    if weight(vector, TrackLabel::WorkspaceEvidenceRisk) >= 0.60 {
        tools.extend(names(&["fs.list", "fs.tree", "workspace.summary"]));
    }
    tools.sort();
    tools.dedup();
    tools
}

pub fn required_context_slices_from_tracks(vector: &StateVector) -> Vec<String> {
    let mut slices = Vec::new();
    if recovery_active(vector) {
        slices.extend(names(&[
            "action grammar",
            "tool schemas",
            "last parser faults",
        ]));
    }
    if artifact_active(vector) {
        slices.extend(names(&[
            "owner objective",
            "artifact contract",
            "drifted paths",
        ]));
    }
    if document_repair_active(vector) {
        slices.extend(names(&[
            "doc topology rules",
            "relation graph",
            "last doc.audit failures",
        ]));
    }
    if weight(vector, TrackLabel::ModelSpecificNaming) >= 0.60 {
        slices.extend(names(&["model-name sanitizer", "raw fixture pointer"]));
    }
    if weight(vector, TrackLabel::MaintenanceNoopRisk) >= 0.60 {
        slices.extend(names(&[
            "maintenance cycle summary",
            "no-op suppression key",
        ]));
    }
    if weight(vector, TrackLabel::WorkspaceEvidenceRisk) >= 0.60 {
        slices.extend(names(&[
            "workspace evidence rule",
            "filesystem observation",
        ]));
    }
    if weight(vector, TrackLabel::ContextPressure) >= 0.60 {
        slices.extend(names(&["context budget", "post-compaction checklist"]));
    }
    slices.sort();
    slices.dedup();
    slices
}

fn recovery_active(vector: &StateVector) -> bool {
    weight(vector, TrackLabel::ParseRecovery) >= 0.80
        || weight(vector, TrackLabel::ActionParamReliability) >= 0.60
}

fn artifact_active(vector: &StateVector) -> bool {
    weight(vector, TrackLabel::ArtifactDrift) >= 0.75
        || weight(vector, TrackLabel::ArtifactReadiness) >= 0.60
}

fn document_repair_active(vector: &StateVector) -> bool {
    weight(vector, TrackLabel::DocumentStructure) >= 0.60
        || weight(vector, TrackLabel::StructureConnectivity) >= 0.60
        || weight(vector, TrackLabel::MockContentRisk) >= 0.70
}

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

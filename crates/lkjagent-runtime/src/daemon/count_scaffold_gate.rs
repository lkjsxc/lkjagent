use lkjagent_graph::{completion_decision, GraphNodeId, TaskGraphState, TransitionDecision};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CountedScaffoldClosure {
    Admit { target: GraphNodeId },
    Wait { question: String },
}

pub(super) fn counted_scaffold_closure(graph: Option<&TaskGraphState>) -> CountedScaffoldClosure {
    let Some(graph) = graph else {
        return CountedScaffoldClosure::Wait {
            question: counted_wait_question("missing_graph_case"),
        };
    };
    match completion_decision(graph) {
        TransitionDecision::Admit { target } => CountedScaffoldClosure::Admit { target },
        TransitionDecision::Defer { missing } => CountedScaffoldClosure::Wait {
            question: counted_wait_question(&format!("missing={}", missing_items(&missing))),
        },
        TransitionDecision::Recover { reason, .. } | TransitionDecision::Refuse { reason } => {
            CountedScaffoldClosure::Wait {
                question: counted_wait_question(&format!("reason={reason}")),
            }
        }
    }
}

fn counted_wait_question(reason: &str) -> String {
    format!(
        "Counted scaffold generated but graph completion gate is not admitted; {reason}. Send guidance or evidence to continue."
    )
}

fn missing_items(items: &[String]) -> String {
    if items.is_empty() {
        "unknown".to_string()
    } else {
        items.join(",")
    }
}

#[cfg(test)]
mod tests {
    use lkjagent_graph::{initial_state, EvidenceKind, EvidenceRecord};

    use super::{counted_scaffold_closure, CountedScaffoldClosure};

    #[test]
    fn counted_scaffold_closure_admits_only_complete_graph_evidence() {
        let mut graph = initial_state("Create exactly 5 files total.", None);
        graph.pending_checks.clear();
        graph.evidence_requirements = vec![
            "plan".to_string(),
            "observation".to_string(),
            "document-structure".to_string(),
            "verification".to_string(),
        ];
        graph.evidence = graph
            .evidence_requirements
            .iter()
            .map(|requirement| evidence(requirement))
            .collect();

        assert!(matches!(
            counted_scaffold_closure(Some(&graph)),
            CountedScaffoldClosure::Admit { .. }
        ));
    }

    #[test]
    fn counted_scaffold_closure_waits_when_graph_gate_is_missing_evidence() {
        let mut graph = initial_state("Create exactly 5 files total.", None);
        graph.pending_checks.clear();
        graph.evidence_requirements = vec!["verification".to_string()];
        graph.evidence.clear();

        assert!(matches!(
            counted_scaffold_closure(Some(&graph)),
            CountedScaffoldClosure::Wait { question }
                if question.contains("missing=verification")
        ));
    }

    fn evidence(requirement: &str) -> EvidenceRecord {
        EvidenceRecord {
            requirement: requirement.to_string(),
            kind: EvidenceKind::Verification,
            summary: "ok".to_string(),
            path: None,
        }
    }
}

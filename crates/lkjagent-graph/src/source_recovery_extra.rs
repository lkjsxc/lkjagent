use crate::model::{GraphNode, NodeKind};
use crate::source_nodes::{
    node, NO_EVIDENCE, RECOVERY_PACKAGES, RECOVERY_TOOLS, SHELL_ESCAPE_TOOLS,
};

pub(crate) const NODES: &[GraphNode] = &[
    recovery("recover-parse", "recover from malformed action output"),
    recovery(
        "recover-endpoint",
        "recover from endpoint outage or max tokens",
    ),
    recovery("recover-context", "recover from stale or oversized context"),
    recovery(
        "recover-budget",
        "recover by narrowing scope after budget pressure",
    ),
    recovery(
        "recover-verification",
        "recover from failed verification evidence",
    ),
    recovery(
        "recover-by-alternate-tool",
        "choose a different native tool class",
    ),
    recovery(
        "recover-by-smaller-scope",
        "split the failing step into smaller work",
    ),
    recovery(
        "recover-by-state-inspection",
        "inspect graph and workspace state before retrying",
    ),
    node(
        "recover-by-shell-escape",
        NodeKind::Recovery,
        "use shell only after graph admission proves native tools insufficient",
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        SHELL_ESCAPE_TOOLS,
    ),
];

const fn recovery(id: &'static str, label: &'static str) -> GraphNode {
    node(
        id,
        NodeKind::Recovery,
        label,
        NO_EVIDENCE,
        RECOVERY_PACKAGES,
        RECOVERY_TOOLS,
    )
}

from __future__ import annotations


ANCHORED_CHECKS = {
    "daily-life-recall": {
        "journal-path-date",
        "journal-body-semantic",
        "multi-intent-decomposition",
        "old-record-recall",
        "todo-roundtrip",
    },
    "multi-project-development": {
        "context-project-isolation",
        "project-separation",
        "source-edit-verified",
        "workspace-visible",
    },
    "long-artifact-recovery": {
        "artifact-units-complete",
        "all-files-verified",
        "output-limit-recovered",
        "strategy-changed",
    },
}

ANCHORED_EVENT_KINDS = {
    "daily-life-recall": {"workspace.external_edit", "context.retrieval"},
    "multi-project-development": {"daemon.restart", "effect.commit"},
    "long-artifact-recovery": {
        "artifact.unit.commit",
        "provider.output_limit",
        "recovery.strategy_changed",
    },
}


MINIMUM_DURATION_SECONDS = 840
MINIMUM_DECISION_SPAN_SECONDS = 600
MINIMUM_OWNER_SPAN_SECONDS = 600
MINIMUM_OWNER_TURNS = 3
MINIMUM_DECISIONS = 8
MINIMUM_USEFUL_DECISIONS = 5
MINIMUM_PROGRESS_DECISIONS = 3

use crate::task::{RuntimeState, TaskState};

mod store;

pub use store::{
    defer_all_directives, load_directive_stamps, maintenance_due, prepare_idle_cycle,
    stamp_directive,
};

pub const DEFAULT_MAINTENANCE_TURN_BUDGET: u16 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceDirective {
    Distill,
    RefineGraphPolicy,
    PruneMemory,
    AuditSelf,
}

impl MaintenanceDirective {
    pub fn all() -> &'static [Self] {
        &[
            Self::Distill,
            Self::RefineGraphPolicy,
            Self::PruneMemory,
            Self::AuditSelf,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Distill => "distill",
            Self::RefineGraphPolicy => "refine-graph-policy",
            Self::PruneMemory => "prune-memory",
            Self::AuditSelf => "audit-self",
        }
    }

    pub fn work(self) -> &'static str {
        match self {
            Self::Distill => "read recent transcript spans and save durable lessons",
            Self::RefineGraphPolicy => {
                "record graph policy and context package improvement candidates"
            }
            Self::PruneMemory => "merge, rewrite, or drop stale memory rows",
            Self::AuditSelf => "record contract mismatches from recent failures",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveStamp {
    pub directive: MaintenanceDirective,
    pub last_run: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintenanceCycle {
    pub directive: MaintenanceDirective,
    pub turns_remaining: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryDecision {
    StartCycle {
        directive: MaintenanceDirective,
        budget: u16,
    },
    ContinueCycle {
        directive: MaintenanceDirective,
        turns_remaining: u16,
    },
    PreemptForQueue {
        pending: usize,
    },
    NotIdle,
}

pub fn choose_directive(stamps: &[DirectiveStamp]) -> MaintenanceDirective {
    let mut best = None;
    for directive in MaintenanceDirective::all() {
        let stamp = stamp_for(stamps, *directive);
        if best
            .as_ref()
            .is_none_or(|(_, best_stamp)| is_staler(stamp, *best_stamp))
        {
            best = Some((*directive, stamp));
        }
    }
    best.map_or(MaintenanceDirective::Distill, |(directive, _)| directive)
}

pub fn idle_boundary(
    state: &RuntimeState,
    pending_queue: usize,
    stamps: &[DirectiveStamp],
) -> BoundaryDecision {
    if let Some(cycle) = state
        .maintenance
        .as_ref()
        .filter(|_| state.pending_action.is_some())
    {
        return BoundaryDecision::ContinueCycle {
            directive: cycle.directive,
            turns_remaining: cycle.turns_remaining,
        };
    }
    if !matches!(state.task, TaskState::Idle | TaskState::Closed { .. }) {
        return BoundaryDecision::NotIdle;
    }
    if pending_queue > 0 {
        return BoundaryDecision::PreemptForQueue {
            pending: pending_queue,
        };
    }
    match &state.maintenance {
        Some(cycle) => BoundaryDecision::ContinueCycle {
            directive: cycle.directive,
            turns_remaining: cycle.turns_remaining,
        },
        None => BoundaryDecision::StartCycle {
            directive: choose_directive(stamps),
            budget: DEFAULT_MAINTENANCE_TURN_BUDGET,
        },
    }
}

pub fn maintenance_notice(directive: MaintenanceDirective, budget: u16) -> String {
    format!(
        "maintenance cycle opened\ndirective={}\nturn_budget={budget}\nwork={}",
        directive.as_str(),
        directive.work()
    )
}

pub fn spend_cycle(cycle: &Option<MaintenanceCycle>) -> (Option<MaintenanceCycle>, bool) {
    match cycle {
        Some(current) if current.turns_remaining > 1 => (
            Some(MaintenanceCycle {
                directive: current.directive,
                turns_remaining: current.turns_remaining.saturating_sub(1),
            }),
            false,
        ),
        Some(_) => (None, true),
        None => (None, false),
    }
}

pub fn task_distillation_prompt(summary: &str) -> String {
    format!(
        "distill closed task\nmax_turns=2\nuse memory.save for durable lessons, facts, or incidents\nsummary={summary}"
    )
}

pub fn task_summary_required(task: &TaskState) -> bool {
    matches!(task, TaskState::Open { .. } | TaskState::Waiting { .. })
}

fn stamp_for(stamps: &[DirectiveStamp], directive: MaintenanceDirective) -> Option<&str> {
    stamps
        .iter()
        .find(|stamp| stamp.directive == directive)
        .and_then(|stamp| stamp.last_run.as_deref())
}

fn is_staler(candidate: Option<&str>, current: Option<&str>) -> bool {
    match (candidate, current) {
        (None, Some(_)) => true,
        (Some(_), None) | (None, None) => false,
        (Some(candidate), Some(current)) => candidate < current,
    }
}

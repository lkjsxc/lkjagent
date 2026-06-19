use super::{
    prefix_cap_total, BudgetRow, BUDGET_ROWS, DEFAULT_GENERATION_RESERVE, DEFAULT_HARD_TRIGGER,
    DEFAULT_POST_COMPACTION_TARGET, DEFAULT_SOFT_TRIGGER, DEFAULT_WINDOW_TOKENS, LOG_OBSERVATION,
    MIN_CONTEXT_WINDOW, MIN_LOG_SPACE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextBudgetPolicy {
    pub window: usize,
    pub reserve: usize,
    pub soft_trigger: usize,
    pub hard_trigger: usize,
    pub post_compaction_target: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextBudgetError {
    WindowTooSmall { minimum: usize, actual: usize },
    InvalidReserve { window: usize, reserve: usize },
    StarvedLog { minimum: usize, available: usize },
    InvalidDerivedPolicy { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextPressure {
    Green,
    Yellow,
    Orange,
    Red,
    BlackInvalid,
}

impl ContextBudgetPolicy {
    pub fn derive(
        window: usize,
        reserve: usize,
        trigger: Option<usize>,
    ) -> Result<Self, ContextBudgetError> {
        validate_window(window)?;
        validate_reserve(window, reserve)?;
        let available = available_log_space_for(window, reserve);
        if available < MIN_LOG_SPACE {
            return Err(ContextBudgetError::StarvedLog {
                minimum: MIN_LOG_SPACE,
                available,
            });
        }
        let hard_limit = window.saturating_sub(reserve);
        let derived = derived_hard_trigger(window, reserve);
        let target = derived_post_target(window, derived)?;
        let hard = trigger
            .filter(|value| trigger_is_safe(*value, hard_limit, target))
            .unwrap_or(derived);
        if hard <= target || hard >= hard_limit {
            return Err(ContextBudgetError::InvalidDerivedPolicy {
                reason: "hard trigger must be between target and reserve limit".to_string(),
            });
        }
        Ok(Self {
            window,
            reserve,
            soft_trigger: derived_soft_trigger(window, hard)?,
            hard_trigger: hard,
            post_compaction_target: target.min(hard.saturating_sub(1)),
        })
    }

    pub fn pressure(self, used_tokens: usize, predicted_next_input: usize) -> ContextPressure {
        if self.hard_trigger >= self.window.saturating_sub(self.reserve)
            || self.post_compaction_target >= self.hard_trigger
        {
            return ContextPressure::BlackInvalid;
        }
        if used_tokens.saturating_add(self.reserve) > self.window {
            return ContextPressure::BlackInvalid;
        }
        let projected = used_tokens.saturating_add(predicted_next_input);
        if projected.saturating_add(self.reserve) > self.window || projected >= self.hard_trigger {
            ContextPressure::Red
        } else if used_tokens >= self.soft_trigger {
            ContextPressure::Orange
        } else if projected >= self.soft_trigger {
            ContextPressure::Yellow
        } else {
            ContextPressure::Green
        }
    }

    pub fn available_log_space(self) -> usize {
        self.window
            .saturating_sub(self.reserve)
            .saturating_sub(prefix_cap_total())
    }
}

impl Default for ContextBudgetPolicy {
    fn default() -> Self {
        Self {
            window: DEFAULT_WINDOW_TOKENS,
            reserve: DEFAULT_GENERATION_RESERVE,
            soft_trigger: DEFAULT_SOFT_TRIGGER,
            hard_trigger: DEFAULT_HARD_TRIGGER,
            post_compaction_target: DEFAULT_POST_COMPACTION_TARGET,
        }
    }
}

impl std::fmt::Display for ContextBudgetError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowTooSmall { minimum, actual } => {
                write!(formatter, "context.window must be at least {minimum}; got {actual}")
            }
            Self::InvalidReserve { window, reserve } => write!(formatter, "context.reserve must be positive and below context.window; window={window} reserve={reserve}"),
            Self::StarvedLog { minimum, available } => write!(formatter, "context reserve and prefix leave {available} log tokens; minimum is {minimum}"),
            Self::InvalidDerivedPolicy { reason } => formatter.write_str(reason),
        }
    }
}

pub fn budget_rows_for(policy: ContextBudgetPolicy) -> Vec<BudgetRow> {
    BUDGET_ROWS
        .iter()
        .map(|row| BudgetRow {
            region: row.region,
            cap: match row.region {
                "generation reserve" => policy.reserve,
                "soft compaction trigger" => policy.soft_trigger,
                "hard compaction trigger" => policy.hard_trigger,
                "post-compaction target" => policy.post_compaction_target,
                _ => row.cap,
            },
        })
        .collect()
}

fn validate_window(window: usize) -> Result<(), ContextBudgetError> {
    if window < MIN_CONTEXT_WINDOW {
        Err(ContextBudgetError::WindowTooSmall {
            minimum: MIN_CONTEXT_WINDOW,
            actual: window,
        })
    } else {
        Ok(())
    }
}

fn validate_reserve(window: usize, reserve: usize) -> Result<(), ContextBudgetError> {
    if reserve == 0 || reserve >= window {
        Err(ContextBudgetError::InvalidReserve { window, reserve })
    } else {
        Ok(())
    }
}

fn available_log_space_for(window: usize, reserve: usize) -> usize {
    window
        .saturating_sub(reserve)
        .saturating_sub(prefix_cap_total())
}

fn derived_hard_trigger(window: usize, reserve: usize) -> usize {
    let reserve_limit = window.saturating_sub(reserve);
    let safety = reserve.saturating_div(2).max(1);
    window
        .saturating_mul(7)
        .saturating_div(8)
        .min(reserve_limit.saturating_sub(safety))
}

fn derived_soft_trigger(window: usize, hard: usize) -> Result<usize, ContextBudgetError> {
    let soft = window
        .saturating_mul(3)
        .saturating_div(4)
        .min(hard.saturating_sub(LOG_OBSERVATION.saturating_div(2)));
    if soft >= hard {
        Err(ContextBudgetError::InvalidDerivedPolicy {
            reason: "soft trigger must be below hard trigger".to_string(),
        })
    } else {
        Ok(soft)
    }
}

fn derived_post_target(window: usize, hard: usize) -> Result<usize, ContextBudgetError> {
    let target = prefix_cap_total()
        .saturating_add(DEFAULT_GENERATION_RESERVE)
        .max(window.saturating_div(3));
    if target >= hard {
        Err(ContextBudgetError::InvalidDerivedPolicy {
            reason: "post-compaction target must be below hard trigger".to_string(),
        })
    } else {
        Ok(target)
    }
}

fn trigger_is_safe(trigger: usize, hard_limit: usize, target: usize) -> bool {
    trigger > target && trigger < hard_limit
}

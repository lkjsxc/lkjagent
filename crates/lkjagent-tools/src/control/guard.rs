use crate::count_guard::{CountGuard, CountKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionGuard {
    None,
    FileCount { target: usize },
    MarkdownCount { target: usize },
    RecursiveStructure,
    RecursiveKnowledge,
    RecursiveStructureCount { count: CountGuard },
    RecursiveKnowledgeCount { count: CountGuard },
}

impl CompletionGuard {
    pub fn as_state_value(self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::FileCount { target } => format!("file-count:{target}"),
            Self::MarkdownCount { target } => format!("markdown-count:{target}"),
            Self::RecursiveStructure => "recursive-structure".to_string(),
            Self::RecursiveKnowledge => "recursive-knowledge".to_string(),
            Self::RecursiveStructureCount { count } => {
                format!("recursive-structure+{}", count.as_state_value())
            }
            Self::RecursiveKnowledgeCount { count } => {
                format!("recursive-knowledge+{}", count.as_state_value())
            }
        }
    }

    pub fn from_state_value(value: &str) -> Self {
        if let Some(count) = CountGuard::from_state_value(value) {
            return Self::from_count(count);
        }
        if let Some(raw) = value.strip_prefix("recursive-structure+") {
            if let Some(count) = CountGuard::from_state_value(raw) {
                return Self::RecursiveStructureCount { count };
            }
        }
        if let Some(raw) = value.strip_prefix("recursive-knowledge+") {
            if let Some(count) = CountGuard::from_state_value(raw) {
                return Self::RecursiveKnowledgeCount { count };
            }
        }
        match value {
            "recursive-structure" => Self::RecursiveStructure,
            "recursive-knowledge" => Self::RecursiveKnowledge,
            _ => Self::None,
        }
    }

    pub fn is_recursive(self) -> bool {
        matches!(
            self,
            Self::RecursiveStructure
                | Self::RecursiveKnowledge
                | Self::RecursiveStructureCount { .. }
                | Self::RecursiveKnowledgeCount { .. }
        )
    }

    pub fn is_knowledge(self) -> bool {
        matches!(
            self,
            Self::RecursiveKnowledge | Self::RecursiveKnowledgeCount { .. }
        )
    }

    pub fn markdown_target(self) -> Option<usize> {
        self.count_guard().and_then(CountGuard::markdown_target)
    }

    pub fn has_count(self) -> bool {
        self.count_guard().is_some()
    }

    pub fn count_guard(self) -> Option<CountGuard> {
        match self {
            Self::FileCount { target } => Some(CountGuard {
                kind: CountKind::File,
                target,
            }),
            Self::MarkdownCount { target } => Some(CountGuard {
                kind: CountKind::Markdown,
                target,
            }),
            Self::RecursiveStructureCount { count } | Self::RecursiveKnowledgeCount { count } => {
                Some(count)
            }
            _ => None,
        }
    }

    pub(super) fn structural_base(self) -> Self {
        match self {
            Self::RecursiveStructure | Self::RecursiveStructureCount { .. } => {
                Self::RecursiveStructure
            }
            Self::RecursiveKnowledge | Self::RecursiveKnowledgeCount { .. } => {
                Self::RecursiveKnowledge
            }
            _ => Self::None,
        }
    }

    pub(super) fn with_count(self, count: CountGuard) -> Self {
        match self.structural_base() {
            Self::RecursiveStructure => Self::RecursiveStructureCount { count },
            Self::RecursiveKnowledge => Self::RecursiveKnowledgeCount { count },
            _ => Self::from_count(count),
        }
    }

    fn from_count(count: CountGuard) -> Self {
        match count.kind {
            CountKind::File => Self::FileCount {
                target: count.target,
            },
            CountKind::Markdown => Self::MarkdownCount {
                target: count.target,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveMode {
    OwnerTask,
    Recovery,
    Maintenance,
    Compaction,
    ClosedIdle,
}

impl ActiveMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OwnerTask => "owner_task",
            Self::Recovery => "recovery",
            Self::Maintenance => "maintenance",
            Self::Compaction => "compaction",
            Self::ClosedIdle => "closed_idle",
        }
    }

    pub fn allows_model_call(self) -> bool {
        matches!(self, Self::OwnerTask | Self::Recovery | Self::Maintenance)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
    Missing,
    File,
    Directory,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootPathProblem {
    RootIsFile,
    RootEndsWithMarkdownSuffix,
    RootMissing,
    RootNotDirectory,
    RootOutsideWorkspace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactAddressKind {
    RootDirectory,
    MissingRoot,
    FileUnderKnownRoot,
    FileWithoutKnownRoot,
    InvalidRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactAddress {
    pub requested: String,
    pub root: Option<String>,
    pub weak_path: Option<String>,
    pub kind: ArtifactAddressKind,
    pub problem: Option<RootPathProblem>,
    pub detected: PathKind,
    pub next_action: AddressNextAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressNextAction {
    ApplyRoot {
        root: String,
        kind: String,
    },
    AuditRoot {
        root: String,
        kind: String,
    },
    RepairPath {
        root: String,
        path: String,
        kind: String,
    },
    InspectParent {
        path: String,
    },
    Refuse {
        reason: String,
    },
}

use crate::registry::{ParamSpec, RequiredAnySpec};

pub const GRAPH_PLAN: &[ParamSpec] = &[
    ParamSpec::req("objective"),
    ParamSpec::opt("constraints", None),
    ParamSpec::opt("assumptions", None),
    ParamSpec::opt("risks", None),
    ParamSpec::req("steps"),
    ParamSpec::opt("checks", None),
    ParamSpec::opt("paths", None),
    ParamSpec::req("reason"),
];

pub const GRAPH_PLAN_ANY: &[RequiredAnySpec] = &[RequiredAnySpec {
    names: &["checks", "paths"],
    label: "checks|paths",
}];

pub const GRAPH_TRANSITION: &[ParamSpec] = &[ParamSpec::req("target"), ParamSpec::req("reason")];
pub const GRAPH_CONTEXT: &[ParamSpec] = &[ParamSpec::req("packages"), ParamSpec::req("reason")];
pub const GRAPH_NOTE: &[ParamSpec] = &[
    ParamSpec::req("kind"),
    ParamSpec::req("summary"),
    ParamSpec::opt("path", None),
];
pub const GRAPH_EVIDENCE: &[ParamSpec] = &[
    ParamSpec::req("kind"),
    ParamSpec::req("summary"),
    ParamSpec::opt("path", None),
];
pub const GRAPH_COMPACT: &[ParamSpec] = &[ParamSpec::req("reason")];

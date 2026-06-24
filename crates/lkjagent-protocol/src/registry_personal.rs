use crate::registry::ParamSpec;

pub const DIARY_RECORD: &[ParamSpec] = &[
    ParamSpec::opt("date", None),
    ParamSpec::req("title"),
    ParamSpec::req("content"),
    ParamSpec::opt("tags", None),
];

pub const DIARY_FIND: &[ParamSpec] = &[
    ParamSpec::opt("query", None),
    ParamSpec::opt("start", None),
    ParamSpec::opt("end", None),
    ParamSpec::opt("tags", None),
    ParamSpec::opt("limit", Some("10")),
];

pub const SCHEDULE_ADD: &[ParamSpec] = &[
    ParamSpec::req("title"),
    ParamSpec::req("start"),
    ParamSpec::opt("end", None),
    ParamSpec::opt("timezone", None),
    ParamSpec::opt("location", None),
    ParamSpec::opt("notes", None),
    ParamSpec::opt("recurrence", None),
    ParamSpec::opt("tags", None),
];

pub const SCHEDULE_LIST: &[ParamSpec] = &[
    ParamSpec::opt("start", None),
    ParamSpec::opt("end", None),
    ParamSpec::opt("query", None),
    ParamSpec::opt("status", Some("all")),
    ParamSpec::opt("limit", Some("20")),
];

pub const SCHEDULE_UPDATE: &[ParamSpec] = &[
    ParamSpec::req("id"),
    ParamSpec::opt("title", None),
    ParamSpec::opt("start", None),
    ParamSpec::opt("end", None),
    ParamSpec::opt("status", None),
    ParamSpec::opt("notes", None),
    ParamSpec::opt("tags", None),
];

pub const TODO_ADD: &[ParamSpec] = &[
    ParamSpec::req("title"),
    ParamSpec::opt("details", None),
    ParamSpec::opt("due", None),
    ParamSpec::opt("priority", Some("normal")),
    ParamSpec::opt("project", None),
    ParamSpec::opt("tags", None),
];

pub const TODO_LIST: &[ParamSpec] = &[
    ParamSpec::opt("status", Some("open")),
    ParamSpec::opt("query", None),
    ParamSpec::opt("due_before", None),
    ParamSpec::opt("project", None),
    ParamSpec::opt("limit", Some("20")),
];

pub const TODO_UPDATE: &[ParamSpec] = &[
    ParamSpec::req("id"),
    ParamSpec::opt("title", None),
    ParamSpec::opt("details", None),
    ParamSpec::opt("due", None),
    ParamSpec::opt("priority", None),
    ParamSpec::opt("project", None),
    ParamSpec::opt("status", None),
    ParamSpec::opt("tags", None),
];

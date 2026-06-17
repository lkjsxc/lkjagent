#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamSpec {
    pub name: &'static str,
    pub required: bool,
    pub default: Option<&'static str>,
}

impl ParamSpec {
    pub const fn req(name: &'static str) -> Self {
        Self {
            name,
            required: true,
            default: None,
        }
    }

    pub const fn opt(name: &'static str, default: Option<&'static str>) -> Self {
        Self {
            name,
            required: false,
            default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: &'static str,
    pub params: &'static [ParamSpec],
    pub contract: &'static str,
}

const FS_READ: &[ParamSpec] = &[
    ParamSpec::req("path"),
    ParamSpec::opt("start", Some("1")),
    ParamSpec::opt("count", Some("200")),
];
const FS_WRITE: &[ParamSpec] = &[ParamSpec::req("path"), ParamSpec::req("content")];
const FS_EDIT: &[ParamSpec] = &[
    ParamSpec::req("path"),
    ParamSpec::req("find"),
    ParamSpec::req("replace"),
];
const SHELL_RUN: &[ParamSpec] = &[
    ParamSpec::req("command"),
    ParamSpec::opt("timeout", Some("60")),
    ParamSpec::opt("max", Some("600")),
];
const QUEUE_LIST: &[ParamSpec] = &[
    ParamSpec::opt("status", Some("all")),
    ParamSpec::opt("limit", Some("20")),
];
const QUEUE_ENQUEUE: &[ParamSpec] = &[ParamSpec::req("content"), ParamSpec::req("reason")];
const QUEUE_EDIT: &[ParamSpec] = &[
    ParamSpec::req("id"),
    ParamSpec::req("content"),
    ParamSpec::req("reason"),
];
const QUEUE_DELETE: &[ParamSpec] = &[ParamSpec::req("id"), ParamSpec::req("reason")];
const QUEUE_REDELIVER: &[ParamSpec] = &[
    ParamSpec::req("id"),
    ParamSpec::req("reason"),
    ParamSpec::opt("content", None),
];
const MEMORY_SAVE: &[ParamSpec] = &[
    ParamSpec::req("kind"),
    ParamSpec::req("title"),
    ParamSpec::opt("tags", None),
    ParamSpec::req("content"),
];
const MEMORY_FIND: &[ParamSpec] = &[ParamSpec::req("query"), ParamSpec::opt("limit", Some("5"))];
const SKILL_USE: &[ParamSpec] = &[ParamSpec::req("name")];
const SKILL_SAVE: &[ParamSpec] = &[ParamSpec::req("name"), ParamSpec::req("content")];
const AGENT_DONE: &[ParamSpec] = &[ParamSpec::req("summary")];
const AGENT_ASK: &[ParamSpec] = &[ParamSpec::req("question")];

pub const TOOLS: &[ToolSpec] = &[
    ToolSpec {
        name: "fs.read",
        params: FS_READ,
        contract: "ranged raw read, one header line",
    },
    ToolSpec {
        name: "fs.write",
        params: FS_WRITE,
        contract: "write file, create parent directories",
    },
    ToolSpec {
        name: "fs.edit",
        params: FS_EDIT,
        contract: "replace exactly one match of find",
    },
    ToolSpec {
        name: "shell.run",
        params: SHELL_RUN,
        contract: "run /bin/sh -lc in the workspace",
    },
    ToolSpec {
        name: "queue.list",
        params: QUEUE_LIST,
        contract: "list queue rows by status",
    },
    ToolSpec {
        name: "queue.enqueue",
        params: QUEUE_ENQUEUE,
        contract: "append a pending queue row",
    },
    ToolSpec {
        name: "queue.edit",
        params: QUEUE_EDIT,
        contract: "replace pending queue content",
    },
    ToolSpec {
        name: "queue.delete",
        params: QUEUE_DELETE,
        contract: "tombstone a pending queue row",
    },
    ToolSpec {
        name: "queue.redeliver",
        params: QUEUE_REDELIVER,
        contract: "create a linked pending row",
    },
    ToolSpec {
        name: "memory.save",
        params: MEMORY_SAVE,
        contract: "insert one memory row",
    },
    ToolSpec {
        name: "memory.find",
        params: MEMORY_FIND,
        contract: "ranked memory search",
    },
    ToolSpec {
        name: "skill.use",
        params: SKILL_USE,
        contract: "append skill body as a frame",
    },
    ToolSpec {
        name: "skill.save",
        params: SKILL_SAVE,
        contract: "validate and write a skill",
    },
    ToolSpec {
        name: "agent.done",
        params: AGENT_DONE,
        contract: "close the task or cycle",
    },
    ToolSpec {
        name: "agent.ask",
        params: AGENT_ASK,
        contract: "ask the owner",
    },
];

pub fn find_tool(name: &str) -> Option<&'static ToolSpec> {
    TOOLS.iter().find(|tool| tool.name == name)
}

pub fn render_registry_section() -> String {
    TOOLS
        .iter()
        .map(|tool| format!("{}: {}; {}", tool.name, render_params(tool), tool.contract))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_params(tool: &ToolSpec) -> String {
    tool.params
        .iter()
        .map(|param| {
            if param.required {
                format!("{} req", param.name)
            } else if let Some(default) = param.default {
                format!("{} opt {}", param.name, default)
            } else {
                format!("{} opt", param.name)
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

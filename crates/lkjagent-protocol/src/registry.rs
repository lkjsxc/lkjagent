pub use crate::registry_render::render_registry_section;

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
const FS_LIST: &[ParamSpec] = &[
    ParamSpec::opt("path", Some(".")),
    ParamSpec::opt("depth", Some("2")),
    ParamSpec::opt("kind", Some("all")),
    ParamSpec::opt("limit", Some("200")),
];
const FS_SEARCH: &[ParamSpec] = &[
    ParamSpec::req("query"),
    ParamSpec::opt("path", Some(".")),
    ParamSpec::opt("include", None),
    ParamSpec::opt("case", Some("insensitive")),
    ParamSpec::opt("context", Some("1")),
    ParamSpec::opt("limit", Some("50")),
];
const FS_STAT: &[ParamSpec] = &[ParamSpec::req("path")];
const FS_MKDIR: &[ParamSpec] = &[ParamSpec::req("path")];
const FS_BATCH_WRITE: &[ParamSpec] = &[ParamSpec::req("files")];
const SHELL_RUN: &[ParamSpec] = &[
    ParamSpec::req("command"),
    ParamSpec::opt("timeout", Some("60")),
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
const GRAPH_PLAN: &[ParamSpec] = &[
    ParamSpec::req("objective"),
    ParamSpec::opt("constraints", None),
    ParamSpec::opt("assumptions", None),
    ParamSpec::opt("risks", None),
    ParamSpec::req("steps"),
    ParamSpec::opt("checks", None),
    ParamSpec::opt("paths", None),
    ParamSpec::req("reason"),
];
const GRAPH_TRANSITION: &[ParamSpec] = &[ParamSpec::req("target"), ParamSpec::req("reason")];
const GRAPH_CONTEXT: &[ParamSpec] = &[ParamSpec::req("packages"), ParamSpec::req("reason")];
const GRAPH_NOTE: &[ParamSpec] = &[
    ParamSpec::req("kind"),
    ParamSpec::req("summary"),
    ParamSpec::opt("path", None),
];
const GRAPH_EVIDENCE: &[ParamSpec] = &[
    ParamSpec::req("kind"),
    ParamSpec::req("summary"),
    ParamSpec::opt("path", None),
];
const GRAPH_COMPACT: &[ParamSpec] = &[ParamSpec::req("reason")];
const WORKSPACE_SUMMARY: &[ParamSpec] = &[
    ParamSpec::opt("path", Some(".")),
    ParamSpec::opt("depth", Some("3")),
    ParamSpec::opt("limit", Some("200")),
];
const VERIFY_CARGO: &[ParamSpec] = &[
    ParamSpec::req("gate"),
    ParamSpec::opt("package", None),
    ParamSpec::opt("timeout", Some("120")),
];
const VERIFY_XTASK: &[ParamSpec] = &[
    ParamSpec::req("gate"),
    ParamSpec::opt("timeout", Some("120")),
];
const DOC_SCAFFOLD: &[ParamSpec] = &[
    ParamSpec::req("root"),
    ParamSpec::opt("kind", Some("documentation")),
    ParamSpec::opt("count", None),
    ParamSpec::opt("mode", Some("approx")),
    ParamSpec::req("title"),
    ParamSpec::opt("sections", None),
];
const DOC_AUDIT: &[ParamSpec] = &[
    ParamSpec::req("root"),
    ParamSpec::opt("count", None),
    ParamSpec::opt("mode", Some("approx")),
];
const AGENT_DONE: &[ParamSpec] = &[ParamSpec::req("summary")];
const AGENT_ASK: &[ParamSpec] = &[ParamSpec::req("question")];

#[rustfmt::skip]
pub const TOOLS: &[ToolSpec] = &[
    tool("fs.read", FS_READ, "ranged raw read, one header line"),
    tool("fs.write", FS_WRITE, "write file, create parent directories"),
    tool("fs.edit", FS_EDIT, "replace exactly one match of find"),
    tool("fs.list", FS_LIST, "sorted bounded workspace listing"),
    tool("fs.search", FS_SEARCH, "bounded substring search"),
    tool("fs.stat", FS_STAT, "kind, bytes, lines, stable checksum"),
    tool("fs.mkdir", FS_MKDIR, "create a workspace directory"),
    tool("fs.batch_write", FS_BATCH_WRITE, "write multiple files from line protocol"),
    tool("shell.run", SHELL_RUN, "escape hatch /bin/sh -lc in workspace"),
    tool("queue.list", QUEUE_LIST, "list queue rows by status"),
    tool("queue.enqueue", QUEUE_ENQUEUE, "append a pending queue row"),
    tool("queue.edit", QUEUE_EDIT, "replace pending queue content"),
    tool("queue.delete", QUEUE_DELETE, "tombstone a pending queue row"),
    tool("queue.redeliver", QUEUE_REDELIVER, "create a linked pending row"),
    tool("memory.save", MEMORY_SAVE, "insert one memory row"),
    tool("memory.find", MEMORY_FIND, "ranked memory search"),
    tool("graph.state", &[], "show active graph case state"),
    tool("graph.plan", GRAPH_PLAN, "record structured plan"),
    tool("graph.transition", GRAPH_TRANSITION, "request guarded graph transition"),
    tool("graph.context", GRAPH_CONTEXT, "select context packages"),
    tool("graph.note", GRAPH_NOTE, "record structured graph note"),
    tool("graph.evidence", GRAPH_EVIDENCE, "record evidence against known requirement"),
    tool("graph.compact", GRAPH_COMPACT, "request graph compaction checkpoint"),
    tool("workspace.summary", WORKSPACE_SUMMARY, "bounded workspace map"),
    tool("verify.cargo", VERIFY_CARGO, "run a direct cargo gate"),
    tool("verify.xtask", VERIFY_XTASK, "run a direct xtask gate"),
    tool("doc.scaffold", DOC_SCAFFOLD, "create compact README-indexed document tree"),
    tool("doc.audit", DOC_AUDIT, "audit document topology"),
    tool("agent.done", AGENT_DONE, "close the task or cycle"),
    tool("agent.ask", AGENT_ASK, "ask the owner"),
];

pub fn find_tool(name: &str) -> Option<&'static ToolSpec> {
    TOOLS.iter().find(|tool| tool.name == name)
}

const fn tool(
    name: &'static str,
    params: &'static [ParamSpec],
    contract: &'static str,
) -> ToolSpec {
    ToolSpec {
        name,
        params,
        contract,
    }
}

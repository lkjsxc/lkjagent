pub struct CommandGroup {
    pub name: &'static str,
    pub summary: &'static str,
    pub commands: &'static [CommandDoc],
}

pub struct CommandDoc {
    pub usage: &'static str,
    pub summary: &'static str,
}

pub const GROUPS: &[CommandGroup] = &[
    CommandGroup {
        name: "daemon",
        summary: "daemon and owner queue",
        commands: &[
            CommandDoc {
                usage: "run",
                summary: "start the daemon in the foreground",
            },
            CommandDoc {
                usage: "send <text>",
                summary: "enqueue an owner message",
            },
            CommandDoc {
                usage: "queue list [--limit N]",
                summary: "print queued owner messages",
            },
            CommandDoc {
                usage: "queue show <id>",
                summary: "print one queue row",
            },
        ],
    },
    CommandGroup {
        name: "observe",
        summary: "status, transcript, and terminal watch",
        commands: &[
            CommandDoc {
                usage: "status",
                summary: "print daemon and task status",
            },
            CommandDoc {
                usage: "log [--limit N] [--full] [--follow]",
                summary: "print transcript events",
            },
            CommandDoc {
                usage: "watch",
                summary: "open the owner terminal console",
            },
            CommandDoc {
                usage: "console",
                summary: "explicit name for the owner terminal console",
            },
        ],
    },
    CommandGroup {
        name: "work",
        summary: "task, graph, and memory inspection",
        commands: &[
            CommandDoc {
                usage: "task list [--status S] [--limit N]",
                summary: "print durable task cases",
            },
            CommandDoc {
                usage: "task show <id>",
                summary: "print one task case",
            },
            CommandDoc {
                usage: "graph",
                summary: "print graph state",
            },
            CommandDoc {
                usage: "memory <query>",
                summary: "search distilled memory",
            },
        ],
    },
    CommandGroup {
        name: "logs",
        summary: "model exchange logs",
        commands: &[CommandDoc {
            usage: "model-log [--print|list|show|export|raw-case]",
            summary: "inspect model handoff and provider exchanges",
        }],
    },
    CommandGroup {
        name: "personal",
        summary: "personal records",
        commands: &[
            CommandDoc {
                usage: "personal list [--kind K] [--status S] [--project P] [--limit N]",
                summary: "inspect personal records",
            },
            CommandDoc {
                usage: "personal render",
                summary: "regenerate personal Markdown projections",
            },
        ],
    },
];

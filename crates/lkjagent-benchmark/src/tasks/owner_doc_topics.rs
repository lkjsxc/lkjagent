use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const ROOT: &str = "# Docs\n\n## Purpose\n\nConnected multi-topic seed.\n\n## Table of Contents\n\n- [project/](project/README.md): lkjagent.\n- [model-interface/](model-interface/README.md): model endpoint.\n- [domain-examples/](domain-examples/README.md): Asia foods, Minecraft, Factorio.\n- [relations/](relations/README.md): relation graph.\n- [.lkj-doc-graph.md](.lkj-doc-graph.md): graph ledger.\n";
const README_PROJECT: &str = "# Project\n\n## Purpose\n\nProject contract.\n\n- [lkjagent.md](lkjagent.md): central project.\n";
const README_MODEL: &str = "# Model Interface\n\n## Purpose\n\nProvider-neutral model boundary.\n\n- [model-endpoint.md](model-endpoint.md): endpoint.\n";
const README_DOMAIN: &str = "# Domain Examples\n\n## Purpose\n\nExternal topics as objective examples.\n\n- [asia-foods.md](asia-foods.md): Asia foods.\n- [minecraft.md](minecraft.md): Minecraft.\n- [factorio.md](factorio.md): Factorio.\n";
const README_RELATIONS: &str = "# Relations\n\n## Purpose\n\nRelation index.\n\n- [project-model-domain-examples.md](project-model-domain-examples.md): requested topics.\n";
const GRAPH: &str = "# Document Graph\n\n## Nodes\n\n| id | path | role | status |\n| --- | --- | --- | --- |\n| root | README.md | root | seeded |\n\n## Edges\n\n| from | to | kind | reason |\n| --- | --- | --- | --- |\n| root | relation | tests | objective coverage |\n\n## Coverage\n\n| owner requirement | covered by | status |\n| --- | --- | --- |\n| all requested topics | relation page | satisfied |\n";
const RELATION: &str = "# Project, Model, and Domain Examples\n\n## Purpose\n\nConnect requested topics.\n\nlkjagent uses a provider-neutral model endpoint. Asia foods, Minecraft, and Factorio test objective-match audits and controlled documentation growth.\n";

const GOOD: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: ROOT,
    },
    FileSpec {
        path: "docs/.lkj-doc-graph.md",
        content: GRAPH,
    },
    FileSpec {
        path: "docs/project/README.md",
        content: README_PROJECT,
    },
    FileSpec {
        path: "docs/project/lkjagent.md",
        content: "# lkjagent\n\n## Purpose\n\nCentral project runtime.\n",
    },
    FileSpec {
        path: "docs/model-interface/README.md",
        content: README_MODEL,
    },
    FileSpec {
        path: "docs/model-interface/model-endpoint.md",
        content: "# Model Endpoint\n\n## Purpose\n\nProvider-neutral model endpoint.\n",
    },
    FileSpec {
        path: "docs/domain-examples/README.md",
        content: README_DOMAIN,
    },
    FileSpec {
        path: "docs/domain-examples/asia-foods.md",
        content: "# Asia Foods\n\n## Purpose\n\nScoped domain example.\n",
    },
    FileSpec {
        path: "docs/domain-examples/minecraft.md",
        content: "# Minecraft\n\n## Purpose\n\nScoped domain example.\n",
    },
    FileSpec {
        path: "docs/domain-examples/factorio.md",
        content: "# Factorio\n\n## Purpose\n\nScoped domain example.\n",
    },
    FileSpec {
        path: "docs/relations/README.md",
        content: README_RELATIONS,
    },
    FileSpec {
        path: "docs/relations/project-model-domain-examples.md",
        content: RELATION,
    },
];

const BAD_GENERIC: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: "# Docs\n\n## Purpose\n\nGeneric lkjagent docs.\n\n- [architecture/](architecture/README.md): architecture.\n- [guides/](guides/README.md): guides.\n",
    },
    FileSpec {
        path: "docs/.lkj-doc-graph.md",
        content: GRAPH,
    },
    FileSpec {
        path: "docs/architecture/README.md",
        content: "# Architecture\n\n## Purpose\n\nArchitecture only.\n",
    },
];

const BAD_BLURBS: &[FileSpec] = &[FileSpec {
    path: "docs/README.md",
    content:
        "# Docs\n\n## Purpose\n\nlkjagent, model endpoint, Asia foods, Minecraft, and Factorio.\n",
}];

const BAD_MOCK: &[FileSpec] = &[
    FileSpec {
        path: "docs/README.md",
        content: ROOT,
    },
    FileSpec {
        path: "docs/project/lkjagent.md",
        content: "# lkjagent\n\n## Purpose\n\nThis section contains concrete artifact content tied to the requested root.\n",
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "owner-docs-multi-topic-001",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Tiny,
    tags: &["owner-failure", "docs", "multi-topic"],
    prompt: "Create docs. about lkjagent, model endpoint, asia foods, minecraft, factorio",
    follow_up: None,
    starter_files: &[],
    good: &[Fixture {
        name: "connected-multi-topic-seed",
        files: GOOD,
    }],
    bad: &[
        Fixture {
            name: "generic-lkjagent-only-scaffold",
            files: BAD_GENERIC,
        },
        Fixture {
            name: "root-only-topic-blurbs",
            files: BAD_BLURBS,
        },
        Fixture {
            name: "mock-repeated-content",
            files: BAD_MOCK,
        },
    ],
    judge: JudgeKind::MultiTopicDocumentation,
    seed: 8123,
    points: 1,
    timeout_seconds: 120,
};

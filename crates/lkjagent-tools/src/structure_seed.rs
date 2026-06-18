use std::fs;
use std::path::Path;

use crate::error::ToolResult;

struct ReadmeSeed {
    path: &'static str,
    title: &'static str,
    purpose: &'static str,
    entries: &'static str,
}

struct LeafSeed {
    path: &'static str,
    title: &'static str,
    purpose: &'static str,
}

const README_SEEDS: &[ReadmeSeed] = &[
    readme("docs/README.md", "Docs", "Root index for recursive documentation.", "- [guides/](guides/README.md): task guides.\n- [api/](api/README.md): API contracts.\n- [architecture/](architecture/README.md): design records.\n- [operations/](operations/README.md): run and verify work.\n- [reference/](reference/README.md): shared terms."),
    readme("docs/guides/README.md", "Guides", "Workflow guidance by owner task.", "- [setup/](setup/README.md): setup path.\n- [usage/](usage/README.md): usage path.\n- [troubleshooting.md](troubleshooting.md): repair guide."),
    readme("docs/guides/setup/README.md", "Setup", "Setup guide index.", "- [install.md](install.md): install steps.\n- [configure.md](configure.md): configuration steps."),
    readme("docs/guides/usage/README.md", "Usage", "Usage guide index.", "- [workflow.md](workflow.md): primary workflow.\n- [handoff.md](handoff.md): handoff checklist."),
    readme("docs/api/README.md", "API", "API contract index.", "- [v1/](v1/README.md): version one.\n- [models.md](models.md): shared models."),
    readme("docs/api/v1/README.md", "API V1", "Versioned API index.", "- [users/](users/README.md): users endpoints.\n- [projects/](projects/README.md): projects endpoints."),
    readme("docs/api/v1/users/README.md", "Users API", "Users endpoint index.", "- [list.md](list.md): list endpoint.\n- [create.md](create.md): create endpoint."),
    readme("docs/api/v1/projects/README.md", "Projects API", "Projects endpoint index.", "- [list.md](list.md): list endpoint.\n- [archive.md](archive.md): archive endpoint."),
    readme("docs/architecture/README.md", "Architecture", "Architecture index.", "- [components/](components/README.md): components.\n- [decisions.md](decisions.md): decisions."),
    readme("docs/architecture/components/README.md", "Components", "Component index.", "- [core/](core/README.md): core component.\n- [integrations.md](integrations.md): integration notes."),
    readme("docs/architecture/components/core/README.md", "Core Component", "Core component index.", "- [lifecycle.md](lifecycle.md): lifecycle.\n- [state.md](state.md): state contract."),
    readme("docs/operations/README.md", "Operations", "Operations index.", "- [deployment/](deployment/README.md): deployment.\n- [verification.md](verification.md): verification."),
    readme("docs/operations/deployment/README.md", "Deployment", "Deployment index.", "- [environments.md](environments.md): environments.\n- [rollback.md](rollback.md): rollback."),
    readme("docs/reference/README.md", "Reference", "Reference index.", "- [glossary.md](glossary.md): terms.\n- [ownership.md](ownership.md): owners."),
];

const LEAF_SEEDS: &[LeafSeed] = &[
    leaf(
        "docs/guides/setup/install.md",
        "Install",
        "Install the project.",
    ),
    leaf(
        "docs/guides/setup/configure.md",
        "Configure",
        "Configure the project.",
    ),
    leaf(
        "docs/guides/usage/workflow.md",
        "Workflow",
        "Run the primary workflow.",
    ),
    leaf(
        "docs/guides/usage/handoff.md",
        "Handoff",
        "Transfer task state.",
    ),
    leaf(
        "docs/guides/troubleshooting.md",
        "Troubleshooting",
        "Repair common failures.",
    ),
    leaf("docs/api/models.md", "Models", "Shared API models."),
    leaf(
        "docs/api/v1/users/list.md",
        "List Users",
        "List users endpoint.",
    ),
    leaf(
        "docs/api/v1/users/create.md",
        "Create User",
        "Create user endpoint.",
    ),
    leaf(
        "docs/api/v1/projects/list.md",
        "List Projects",
        "List projects endpoint.",
    ),
    leaf(
        "docs/api/v1/projects/archive.md",
        "Archive Project",
        "Archive project endpoint.",
    ),
    leaf(
        "docs/architecture/decisions.md",
        "Decisions",
        "Architecture decisions.",
    ),
    leaf(
        "docs/architecture/components/integrations.md",
        "Integrations",
        "Integration notes.",
    ),
    leaf(
        "docs/architecture/components/core/lifecycle.md",
        "Lifecycle",
        "Core lifecycle.",
    ),
    leaf(
        "docs/architecture/components/core/state.md",
        "State",
        "Core state.",
    ),
    leaf(
        "docs/operations/verification.md",
        "Verification",
        "Verification gates.",
    ),
    leaf(
        "docs/operations/deployment/environments.md",
        "Environments",
        "Deployment environments.",
    ),
    leaf(
        "docs/operations/deployment/rollback.md",
        "Rollback",
        "Rollback procedure.",
    ),
    leaf("docs/reference/glossary.md", "Glossary", "Shared terms."),
    leaf("docs/reference/ownership.md", "Ownership", "Ownership map."),
];

pub fn scaffold_recursive_docs(workspace: &Path) -> ToolResult<String> {
    let mut created = 0usize;
    let mut skipped = 0usize;
    for seed in README_SEEDS {
        let content = readme_content(seed);
        write_missing(workspace, seed.path, &content, &mut created, &mut skipped)?;
    }
    for seed in LEAF_SEEDS {
        let content = leaf_content(seed);
        write_missing(workspace, seed.path, &content, &mut created, &mut skipped)?;
    }
    crate::structure::verify_recursive_tree(workspace)?;
    Ok(format!(
        "recursive docs scaffold root=docs\ncreated_files={created}\nskipped_existing={skipped}\nverification=ok"
    ))
}

const fn readme(
    path: &'static str,
    title: &'static str,
    purpose: &'static str,
    entries: &'static str,
) -> ReadmeSeed {
    ReadmeSeed {
        path,
        title,
        purpose,
        entries,
    }
}

const fn leaf(path: &'static str, title: &'static str, purpose: &'static str) -> LeafSeed {
    LeafSeed {
        path,
        title,
        purpose,
    }
}

fn readme_content(seed: &ReadmeSeed) -> String {
    format!(
        "# {}\n\n## Purpose\n\n{}\n\n## Table of Contents\n\n{}\n",
        seed.title, seed.purpose, seed.entries
    )
}

fn leaf_content(seed: &LeafSeed) -> String {
    format!(
        "# {}\n\n## Purpose\n\n{}\n\n## Status\n\ndraft.\n",
        seed.title, seed.purpose
    )
}

fn write_missing(
    workspace: &Path,
    relative: &str,
    content: &str,
    created: &mut usize,
    skipped: &mut usize,
) -> ToolResult<()> {
    let path = workspace.join(relative);
    if path.exists() {
        *skipped = skipped.saturating_add(1);
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    *created = created.saturating_add(1);
    Ok(())
}

use std::path::Path;

use super::model::{
    leaf, readme, write_catalog, write_leaves, write_readmes, Counts, LeafSeed, ReadmeSeed,
};
use crate::error::ToolResult;

const ROOT_LINKS: &str = "- [docs/README.md](README.md): docs root.";

const README_SEEDS: &[ReadmeSeed] = &[
    readme("docs/README.md", "Docs", "Root index for recursive documentation.", "- [guides/](guides/README.md): task guides.\n- [contracts/](contracts/README.md): interface contracts.\n- [architecture/](architecture/README.md): design records.\n- [operations/](operations/README.md): run and verify work.\n- [reference/](reference/README.md): shared terms.\n- [catalog.toml](catalog.toml): compact scaffold metadata."),
    readme("docs/guides/README.md", "Guides", "Workflow guidance by owner task.", "- [setup/](setup/README.md): setup path.\n- [usage/](usage/README.md): usage path.\n- [troubleshooting.md](troubleshooting.md): repair guide."),
    readme("docs/guides/setup/README.md", "Setup", "Setup guide index.", "- [install.md](install.md): install steps.\n- [configure.md](configure.md): configuration steps."),
    readme("docs/guides/usage/README.md", "Usage", "Usage guide index.", "- [workflow.md](workflow.md): primary workflow.\n- [handoff.md](handoff.md): handoff checklist."),
    readme("docs/contracts/README.md", "Contracts", "Interface contract index.", "- [actions/](actions/README.md): action contracts.\n- [data-model.md](data-model.md): shared data model."),
    readme("docs/contracts/actions/README.md", "Actions", "Action contract index.", "- [graph/](graph/README.md): graph actions.\n- [documents/](documents/README.md): document actions."),
    readme("docs/contracts/actions/graph/README.md", "Graph Actions", "Graph action index.", "- [state.md](state.md): state action.\n- [evidence.md](evidence.md): evidence action."),
    readme("docs/contracts/actions/documents/README.md", "Document Actions", "Document action index.", "- [scaffold.md](scaffold.md): scaffold action.\n- [audit.md](audit.md): audit action."),
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
        ROOT_LINKS,
    ),
    leaf(
        "docs/guides/setup/configure.md",
        "Configure",
        "Configure the project.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/guides/usage/workflow.md",
        "Workflow",
        "Run the primary workflow.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/guides/usage/handoff.md",
        "Handoff",
        "Transfer task state.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/guides/troubleshooting.md",
        "Troubleshooting",
        "Repair common failures.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/contracts/data-model.md",
        "Data Model",
        "Shared interface data model.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/contracts/actions/graph/state.md",
        "State Action",
        "Graph state action contract.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/contracts/actions/graph/evidence.md",
        "Evidence Action",
        "Graph evidence action contract.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/contracts/actions/documents/scaffold.md",
        "Scaffold Action",
        "Document scaffold action contract.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/contracts/actions/documents/audit.md",
        "Audit Action",
        "Document audit action contract.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/architecture/decisions.md",
        "Decisions",
        "Architecture decisions.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/architecture/components/integrations.md",
        "Integrations",
        "Integration notes.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/architecture/components/core/lifecycle.md",
        "Lifecycle",
        "Core lifecycle.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/architecture/components/core/state.md",
        "State",
        "Core state.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/operations/verification.md",
        "Verification",
        "Verification gates.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/operations/deployment/environments.md",
        "Environments",
        "Deployment environments.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/operations/deployment/rollback.md",
        "Rollback",
        "Rollback procedure.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/reference/glossary.md",
        "Glossary",
        "Shared terms.",
        ROOT_LINKS,
    ),
    leaf(
        "docs/reference/ownership.md",
        "Ownership",
        "Ownership map.",
        ROOT_LINKS,
    ),
];

pub fn scaffold(workspace: &Path) -> ToolResult<String> {
    let mut counts = Counts::default();
    write_readmes(workspace, README_SEEDS, &mut counts)?;
    write_leaves(workspace, LEAF_SEEDS, &mut counts)?;
    write_catalog(workspace, "generic", &mut counts)?;
    crate::structure::verify_recursive_tree(workspace)?;
    Ok(format!(
        "recursive docs scaffold profile=generic root=docs\ncatalog=docs/catalog.toml\ncreated_files={}\nskipped_existing={}\nverification=ok",
        counts.created, counts.skipped
    ))
}

use super::model::{PlannedFile, ScaffoldInput, ScaffoldPlan, ScaffoldProfile};
use super::semantic_seed_body as body;
use super::semantic_seed_domain as domain;
use super::semantic_seed_extra as extra;
use super::semantic_seed_select::requested_scopes;

pub fn plan(input: &ScaffoldInput) -> ScaffoldPlan {
    let (include_impl, include_domains) = requested_scopes(input);
    let mut files = base_files(include_impl, include_domains);
    if include_impl {
        add_implementation(&mut files);
    }
    if include_domains {
        add_domains(&mut files);
    }
    add_relations(&mut files, include_impl, include_domains);
    files.push(file(
        "catalog.toml",
        "Catalog",
        "scaffold metadata",
        "title = \"lkjagent Documentation\"\nkind = \"documentation\"\nprofile = \"LkjagentSemanticSeed\"\n".to_string(),
    ));
    ScaffoldPlan {
        root: input.root.clone(),
        profile: ScaffoldProfile::LkjagentSemanticSeed,
        files,
    }
}

fn base_files(include_impl: bool, include_domains: bool) -> Vec<PlannedFile> {
    vec![
        file(
            "README.md",
            "lkjagent Documentation",
            "root",
            extra::root(include_impl, include_domains),
        ),
        file(
            "project/README.md",
            "Project",
            "project",
            extra::project_readme(),
        ),
        file(
            "project/lkjagent.md",
            "lkjagent",
            "central project",
            extra::lkjagent(),
        ),
        file(
            "project/purpose.md",
            "Purpose",
            "project purpose",
            extra::project_purpose(),
        ),
        file(
            "project/operating-model.md",
            "Operating Model",
            "project model",
            extra::project_operating_model(),
        ),
        file(
            "model-interface/README.md",
            "Model Interface",
            "model",
            extra::model_readme(),
        ),
        file(
            "model-interface/model-endpoint.md",
            "Model Endpoint",
            "endpoint",
            extra::model_endpoint(),
        ),
        file(
            "model-interface/contract.md",
            "Contract",
            "contract",
            extra::model_contract(),
        ),
        file(
            "model-interface/action-protocol.md",
            "Action Protocol",
            "action protocol",
            extra::action_protocol(),
        ),
    ]
}

fn add_implementation(files: &mut Vec<PlannedFile>) {
    files.extend([
        file(
            "implementation/README.md",
            "Implementation",
            "Rust",
            body::implementation_readme(),
        ),
        file("implementation/rust.md", "Rust", "Rust", body::rust_page()),
        file(
            "implementation/functional-core.md",
            "Functional Core",
            "core",
            body::functional_core(),
        ),
    ]);
}

fn add_domains(files: &mut Vec<PlannedFile>) {
    files.extend([
        file(
            "domain-examples/README.md",
            "Domain Examples",
            "domains",
            domain::domain_readme(),
        ),
        file(
            "domain-examples/asia-foods.md",
            "Asia Foods",
            "domain",
            domain::asia_foods(),
        ),
        file(
            "domain-examples/minecraft.md",
            "Minecraft",
            "domain",
            domain::minecraft(),
        ),
        file(
            "domain-examples/factorio.md",
            "Factorio",
            "domain",
            domain::factorio(),
        ),
    ]);
}

fn add_relations(files: &mut Vec<PlannedFile>, include_impl: bool, include_domains: bool) {
    files.extend([
        file(
            "relations/README.md",
            "Relations",
            "relation index",
            extra::relations_readme(include_impl, include_domains),
        ),
        file(
            "relations/project-model-interface.md",
            "Project Model Interface",
            "project model relation",
            extra::project_model_interface(),
        ),
        file(
            "relations/state-to-prompt.md",
            "State To Prompt",
            "state prompt relation",
            extra::state_to_prompt(),
        ),
    ]);
    if include_impl {
        files.push(file(
            "relations/project-model-implementation.md",
            "Project Model Implementation",
            "project implementation relation",
            body::relation_main(),
        ));
    }
    if include_domains {
        files.push(file(
            "relations/project-model-domain-examples.md",
            "Project Model Domain Examples",
            "project domain relation",
            domain::project_model_domain_examples(),
        ));
    }
}

fn file(path: &str, title: &str, role: &str, body: String) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body,
    }
}

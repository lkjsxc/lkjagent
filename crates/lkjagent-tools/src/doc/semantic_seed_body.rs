pub fn root() -> String {
    doc(
        "lkjagent Documentation",
        "This root maps the project runtime, model interface, Rust implementation, and relations.",
        &[
            "[project/](project/README.md): lkjagent purpose and operating model.",
            "[model-interface/](model-interface/README.md): provider-neutral model boundary.",
            "[implementation/](implementation/README.md): Rust substrate and core boundary.",
            "[relations/](relations/README.md): runtime, model, and implementation links.",
            "[.lkj-doc-graph.md](.lkj-doc-graph.md): machine-readable relation ledger.",
        ],
        "Read project first, then model-interface, implementation, and relations.",
    )
}

pub fn project_readme() -> String {
    doc(
        "Project",
        "Project docs own what lkjagent is and how the daemon works.",
        &[
            "[purpose.md](purpose.md): central runtime purpose.",
            "[operating-model.md](operating-model.md): daemon, queue, graph, and evidence loop.",
        ],
        "The project depends on the model interface and Rust implementation.",
    )
}

pub fn project_purpose() -> String {
    leaf(
        "Purpose",
        "lkjagent is a graph-governed local agent operating system.",
        &[
            "It keeps durable cases, typed transitions, evidence, and completion gates.",
            "It treats the model endpoint as an action proposer, not an authority.",
            "Rust modules implement pure reducers while tools keep effects at the edge.",
        ],
    )
}

pub fn project_operating_model() -> String {
    leaf(
        "Operating Model",
        "The daemon advances one case through state-owned turns.",
        &[
            "Owner messages enter a persistent queue before mutation.",
            "Prompt frames name active state, guards, allowed tools, and missing evidence.",
            "After each observation the reducer updates state and audit requirements.",
        ],
    )
}

pub fn model_readme() -> String {
    doc(
        "Model Interface",
        "The model boundary is provider-neutral and runtime-owned.",
        &[
            "[contract.md](contract.md): action proposal pipeline.",
            "[action-protocol.md](action-protocol.md): tag grammar and validation boundary.",
        ],
        "The boundary is constrained by project state and observed by audits.",
    )
}

pub fn model_contract() -> String {
    leaf(
        "Contract",
        "Raw model text never executes directly.",
        &[
            "The parser extracts one action candidate.",
            "The schema validator normalizes parameters and rejects unknown fields.",
            "Graph authorization and guard tracks decide whether an effect can run.",
        ],
    )
}

pub fn action_protocol() -> String {
    leaf(
        "Action Protocol",
        "The action language is small and deterministic.",
        &[
            "One action block proposes one tool intent.",
            "Large Markdown bodies use line-oriented file blocks.",
            "Parse recovery narrows the next prompt to a small valid action.",
        ],
    )
}

pub fn implementation_readme() -> String {
    doc(
        "Implementation",
        "Implementation docs own Rust as the substrate for runtime authority.",
        &[
            "[rust.md](rust.md): Rust ownership in the workspace.",
            "[functional-core.md](functional-core.md): pure reducer and effects boundary.",
        ],
        "Implementation supports typed state, tool schemas, and verification gates.",
    )
}

pub fn rust_page() -> String {
    leaf(
        "Rust",
        "Rust provides typed state, explicit errors, and workspace gates.",
        &[
            "Newtypes identify cases, tasks, evidence, tracks, and tool intents.",
            "Pure functions update state without filesystem, shell, queue, or model effects.",
            "Docker Compose verifies the workspace from a clean build context.",
        ],
    )
}

pub fn functional_core() -> String {
    leaf(
        "Functional Core",
        "The core returns decisions while adapters perform effects.",
        &[
            "Reducers transform events into state vectors and hard states.",
            "Authorization returns a decision instead of executing a tool.",
            "Audits record owned evidence after deterministic checks pass.",
        ],
    )
}

pub fn relations_readme() -> String {
    doc(
        "Relations",
        "Relation docs connect requested topics before expansion.",
        &[
            "[project-model-implementation.md](project-model-implementation.md): central relation.",
            "[state-to-prompt.md](state-to-prompt.md): state selected prompt relation.",
        ],
        "Every requested topic has an outgoing relation and backlink.",
    )
}

pub fn relation_main() -> String {
    leaf(
        "Project Model Implementation",
        "lkjagent connects runtime, model boundary, and Rust.",
        &[
            "The project depends on Rust for typed reducers and tool schemas.",
            "The project constrains the model through prompt frames and action validation.",
            "Rust implementation audits the evidence needed before completion.",
        ],
    )
}

pub fn relation_state_prompt() -> String {
    leaf(
        "State To Prompt",
        "State tracks select prompt mode and context slices.",
        &[
            "Hard state selects the legal phase and allowed tools.",
            "Weighted guards add completion blockers and recovery instructions.",
            "The prompt frame exposes the evidence needed for the next safe action.",
        ],
    )
}

fn doc(title: &str, purpose: &str, entries: &[&str], map: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Table of Contents\n\n- {}\n\n## Local Map\n\n{map}\n\n## Status\n\nimplemented\n",
        entries.join("\n- ")
    )
}

fn leaf(title: &str, purpose: &str, facts: &[&str]) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Contract\n\n- {}\n\n## Links\n\n- Relation: ../relations/project-model-implementation.md.\n- Graph ledger: ../.lkj-doc-graph.md.\n\n## Status\n\nimplemented\n",
        facts.join("\n- ")
    )
}

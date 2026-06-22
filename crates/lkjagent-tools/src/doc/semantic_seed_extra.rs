pub fn root(include_impl: bool, include_domains: bool) -> String {
    let mut entries = vec![
        "[project/](project/README.md): lkjagent purpose and operating model.",
        "[model-interface/](model-interface/README.md): provider-neutral model boundary.",
    ];
    if include_impl {
        entries
            .push("[implementation/](implementation/README.md): Rust substrate and core boundary.");
    }
    if include_domains {
        entries.push("[domain-examples/](domain-examples/README.md): scoped external domains used for objective tests.");
    }
    entries.push("[relations/](relations/README.md): runtime, model, and topic links.");
    entries.push("[.lkj-doc-graph.md](.lkj-doc-graph.md): machine-readable relation ledger.");
    doc(
        "lkjagent Documentation",
        "This root maps the project runtime, model interface, requested domains, and relations.",
        entries,
        "Read project first, then model-interface, optional domains, and relations.",
    )
}

pub fn project_readme() -> String {
    doc(
        "Project",
        "Project docs own what lkjagent is and how the daemon works.",
        vec![
            "[lkjagent.md](lkjagent.md): central runtime subject.",
            "[purpose.md](purpose.md): project purpose and boundary.",
            "[operating-model.md](operating-model.md): daemon, queue, graph, and evidence loop.",
        ],
        "The project constrains the model interface and owns completion gates.",
    )
}

pub fn lkjagent() -> String {
    leaf(
        "lkjagent",
        "lkjagent is the central runtime subject for this documentation seed.",
        vec![
            "It manages durable cases, hard state, weighted guards, evidence, and completion gates.",
            "It treats model text as a proposal that must pass parsing, schemas, and authorization.",
            "It uses documentation and audits as implementation contracts, not decoration.",
        ],
        "../relations/project-model-interface.md",
    )
}

pub fn project_purpose() -> String {
    leaf(
        "Purpose",
        "lkjagent exists to make local model-guided work stateful, auditable, and verified.",
        vec![
            "The runtime preserves owner objectives before planning artifacts.",
            "The graph guides context and actions while runtime authority admits effects.",
            "Completion requires owned evidence instead of memory-only or topology-only claims.",
        ],
        "../relations/project-model-interface.md",
    )
}

pub fn project_operating_model() -> String {
    leaf(
        "Operating Model",
        "The daemon advances one case through state-owned turns.",
        vec![
            "Owner messages enter a persistent queue before mutation.",
            "Prompt frames name active state, guards, allowed tools, and missing evidence.",
            "After each observation the reducer updates state and audit requirements.",
        ],
        "../relations/project-model-interface.md",
    )
}

pub fn model_readme() -> String {
    doc(
        "Model Interface",
        "The model boundary is provider-neutral and runtime-owned.",
        vec![
            "[model-endpoint.md](model-endpoint.md): configured endpoint boundary.",
            "[contract.md](contract.md): action proposal pipeline.",
            "[action-protocol.md](action-protocol.md): tag grammar and validation boundary.",
        ],
        "The boundary is constrained by project state and observed by audits.",
    )
}

pub fn model_endpoint() -> String {
    leaf(
        "Model Endpoint",
        "The configured model endpoint proposes bounded actions only.",
        vec![
            "Provider-specific names stay out of durable generic docs unless an adapter page owns them.",
            "The endpoint receives a compiled prompt frame instead of a raw transcript replay.",
            "Raw output is parsed, validated, authorized, executed, and reduced before state changes.",
        ],
        "../relations/project-model-interface.md",
    )
}

pub fn model_contract() -> String {
    leaf(
        "Contract",
        "Raw model text never executes directly.",
        vec![
            "The parser extracts one action candidate.",
            "The schema validator normalizes parameters and rejects unknown fields.",
            "Graph authorization and guard tracks decide whether an effect can run.",
        ],
        "../relations/project-model-interface.md",
    )
}

pub fn action_protocol() -> String {
    leaf(
        "Action Protocol",
        "The action language is small and deterministic.",
        vec![
            "One action block proposes one tool intent.",
            "Large Markdown bodies use line-oriented file blocks.",
            "Parse recovery narrows the next prompt to a small valid action.",
        ],
        "../relations/project-model-interface.md",
    )
}

pub fn relations_readme(include_impl: bool, include_domains: bool) -> String {
    let mut entries = vec![
        "[project-model-interface.md](project-model-interface.md): central runtime and model relation.",
        "[state-to-prompt.md](state-to-prompt.md): state selected prompt relation.",
    ];
    if include_impl {
        entries.push("[project-model-implementation.md](project-model-implementation.md): Rust implementation relation.");
    }
    if include_domains {
        entries.push("[project-model-domain-examples.md](project-model-domain-examples.md): requested domains relation.");
    }
    doc(
        "Relations",
        "Relation docs connect requested topics before expansion.",
        entries,
        "Every requested topic has an outgoing relation and a backlink.",
    )
}

pub fn project_model_interface() -> String {
    leaf(
        "Project Model Interface",
        "lkjagent constrains the model endpoint through runtime authority.",
        vec![
            "The project implements the case, evidence, audit, and completion contract.",
            "The model endpoint proposes one bounded action inside the prompt grammar.",
            "Authorization blocks unsupported claims, repeated actions, and premature completion.",
        ],
        "../project/lkjagent.md",
    )
}

pub fn state_to_prompt() -> String {
    leaf(
        "State To Prompt",
        "State tracks select prompt mode and context slices.",
        vec![
            "Hard state selects the legal phase and allowed tools.",
            "Weighted guards add completion blockers and recovery instructions.",
            "The prompt frame exposes the evidence needed for the next safe action.",
        ],
        "../project/lkjagent.md",
    )
}

fn doc(title: &str, purpose: &str, entries: Vec<&str>, map: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Table of Contents\n\n- {}\n\n## Local Map\n\n{map}\n\n## Status\n\nimplemented\n",
        entries.join("\n- ")
    )
}

fn leaf(title: &str, purpose: &str, facts: Vec<&str>, relation: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Contract\n\n- {}\n\n## Links\n\n- Relation: {relation}.\n- Graph ledger: ../.lkj-doc-graph.md.\n\n## Status\n\nimplemented\n",
        facts.join("\n- ")
    )
}

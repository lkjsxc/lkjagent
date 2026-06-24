use lkjagent_protocol::{
    parse_completion, parse_live_completion, render_action, render_graph, render_notice,
    render_observation, render_owner, Action, EnvelopeMode, Param, ParseFault,
};

#[test]
fn parses_clean_turns_and_round_trips_rendered_actions() {
    let fixtures = vec![
        (
            "<action>\n<tool>fs.read</tool>\n<path>README.md</path>\n<count>10</count>\n</action>",
            Action::new("fs.read", vec![Param::new("path", "README.md"), Param::new("count", "10")]),
        ),
        (
            "<action>\n<tool>fs.write</tool>\n<path>notes.md</path>\n<content>\nline one\n<action>\n  </content>\nline two\n</content>\n</action>",
            Action::new(
                "fs.write",
                vec![
                    Param::new("path", "notes.md"),
                    Param::new("content", "line one\n<action>\n  </content>\nline two"),
                ],
            ),
        ),
        (
            "<action>   \n<tool>fs.read</tool>   \n<path> notes.md </path>\n</action>   ",
            Action::new("fs.read", vec![Param::new("path", " notes.md ")]),
        ),
        (
            "<action>\n<tool>shell.run</tool>\n<timeout>20</timeout>\n<command>cargo test</command>\n</action>",
            Action::new("shell.run", vec![Param::new("timeout", "20"), Param::new("command", "cargo test")]),
        ),
    ];

    for (text, expected) in fixtures {
        assert_eq!(parse_completion(text), Ok(expected.clone()));
        assert_eq!(parse_completion(&render_action(&expected)), Ok(expected));
    }
}

#[test]
fn produces_each_parse_fault_variant() {
    let cases = vec![
        ("no action here", ParseFault::MissingActionEnvelope),
        (
            "<action>\n<tool>agent.done</tool>\n<summary>x</summary>\n</action>\n<action>\n<tool>agent.done</tool>\n<summary>y</summary>\n</action>",
            ParseFault::MultipleActionEnvelopes,
        ),
        ("<action>\n<path>README.md</path>\n</action>", ParseFault::MissingTool),
        (
            "<action>\n<tool>missing.tool</tool>\n</action>",
            ParseFault::UnknownTool {
                tool: "missing.tool".to_string(),
            },
        ),
        (
            "<action>\n<tool>fs.write</tool>\n<path>x</path>\n<content>\nbody\n</action>",
            ParseFault::UnclosedTag {
                tag: "content".to_string(),
            },
        ),
        (
            "<action>\n<tool>fs.read</tool>\n<path>a</path>\n<path>b</path>\n</action>",
            ParseFault::DuplicateParam {
                name: "path".to_string(),
            },
        ),
        (
            "<action>\n<tool>fs.read</tool>\n<bogus>x</bogus>\n<extra>y</extra>\n</action>",
            ParseFault::BadParams {
                tool: "fs.read".to_string(),
                missing: vec!["path".to_string()],
                unknown: vec!["bogus".to_string(), "extra".to_string()],
            },
        ),
        (
            "<action>\n<tool>shell.run</tool>\n<timeout>30</timeout>\n</action>",
            ParseFault::BadParams {
                tool: "shell.run".to_string(),
                missing: vec!["command".to_string()],
                unknown: vec![],
            },
        ),
    ];

    for (text, fault) in cases {
        assert_eq!(parse_completion(text), Err(fault));
    }
}

#[test]
fn attribute_like_tag_is_dedicated_fault() {
    let text = "<action>\n<tool>graph.plan</tool>\n<objective>Create structured science-fiction story bible for Chronos Fracture.</objective>\n<steps>Record plan, write bounded batches, audit readiness.</steps>\n<path=stories/chronos-fracture</path>\n<reason>Owner requires a story bible.</reason>\n</action>";

    assert_eq!(
        parse_completion(text),
        Err(ParseFault::AttributeLikeTag {
            tag_name: "path=stories/chronos-fracture".to_string(),
            value_hint: Some("stories/chronos-fracture".to_string()),
        })
    );
}

#[test]
fn graph_plan_missing_paths_or_checks_is_bad_params() {
    let text = "<action>\n<tool>graph.plan</tool>\n<objective>Create structured story bible.</objective>\n<steps>Record plan.</steps>\n<reason>Owner requested it.</reason>\n</action>";

    assert_eq!(
        parse_completion(text),
        Err(ParseFault::BadParams {
            tool: "graph.plan".to_string(),
            missing: vec!["checks|paths".to_string()],
            unknown: Vec::new(),
        })
    );
}

#[test]
fn parses_line_action_grammar_and_file_blocks() {
    assert_eq!(
        parse_completion("<action>\ntool: doc.audit\nroot: docs\n</action>"),
        Ok(Action::new("doc.audit", vec![Param::new("root", "docs")]))
    );

    let batch = "<action>\ntool: fs.batch_write\ncase: current\nfiles:\n-- file --\npath: notes/food.md\ncontent:\n# Food\n\nRice notes.\n-- end-file --\n-- file --\npath: notes/code.md\ncontent:\n```text\n<action>literal</action>\n```\n-- end-file --\n</action>";
    assert_eq!(
        parse_completion(batch),
        Ok(Action::new(
            "fs.batch_write",
            vec![Param::new(
                "files",
                "path: notes/food.md\ncontent:\n# Food\n\nRice notes.\n-- lkjagent-next-file --\npath: notes/code.md\ncontent:\n```text\n<action>literal</action>\n```"
            )]
        ))
    );
}

#[test]
fn implicit_envelope_accepts_exact_tool_body() {
    let outcome = parse_live_completion("<tool>graph.state</tool>", Default::default());

    assert_eq!(outcome.action, Some(Action::new("graph.state", Vec::new())));
    assert_eq!(outcome.fault, None);
    assert_eq!(outcome.envelope_mode, EnvelopeMode::Implicit);

    let line = parse_live_completion("tool: graph.state", Default::default());
    assert_eq!(line.action, Some(Action::new("graph.state", Vec::new())));
    assert_eq!(line.envelope_mode, EnvelopeMode::Implicit);
}

#[test]
fn prose_without_action_remains_missing_envelope() {
    let text = "I will inspect the graph state next and then continue.";
    assert_eq!(
        parse_completion(text),
        Err(ParseFault::MissingActionEnvelope)
    );
}

#[test]
fn top_level_json_is_rejected_for_live_parser() {
    assert_eq!(
        parse_completion(r#"{"action":{"tool":"graph.state"}}"#),
        Err(ParseFault::JsonActionRejected)
    );
}

#[test]
fn renders_context_frames() {
    assert_eq!(
        render_observation("ok", "done"),
        "<observation>\n<status>ok</status>\n<content>done</content>\n</observation>"
    );
    assert_eq!(
        render_notice("error", "bad params"),
        "<notice>\n<kind>error</kind>\n<content>bad params</content>\n</notice>"
    );
    assert_eq!(render_owner("hello"), "<owner>\nhello\n</owner>");
    assert_eq!(
        render_graph("case=new\nphase=planning"),
        "<graph>\ncase=new\nphase=planning\n</graph>"
    );
}

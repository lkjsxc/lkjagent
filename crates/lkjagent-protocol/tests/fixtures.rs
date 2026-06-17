use lkjagent_protocol::{
    parse_completion, render_action, render_notice, render_observation, render_owner, render_skill,
    Action, Param, ParseFault,
};

#[test]
fn parses_clean_turns_and_round_trips_rendered_actions() {
    let fixtures = vec![
        (
            "<think>\nread first\n</think>\n<act>\n<tool>fs.read</tool>\n<path>README.md</path>\n<count>10</count>\n</act>",
            Action::new("fs.read", vec![Param::new("path", "README.md"), Param::new("count", "10")]),
        ),
        (
            "<act>\n<tool>fs.write</tool>\n<path>notes.md</path>\n<content>\nline one\n<act>\n  </content>\nline two\n</content>\n</act>",
            Action::new(
                "fs.write",
                vec![
                    Param::new("path", "notes.md"),
                    Param::new("content", "line one\n<act>\n  </content>\nline two"),
                ],
            ),
        ),
        (
            "<act>   \n<tool>fs.read</tool>   \n<path> notes.md </path>\n</act>   ",
            Action::new("fs.read", vec![Param::new("path", " notes.md ")]),
        ),
        (
            "<act>\n<tool>shell.run</tool>\n<timeout>20</timeout>\n<command>cargo test</command>\n</act>",
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
        ("no act here", ParseFault::MissingAct),
        (
            "<act>\n<tool>agent.done</tool>\n<summary>x</summary>\n</act>\n<act>\n<tool>agent.done</tool>\n<summary>y</summary>\n</act>",
            ParseFault::MultipleAct,
        ),
        ("<act>\n<path>README.md</path>\n</act>", ParseFault::MissingTool),
        (
            "<act>\n<tool>missing.tool</tool>\n</act>",
            ParseFault::UnknownTool {
                tool: "missing.tool".to_string(),
            },
        ),
        (
            "<act>\n<tool>fs.write</tool>\n<path>x</path>\n<content>\nbody\n</act>",
            ParseFault::UnclosedTag {
                tag: "content".to_string(),
            },
        ),
        (
            "<act>\n<tool>fs.read</tool>\n<path>a</path>\n<path>b</path>\n</act>",
            ParseFault::DuplicateParam {
                name: "path".to_string(),
            },
        ),
        (
            "<act>\n<tool>fs.read</tool>\n<bogus>x</bogus>\n<extra>y</extra>\n</act>",
            ParseFault::BadParams {
                missing: vec!["path".to_string()],
                unknown: vec!["bogus".to_string(), "extra".to_string()],
            },
        ),
    ];

    for (text, fault) in cases {
        assert_eq!(parse_completion(text), Err(fault));
    }
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
        render_skill("# Skill: demo"),
        "<skill>\n# Skill: demo\n</skill>"
    );
}

use lkjagent_protocol::{parse_completion, Action, Param, ParseFault};

#[test]
fn parses_json_action_envelope() {
    let text = r##"{
  "schema": "lkj-action",
  "action": {
    "tool": "fs.write",
    "params": { "path": "docs/a.md", "content": "# A\n\n## Purpose\n\nA." }
  }
}"##;

    assert_eq!(
        parse_completion(text),
        Ok(Action::new(
            "fs.write",
            vec![
                Param::new("content", "# A\n\n## Purpose\n\nA."),
                Param::new("path", "docs/a.md"),
            ]
        ))
    );
}

#[test]
fn converts_json_batch_files_to_line_protocol() {
    let text = r#"{
  "action": {
    "tool": "fs.batch_write",
    "params": {
      "files": [
        { "path": "a.md", "content": "A" },
        { "path": "b.md", "content": "B" }
      ]
    }
  }
}"#;

    assert_eq!(
        parse_completion(text),
        Ok(Action::new(
            "fs.batch_write",
            vec![Param::new(
                "files",
                "path: a.md\ncontent:\nA\n-- lkjagent-next-file --\npath: b.md\ncontent:\nB"
            )]
        ))
    );
}

#[test]
fn rejects_unknown_json_envelope_fields() {
    let text = r#"{
  "schema_version": "bad",
  "action": { "tool": "graph.state", "params": {} }
}"#;

    assert!(matches!(
        parse_completion(text),
        Err(ParseFault::BadEnvelope { reason }) if reason.contains("schema_version")
    ));
}

#[test]
fn rejects_json_unknown_params_with_typed_fault() {
    let text = r#"{
  "action": { "tool": "fs.read", "params": { "path": "a.md", "bogus": true } }
}"#;

    assert_eq!(
        parse_completion(text),
        Err(ParseFault::BadParams {
            tool: "fs.read".to_string(),
            missing: Vec::new(),
            unknown: vec!["bogus".to_string()],
        })
    );
}

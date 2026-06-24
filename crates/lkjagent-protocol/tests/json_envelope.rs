use lkjagent_protocol::{parse_completion, ParseFault};

#[test]
fn rejects_json_action_envelope_in_live_parser() {
    let text = r##"{
  "schema": "lkj-action",
  "action": {
    "tool": "fs.write",
    "params": { "path": "docs/a.md", "content": "# A\n\n## Purpose\n\nA." }
  }
}"##;

    assert_eq!(parse_completion(text), Err(ParseFault::JsonActionRejected));
}

#[test]
fn rejects_json_batch_files_in_live_parser() {
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

    assert_eq!(parse_completion(text), Err(ParseFault::JsonActionRejected));
}

#[test]
fn rejects_malformed_json_as_json_action_output() {
    assert_eq!(
        parse_completion(r#"{"action": { "tool": "graph.state" }"#),
        Err(ParseFault::JsonActionRejected)
    );
}

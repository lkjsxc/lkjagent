use lkjagent_protocol::{parse_completion, Action, Param};

#[test]
fn opening_parameter_tag_may_start_value_on_same_line() {
    let text = "<action>\n<tool>fs.write</tool>\n<path>stories/chronos-fracture/project/premise.md</path>\n<content># Premise\nIn 2342 the Chronos Fracture broke causal time.\n</content>\n</action>";

    assert_eq!(
        parse_completion(text),
        Ok(Action::new(
            "fs.write",
            vec![
                Param::new("path", "stories/chronos-fracture/project/premise.md"),
                Param::new(
                    "content",
                    "# Premise\nIn 2342 the Chronos Fracture broke causal time."
                ),
            ]
        ))
    );
}

#[test]
fn batch_write_merges_repeated_files_wrappers() {
    let text = "<action>\n<tool>fs.batch_write</tool>\n<files>\npath: a.md\ncontent:\n# A\n</files>\n</files>\n<files>\npath: b.md\ncontent:\n# B\n</files>\n</files>\n</action>";

    assert_eq!(
        parse_completion(text),
        Ok(Action::new(
            "fs.batch_write",
            vec![Param::new(
                "files",
                "path: a.md\ncontent:\n# A\n-- lkjagent-next-file --\npath: b.md\ncontent:\n# B"
            )]
        ))
    );
}

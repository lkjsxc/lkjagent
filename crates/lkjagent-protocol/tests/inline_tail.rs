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

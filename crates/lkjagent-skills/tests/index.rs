use lkjagent_skills::index::{render_index, IndexEntry};

#[test]
fn index_degrades_oldest_use_stamps_deterministically() {
    let entries = vec![
        entry("alpha-skill", "one two three", 10),
        entry("beta-skill", "one two three", 1),
        entry("gamma-skill", "one two three", 5),
    ];
    let lines = render_index(&entries, 9);
    assert_eq!(
        lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>(),
        vec![
            "alpha-skill: one two three",
            "beta-skill",
            "gamma-skill: one two three",
        ]
    );
    assert_eq!(
        lines
            .iter()
            .map(|line| (line.name.as_str(), line.degraded))
            .collect::<Vec<_>>(),
        vec![
            ("alpha-skill", false),
            ("beta-skill", true),
            ("gamma-skill", false),
        ]
    );
}

fn entry(name: &str, trigger: &str, use_stamp: u64) -> IndexEntry {
    IndexEntry {
        name: name.to_string(),
        trigger: trigger.to_string(),
        use_stamp,
    }
}

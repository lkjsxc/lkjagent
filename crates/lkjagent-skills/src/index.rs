use crate::model::Skill;

pub const SKILL_INDEX_BUDGET: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexEntry {
    pub name: String,
    pub trigger: String,
    pub use_stamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexLine {
    pub name: String,
    pub text: String,
    pub degraded: bool,
}

pub fn entry_from_skill(skill: &Skill, use_stamp: u64) -> IndexEntry {
    IndexEntry {
        name: skill.name.clone(),
        trigger: skill.trigger.clone(),
        use_stamp,
    }
}

pub fn render_index(entries: &[IndexEntry], budget: usize) -> Vec<IndexLine> {
    let mut lines: Vec<IndexLine> = entries
        .iter()
        .map(|entry| IndexLine {
            name: entry.name.clone(),
            text: full_text(entry),
            degraded: false,
        })
        .collect();
    lines.sort_by(|left, right| left.name.cmp(&right.name));

    let mut degrade_order: Vec<&IndexEntry> = entries.iter().collect();
    degrade_order.sort_by(|left, right| {
        left.use_stamp
            .cmp(&right.use_stamp)
            .then_with(|| left.name.cmp(&right.name))
    });
    for entry in degrade_order {
        if token_count(&lines) <= budget {
            break;
        }
        if let Some(line) = lines.iter_mut().find(|line| line.name == entry.name) {
            line.text = entry.name.clone();
            line.degraded = true;
        }
    }
    lines
}

pub fn render_index_text(entries: &[IndexEntry]) -> String {
    render_index(entries, SKILL_INDEX_BUDGET)
        .into_iter()
        .map(|line| line.text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn full_text(entry: &IndexEntry) -> String {
    format!("{}: {}", entry.name, entry.trigger)
}

fn token_count(lines: &[IndexLine]) -> usize {
    lines
        .iter()
        .map(|line| line.text.split_whitespace().count())
        .sum()
}

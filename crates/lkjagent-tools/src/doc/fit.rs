use super::model::ShapeGroup;

pub fn exact_group_count(groups: &[ShapeGroup], target: usize) -> usize {
    (2..=groups.len())
        .rev()
        .find(|count| {
            let selected = &groups[..*count];
            let base = 1usize.saturating_add(*count).saturating_add(*count * 2);
            let cap = 1usize.saturating_add(*count).saturating_add(
                selected
                    .iter()
                    .map(|group| group.leaves.len())
                    .sum::<usize>(),
            );
            base <= target && target <= cap
        })
        .unwrap_or(2)
}

pub fn max_tree_count(groups: &[ShapeGroup]) -> usize {
    2usize
        .saturating_add(groups.len())
        .saturating_add(groups.iter().map(|group| group.leaves.len()).sum::<usize>())
}

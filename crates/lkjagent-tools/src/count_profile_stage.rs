use crate::count_profile::Language;

pub(crate) fn stage_name(language: Language, index: usize, total: usize) -> &'static str {
    stage_label(language, stage_slot(index, total))
}

pub(crate) fn stage_label(language: Language, slot: usize) -> &'static str {
    let slot = slot.min(5);
    match language {
        Language::Japanese => JP_STAGES[slot],
        Language::English => EN_STAGES[slot],
    }
}

pub(crate) fn stage_range(total: usize, slot: usize) -> Option<(usize, usize)> {
    let mut start = None;
    let mut end = None;
    for index in 1..=total {
        if stage_slot(index, total) == slot {
            start.get_or_insert(index);
            end = Some(index);
        }
    }
    start.zip(end)
}

fn stage_slot(index: usize, total: usize) -> usize {
    index
        .saturating_sub(1)
        .saturating_mul(6)
        .checked_div(total.max(1))
        .unwrap_or(0)
        .min(5)
}

const JP_STAGES: [&str; 6] = ["導入", "探索", "対立拡大", "中盤反転", "危機", "収束"];
const EN_STAGES: [&str; 6] = [
    "opening",
    "exploration",
    "rising conflict",
    "midpoint reversal",
    "crisis",
    "resolution",
];

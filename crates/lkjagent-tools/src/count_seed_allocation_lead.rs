use crate::count_number::Span;

pub(crate) fn allocation_lead_before(text: &str, number: Span) -> bool {
    let start = number.start.saturating_sub(32);
    text.get(start..number.start).is_some_and(|prefix| {
        let lower = prefix.to_lowercase();
        lower.split_whitespace().any(allocation_lead_word)
    })
}

fn allocation_lead_word(word: &str) -> bool {
    matches!(
        word,
        "with"
            | "including"
            | "include"
            | "containing"
            | "contains"
            | "split"
            | "divide"
            | "dividing"
            | "divided"
            | "allocate"
            | "allocated"
            | "assign"
            | "assigned"
            | "use"
            | "using"
            | "prepare"
            | "prepared"
            | "write"
            | "written"
    )
}

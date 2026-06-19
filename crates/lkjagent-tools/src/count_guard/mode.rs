use super::CountMode;
use crate::count_number::{span_distance, span_matches, Span};

const MAX_MODE_DISTANCE: usize = 80;
const DIRECT_EXACT_DISTANCE: usize = 12;

pub(super) fn count_mode(target: Span, lower: &str, content: &str) -> CountMode {
    let approximate = min_score(
        mode_score(target, &approximate_spans(lower, content)),
        number_ish_score(target, lower),
    );
    let exact = mode_score(target, &exact_spans(lower, content));
    if exact.is_some_and(|score| score <= DIRECT_EXACT_DISTANCE) {
        CountMode::Exact
    } else if approximate.is_some() && (exact.is_none() || approximate < exact) {
        CountMode::Approximate
    } else {
        CountMode::Exact
    }
}

fn mode_score(target: Span, spans: &[Span]) -> Option<usize> {
    spans
        .iter()
        .map(|span| span_distance(target, *span))
        .min()
        .filter(|score| *score <= MAX_MODE_DISTANCE)
}

fn approximate_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in [
        "about",
        "around",
        "roughly",
        "approximately",
        "approx ",
        "or so",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["ぐらい", "くらい", "程度", "ほど", "前後", "約"] {
        spans.extend(span_matches(content, needle));
    }
    spans.extend(negated_exact_spans(lower));
    spans
}

fn exact_spans(lower: &str, content: &str) -> Vec<Span> {
    let negated = negated_exact_spans(lower);
    let mut spans = Vec::new();
    for needle in ["exact", "exactly", "precisely"] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["ちょうど", "ぴったり", "正確"] {
        spans.extend(span_matches(content, needle));
    }
    spans
        .into_iter()
        .filter(|span| !negated.iter().any(|negation| overlaps(*span, *negation)))
        .collect()
}

fn number_ish_score(target: Span, lower: &str) -> Option<usize> {
    span_matches(lower, "ish")
        .into_iter()
        .filter(|ish| target.end <= ish.start && ish.start.saturating_sub(target.end) <= 1)
        .map(|ish| span_distance(target, ish))
        .min()
}

fn negated_exact_spans(lower: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in [
        "not exact",
        "not exactly",
        "not precise",
        "not precisely",
        "not an exact",
        "no need to be exact",
        "no need to be precise",
        "need not be exact",
        "need not be precise",
        "do not need to be exact",
        "do not have to be exact",
        "does not need to be exact",
        "does not have to be exact",
        "don't need to be exact",
        "don't have to be exact",
        "doesn't need to be exact",
        "doesn't have to be exact",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    spans
}

fn overlaps(left: Span, right: Span) -> bool {
    left.start < right.end && right.start < left.end
}

fn min_score(left: Option<usize>, right: Option<usize>) -> Option<usize> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

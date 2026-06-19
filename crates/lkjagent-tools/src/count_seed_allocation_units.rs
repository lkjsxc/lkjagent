use crate::count_number::{span_matches, Span};

pub(crate) fn split_unit_spans(lower: &str, file_signals: &[Span]) -> Vec<Span> {
    let mut spans = file_signals.to_vec();
    for needle in [
        "source packet",
        "source packets",
        "research packet",
        "research packets",
        "reference packet",
        "reference packets",
        "research note",
        "research notes",
        "source note",
        "source notes",
        "reference note",
        "reference notes",
        "background note",
        "background notes",
        "evidence summary",
        "evidence summaries",
        "research summary",
        "research summaries",
        "source summary",
        "source summaries",
        "reference summary",
        "reference summaries",
        "background summary",
        "background summaries",
        "supporting exhibit",
        "supporting exhibits",
        "evidence exhibit",
        "evidence exhibits",
        "research exhibit",
        "research exhibits",
        "source exhibit",
        "source exhibits",
        "reference exhibit",
        "reference exhibits",
        "background exhibit",
        "background exhibits",
        "packet",
        "packets",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    spans
}

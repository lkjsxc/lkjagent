use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut rows = Vec::new();
    let mut row = String::new();
    let mut row_width: usize = 0;
    for grapheme in text.graphemes(true) {
        if grapheme.contains('\n') {
            rows.push(std::mem::take(&mut row));
            row_width = 0;
            continue;
        }
        let grapheme_width = display_width(grapheme);
        if row_width > 0 && row_width.saturating_add(grapheme_width) > width {
            rows.push(std::mem::take(&mut row));
            row_width = 0;
        }
        row.push_str(grapheme);
        row_width = row_width.saturating_add(grapheme_width);
    }
    rows.push(row);
    rows
}

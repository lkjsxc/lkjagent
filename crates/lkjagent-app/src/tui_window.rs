pub fn visible_text(text: &str, height: usize, follow: bool, scroll: usize) -> String {
    let lines = text.lines().collect::<Vec<_>>();
    let start = window_start(lines.len(), height, follow, scroll);
    lines
        .into_iter()
        .skip(start)
        .take(height)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn window_start(line_count: usize, height: usize, follow: bool, scroll: usize) -> usize {
    let max_top = line_count.saturating_sub(height);
    if follow {
        max_top
    } else {
        scroll.min(max_top)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_mode_anchors_to_bottom() {
        assert_eq!(window_start(12, 5, true, 0), 7);
        assert_eq!(
            visible_text(&lines(6), 3, true, 0),
            "line-4\nline-5\nline-6"
        );
    }

    #[test]
    fn manual_scroll_is_clamped_to_bottom() {
        assert_eq!(window_start(4, 10, false, 99), 0);
        assert_eq!(window_start(20, 6, false, 99), 14);
        assert_eq!(
            visible_text(&lines(4), 3, false, 99),
            "line-2\nline-3\nline-4"
        );
    }

    #[test]
    fn manual_scroll_preserves_requested_top_when_valid() {
        assert_eq!(window_start(20, 6, false, 4), 4);
        assert_eq!(
            visible_text(&lines(6), 3, false, 2),
            "line-3\nline-4\nline-5"
        );
    }

    fn lines(count: usize) -> String {
        (1..=count)
            .map(|index| format!("line-{index}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

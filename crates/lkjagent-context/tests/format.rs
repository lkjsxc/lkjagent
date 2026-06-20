use lkjagent_context::format::{optional_count, ratio_percent, short_count};

#[test]
fn short_count_formats_k_m_b() {
    assert_eq!(short_count(0), "0");
    assert_eq!(short_count(999), "999");
    assert_eq!(short_count(1_000), "1.00K");
    assert_eq!(short_count(15_320), "15.32K");
    assert_eq!(short_count(1_234_567), "1.23M");
    assert_eq!(short_count(2_000_000_000), "2.00B");
}

#[test]
fn ratio_percent_formats_two_decimals() {
    assert_eq!(ratio_percent(1_234, 10_000), "12.34%");
    assert_eq!(ratio_percent(0, 0), "unknown");
}

#[test]
fn optional_count_preserves_unknown() {
    assert_eq!(optional_count(None), "unknown");
    assert_eq!(optional_count(Some(1_000)), "1.00K");
}

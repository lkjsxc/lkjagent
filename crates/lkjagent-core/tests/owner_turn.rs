use lkjagent_core::owner_turn::record_intent;

#[test]
fn routes_japanese_record_requests_to_records() -> Result<(), String> {
    for text in [
        "記録してほしい",
        "ファイルに記録してほしいんだけど",
        "今日はcodexの枠がリセットされる日だったので急いでたくさんaiを使ったと記録してほしい",
        "書きたいというよりかは、記録したいって感じかも",
    ] {
        let intent = record_intent(text).ok_or_else(|| format!("not routed: {text}"))?;
        assert_eq!(intent.kind, "journal");
        assert_eq!(intent.body, text);
    }
    Ok(())
}

#[test]
fn routes_daily_record_kinds() -> Result<(), String> {
    let cases = [
        ("todo buy milk", "todo"),
        ("record meeting with Emi tomorrow", "calendar"),
        ("record that I paid 1200 yen", "finance"),
        ("project note for lkjagent", "project"),
        ("artifact record for the report", "artifact"),
        ("note that local endpoint is offline", "note"),
    ];
    for (text, kind) in cases {
        let intent = record_intent(text).ok_or_else(|| format!("not routed: {text}"))?;
        assert_eq!(intent.kind, kind, "{text}");
    }
    Ok(())
}

#[test]
fn does_not_route_plain_questions() {
    assert!(record_intent("what is the current state?").is_none());
}

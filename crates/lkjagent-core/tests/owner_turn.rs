use lkjagent_core::owner_turn::{record_intent, route_turn, RouteContext, TurnRoute};

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
        assert_ne!(intent.body, text);
        assert!(intent.body.contains("Summary"));
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
fn routes_japanese_diary_and_save_wording() -> Result<(), String> {
    for text in [
        "今日の日記を書いて保存して",
        "この内容を残しておいて",
        "あとで使うから覚えておいて",
    ] {
        let intent = record_intent(text).ok_or_else(|| format!("not routed: {text}"))?;
        assert_eq!(intent.kind, "journal");
    }
    Ok(())
}

#[test]
fn routes_existing_matter_turns() -> Result<(), String> {
    let answer = route(
        "README.md is the file to inspect",
        RouteContext {
            waiting_matter: true,
            ..RouteContext::default()
        },
    )?;
    assert_route(&answer, "existing_matter", "queue_answer", false);
    let continuation = route(
        "also add that evidence to this matter",
        RouteContext {
            open_matter: true,
            ..RouteContext::default()
        },
    )?;
    assert_route(&continuation, "existing_matter", "matter_update", true);
    Ok(())
}

#[test]
fn routes_artifact_inspection_and_system_operation() -> Result<(), String> {
    let artifact = route(
        "create an artifact report from these notes",
        RouteContext::default(),
    )?;
    assert_route(&artifact, "artifact_request", "runtime_decision", true);
    for text in [
        "status",
        "show the current status",
        "What is the current state?",
    ] {
        let inspection = route(text, RouteContext::default())?;
        assert_route(&inspection, "inspection", "read_only_report", false);
    }
    let operation = route(
        "run cargo test and report failures",
        RouteContext::default(),
    )?;
    assert_route(&operation, "system_operation", "runtime_decision", false);
    Ok(())
}

#[test]
fn english_work_is_not_misrouted_as_status_inspection() -> Result<(), String> {
    for text in [
        "Inspect project-orbit and cite current source.",
        "Answer about project-orbital without cross-project facts.",
        "Edit one bounded Rust behavior and run focused checks.",
        "Restart and resume without duplicate effects.",
        "Report the exact diff, checks, project note, and risks.",
        "Show differences between these projects.",
        "List the changes required in lib.rs.",
    ] {
        let route = route(text, RouteContext::default())?;
        assert_ne!(route.lane, "inspection", "{text}");
    }
    Ok(())
}

#[test]
fn japanese_status_and_work_remain_distinct() -> Result<(), String> {
    let status = route("現在の状態を見せて。", RouteContext::default())?;
    assert_route(&status, "inspection", "read_only_report", false);
    let work = route("資料を確認して要点を報告して", RouteContext::default())?;
    assert_ne!(work.lane, "inspection");
    Ok(())
}

#[test]
fn ambiguous_save_like_turns_route_to_inbox() -> Result<(), String> {
    for text in ["remember this", "save this", "keep this", "覚えておいて"] {
        assert!(record_intent(text).is_none(), "{text}");
        let route = route(text, RouteContext::default())?;
        assert_route(&route, "inbox", "workspace_inbox", false);
    }
    let intent = record_intent("remember that I paid 1200 yen")
        .ok_or_else(|| "contentful remember should record".to_string())?;
    assert_eq!(intent.kind, "finance");
    assert!(intent.body.contains("I paid 1200 yen"));
    Ok(())
}

#[test]
fn does_not_record_plain_questions() {
    assert!(record_intent("what is the current state?").is_none());
}

fn route(text: &str, context: RouteContext) -> Result<TurnRoute, String> {
    route_turn(text, context).ok_or_else(|| format!("not routed: {text}"))
}

fn assert_route(route: &TurnRoute, lane: &str, durability: &str, allowed: bool) {
    assert_eq!(route.lane, lane);
    assert_eq!(route.desired_durability, durability);
    assert_eq!(route.transformation_allowed, allowed);
    assert!(!route.title_seed.is_empty());
    assert!(!route.body_seed.is_empty());
}

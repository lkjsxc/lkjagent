use lkjagent_benchmark::metrics::metrics_from_status_and_log;

#[test]
fn metrics_are_extracted_from_status_and_transcript() {
    let status = "daemon_state=idle\nturns=7\n";
    let log = "\
id=1 kind=error turn=1
MissingAct
id=2 kind=notice turn=2
repeat action refused
id=3 kind=action turn=3
<tool>shell.run</tool>
id=4 kind=action turn=4
<tool>fs.write</tool>
id=5 kind=action turn=5
<tool>fs.edit</tool>
id=6 kind=observation turn=6
<status>error</status>
id=7 kind=action turn=7
<tool>agent.ask</tool>
";

    let metrics = metrics_from_status_and_log(status, log);

    assert_eq!(metrics.turn_count, 7);
    assert_eq!(metrics.parse_errors, 1);
    assert_eq!(metrics.repeat_action_notices, 1);
    assert_eq!(metrics.tool_errors, 1);
    assert_eq!(metrics.shell_actions, 1);
    assert_eq!(metrics.file_writes_edits, 2);
    assert_eq!(metrics.questions, 1);
}

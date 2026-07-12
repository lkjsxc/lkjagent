use std::error::Error;

use lkjagent_store::error::StoreResult;
use lkjagent_store::native_schema;
use rusqlite::{params, Connection};

fn seeded() -> StoreResult<Connection> {
    let connection = Connection::open_in_memory()?;
    native_schema::setup(&connection)?;
    connection.execute_batch(
        "INSERT INTO matters(id,objective,lifecycle,priority,created_sequence,updated_sequence)
         VALUES('m',x'01','open',0,1,1);
         INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,
         payload,source_kind,source_id) VALUES('e','m',1,'owner',1,'now',x'02','owner','turn');
         INSERT INTO runtime_decisions(id,matter_id,event_id,operation_key,idempotency_key,
         selected_monotonic_ms,selected_state,context_spec,tool_spec,grammar_spec,budget_spec,
         recovery_spec,check_spec,exit_spec,compiler_status,compiler_attachments,rendered_frame,status)
         VALUES('d','m','e',x'01',x'02',1,x'03',x'04',x'05',x'06',x'07',x'08',x'09',x'0a',
         'complete',x'0b',x'0c','selected');",
    )?;
    Ok(connection)
}

fn rejected(connection: &Connection, sql: &str) {
    assert!(
        connection.execute(sql, []).is_err(),
        "accepted invalid SQL: {sql}"
    );
}

fn accepted_effect(connection: &Connection) -> StoreResult<()> {
    connection.execute_batch(
        "BEGIN;
         INSERT INTO tool_admissions(id,decision_id,action_ordinal,action_fingerprint,origin,
         effectful,status,reason,parsed_call,tool_spec,journal_id)
         VALUES('a','d',0,x'01','model',1,'accepted',x'02',x'03',x'04','j');
         INSERT INTO effect_journal(id,admission_id,decision_id,command_ordinal,idempotency_key,
         status,intended_fingerprint) VALUES('j','a','d',0,x'05','prepared',x'06'); COMMIT;",
    )?;
    Ok(())
}

#[test]
fn durable_boundaries_unique_check_and_orphans() -> Result<(), Box<dyn Error>> {
    let connection = seeded()?;
    rejected(&connection, "INSERT INTO matters(id,objective,lifecycle,priority,created_sequence,updated_sequence) VALUES('bad',x'01','done',0,2,2)");
    rejected(&connection, "INSERT INTO matters(id,objective,lifecycle,priority,created_sequence,updated_sequence) VALUES('closed',x'01','closed',0,2,2)");
    rejected(&connection, "INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES('duplicate','m',1,'x',2,'now',x'01','x','x')");
    rejected(&connection, "INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES('orphan','missing',2,'x',2,'now',x'01','x','x')");
    rejected(&connection, "INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES('m',x'01',x'02',x'03','unknown','e',x'04')");
    rejected(&connection, "INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,lifecycle) VALUES('a',1,'system',x'01',x'02','active')");
    connection.execute("INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,lifecycle,matter_id) VALUES('a',1,'agent',x'01',x'02','active','m')", [])?;
    rejected(&connection, "INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,lifecycle,matter_id) VALUES('b',1,'agent',x'01',x'03','active','m')");
    rejected(&connection, "INSERT INTO provider_exchanges(id,decision_id,request_ref,started_monotonic_ms,status) VALUES('p','missing',x'01',0,'intended')");
    connection.execute("INSERT INTO provider_exchanges(id,decision_id,request_ref,started_monotonic_ms,status) VALUES('p','d',x'01',0,'intended')", [])?;
    rejected(&connection, "INSERT INTO provider_exchanges(id,decision_id,request_ref,started_monotonic_ms,status) VALUES('p2','d',x'02',0,'intended')");
    rejected(
        &connection,
        "UPDATE runtime_decisions SET context_spec=x'ff' WHERE id='d'",
    );
    rejected(&connection, "INSERT INTO tool_admissions(id,decision_id,action_ordinal,action_fingerprint,origin,effectful,status,reason,parsed_call,tool_spec) VALUES('r','d',0,x'01','model',1,'accepted',x'01',x'02',x'03')");
    Ok(())
}

#[test]
fn durable_boundaries_effect_cardinality() -> Result<(), Box<dyn Error>> {
    let connection = seeded()?;
    accepted_effect(&connection)?;
    rejected(&connection, "INSERT INTO effect_journal(id,admission_id,decision_id,command_ordinal,idempotency_key,status,intended_fingerprint) VALUES('j2','a','d',1,x'07','prepared',x'08')");
    rejected(
        &connection,
        "UPDATE effect_journal SET status='settled' WHERE id='j'",
    );
    rejected(&connection, "INSERT INTO observations(id,journal_id,status,attempt_outcome,content_ref,fingerprint,contamination) VALUES('o','missing','succeeded',x'01',x'02',x'03','clean')");
    connection.execute_batch(
        "BEGIN; INSERT INTO observations(id,journal_id,status,attempt_outcome,content_ref,
         fingerprint,contamination,event_id) VALUES('o','j','succeeded',x'01',x'02',x'03','clean','e');
         UPDATE effect_journal SET status='settled',observation_id='o' WHERE id='j'; COMMIT;",
    )?;
    rejected(&connection, "INSERT INTO observations(id,journal_id,status,attempt_outcome,content_ref,fingerprint,contamination) VALUES('o2','j','failed',x'01',x'02',x'03','clean')");
    connection.execute("INSERT INTO effect_targets(journal_id,ordinal,normalized_path,intended_bytes,operation,stage_identity) VALUES('j',0,?1,?2,'create',?3)", params![b"a", b"bytes", b"stage"])?;
    rejected(&connection, "INSERT INTO effect_targets(journal_id,ordinal,normalized_path,operation,stage_identity) VALUES('j',1,x'61','delete',x'01')");
    Ok(())
}

#[test]
fn durable_boundaries_checks_and_workspace_identity() -> Result<(), Box<dyn Error>> {
    let connection = seeded()?;
    connection.execute("INSERT INTO obligations(id,matter_id,predicate_kind,predicate_payload,required,status) VALUES('o','m','byte',x'01',1,'open')", [])?;
    rejected(&connection, "INSERT INTO checks(id,matter_id,obligation_id,decision_id,kind,parameters,current,passed,measured,evidence_fingerprint,source_revision,checked_event_id) VALUES('c','m','missing','d','byte',x'01',1,1,x'02',x'03',x'04','e')");
    connection.execute("INSERT INTO checks(id,matter_id,obligation_id,decision_id,kind,parameters,current,passed,measured,evidence_fingerprint,source_revision,checked_event_id) VALUES('c','m','o','d','byte',x'01',1,1,x'02',x'03',x'04','e')", [])?;
    connection.execute(
        "UPDATE obligations SET status='passed',current_check_id='c' WHERE id='o'",
        [],
    )?;
    rejected(&connection, "INSERT INTO checks(id,matter_id,obligation_id,decision_id,kind,parameters,current,passed,measured,evidence_fingerprint,source_revision,checked_event_id) VALUES('c2','m','o','d','other',x'01',1,1,x'02',x'03',x'05','e')");
    rejected(&connection, "INSERT INTO workspace_documents(id,current_path,status,managed) VALUES('w',x'01','unknown',1)");
    connection.execute("INSERT INTO workspace_documents(id,current_path,status,managed) VALUES('w',x'01','active',1)", [])?;
    accepted_effect(&connection)?;
    rejected(&connection, "INSERT INTO workspace_revisions(id,document_id,sha256,content,effect_id,created_event_id) VALUES('bad','w',x'01',x'02','j','e')");
    let sha = [7_u8; 32];
    connection.execute("INSERT INTO workspace_revisions(id,document_id,sha256,content,effect_id,created_event_id) VALUES('v','w',?1,x'02','j','e')", [sha.as_slice()])?;
    connection.execute(
        "UPDATE workspace_documents SET current_revision_id='v' WHERE id='w'",
        [],
    )?;
    assert!(connection.execute("INSERT INTO workspace_revisions(id,document_id,sha256,content,effect_id,created_event_id) VALUES('v2','w',?1,x'03','j','e')", [sha.as_slice()]).is_err());
    rejected(&connection, "INSERT INTO workspace_documents(id,current_path,status,managed) VALUES('w2',x'01','active',1)");
    Ok(())
}

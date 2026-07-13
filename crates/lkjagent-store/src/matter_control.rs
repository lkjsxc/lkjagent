use rusqlite::{params, OptionalExtension};

use crate::error::{StoreError, StoreResult};
use crate::native_schema::MessageIdentity;
use crate::transactions::{Intake, NativeStore};

impl NativeStore {
    pub fn provider_exchanges_in_budget_epoch(&self, matter: &str) -> StoreResult<i64> {
        Ok(self.connection.query_row(
            "SELECT count(*) FROM provider_exchanges p JOIN runtime_decisions d ON d.id=p.decision_id JOIN runtime_events selected ON selected.id=d.event_id WHERE d.matter_id=?1 AND selected.causal_sequence>(SELECT coalesce(max(causal_sequence),0) FROM runtime_events WHERE matter_id=?1 AND kind IN ('owner-intake','owner-resume'))",
            [matter],
            |row| row.get(0),
        )?)
    }

    pub fn latest_blocked_matter(&self) -> StoreResult<Option<String>> {
        Ok(self
            .connection
            .query_row(
                "SELECT id FROM matters WHERE lifecycle='blocked' ORDER BY updated_sequence, id LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?)
    }

    pub fn resume_blocked(&mut self, value: &Intake<'_>) -> StoreResult<MessageIdentity> {
        self.atomic(|tx| {
            let id = format!("owner-turn/{}", value.turn);
            if let Some(sequence) = tx
                .query_row(
                    "SELECT sequence FROM conversation_messages WHERE id=?1 AND matter_id=?2",
                    params![id, value.matter],
                    |row| row.get(0),
                )
                .optional()?
            {
                return Ok(MessageIdentity { id, sequence });
            }
            let lifecycle: String = tx.query_row(
                "SELECT lifecycle FROM matters WHERE id=?1",
                [value.matter],
                |row| row.get(0),
            )?;
            if lifecycle != "blocked" {
                return Err(StoreError::InvalidState("matter is not blocked".into()));
            }
            let sequence: i64 = tx.query_row(
                "SELECT coalesce(max(sequence),0)+1 FROM conversation_messages",
                [],
                |row| row.get(0),
            )?;
            tx.execute("INSERT INTO owner_turns(id,queue_sequence,raw_text,delivery,matter_id,created_at) VALUES(?1,?2,?3,'delivered',?4,?5)", params![value.turn,value.queue_sequence,value.raw_text,value.matter,value.wall_time])?;
            tx.execute("INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'owner-resume',?4,?5,?6,'owner-turn',?7)", params![value.event,value.matter,value.event_sequence,value.monotonic_ms,value.wall_time,value.event_payload,value.turn])?;
            tx.execute("INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,lifecycle,matter_id,owner_turn_id,cause_event_id) VALUES(?1,?2,'owner',?3,?4,'active',?5,?6,?7)", params![id,sequence,value.raw_text,value.message_fingerprint,value.matter,value.turn,value.event])?;
            tx.execute("UPDATE state_cells SET status='suppressed' WHERE matter_id=?1 AND status='active'", [value.matter])?;
            for row in value.cells { tx.execute("INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,?2,?3,?4,'active',?5,?6) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint", params![value.matter,row.0,row.1,row.2,value.event,row.3])?; }
            let changed=tx.execute("UPDATE matters SET objective=?1,lifecycle='open',updated_sequence=?2 WHERE id=?3 AND lifecycle='blocked'",params![value.objective,value.event_sequence,value.matter])?;
            if changed==1 { Ok(MessageIdentity{id,sequence}) } else { Err(StoreError::InvalidState("matter resume conflict".into())) }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn block_budget(
        &mut self,
        matter: &str,
        event: &str,
        sequence: i64,
        monotonic_ms: i64,
        wall_time: &str,
        payload: &[u8],
        fingerprint: &[u8],
    ) -> StoreResult<()> {
        self.atomic(|tx| {
            let lifecycle: Option<String> = tx
                .query_row(
                    "SELECT lifecycle FROM matters WHERE id=?1",
                    [matter],
                    |row| row.get(0),
                )
                .optional()?;
            match lifecycle.as_deref() {
                Some("blocked") => return Ok(()),
                Some("open") => {}
                Some(_) => {
                    return Err(StoreError::InvalidState(
                        "only an open matter can exhaust a budget".into(),
                    ))
                }
                None => return Err(StoreError::NotFound("matter".into())),
            }
            tx.execute(
                "INSERT INTO runtime_events(id,matter_id,causal_sequence,kind,monotonic_ms,wall_time,payload,source_kind,source_id) VALUES(?1,?2,?3,'matter-blocked',?4,?5,?6,'harness',?7)",
                params![event, matter, sequence, monotonic_ms, wall_time, payload, matter],
            )?;
            tx.execute(
                "INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint) VALUES(?1,'block','budget',?2,'active',?3,?4) ON CONFLICT(matter_id,namespace,cell_key) DO UPDATE SET payload=excluded.payload,status='active',source_event_id=excluded.source_event_id,fingerprint=excluded.fingerprint",
                params![matter, payload, event, fingerprint],
            )?;
            let changed = tx.execute(
                "UPDATE matters SET lifecycle='blocked',updated_sequence=?1 WHERE id=?2 AND lifecycle='open'",
                params![sequence, matter],
            )?;
            if changed == 1 {
                Ok(())
            } else {
                Err(StoreError::InvalidState("matter budget block conflict".into()))
            }
        })
    }
}

use rusqlite::{params, OptionalExtension};

use crate::error::{StoreError, StoreResult};
use crate::transactions::NativeStore;

impl NativeStore {
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

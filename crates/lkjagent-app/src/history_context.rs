use lkjagent_core::runtime_context::{ContaminationClass, ContextItem, StalenessClass, TrustClass};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use rusqlite::Connection;
use std::collections::BTreeSet;
use std::path::Path;

type Row = (String, String, Vec<u8>);

pub(crate) fn load(db: &Path, matter: &str, objective: &[u8]) -> Result<Vec<ContextItem>, String> {
    let connection = Connection::open(db).map_err(error)?;
    let mut query = connection
        .prepare("SELECT id,role,body FROM conversation_messages WHERE matter_id<>?1 AND lifecycle='active' ORDER BY sequence DESC LIMIT 16")
        .map_err(error)?;
    let rows = query
        .query_map([matter], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(error)?
        .collect::<Result<Vec<Row>, _>>()
        .map_err(error)?;
    let wanted = project_ids(&String::from_utf8_lossy(objective));
    Ok(rows
        .into_iter()
        .filter(|(_, _, body)| {
            wanted.is_empty() || !wanted.is_disjoint(&project_ids(&String::from_utf8_lossy(body)))
        })
        .take(4)
        .map(|(id, role, body)| item(id, role, body))
        .collect())
}

fn item(id: String, role: String, body: Vec<u8>) -> ContextItem {
    let source_id = format!("{role}:{id}");
    ContextItem {
        id: format!("history-{id}"),
        semantic_key: format!("conversation-{id}"),
        body: String::from_utf8_lossy(&body).into_owned(),
        source_type: "conversation-history".into(),
        source_fingerprint: stable_fingerprint(&source_id).unwrap_or_default(),
        source_id,
        trust: TrustClass::Memory,
        staleness: StalenessClass::Current,
        contamination: ContaminationClass::Clean,
        artifact_refs: vec![],
        decision_id: None,
        created_at: String::new(),
    }
}

fn project_ids(text: &str) -> BTreeSet<String> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .filter(|word| word.starts_with("project-") && word.len() <= 128)
        .map(str::to_ascii_lowercase)
        .collect()
}

fn error(value: impl std::fmt::Display) -> String {
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::load;
    use lkjagent_store::transactions::{Intake, NativeStore};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn exact_project_token_excludes_prefix_neighbor_history(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let db = std::env::temp_dir().join(format!("history-project-{nonce}.db"));
        let mut store = NativeStore::open(&db)?;
        for (matter, turn, event, queue, text) in [
            ("orbit", "t1", "e1", 1, "facts for project-orbit"),
            ("orbital", "t2", "e2", 2, "facts for project-orbital"),
        ] {
            store.owner_intake(&Intake {
                matter,
                objective: text.as_bytes(),
                turn,
                queue_sequence: queue,
                raw_text: text.as_bytes(),
                message_fingerprint: event.as_bytes(),
                event,
                event_sequence: 1,
                event_payload: text.as_bytes(),
                monotonic_ms: 1,
                wall_time: "now",
                obligations: &[],
                cells: &[],
            })?;
        }
        drop(store);
        let rows = load(&db, "current", b"inspect project-orbital")?;
        assert_eq!(rows.len(), 1);
        assert!(rows[0].body.contains("project-orbital"));
        Ok(())
    }
}

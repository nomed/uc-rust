//! SQLite persistence for edge outbox and central inbox synchronization state.

#![forbid(unsafe_code)]

use rusqlite::{params, Connection, OptionalExtension, Transaction};
use std::path::Path;
use uc_sync::{AcceptResult, EdgeEvent, EdgeId, EventId};

/// Failures produced by the SQLite synchronization store.
#[derive(Debug)]
pub enum SqliteSyncStoreError {
    /// SQLite rejected an operation.
    Database(rusqlite::Error),
    /// Stored data cannot reconstruct a valid synchronization event.
    CorruptData(&'static str),
}

impl From<rusqlite::Error> for SqliteSyncStoreError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

/// Durable SQLite store for edge outbox and central inbox state.
#[derive(Debug)]
pub struct SqliteSyncStore {
    connection: Connection,
}

impl SqliteSyncStore {
    /// Opens or creates a file-backed synchronization store.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SqliteSyncStoreError> {
        Self::new(Connection::open(path)?)
    }

    /// Creates an in-memory synchronization store.
    pub fn open_in_memory() -> Result<Self, SqliteSyncStoreError> {
        Self::new(Connection::open_in_memory()?)
    }

    fn new(connection: Connection) -> Result<Self, SqliteSyncStoreError> {
        connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS edge_outbox (
                 event_id TEXT PRIMARY KEY,
                 edge_id TEXT NOT NULL,
                 sequence_no INTEGER NOT NULL,
                 payload TEXT NOT NULL,
                 UNIQUE(edge_id, sequence_no)
             );
             CREATE TABLE IF NOT EXISTS central_inbox (
                 edge_id TEXT NOT NULL,
                 sequence_no INTEGER NOT NULL,
                 event_id TEXT NOT NULL UNIQUE,
                 payload TEXT NOT NULL,
                 state TEXT NOT NULL CHECK (state IN ('buffered', 'applied')),
                 PRIMARY KEY(edge_id, sequence_no)
             );",
        )?;
        Ok(Self { connection })
    }

    /// Appends an edge event to the durable outbox.
    pub fn enqueue(&mut self, event: &EdgeEvent) -> Result<(), SqliteSyncStoreError> {
        self.connection.execute(
            "INSERT INTO edge_outbox (event_id, edge_id, sequence_no, payload)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                event.event_id().as_str(),
                event.edge_id().as_str(),
                to_i64(event.sequence())?,
                event.payload(),
            ],
        )?;
        Ok(())
    }

    /// Returns all unacknowledged events in edge sequence order.
    pub fn pending(&self, edge_id: &EdgeId) -> Result<Vec<EdgeEvent>, SqliteSyncStoreError> {
        let mut statement = self.connection.prepare(
            "SELECT event_id, sequence_no, payload
             FROM edge_outbox WHERE edge_id = ?1 ORDER BY sequence_no",
        )?;
        let rows = statement.query_map(params![edge_id.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        rows.map(|row| {
            let (event_id, sequence, payload) = row?;
            Ok(EdgeEvent::new(
                edge_id.clone(),
                EventId::new(event_id),
                to_u64(sequence)?,
                payload,
            ))
        })
        .collect()
    }

    /// Removes an acknowledged event from the durable outbox.
    pub fn acknowledge(&mut self, event_id: &EventId) -> Result<bool, SqliteSyncStoreError> {
        Ok(self.connection.execute(
            "DELETE FROM edge_outbox WHERE event_id = ?1",
            params![event_id.as_str()],
        )? == 1)
    }

    /// Accepts an event centrally with durable deduplication, ordering and conflict detection.
    pub fn accept(&mut self, event: &EdgeEvent) -> Result<AcceptResult, SqliteSyncStoreError> {
        let transaction = self.connection.transaction()?;
        let result = accept_transaction(&transaction, event)?;
        transaction.commit()?;
        Ok(result)
    }

    /// Returns centrally applied events in deterministic edge/sequence order.
    pub fn applied(&self, edge_id: &EdgeId) -> Result<Vec<EdgeEvent>, SqliteSyncStoreError> {
        read_inbox_events(&self.connection, edge_id, "applied")
    }

    /// Returns centrally buffered events in sequence order.
    pub fn buffered(&self, edge_id: &EdgeId) -> Result<Vec<EdgeEvent>, SqliteSyncStoreError> {
        read_inbox_events(&self.connection, edge_id, "buffered")
    }
}

fn accept_transaction(
    transaction: &Transaction<'_>,
    event: &EdgeEvent,
) -> Result<AcceptResult, SqliteSyncStoreError> {
    let by_event_id = transaction
        .query_row(
            "SELECT edge_id, sequence_no FROM central_inbox WHERE event_id = ?1",
            params![event.event_id().as_str()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
        )
        .optional()?;
    if by_event_id.is_some() {
        return Ok(AcceptResult::Duplicate);
    }

    let by_sequence = transaction
        .query_row(
            "SELECT event_id FROM central_inbox WHERE edge_id = ?1 AND sequence_no = ?2",
            params![event.edge_id().as_str(), to_i64(event.sequence())?],
            |row| row.get::<_, String>(0),
        )
        .optional()?;
    if let Some(existing_event_id) = by_sequence {
        return Ok(if existing_event_id == event.event_id().as_str() {
            AcceptResult::Duplicate
        } else {
            AcceptResult::SequenceConflict
        });
    }

    let last_applied = transaction.query_row(
        "SELECT COALESCE(MAX(sequence_no), 0) FROM central_inbox
         WHERE edge_id = ?1 AND state = 'applied'",
        params![event.edge_id().as_str()],
        |row| row.get::<_, i64>(0),
    )?;
    let expected = to_u64(last_applied)?.saturating_add(1);

    if event.sequence() < expected {
        return Ok(AcceptResult::SequenceConflict);
    }

    let state = if event.sequence() == expected {
        "applied"
    } else {
        "buffered"
    };
    transaction.execute(
        "INSERT INTO central_inbox (edge_id, sequence_no, event_id, payload, state)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            event.edge_id().as_str(),
            to_i64(event.sequence())?,
            event.event_id().as_str(),
            event.payload(),
            state,
        ],
    )?;

    if state == "buffered" {
        return Ok(AcceptResult::Buffered);
    }

    drain_ready(transaction, event.edge_id(), event.sequence().saturating_add(1))?;
    Ok(AcceptResult::Applied)
}

fn drain_ready(
    transaction: &Transaction<'_>,
    edge_id: &EdgeId,
    mut expected: u64,
) -> Result<(), SqliteSyncStoreError> {
    loop {
        let updated = transaction.execute(
            "UPDATE central_inbox SET state = 'applied'
             WHERE edge_id = ?1 AND sequence_no = ?2 AND state = 'buffered'",
            params![edge_id.as_str(), to_i64(expected)?],
        )?;
        if updated == 0 {
            return Ok(());
        }
        expected = expected.saturating_add(1);
    }
}

fn read_inbox_events(
    connection: &Connection,
    edge_id: &EdgeId,
    state: &str,
) -> Result<Vec<EdgeEvent>, SqliteSyncStoreError> {
    let mut statement = connection.prepare(
        "SELECT event_id, sequence_no, payload FROM central_inbox
         WHERE edge_id = ?1 AND state = ?2 ORDER BY sequence_no",
    )?;
    let rows = statement.query_map(params![edge_id.as_str(), state], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    rows.map(|row| {
        let (event_id, sequence, payload) = row?;
        Ok(EdgeEvent::new(
            edge_id.clone(),
            EventId::new(event_id),
            to_u64(sequence)?,
            payload,
        ))
    })
    .collect()
}

fn to_i64(value: u64) -> Result<i64, SqliteSyncStoreError> {
    i64::try_from(value).map_err(|_| SqliteSyncStoreError::CorruptData("sequence exceeds SQLite range"))
}

fn to_u64(value: i64) -> Result<u64, SqliteSyncStoreError> {
    u64::try_from(value).map_err(|_| SqliteSyncStoreError::CorruptData("negative sequence"))
}

#[cfg(test)]
mod tests {
    use super::SqliteSyncStore;
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};
    use uc_sync::{AcceptResult, EdgeEvent, EdgeId, EventId};

    fn event(sequence: u64) -> EdgeEvent {
        EdgeEvent::new(
            EdgeId::new("store-1"),
            EventId::new(format!("event-{sequence}")),
            sequence,
            format!("sale-{sequence}"),
        )
    }

    fn unique_database_path(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock follows epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("uc-rust-{name}-{nonce}.sqlite"))
    }

    #[test]
    fn edge_outbox_survives_restart_and_acknowledgement() {
        let path = unique_database_path("outbox");
        {
            let mut store = SqliteSyncStore::open(&path).expect("store opens");
            store.enqueue(&event(1)).expect("first event persists");
            store.enqueue(&event(2)).expect("second event persists");
        }
        {
            let mut restarted = SqliteSyncStore::open(&path).expect("store reopens");
            let pending = restarted
                .pending(&EdgeId::new("store-1"))
                .expect("backlog reloads");
            assert_eq!(pending.len(), 2);
            assert!(restarted
                .acknowledge(pending[0].event_id())
                .expect("ack persists"));
        }
        {
            let restarted = SqliteSyncStore::open(&path).expect("store reopens again");
            assert_eq!(restarted.pending(&EdgeId::new("store-1")).expect("reload succeeds").len(), 1);
        }
        fs::remove_file(path).expect("temporary database removed");
    }

    #[test]
    fn central_buffer_and_deduplication_survive_restart() {
        let path = unique_database_path("inbox");
        {
            let mut store = SqliteSyncStore::open(&path).expect("store opens");
            assert_eq!(store.accept(&event(2)).expect("event buffers"), AcceptResult::Buffered);
        }
        {
            let mut restarted = SqliteSyncStore::open(&path).expect("store reopens");
            assert_eq!(restarted.buffered(&EdgeId::new("store-1")).expect("buffer reloads").len(), 1);
            assert_eq!(restarted.accept(&event(1)).expect("first event applies"), AcceptResult::Applied);
            assert_eq!(restarted.applied(&EdgeId::new("store-1")).expect("applied reloads").len(), 2);
            assert_eq!(restarted.accept(&event(2)).expect("duplicate detected"), AcceptResult::Duplicate);
        }
        fs::remove_file(path).expect("temporary database removed");
    }

    #[test]
    fn sequence_collision_is_persisted_as_conflict_without_overwrite() {
        let mut store = SqliteSyncStore::open_in_memory().expect("store opens");
        assert_eq!(store.accept(&event(2)).expect("event buffers"), AcceptResult::Buffered);
        let conflicting = EdgeEvent::new(
            EdgeId::new("store-1"),
            EventId::new("different-event"),
            2,
            "different-sale",
        );
        assert_eq!(
            store.accept(&conflicting).expect("conflict evaluated"),
            AcceptResult::SequenceConflict
        );
        let buffered = store.buffered(&EdgeId::new("store-1")).expect("buffer reads");
        assert_eq!(buffered.len(), 1);
        assert_eq!(buffered[0].event_id().as_str(), "event-2");
    }
}

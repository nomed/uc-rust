//! Economics by Design measurement harness for the durable `SQLite` sync adapter.
//!
//! The harness records directly observable workload facts and keeps inferred
//! database-operation counts explicitly classified as attributed values. It does
//! not assign monetary rates and does not claim full Cost-to-Serve.

#![forbid(unsafe_code)]

use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uc_economics::{
    Confidence, ConsumptionDriverId, ConsumptionMeasurement, EconomicObservation, EconomicUnitId,
    ObservationProvenance, OperationId, ValidationError, ValueClassification,
};
use uc_sync::{AcceptResult, EdgeEvent, EdgeId, EventId};
use uc_sync_sqlite::{SqliteSyncStore, SqliteSyncStoreError};

/// Stable workload identifier for the first durable-sync economic slice.
pub const WORKLOAD_ID: &str = "sync-durable-event-v1";
/// Stable costing policy revision used by the Crawl measurement slice.
pub const COSTING_POLICY_REVISION: &str = "uc-rust-ebd-crawl-v1";
/// Economic Unit measured by this harness.
pub const ECONOMIC_UNIT_ID: &str = "uc.synced_business_event";

/// Input parameters for a deterministic sync measurement run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncWorkload {
    /// Number of unique business events to synchronize.
    pub event_count: u64,
    /// UTF-8 payload size generated for each event.
    pub payload_bytes: usize,
    /// Number of duplicate delivery attempts after the unique stream.
    pub duplicate_attempts: u64,
    /// Revision or commit measured by the run.
    pub software_revision: String,
    /// Deployment profile represented by the workload.
    pub deployment_profile: String,
}

impl SyncWorkload {
    /// Creates the default representative store-edge workload.
    #[must_use]
    pub fn store_edge(software_revision: impl Into<String>) -> Self {
        Self {
            event_count: 10_000,
            payload_bytes: 512,
            duplicate_attempts: 100,
            software_revision: software_revision.into(),
            deployment_profile: "store-edge".to_owned(),
        }
    }

    fn validate(&self) -> Result<(), HarnessError> {
        if self.event_count == 0 {
            return Err(HarnessError::InvalidWorkload(
                "event_count must be greater than zero",
            ));
        }
        if self.payload_bytes == 0 {
            return Err(HarnessError::InvalidWorkload(
                "payload_bytes must be greater than zero",
            ));
        }
        if self.duplicate_attempts > self.event_count {
            return Err(HarnessError::InvalidWorkload(
                "duplicate_attempts cannot exceed event_count",
            ));
        }
        if self.software_revision.trim().is_empty() {
            return Err(HarnessError::InvalidWorkload(
                "software_revision must not be blank",
            ));
        }
        if self.deployment_profile.trim().is_empty() {
            return Err(HarnessError::InvalidWorkload(
                "deployment_profile must not be blank",
            ));
        }
        Ok(())
    }
}

/// Raw workload facts retained beside the canonical economic observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncRunFacts {
    /// Unique events successfully accepted by the central inbox.
    pub applied_events: u64,
    /// Duplicate deliveries detected by the central inbox.
    pub duplicate_deliveries: u64,
    /// Total delivery attempts, including duplicates.
    pub delivery_attempts: u64,
    /// Total payload bytes submitted over the logical edge-to-central boundary.
    pub logical_wan_bytes: u128,
    /// File growth observed between an empty initialized database and the final state.
    pub sqlite_file_growth_bytes: u128,
    /// Wall-clock elapsed time for enqueue, read, accept, duplicate and acknowledge work.
    pub elapsed: Duration,
}

/// Result of one measurement run.
#[derive(Debug, Eq, PartialEq)]
pub struct SyncEconomicReport {
    observation: EconomicObservation,
    facts: SyncRunFacts,
}

impl SyncEconomicReport {
    /// Returns the canonical Economics by Design observation.
    #[must_use]
    pub const fn observation(&self) -> &EconomicObservation {
        &self.observation
    }

    /// Returns directly retained workload facts.
    #[must_use]
    pub const fn facts(&self) -> &SyncRunFacts {
        &self.facts
    }

    /// Returns logical duplicate amplification as attempts divided by unique units.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn delivery_amplification(&self) -> f64 {
        self.facts.delivery_attempts as f64 / self.observation.unit_count() as f64
    }

    /// Returns unique applied events per elapsed second.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn applied_events_per_second(&self) -> f64 {
        let seconds = self.facts.elapsed.as_secs_f64();
        if seconds == 0.0 {
            f64::INFINITY
        } else {
            self.facts.applied_events as f64 / seconds
        }
    }
}

/// Executes the first durable-sync economic workload against a temporary `SQLite` database.
pub fn run_sync_workload(workload: &SyncWorkload) -> Result<SyncEconomicReport, HarnessError> {
    workload.validate()?;
    let path = unique_database_path();
    let result = run_at_path(workload, &path);
    let cleanup = remove_database_files(&path);

    match (result, cleanup) {
        (Ok(report), Ok(())) => Ok(report),
        (Err(error), _) | (Ok(_), Err(error)) => Err(error),
    }
}

fn run_at_path(workload: &SyncWorkload, path: &Path) -> Result<SyncEconomicReport, HarnessError> {
    let mut store = SqliteSyncStore::open(path)?;
    let initialized_bytes = file_size(path)?;
    let edge_id = EdgeId::new("economic-store-1");
    let payload = "x".repeat(workload.payload_bytes);
    let started = Instant::now();

    for sequence in 1..=workload.event_count {
        store.enqueue(&event(&edge_id, sequence, &payload))?;
    }

    let pending = store.pending(&edge_id)?;
    let mut applied_events = 0_u64;
    for current in &pending {
        if store.accept(current)? == AcceptResult::Applied {
            applied_events = applied_events.saturating_add(1);
        }
    }

    let duplicate_start = workload.event_count - workload.duplicate_attempts;
    let mut duplicate_deliveries = 0_u64;
    for current in pending
        .iter()
        .skip(usize::try_from(duplicate_start).map_err(|_| {
            HarnessError::InvalidWorkload("event_count exceeds addressable collection size")
        })?)
    {
        if store.accept(current)? == AcceptResult::Duplicate {
            duplicate_deliveries = duplicate_deliveries.saturating_add(1);
        }
    }

    for current in &pending {
        if !store.acknowledge(current.event_id())? {
            return Err(HarnessError::UnexpectedResult(
                "an enqueued event could not be acknowledged",
            ));
        }
    }

    let elapsed = started.elapsed();
    drop(store);
    let final_bytes = file_size(path)?;
    let file_growth = final_bytes.saturating_sub(initialized_bytes);
    let delivery_attempts = workload
        .event_count
        .saturating_add(workload.duplicate_attempts);
    let logical_wan_bytes =
        u128::from(delivery_attempts).saturating_mul(workload.payload_bytes as u128);

    let facts = SyncRunFacts {
        applied_events,
        duplicate_deliveries,
        delivery_attempts,
        logical_wan_bytes,
        sqlite_file_growth_bytes: u128::from(file_growth),
        elapsed,
    };
    let observation = build_observation(workload, &facts)?;
    Ok(SyncEconomicReport { observation, facts })
}

fn build_observation(
    workload: &SyncWorkload,
    facts: &SyncRunFacts,
) -> Result<EconomicObservation, HarnessError> {
    let measurements = [
        measurement(
            "compute.elapsed_time",
            facts.elapsed.as_nanos(),
            "nanosecond",
            ValueClassification::Measured,
        )?,
        measurement(
            "storage.bytes_written",
            facts.sqlite_file_growth_bytes,
            "byte",
            ValueClassification::Measured,
        )?,
        measurement(
            "network.wan_bytes",
            facts.logical_wan_bytes,
            "byte",
            ValueClassification::Measured,
        )?,
        measurement(
            "sync.delivery_attempt",
            u128::from(facts.delivery_attempts),
            "attempt",
            ValueClassification::Measured,
        )?,
        measurement(
            "sync.duplicate_delivery",
            u128::from(facts.duplicate_deliveries),
            "delivery",
            ValueClassification::Measured,
        )?,
        measurement(
            "database.transaction",
            u128::from(workload.event_count),
            "transaction",
            ValueClassification::Attributed,
        )?,
    ];

    Ok(EconomicObservation::new(
        EconomicUnitId::new(ECONOMIC_UNIT_ID)?,
        OperationId::new("sync.sqlite.round_trip")?,
        workload.event_count,
        measurements,
        ObservationProvenance::new(
            workload.software_revision.clone(),
            workload.deployment_profile.clone(),
            WORKLOAD_ID,
            COSTING_POLICY_REVISION,
            None,
            Confidence::Medium,
        )?,
    )?)
}

fn measurement(
    driver: &str,
    quantity: u128,
    unit: &str,
    classification: ValueClassification,
) -> Result<ConsumptionMeasurement, ValidationError> {
    ConsumptionMeasurement::new(
        ConsumptionDriverId::new(driver)?,
        quantity,
        unit,
        classification,
    )
}

fn event(edge_id: &EdgeId, sequence: u64, payload: &str) -> EdgeEvent {
    EdgeEvent::new(
        edge_id.clone(),
        EventId::new(format!("economic-event-{sequence}")),
        sequence,
        payload.to_owned(),
    )
}

fn unique_database_path() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!("uc-sync-economics-{nonce}.sqlite"))
}

fn file_size(path: &Path) -> Result<u64, HarnessError> {
    Ok(fs::metadata(path)?.len())
}

fn remove_database_files(path: &Path) -> Result<(), HarnessError> {
    for candidate in [
        path.to_path_buf(),
        path.with_extension("sqlite-wal"),
        path.with_extension("sqlite-shm"),
    ] {
        match fs::remove_file(candidate) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(HarnessError::Io(error)),
        }
    }
    Ok(())
}

/// Failure produced by the economic measurement harness.
#[derive(Debug)]
pub enum HarnessError {
    /// The workload definition is invalid.
    InvalidWorkload(&'static str),
    /// `SQLite` synchronization failed.
    Sync(SqliteSyncStoreError),
    /// Economic contract validation failed.
    Economics(ValidationError),
    /// Filesystem measurement or cleanup failed.
    Io(std::io::Error),
    /// The sync adapter returned a result that violates the workload invariant.
    UnexpectedResult(&'static str),
}

impl From<SqliteSyncStoreError> for HarnessError {
    fn from(error: SqliteSyncStoreError) -> Self {
        Self::Sync(error)
    }
}

impl From<ValidationError> for HarnessError {
    fn from(error: ValidationError) -> Self {
        Self::Economics(error)
    }
}

impl From<std::io::Error> for HarnessError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_workloads() {
        let mut workload = SyncWorkload::store_edge("commit-1");
        workload.event_count = 0;
        assert!(matches!(
            run_sync_workload(&workload),
            Err(HarnessError::InvalidWorkload(_))
        ));
    }

    #[test]
    fn records_direct_facts_and_attributed_transactions() {
        let workload = SyncWorkload {
            event_count: 10,
            payload_bytes: 32,
            duplicate_attempts: 2,
            software_revision: "commit-1".to_owned(),
            deployment_profile: "store-edge".to_owned(),
        };

        let report = run_sync_workload(&workload).expect("measurement run succeeds");
        assert_eq!(report.observation().unit_count(), 10);
        assert_eq!(report.facts().applied_events, 10);
        assert_eq!(report.facts().duplicate_deliveries, 2);
        assert_eq!(report.facts().delivery_attempts, 12);
        assert_eq!(report.facts().logical_wan_bytes, 384);
        assert!((report.delivery_amplification() - 1.2_f64).abs() < f64::EPSILON);
        assert!(report.applied_events_per_second().is_sign_positive());
        assert_eq!(report.observation().measurements().len(), 6);
    }
}

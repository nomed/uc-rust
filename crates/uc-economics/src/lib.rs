//! Provider-neutral contracts for Economics by Design observations.
//!
//! This crate models economic identity, physical consumption and provenance. It
//! deliberately does not import cloud billing SDKs, assign monetary rates or
//! claim full Cost-to-Serve.

use std::collections::BTreeMap;

/// Stable identifier of an economically meaningful or candidate unit.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EconomicUnitId(String);

impl EconomicUnitId {
    /// Creates an economic-unit identifier when the value is not blank.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        non_blank(value, "economic unit identifier").map(Self)
    }

    /// Returns the identifier as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identifier of an application operation or measured technical step.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationId(String);

impl OperationId {
    /// Creates an operation identifier when the value is not blank.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        non_blank(value, "operation identifier").map(Self)
    }

    /// Returns the identifier as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identifier of a canonical consumption driver.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConsumptionDriverId(String);

impl ConsumptionDriverId {
    /// Creates a consumption-driver identifier when the value is not blank.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        non_blank(value, "consumption driver identifier").map(Self)
    }

    /// Returns the identifier as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Describes how a value was obtained under the active costing policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueClassification {
    /// Directly observed physical consumption or billed monetary value.
    Measured,
    /// Causally assigned through a declared measurable driver.
    Attributed,
    /// Shared cost distributed through a declared allocation rule.
    Allocated,
    /// Modelled value used before sufficient observation exists.
    Estimated,
}

/// Confidence attached to an economic observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Confidence {
    /// Exploratory value that must not enforce a release gate.
    Low,
    /// Repeatable value with known limitations.
    Medium,
    /// Representative, reproducible value suitable for governed comparison.
    High,
}

/// One non-negative physical consumption measurement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumptionMeasurement {
    driver: ConsumptionDriverId,
    quantity: u128,
    unit: String,
    classification: ValueClassification,
}

impl ConsumptionMeasurement {
    /// Creates a physical consumption measurement.
    pub fn new(
        driver: ConsumptionDriverId,
        quantity: u128,
        unit: impl Into<String>,
        classification: ValueClassification,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            driver,
            quantity,
            unit: non_blank(unit, "measurement unit")?,
            classification,
        })
    }

    /// Returns the canonical consumption driver.
    pub fn driver(&self) -> &ConsumptionDriverId {
        &self.driver
    }

    /// Returns the non-negative quantity expressed in [`Self::unit`].
    pub const fn quantity(&self) -> u128 {
        self.quantity
    }

    /// Returns the physical unit of measure.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Returns how the measurement was obtained.
    pub const fn classification(&self) -> ValueClassification {
        self.classification
    }
}

/// Provenance required to compare or cost an observation reproducibly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationProvenance {
    software_revision: String,
    deployment_profile: String,
    workload_id: String,
    costing_policy_revision: String,
    rate_card_revision: Option<String>,
    confidence: Confidence,
}

impl ObservationProvenance {
    /// Creates provenance for a physical or monetary observation.
    pub fn new(
        software_revision: impl Into<String>,
        deployment_profile: impl Into<String>,
        workload_id: impl Into<String>,
        costing_policy_revision: impl Into<String>,
        rate_card_revision: Option<String>,
        confidence: Confidence,
    ) -> Result<Self, ValidationError> {
        let rate_card_revision = rate_card_revision
            .map(|value| non_blank(value, "rate-card revision"))
            .transpose()?;

        Ok(Self {
            software_revision: non_blank(software_revision, "software revision")?,
            deployment_profile: non_blank(deployment_profile, "deployment profile")?,
            workload_id: non_blank(workload_id, "workload identifier")?,
            costing_policy_revision: non_blank(
                costing_policy_revision,
                "costing-policy revision",
            )?,
            rate_card_revision,
            confidence,
        })
    }

    /// Returns the measured software revision.
    pub fn software_revision(&self) -> &str {
        &self.software_revision
    }

    /// Returns the deployment profile used by the workload.
    pub fn deployment_profile(&self) -> &str {
        &self.deployment_profile
    }

    /// Returns the representative workload identifier.
    pub fn workload_id(&self) -> &str {
        &self.workload_id
    }

    /// Returns the costing-policy revision.
    pub fn costing_policy_revision(&self) -> &str {
        &self.costing_policy_revision
    }

    /// Returns the monetary rate-card revision, when monetary costing occurred.
    pub fn rate_card_revision(&self) -> Option<&str> {
        self.rate_card_revision.as_deref()
    }

    /// Returns the declared confidence.
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }
}

/// Consumption attributable to one operation over a declared number of units.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EconomicObservation {
    economic_unit: EconomicUnitId,
    operation: OperationId,
    unit_count: u64,
    measurements: BTreeMap<ConsumptionDriverId, ConsumptionMeasurement>,
    provenance: ObservationProvenance,
}

impl EconomicObservation {
    /// Creates an observation for at least one economic unit.
    pub fn new(
        economic_unit: EconomicUnitId,
        operation: OperationId,
        unit_count: u64,
        measurements: impl IntoIterator<Item = ConsumptionMeasurement>,
        provenance: ObservationProvenance,
    ) -> Result<Self, ValidationError> {
        if unit_count == 0 {
            return Err(ValidationError::ZeroUnitCount);
        }

        let mut indexed = BTreeMap::new();
        for measurement in measurements {
            let driver = measurement.driver.clone();
            if indexed.insert(driver.clone(), measurement).is_some() {
                return Err(ValidationError::DuplicateDriver(driver));
            }
        }

        if indexed.is_empty() {
            return Err(ValidationError::MissingMeasurements);
        }

        Ok(Self {
            economic_unit,
            operation,
            unit_count,
            measurements: indexed,
            provenance,
        })
    }

    /// Returns the measured Economic Unit.
    pub fn economic_unit(&self) -> &EconomicUnitId {
        &self.economic_unit
    }

    /// Returns the measured operation.
    pub fn operation(&self) -> &OperationId {
        &self.operation
    }

    /// Returns the observed count of Economic Units.
    pub const fn unit_count(&self) -> u64 {
        self.unit_count
    }

    /// Returns all measurements in deterministic driver order.
    pub fn measurements(&self) -> impl ExactSizeIterator<Item = &ConsumptionMeasurement> {
        self.measurements.values()
    }

    /// Returns reproducibility and costing provenance.
    pub const fn provenance(&self) -> &ObservationProvenance {
        &self.provenance
    }
}

/// Validation failure for an economic contract value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError {
    /// A required textual value was blank.
    BlankValue(&'static str),
    /// An observation attempted to describe zero Economic Units.
    ZeroUnitCount,
    /// An observation contained no consumption measurements.
    MissingMeasurements,
    /// An observation contained the same canonical driver more than once.
    DuplicateDriver(ConsumptionDriverId),
}

fn non_blank(value: impl Into<String>, field: &'static str) -> Result<String, ValidationError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(ValidationError::BlankValue(field))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> ObservationProvenance {
        ObservationProvenance::new(
            "commit-1",
            "store-edge",
            "sync-durable-event-v1",
            "uc-rust-ebd-crawl-v1",
            None,
            Confidence::Medium,
        )
        .expect("valid fixture")
    }

    fn cpu(quantity: u128) -> ConsumptionMeasurement {
        ConsumptionMeasurement::new(
            ConsumptionDriverId::new("compute.cpu_time").expect("valid fixture"),
            quantity,
            "nanosecond",
            ValueClassification::Measured,
        )
        .expect("valid fixture")
    }

    #[test]
    fn rejects_blank_identifiers() {
        assert_eq!(
            EconomicUnitId::new("  "),
            Err(ValidationError::BlankValue("economic unit identifier"))
        );
    }

    #[test]
    fn builds_a_deterministic_observation() {
        let observation = EconomicObservation::new(
            EconomicUnitId::new("uc.synced_business_event").expect("valid fixture"),
            OperationId::new("sync.accept").expect("valid fixture"),
            100,
            [cpu(50_000)],
            provenance(),
        )
        .expect("valid observation");

        assert_eq!(observation.unit_count(), 100);
        assert_eq!(observation.measurements().len(), 1);
        assert_eq!(observation.provenance().rate_card_revision(), None);
    }

    #[test]
    fn rejects_duplicate_drivers() {
        let result = EconomicObservation::new(
            EconomicUnitId::new("uc.synced_business_event").expect("valid fixture"),
            OperationId::new("sync.accept").expect("valid fixture"),
            1,
            [cpu(1), cpu(2)],
            provenance(),
        );

        assert!(matches!(result, Err(ValidationError::DuplicateDriver(_))));
    }

    #[test]
    fn rejects_empty_or_zero_unit_observations() {
        let unit = EconomicUnitId::new("uc.synced_business_event").expect("valid fixture");
        let operation = OperationId::new("sync.accept").expect("valid fixture");

        assert_eq!(
            EconomicObservation::new(unit.clone(), operation.clone(), 0, [cpu(1)], provenance()),
            Err(ValidationError::ZeroUnitCount)
        );
        assert_eq!(
            EconomicObservation::new(unit, operation, 1, [], provenance()),
            Err(ValidationError::MissingMeasurements)
        );
    }
}

//! Provider-neutral synchronization primitives for distributed retail runtimes.
//!
//! This crate models durable edge publication, central deduplication and
//! explicit sequence handling without depending on a transport or database.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashSet, VecDeque};

/// Stable identity of an edge node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EdgeId(String);

impl EdgeId {
    /// Creates an edge identity.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

/// Globally unique identity of one published event.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventId(String);

impl EventId {
    /// Creates an event identity.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

/// Business effect emitted by an edge runtime.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeEvent {
    edge_id: EdgeId,
    event_id: EventId,
    sequence: u64,
    payload: String,
}

impl EdgeEvent {
    /// Creates a sequenced event for one edge.
    #[must_use]
    pub fn new(
        edge_id: EdgeId,
        event_id: EventId,
        sequence: u64,
        payload: impl Into<String>,
    ) -> Self {
        Self {
            edge_id,
            event_id,
            sequence,
            payload: payload.into(),
        }
    }

    /// Returns the source edge.
    #[must_use]
    pub const fn edge_id(&self) -> &EdgeId {
        &self.edge_id
    }

    /// Returns the event identity.
    #[must_use]
    pub const fn event_id(&self) -> &EventId {
        &self.event_id
    }

    /// Returns the monotonically increasing edge sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the opaque business payload used by this contract harness.
    #[must_use]
    pub fn payload(&self) -> &str {
        &self.payload
    }
}

/// Durable edge-side queue of unacknowledged events.
#[derive(Debug, Default)]
pub struct EdgeOutbox {
    pending: VecDeque<EdgeEvent>,
}

impl EdgeOutbox {
    /// Appends one event durably in publication order.
    pub fn enqueue(&mut self, event: EdgeEvent) {
        self.pending.push_back(event);
    }

    /// Returns all unacknowledged events without removing them.
    #[must_use]
    pub fn pending(&self) -> Vec<EdgeEvent> {
        self.pending.iter().cloned().collect()
    }

    /// Acknowledges one event after central persistence.
    pub fn acknowledge(&mut self, event_id: &EventId) -> bool {
        let Some(position) = self
            .pending
            .iter()
            .position(|event| event.event_id() == event_id)
        else {
            return false;
        };
        self.pending.remove(position);
        true
    }

    /// Returns the current backlog size.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Returns whether no events await acknowledgement.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

/// Result of offering one event to the central inbox.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcceptResult {
    /// The event was accepted and became the next business effect.
    Applied,
    /// The event was already processed and produced no duplicate effect.
    Duplicate,
    /// The event is valid but waits for an earlier sequence.
    Buffered,
}

/// Central inbox that deduplicates events and applies each edge sequence in order.
#[derive(Debug, Default)]
pub struct CentralInbox {
    seen: HashSet<EventId>,
    next_sequence: BTreeMap<EdgeId, u64>,
    buffered: BTreeMap<EdgeId, BTreeMap<u64, EdgeEvent>>,
    applied: Vec<EdgeEvent>,
}

impl CentralInbox {
    /// Accepts one event with deduplication and per-edge sequence ordering.
    pub fn accept(&mut self, event: EdgeEvent) -> AcceptResult {
        if self.seen.contains(event.event_id()) {
            return AcceptResult::Duplicate;
        }

        let expected = self
            .next_sequence
            .get(event.edge_id())
            .copied()
            .unwrap_or(1);

        if event.sequence() > expected {
            self.buffered
                .entry(event.edge_id().clone())
                .or_default()
                .insert(event.sequence(), event);
            return AcceptResult::Buffered;
        }

        if event.sequence() < expected {
            self.seen.insert(event.event_id().clone());
            return AcceptResult::Duplicate;
        }

        let edge_id = event.edge_id().clone();
        self.apply(event);
        self.drain_ready(&edge_id);
        AcceptResult::Applied
    }

    fn apply(&mut self, event: EdgeEvent) {
        let next = event.sequence().saturating_add(1);
        self.next_sequence.insert(event.edge_id().clone(), next);
        self.seen.insert(event.event_id().clone());
        self.applied.push(event);
    }

    fn drain_ready(&mut self, edge_id: &EdgeId) {
        loop {
            let expected = self.next_sequence.get(edge_id).copied().unwrap_or(1);
            let event = self
                .buffered
                .get_mut(edge_id)
                .and_then(|events| events.remove(&expected));
            let Some(event) = event else {
                break;
            };
            self.apply(event);
        }
    }

    /// Returns applied business effects in central order.
    #[must_use]
    pub fn applied(&self) -> &[EdgeEvent] {
        &self.applied
    }

    /// Returns the number of events waiting for an earlier sequence.
    #[must_use]
    pub fn buffered_len(&self) -> usize {
        self.buffered.values().map(BTreeMap::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{AcceptResult, CentralInbox, EdgeEvent, EdgeId, EdgeOutbox, EventId};

    fn event(sequence: u64) -> EdgeEvent {
        EdgeEvent::new(
            EdgeId::new("store-1"),
            EventId::new(format!("event-{sequence}")),
            sequence,
            format!("sale-{sequence}"),
        )
    }

    #[test]
    fn wan_partition_preserves_the_complete_edge_backlog() {
        let mut outbox = EdgeOutbox::default();
        outbox.enqueue(event(1));
        outbox.enqueue(event(2));

        let after_restart = outbox.pending();

        assert_eq!(after_restart.len(), 2);
        assert_eq!(after_restart[0].payload(), "sale-1");
        assert_eq!(after_restart[1].payload(), "sale-2");
    }

    #[test]
    fn duplicate_delivery_produces_one_business_effect() {
        let mut inbox = CentralInbox::default();
        let first = event(1);

        assert_eq!(inbox.accept(first.clone()), AcceptResult::Applied);
        assert_eq!(inbox.accept(first), AcceptResult::Duplicate);
        assert_eq!(inbox.applied().len(), 1);
    }

    #[test]
    fn reordered_delivery_is_buffered_then_applied_in_sequence() {
        let mut inbox = CentralInbox::default();

        assert_eq!(inbox.accept(event(2)), AcceptResult::Buffered);
        assert_eq!(inbox.buffered_len(), 1);
        assert_eq!(inbox.accept(event(1)), AcceptResult::Applied);
        assert_eq!(inbox.buffered_len(), 0);
        assert_eq!(inbox.applied()[0].sequence(), 1);
        assert_eq!(inbox.applied()[1].sequence(), 2);
    }

    #[test]
    fn acknowledgements_remove_only_confirmed_events() {
        let mut outbox = EdgeOutbox::default();
        let first = event(1);
        let second = event(2);
        outbox.enqueue(first.clone());
        outbox.enqueue(second);

        assert!(outbox.acknowledge(first.event_id()));
        assert!(!outbox.acknowledge(first.event_id()));
        assert_eq!(outbox.len(), 1);
        assert!(!outbox.is_empty());
    }
}

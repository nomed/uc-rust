//! Core Unified Commerce domain types and invariants.
//!
//! This crate is independent from transport, persistence, messaging and provider SDKs.

#![forbid(unsafe_code)]

use std::fmt;

/// Stable identifier of a basket.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BasketId(String);

impl BasketId {
    /// Creates a basket identifier from its canonical string representation.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the canonical string representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BasketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable identifier of a sellable product.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProductId(String);

impl ProductId {
    /// Creates a product identifier from its canonical string representation.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the canonical string representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Monetary amount represented in minor units and an ISO-style three-byte currency code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Money {
    minor_units: i64,
    currency: [u8; 3],
}

impl Money {
    /// Creates a monetary amount without floating-point arithmetic.
    #[must_use]
    pub const fn new(minor_units: i64, currency: [u8; 3]) -> Self {
        Self { minor_units, currency }
    }

    /// Returns the signed amount in the currency's minor units.
    #[must_use]
    pub const fn minor_units(self) -> i64 { self.minor_units }

    /// Returns the three-byte currency code.
    #[must_use]
    pub const fn currency(self) -> [u8; 3] { self.currency }
}

/// One product line in a basket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasketLine {
    product_id: ProductId,
    quantity: u32,
    unit_price: Money,
}

impl BasketLine {
    /// Returns the product identifier.
    #[must_use]
    pub const fn product_id(&self) -> &ProductId { &self.product_id }

    /// Returns the strictly positive line quantity.
    #[must_use]
    pub const fn quantity(&self) -> u32 { self.quantity }

    /// Returns the unit price.
    #[must_use]
    pub const fn unit_price(&self) -> Money { self.unit_price }
}

/// Domain failures produced while changing a basket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasketError {
    /// The requested quantity is zero.
    InvalidQuantity,
    /// A line currency differs from currencies already present in the basket.
    CurrencyMismatch,
}

/// Aggregate root representing a retail basket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Basket {
    id: BasketId,
    lines: Vec<BasketLine>,
}

impl Basket {
    /// Creates an empty basket.
    #[must_use]
    pub const fn new(id: BasketId) -> Self { Self { id, lines: Vec::new() } }

    /// Adds a product line while enforcing positive quantity and one basket currency.
    pub fn add_product(&mut self, product_id: ProductId, quantity: u32, unit_price: Money) -> Result<(), BasketError> {
        if quantity == 0 { return Err(BasketError::InvalidQuantity); }
        if self.lines.iter().any(|line| line.unit_price.currency() != unit_price.currency()) {
            return Err(BasketError::CurrencyMismatch);
        }
        self.lines.push(BasketLine { product_id, quantity, unit_price });
        Ok(())
    }

    /// Calculates the basket total, or `None` when the basket is empty.
    #[must_use]
    pub fn total(&self) -> Option<Money> {
        let first = self.lines.first()?;
        let total = self.lines.iter().map(|line| line.unit_price.minor_units() * i64::from(line.quantity)).sum();
        Some(Money::new(total, first.unit_price.currency()))
    }

    /// Returns the basket identifier.
    #[must_use]
    pub const fn id(&self) -> &BasketId { &self.id }

    /// Returns basket lines in domain order.
    #[must_use]
    pub fn lines(&self) -> &[BasketLine] { &self.lines }
}

#[cfg(test)]
mod tests {
    use super::{Basket, BasketError, BasketId, Money, ProductId};

    #[test]
    fn calculates_basket_total_in_minor_units() {
        let mut basket = Basket::new(BasketId::new("basket-1"));
        basket.add_product(ProductId::new("sku-1"), 2, Money::new(1_250, *b"EUR")).expect("valid line");
        assert_eq!(basket.total(), Some(Money::new(2_500, *b"EUR")));
    }

    #[test]
    fn empty_basket_has_no_total() {
        assert_eq!(Basket::new(BasketId::new("basket-1")).total(), None);
    }

    #[test]
    fn rejects_zero_quantity() {
        let mut basket = Basket::new(BasketId::new("basket-1"));
        assert_eq!(basket.add_product(ProductId::new("sku-1"), 0, Money::new(1_250, *b"EUR")), Err(BasketError::InvalidQuantity));
    }

    #[test]
    fn rejects_currency_mismatch() {
        let mut basket = Basket::new(BasketId::new("basket-1"));
        basket.add_product(ProductId::new("sku-1"), 1, Money::new(100, *b"EUR")).expect("first line establishes currency");
        assert_eq!(basket.add_product(ProductId::new("sku-2"), 1, Money::new(100, *b"USD")), Err(BasketError::CurrencyMismatch));
    }

    #[test]
    fn identifiers_expose_canonical_value() {
        assert_eq!(BasketId::new("basket-1").as_str(), "basket-1");
        assert_eq!(ProductId::new("sku-1").as_str(), "sku-1");
    }
}

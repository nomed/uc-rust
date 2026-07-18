#![forbid(unsafe_code)]

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BasketId(String);

impl BasketId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for BasketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProductId(String);

impl ProductId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Money {
    minor_units: i64,
    currency: [u8; 3],
}

impl Money {
    #[must_use]
    pub const fn new(minor_units: i64, currency: [u8; 3]) -> Self {
        Self {
            minor_units,
            currency,
        }
    }

    #[must_use]
    pub const fn minor_units(self) -> i64 {
        self.minor_units
    }

    #[must_use]
    pub const fn currency(self) -> [u8; 3] {
        self.currency
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasketLine {
    product_id: ProductId,
    quantity: u32,
    unit_price: Money,
}

impl BasketLine {
    #[must_use]
    pub const fn quantity(&self) -> u32 {
        self.quantity
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasketError {
    InvalidQuantity,
    CurrencyMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Basket {
    id: BasketId,
    lines: Vec<BasketLine>,
}

impl Basket {
    #[must_use]
    pub const fn new(id: BasketId) -> Self {
        Self {
            id,
            lines: Vec::new(),
        }
    }

    pub fn add_product(
        &mut self,
        product_id: ProductId,
        quantity: u32,
        unit_price: Money,
    ) -> Result<(), BasketError> {
        if quantity == 0 {
            return Err(BasketError::InvalidQuantity);
        }

        if self
            .lines
            .iter()
            .any(|line| line.unit_price.currency() != unit_price.currency())
        {
            return Err(BasketError::CurrencyMismatch);
        }

        self.lines.push(BasketLine {
            product_id,
            quantity,
            unit_price,
        });
        Ok(())
    }

    #[must_use]
    pub fn total(&self) -> Option<Money> {
        let first = self.lines.first()?;
        let total = self.lines.iter().map(|line| {
            line.unit_price.minor_units() * i64::from(line.quantity)
        }).sum();

        Some(Money::new(total, first.unit_price.currency()))
    }

    #[must_use]
    pub const fn id(&self) -> &BasketId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::{Basket, BasketError, BasketId, Money, ProductId};

    #[test]
    fn calculates_basket_total_in_minor_units() {
        let mut basket = Basket::new(BasketId::new("basket-1"));
        basket
            .add_product(
                ProductId::new("sku-1"),
                2,
                Money::new(1_250, *b"EUR"),
            )
            .expect("valid line");

        assert_eq!(basket.total(), Some(Money::new(2_500, *b"EUR")));
    }

    #[test]
    fn rejects_zero_quantity() {
        let mut basket = Basket::new(BasketId::new("basket-1"));
        let result = basket.add_product(
            ProductId::new("sku-1"),
            0,
            Money::new(1_250, *b"EUR"),
        );

        assert_eq!(result, Err(BasketError::InvalidQuantity));
    }
}

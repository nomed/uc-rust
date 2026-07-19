//! PostgreSQL implementation of the basket persistence port.

#![forbid(unsafe_code)]

use postgres::{Client, NoTls, Transaction};
use std::fmt;
use uc_application::BasketRepository;
use uc_domain::{Basket, BasketId, Money, ProductId};

/// Failures produced by the PostgreSQL basket repository.
#[derive(Debug)]
pub enum PostgresBasketRepositoryError {
    /// PostgreSQL rejected an operation.
    Database(postgres::Error),
    /// Stored data cannot reconstruct a valid domain aggregate.
    CorruptData(&'static str),
}

impl From<postgres::Error> for PostgresBasketRepositoryError {
    fn from(error: postgres::Error) -> Self {
        Self::Database(error)
    }
}

/// PostgreSQL-backed basket repository.
pub struct PostgresBasketRepository {
    client: Client,
}

impl fmt::Debug for PostgresBasketRepository {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PostgresBasketRepository")
            .finish_non_exhaustive()
    }
}

impl PostgresBasketRepository {
    /// Connects to PostgreSQL and applies the idempotent schema.
    pub fn connect(connection_string: &str) -> Result<Self, PostgresBasketRepositoryError> {
        let mut client = Client::connect(connection_string, NoTls)?;
        client.batch_execute(
            "CREATE TABLE IF NOT EXISTS baskets (
                 basket_id TEXT PRIMARY KEY
             );
             CREATE TABLE IF NOT EXISTS basket_lines (
                 basket_id TEXT NOT NULL REFERENCES baskets(basket_id) ON DELETE CASCADE,
                 line_no BIGINT NOT NULL,
                 product_id TEXT NOT NULL,
                 quantity BIGINT NOT NULL CHECK (quantity > 0),
                 unit_price_minor BIGINT NOT NULL,
                 currency BYTEA NOT NULL CHECK (octet_length(currency) = 3),
                 PRIMARY KEY (basket_id, line_no)
             );",
        )?;
        Ok(Self { client })
    }

    fn write_basket(
        transaction: &mut Transaction<'_>,
        basket: &Basket,
    ) -> Result<(), PostgresBasketRepositoryError> {
        transaction.execute(
            "INSERT INTO baskets (basket_id) VALUES ($1)
             ON CONFLICT (basket_id) DO NOTHING",
            &[&basket.id().as_str()],
        )?;
        transaction.execute(
            "DELETE FROM basket_lines WHERE basket_id = $1",
            &[&basket.id().as_str()],
        )?;
        for (line_no, line) in basket.lines().iter().enumerate() {
            let line_no = i64::try_from(line_no)
                .map_err(|_| PostgresBasketRepositoryError::CorruptData("line number overflow"))?;
            let quantity = i64::from(line.quantity());
            let currency = line.unit_price().currency().to_vec();
            transaction.execute(
                "INSERT INTO basket_lines
                 (basket_id, line_no, product_id, quantity, unit_price_minor, currency)
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &basket.id().as_str(),
                    &line_no,
                    &line.product_id().as_str(),
                    &quantity,
                    &line.unit_price().minor_units(),
                    &currency,
                ],
            )?;
        }
        Ok(())
    }
}

impl BasketRepository for PostgresBasketRepository {
    type Error = PostgresBasketRepositoryError;

    fn save(&mut self, basket: &Basket) -> Result<(), Self::Error> {
        let mut transaction = self.client.transaction()?;
        Self::write_basket(&mut transaction, basket)?;
        transaction.commit()?;
        Ok(())
    }

    fn load(&mut self, basket_id: &BasketId) -> Result<Option<Basket>, Self::Error> {
        let exists = self.client.query_opt(
            "SELECT basket_id FROM baskets WHERE basket_id = $1",
            &[&basket_id.as_str()],
        )?;
        if exists.is_none() {
            return Ok(None);
        }

        let rows = self.client.query(
            "SELECT product_id, quantity, unit_price_minor, currency
             FROM basket_lines WHERE basket_id = $1 ORDER BY line_no",
            &[&basket_id.as_str()],
        )?;
        let mut basket = Basket::new(basket_id.clone());
        for row in rows {
            let quantity = u32::try_from(row.get::<_, i64>(1))
                .map_err(|_| PostgresBasketRepositoryError::CorruptData("quantity out of range"))?;
            let currency: [u8; 3] = row
                .get::<_, Vec<u8>>(3)
                .try_into()
                .map_err(|_| PostgresBasketRepositoryError::CorruptData("currency must be three bytes"))?;
            basket
                .add_product(
                    ProductId::new(row.get::<_, String>(0)),
                    quantity,
                    Money::new(row.get::<_, i64>(2), currency),
                )
                .map_err(|_| {
                    PostgresBasketRepositoryError::CorruptData(
                        "stored basket violates domain invariants",
                    )
                })?;
        }
        Ok(Some(basket))
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresBasketRepository;
    use uc_persistence_contract::assert_basket_repository_contract;

    #[test]
    #[ignore = "requires UC_TEST_POSTGRES_URL and is intentionally opt-in"]
    fn satisfies_basket_repository_contract() {
        let url = std::env::var("UC_TEST_POSTGRES_URL").expect("UC_TEST_POSTGRES_URL is set");
        let mut repository = PostgresBasketRepository::connect(&url).expect("PostgreSQL connects");
        assert_basket_repository_contract(&mut repository);
    }
}

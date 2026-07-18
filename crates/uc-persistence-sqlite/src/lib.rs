//! SQLite implementation of the basket persistence port.

#![forbid(unsafe_code)]

use rusqlite::{params, Connection, OptionalExtension, Transaction};
use uc_application::BasketRepository;
use uc_domain::{Basket, BasketId, Money, ProductId};

/// Failures produced by the SQLite basket repository.
#[derive(Debug)]
pub enum SqliteBasketRepositoryError {
    /// SQLite rejected an operation.
    Database(rusqlite::Error),
    /// Stored data cannot reconstruct a valid domain aggregate.
    CorruptData(&'static str),
}

impl From<rusqlite::Error> for SqliteBasketRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

/// SQLite-backed basket repository.
#[derive(Debug)]
pub struct SqliteBasketRepository {
    connection: Connection,
}

impl SqliteBasketRepository {
    /// Creates a repository and applies its idempotent schema.
    pub fn new(connection: Connection) -> Result<Self, SqliteBasketRepositoryError> {
        let repository = Self { connection };
        repository.connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS baskets (
                 basket_id TEXT PRIMARY KEY
             );
             CREATE TABLE IF NOT EXISTS basket_lines (
                 basket_id TEXT NOT NULL,
                 line_no INTEGER NOT NULL,
                 product_id TEXT NOT NULL,
                 quantity INTEGER NOT NULL CHECK (quantity > 0),
                 unit_price_minor INTEGER NOT NULL,
                 currency BLOB NOT NULL CHECK (length(currency) = 3),
                 PRIMARY KEY (basket_id, line_no),
                 FOREIGN KEY (basket_id) REFERENCES baskets(basket_id) ON DELETE CASCADE
             );",
        )?;
        Ok(repository)
    }

    fn write_basket(transaction: &Transaction<'_>, basket: &Basket) -> Result<(), SqliteBasketRepositoryError> {
        transaction.execute(
            "INSERT INTO baskets (basket_id) VALUES (?1)
             ON CONFLICT(basket_id) DO NOTHING",
            params![basket.id().as_str()],
        )?;
        transaction.execute(
            "DELETE FROM basket_lines WHERE basket_id = ?1",
            params![basket.id().as_str()],
        )?;
        for (line_no, line) in basket.lines().iter().enumerate() {
            transaction.execute(
                "INSERT INTO basket_lines
                 (basket_id, line_no, product_id, quantity, unit_price_minor, currency)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    basket.id().as_str(),
                    i64::try_from(line_no).map_err(|_| SqliteBasketRepositoryError::CorruptData("line number overflow"))?,
                    line.product_id().as_str(),
                    i64::from(line.quantity()),
                    line.unit_price().minor_units(),
                    line.unit_price().currency().to_vec(),
                ],
            )?;
        }
        Ok(())
    }
}

impl BasketRepository for SqliteBasketRepository {
    type Error = SqliteBasketRepositoryError;

    fn save(&mut self, basket: &Basket) -> Result<(), Self::Error> {
        let transaction = self.connection.transaction()?;
        Self::write_basket(&transaction, basket)?;
        transaction.commit()?;
        Ok(())
    }

    fn load(&mut self, basket_id: &BasketId) -> Result<Option<Basket>, Self::Error> {
        let exists = self
            .connection
            .query_row(
                "SELECT basket_id FROM baskets WHERE basket_id = ?1",
                params![basket_id.as_str()],
                |_| Ok(()),
            )
            .optional()?;
        if exists.is_none() {
            return Ok(None);
        }

        let mut statement = self.connection.prepare(
            "SELECT product_id, quantity, unit_price_minor, currency
             FROM basket_lines WHERE basket_id = ?1 ORDER BY line_no",
        )?;
        let rows = statement.query_map(params![basket_id.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, Vec<u8>>(3)?,
            ))
        })?;

        let mut basket = Basket::new(basket_id.clone());
        for row in rows {
            let (product_id, quantity, minor_units, currency) = row?;
            let quantity = u32::try_from(quantity)
                .map_err(|_| SqliteBasketRepositoryError::CorruptData("quantity out of range"))?;
            let currency: [u8; 3] = currency
                .try_into()
                .map_err(|_| SqliteBasketRepositoryError::CorruptData("currency must be three bytes"))?;
            basket
                .add_product(ProductId::new(product_id), quantity, Money::new(minor_units, currency))
                .map_err(|_| SqliteBasketRepositoryError::CorruptData("stored basket violates domain invariants"))?;
        }
        Ok(Some(basket))
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteBasketRepository;
    use rusqlite::Connection;
    use uc_persistence_contract::assert_basket_repository_contract;

    #[test]
    fn satisfies_basket_repository_contract() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite opens");
        let mut repository = SqliteBasketRepository::new(connection).expect("schema applies");
        assert_basket_repository_contract(&mut repository);
    }
}

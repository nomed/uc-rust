//! Use-case-oriented application operations and provider-neutral ports.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use uc_domain::{Basket, BasketError, BasketId, Money, ProductId};

/// Persistence port required by basket application operations.
pub trait BasketRepository {
    /// Provider-specific repository error.
    type Error;

    /// Persists the complete basket aggregate atomically.
    fn save(&mut self, basket: &Basket) -> Result<(), Self::Error>;

    /// Loads a basket by identifier.
    fn load(&mut self, basket_id: &BasketId) -> Result<Option<Basket>, Self::Error>;
}

/// Input contract for adding a product to a basket.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct AddProductRequest {
    /// Basket identifier.
    pub basket_id: String,
    /// Product identifier.
    pub product_id: String,
    /// Strictly positive quantity.
    pub quantity: u32,
    /// Unit price in minor currency units.
    pub unit_price_minor: i64,
    /// Three-letter currency code.
    pub currency: String,
}

/// Output contract returned after adding a product.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct AddProductResponse {
    /// Basket identifier.
    pub basket_id: String,
    /// Basket total in minor currency units.
    pub total_minor: i64,
    /// Three-letter currency code.
    pub currency: String,
}

/// Failures produced by the add-product operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddProductError<E> {
    /// A domain invariant rejected the request.
    Domain(BasketError),
    /// The repository failed while persisting the aggregate.
    Repository(E),
}

/// Canonical application operation that adds one product and persists the basket.
#[derive(Debug)]
pub struct AddProductToBasket<'a, R> {
    repository: &'a mut R,
}

impl<'a, R> AddProductToBasket<'a, R>
where
    R: BasketRepository,
{
    /// Creates the operation with its persistence port.
    pub const fn new(repository: &'a mut R) -> Self {
        Self { repository }
    }

    /// Executes the operation once.
    pub fn execute(
        &mut self,
        basket_id: BasketId,
        product_id: ProductId,
        quantity: u32,
        unit_price: Money,
    ) -> Result<Basket, AddProductError<R::Error>> {
        let mut basket = Basket::new(basket_id);
        basket
            .add_product(product_id, quantity, unit_price)
            .map_err(AddProductError::Domain)?;
        self.repository
            .save(&basket)
            .map_err(AddProductError::Repository)?;
        Ok(basket)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddProductError, AddProductRequest, AddProductResponse, AddProductToBasket,
        BasketRepository,
    };
    use uc_domain::{Basket, BasketError, BasketId, Money, ProductId};

    #[derive(Default)]
    struct InMemoryBasketRepository {
        saved: Vec<Basket>,
        fail: bool,
    }

    impl BasketRepository for InMemoryBasketRepository {
        type Error = &'static str;

        fn save(&mut self, basket: &Basket) -> Result<(), Self::Error> {
            if self.fail {
                return Err("repository unavailable");
            }
            self.saved.retain(|current| current.id() != basket.id());
            self.saved.push(basket.clone());
            Ok(())
        }

        fn load(&mut self, basket_id: &BasketId) -> Result<Option<Basket>, Self::Error> {
            if self.fail {
                return Err("repository unavailable");
            }
            Ok(self
                .saved
                .iter()
                .find(|basket| basket.id() == basket_id)
                .cloned())
        }
    }

    #[test]
    fn persists_a_valid_basket() {
        let mut repository = InMemoryBasketRepository::default();
        let mut use_case = AddProductToBasket::new(&mut repository);
        let basket = use_case
            .execute(
                BasketId::new("basket-1"),
                ProductId::new("sku-1"),
                1,
                Money::new(999, *b"EUR"),
            )
            .expect("valid basket");
        assert_eq!(basket.total(), Some(Money::new(999, *b"EUR")));
        assert_eq!(
            repository.load(&BasketId::new("basket-1")).expect("load"),
            Some(basket)
        );
    }

    #[test]
    fn preserves_domain_failure() {
        let mut repository = InMemoryBasketRepository::default();
        let result = AddProductToBasket::new(&mut repository).execute(
            BasketId::new("basket-1"),
            ProductId::new("sku-1"),
            0,
            Money::new(999, *b"EUR"),
        );
        assert_eq!(
            result,
            Err(AddProductError::Domain(BasketError::InvalidQuantity))
        );
    }

    #[test]
    fn preserves_repository_failure() {
        let mut repository = InMemoryBasketRepository {
            saved: Vec::new(),
            fail: true,
        };
        let result = AddProductToBasket::new(&mut repository).execute(
            BasketId::new("basket-1"),
            ProductId::new("sku-1"),
            1,
            Money::new(999, *b"EUR"),
        );
        assert_eq!(
            result,
            Err(AddProductError::Repository("repository unavailable"))
        );
    }

    #[test]
    fn canonical_request_fixture_round_trips() {
        let source =
            include_str!("../../../fixtures/contracts/v1/basket/add-product-request.valid.json");
        let request: AddProductRequest = serde_json::from_str(source).expect("valid fixture");
        let serialized = serde_json::to_string_pretty(&request).expect("serializable request");
        assert_eq!(
            request,
            serde_json::from_str(&serialized).expect("round trip")
        );
    }

    #[test]
    fn canonical_response_fixture_round_trips() {
        let source =
            include_str!("../../../fixtures/contracts/v1/basket/add-product-response.valid.json");
        let response: AddProductResponse = serde_json::from_str(source).expect("valid fixture");
        let serialized = serde_json::to_string_pretty(&response).expect("serializable response");
        assert_eq!(
            response,
            serde_json::from_str(&serialized).expect("round trip")
        );
    }
}

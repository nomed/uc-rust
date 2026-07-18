#![forbid(unsafe_code)]

use uc_domain::{Basket, BasketError, BasketId, Money, ProductId};

pub trait BasketRepository {
    type Error;

    fn save(&mut self, basket: &Basket) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddProductError<E> {
    Domain(BasketError),
    Repository(E),
}

pub struct AddProductToBasket<'a, R> {
    repository: &'a mut R,
}

impl<'a, R> AddProductToBasket<'a, R>
where
    R: BasketRepository,
{
    pub const fn new(repository: &'a mut R) -> Self {
        Self { repository }
    }

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
    use super::{AddProductToBasket, BasketRepository};
    use uc_domain::{Basket, BasketId, Money, ProductId};

    #[derive(Default)]
    struct InMemoryBasketRepository {
        saved: Vec<Basket>,
    }

    impl BasketRepository for InMemoryBasketRepository {
        type Error = ();

        fn save(&mut self, basket: &Basket) -> Result<(), Self::Error> {
            self.saved.push(basket.clone());
            Ok(())
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
        assert_eq!(repository.saved.len(), 1);
    }
}

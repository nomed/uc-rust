//! Reusable contract tests for basket persistence adapters.

#![forbid(unsafe_code)]

use std::fmt::Debug;
use uc_application::BasketRepository;
use uc_domain::{Basket, BasketId, Money, ProductId};

/// Executes the provider-neutral basket repository contract.
///
/// The repository must support a full aggregate round trip, return `None` for
/// unknown identifiers and replace an existing aggregate with the same ID.
pub fn assert_basket_repository_contract<R>(repository: &mut R)
where
    R: BasketRepository,
    R::Error: Debug,
{
    let id = BasketId::new("contract-basket");
    assert_eq!(repository.load(&id).expect("unknown load succeeds"), None);

    let mut first = Basket::new(id.clone());
    first
        .add_product(ProductId::new("sku-1"), 2, Money::new(1_250, *b"EUR"))
        .expect("valid first basket");
    repository.save(&first).expect("first save succeeds");
    assert_eq!(repository.load(&id).expect("first load succeeds"), Some(first));

    let mut replacement = Basket::new(id.clone());
    replacement
        .add_product(ProductId::new("sku-2"), 1, Money::new(999, *b"EUR"))
        .expect("valid replacement basket");
    repository.save(&replacement).expect("replacement save succeeds");
    assert_eq!(
        repository.load(&id).expect("replacement load succeeds"),
        Some(replacement)
    );
}

use lethal_company_cli::inventory::{InventoryError, ItemKind};
use lethal_company_cli::state::{BuyError, GameState, WalletError};

#[test]
fn buy_valid_item_debits_and_adds_to_inventory() {
    let mut state = GameState::new();
    let result = state.buy("shovel");
    assert_eq!(result, Ok(ItemKind::Shovel));
    assert_eq!(state.credits(), 70);
    assert_eq!(state.inventory().current_quantity(), 1);
}

#[test]
fn buy_invalid_item_returns_unknown_item() {
    let mut state = GameState::new();
    let result = state.buy("unknown");
    assert_eq!(result, Err(BuyError::UnknownItem("unknown".into())));
    assert_eq!(state.credits(), 100);
}

#[test]
fn buy_with_insufficient_credits_returns_insufficient_funds() {
    let mut state = GameState::new();
    for _ in 0..3 {
        let valid_buy = state.buy("shovel");
        assert_eq!(valid_buy, Ok(ItemKind::Shovel));
    }
    let result = state.buy("shovel");
    assert_eq!(
        result,
        Err(BuyError::Wallet(WalletError::InsufficientFunds {
            available: 10,
            requested: 30
        }))
    );
    assert_eq!(state.credits(), 10);
}

#[test]
fn buy_with_inventory_full_returns_inventory_full() {
    let mut state = GameState::new();
    let result = state.add_credits(300);
    assert_eq!(result, Ok(()));
    for _ in 0..16 {
        let valid_buy = state.buy("lockpicker");
        assert_eq!(valid_buy, Ok(ItemKind::Lockpicker));
    }
    let result = state.buy("lockpicker");
    assert_eq!(
        result,
        Err(BuyError::Inventory(InventoryError::Full { capacity: 16 }))
    );
    assert_eq!(state.credits(), 80);
}

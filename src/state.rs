use crate::inventory::{Inventory, InventoryError, ItemKind, ParseItemKindError};
use crate::store::price_of;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct GameState {
    credits: u32,
    day: u16,
    current_planet: Option<String>,
    inventory: Inventory,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            credits: 100,
            day: 1,
            current_planet: None,
            inventory: Inventory::new(),
        }
    }

    pub fn credits(&self) -> u32 {
        self.credits
    }

    pub fn day(&self) -> u16 {
        self.day
    }

    pub fn current_planet(&self) -> Option<&str> {
        self.current_planet.as_deref()
    }

    pub fn advance_day(&mut self) {
        self.day += 1;
    }

    pub fn add_credits(&mut self, amount: u32) -> Result<(), WalletError> {
        match self.credits.checked_add(amount) {
            Some(new_credits) => {
                self.credits = new_credits;
                Ok(())
            }
            None => Err(WalletError::Overflow),
        }
    }

    pub fn spend_credits(&mut self, amount: u32) -> Result<(), WalletError> {
        match self.credits.checked_sub(amount) {
            Some(new_credits) => {
                self.credits = new_credits;
                Ok(())
            }
            None => Err(WalletError::InsufficientFunds {
                available: self.credits,
                requested: amount,
            }),
        }
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn buy(&mut self, item: &str) -> Result<ItemKind, BuyError> {
        let kind = ItemKind::from_str(item)?;
        let item_price = price_of(kind).ok_or(BuyError::NotForSale(kind))?;
        self.inventory.ensure_capacity()?;
        self.spend_credits(item_price)?;
        self.inventory.add_item(kind)?;
        Ok(kind)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let location = self.current_planet().unwrap_or("ORBIT");
        writeln!(f, "{:<10} {} CR", "CREDITS:", self.credits)?;
        writeln!(f, "{:<10} {}", "DAY:", self.day)?;
        write!(f, "{:<10} {}", "LOCATION:", location)
    }
}

#[derive(Debug, PartialEq)]
pub enum WalletError {
    InsufficientFunds { available: u32, requested: u32 },
    Overflow,
}

impl Display for WalletError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::InsufficientFunds {
                available,
                requested,
            } => {
                write!(
                    f,
                    "TRANSACTION DENIED: need {requested} CR, balance {available} CR"
                )
            }
            WalletError::Overflow => {
                write!(
                    f,
                    "TRANSACTION OVERFLOW: credit balance exceeds maximum capacity"
                )
            }
        }
    }
}

impl Error for WalletError {}

#[derive(Debug, PartialEq)]
pub enum BuyError {
    UnknownItem(String),
    NotForSale(ItemKind),
    Wallet(WalletError),
    Inventory(InventoryError),
}

impl Display for BuyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuyError::UnknownItem(item) => write!(f, "UNKNOWN ITEM: {item}"),
            BuyError::NotForSale(item) => write!(f, "ITEM NOT AVAILABLE FOR SALE: {item}"),
            BuyError::Wallet(err) => write!(f, "WALLET ERROR: {err}"),
            BuyError::Inventory(err) => write!(f, "INVENTORY ERROR: {err}"),
        }
    }
}

impl Error for BuyError {}

impl From<WalletError> for BuyError {
    fn from(value: WalletError) -> Self {
        BuyError::Wallet(value)
    }
}

impl From<InventoryError> for BuyError {
    fn from(value: InventoryError) -> Self {
        BuyError::Inventory(value)
    }
}

impl From<ParseItemKindError> for BuyError {
    fn from(value: ParseItemKindError) -> Self {
        BuyError::UnknownItem(value.name().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_initial_state() {
        let state = GameState::new();
        assert_eq!(state.credits(), 100);
        assert_eq!(state.day(), 1);
        assert_eq!(state.current_planet(), None);
        assert_eq!(state.inventory().current_quantity(), 0);
    }

    #[test]
    fn default_returns_initial_state() {
        let state = GameState::default();
        assert_eq!(state.credits(), 100);
        assert_eq!(state.day(), 1);
        assert_eq!(state.current_planet(), None);
        assert_eq!(state.inventory().current_quantity(), 0);
    }

    #[test]
    fn advance_day_increments_day_by_one() {
        let mut state = GameState::new();
        let current_day = state.day();
        state.advance_day();
        assert_eq!(state.day(), current_day + 1);
    }

    #[test]
    fn display_shows_credits_day_and_location() {
        let state = GameState::new();
        let output = format!("{state}");
        assert!(output.contains("CREDITS:"));
        assert!(output.contains("100 CR"));
        assert!(output.contains("DAY:"));
        assert!(output.contains("LOCATION:"));
        assert!(output.contains("ORBIT"));
    }

    #[test]
    fn add_100_credits_increments_credits_by_100() {
        let mut state = GameState::new();
        let result = state.add_credits(100);
        assert!(result.is_ok());
        assert_eq!(state.credits(), 200);
    }

    #[test]
    fn add_max_u32_credits_returns_overflow_error() {
        let mut state = GameState::new();
        let result = state.add_credits(u32::MAX);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), WalletError::Overflow);
    }

    #[test]
    fn spend_insufficient_credits_returns_insufficient_funds_error() {
        let mut state = GameState::new();
        let result = state.spend_credits(1000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::InsufficientFunds {
                available: 100,
                requested: 1000
            }
        );
    }

    #[test]
    fn spend_10_credits_returns_ok() {
        let mut state = GameState::new();
        let result = state.spend_credits(10);
        assert!(result.is_ok());
        assert_eq!(state.credits(), 90);
    }

    #[test]
    fn wallet_error_converts_to_boxed_error() {
        let e = WalletError::Overflow;
        let _boxed: Box<dyn Error> = e.into();
    }

    #[test]
    fn wallet_error_converts_to_buy_error() {
        let e = WalletError::Overflow;
        assert_eq!(BuyError::from(e), BuyError::Wallet(WalletError::Overflow));
    }

    #[test]
    fn inventory_error_converts_to_buy_error() {
        let e = InventoryError::Full { capacity: 16 };
        assert_eq!(
            BuyError::from(e),
            BuyError::Inventory(InventoryError::Full { capacity: 16 })
        );
    }

    #[test]
    fn parse_error_converts_to_buy_error() {
        let e = ParseItemKindError::new("unknown");
        assert_eq!(
            BuyError::from(e),
            BuyError::UnknownItem("unknown".to_string())
        );
    }

    #[test]
    fn buy_item_returns_ok() {
        let mut state = GameState::new();
        let result = state.buy("shovel");
        assert!(result.is_ok());
        assert_eq!(state.credits(), 70);
        assert_eq!(state.inventory().current_quantity(), 1);
    }

    #[test]
    fn buy_unknown_item_returns_err() {
        let mut state = GameState::new();
        let result = state.buy("unknown");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            BuyError::UnknownItem("unknown".to_string())
        );
    }

    #[test]
    fn buy_item_with_insufficient_credits_returns_err() {
        let mut state = GameState::new();
        let result = state.buy("zap gun");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            BuyError::Wallet(WalletError::InsufficientFunds {
                available: 100,
                requested: 400
            })
        );
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemKind {
    RadarBooster,
    Shovel,
    WalkieTalkie,
    ExtensionLadder,
    Boombox,
    Flashlight,
    Jetpack,
    Lockpicker,
    ZapGun,
    StunGrenade,
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::RadarBooster => write!(f, "RADAR BOOSTER"),
            ItemKind::Shovel => write!(f, "SHOVEL"),
            ItemKind::WalkieTalkie => write!(f, "WALKIE-TALKIE"),
            ItemKind::ExtensionLadder => write!(f, "EXTENSION LADDER"),
            ItemKind::Boombox => write!(f, "BOOMBOX"),
            ItemKind::Flashlight => write!(f, "FLASHLIGHT"),
            ItemKind::Jetpack => write!(f, "JETPACK"),
            ItemKind::Lockpicker => write!(f, "LOCKPICKER"),
            ItemKind::ZapGun => write!(f, "ZAP GUN"),
            ItemKind::StunGrenade => write!(f, "STUN GRENADE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item {
    kind: ItemKind,
}

impl Item {
    pub fn kind(&self) -> ItemKind {
        self.kind
    }
}

const CAPACITY: usize = 4;

#[derive(Debug, PartialEq)]
pub struct Inventory {
    items: Vec<Item>,
}

#[derive(Debug, PartialEq)]
pub enum InventoryError {
    Full { capacity: usize },
}

impl Display for InventoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::Full { capacity } => {
                write!(f, "INVENTORY OVERFLOW: cannot exceed {capacity} items")
            }
        }
    }
}

impl Error for InventoryError {}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::with_capacity(CAPACITY),
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), InventoryError> {
        if self.items.len() >= CAPACITY {
            return Err(InventoryError::Full { capacity: CAPACITY });
        }
        self.items.push(item);
        Ok(())
    }

    pub fn current_quantity(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_inventory_returns_initial_state() {
        let inventory = Inventory::new();
        assert_eq!(inventory.items.len(), 0);
    }

    #[test]
    fn default_returns_initial_state() {
        let inventory = Inventory::default();
        assert_eq!(inventory.items.len(), 0);
    }

    #[test]
    fn add_items_returns_ok() {
        let mut inventory = Inventory::new();
        inventory
            .add_item(Item {
                kind: ItemKind::Lockpicker,
            })
            .unwrap();
        assert_eq!(inventory.items.len(), 1);
    }

    #[test]
    fn add_items_when_full_returns_inventory_error() {
        let mut inventory = Inventory::new();
        for _ in 0..CAPACITY {
            inventory
                .add_item(Item {
                    kind: ItemKind::Lockpicker,
                })
                .unwrap();
        }
        let result = inventory.add_item(Item {
            kind: ItemKind::Lockpicker,
        });
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            InventoryError::Full { capacity: CAPACITY }
        );
    }
}

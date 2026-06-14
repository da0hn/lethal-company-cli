use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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
        let name = match self {
            ItemKind::RadarBooster => "RADAR BOOSTER",
            ItemKind::Shovel => "SHOVEL",
            ItemKind::WalkieTalkie => "WALKIE-TALKIE",
            ItemKind::ExtensionLadder => "EXTENSION LADDER",
            ItemKind::Boombox => "BOOMBOX",
            ItemKind::Flashlight => "FLASHLIGHT",
            ItemKind::Jetpack => "JETPACK",
            ItemKind::Lockpicker => "LOCKPICKER",
            ItemKind::ZapGun => "ZAP GUN",
            ItemKind::StunGrenade => "STUN GRENADE",
        };
        f.pad(name)
    }
}

impl FromStr for ItemKind {
    type Err = ParseItemKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "radar booster" => Ok(ItemKind::RadarBooster),
            "shovel" => Ok(ItemKind::Shovel),
            "walkie-talkie" => Ok(ItemKind::WalkieTalkie),
            "extension ladder" => Ok(ItemKind::ExtensionLadder),
            "boombox" => Ok(ItemKind::Boombox),
            "flashlight" => Ok(ItemKind::Flashlight),
            "jetpack" => Ok(ItemKind::Jetpack),
            "lockpicker" => Ok(ItemKind::Lockpicker),
            "zap gun" => Ok(ItemKind::ZapGun),
            "stun grenade" => Ok(ItemKind::StunGrenade),
            _ => Err(ParseItemKindError(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseItemKindError(String);

impl ParseItemKindError {
    pub fn name(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for ParseItemKindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_message = format!("UNKNOWN ITEM: {}", self.0);
        f.pad(&err_message)
    }
}

impl std::error::Error for ParseItemKindError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item {
    kind: ItemKind,
}

impl Item {
    pub fn kind(&self) -> ItemKind {
        self.kind
    }
}

const CAPACITY: usize = 16;

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

impl std::error::Error for InventoryError {}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::with_capacity(CAPACITY),
        }
    }

    pub fn current_quantity(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn counts(&self) -> BTreeMap<ItemKind, u32> {
        let mut grouped_items: BTreeMap<ItemKind, u32> = BTreeMap::new();
        for item in &self.items {
            *grouped_items.entry(item.kind).or_insert(0) += 1;
        }
        grouped_items
    }

    pub fn add_item(&mut self, kind: ItemKind) -> Result<(), InventoryError> {
        let new_item = Item { kind };
        self.push_item(new_item)
    }

    fn push_item(&mut self, item: Item) -> Result<(), InventoryError> {
        if self.items.len() >= CAPACITY {
            return Err(InventoryError::Full { capacity: CAPACITY });
        }
        self.items.push(item);
        Ok(())
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
        inventory.add_item(ItemKind::Lockpicker).unwrap();
        assert_eq!(inventory.items.len(), 1);
    }

    #[test]
    fn add_items_when_full_returns_inventory_error() {
        let mut inventory = Inventory::new();
        for _ in 0..CAPACITY {
            inventory.add_item(ItemKind::Lockpicker).unwrap();
        }
        let result = inventory.add_item(ItemKind::Lockpicker);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            InventoryError::Full { capacity: CAPACITY }
        );
    }

    #[test]
    fn counts_returns_grouped_items() {
        let mut inventory = Inventory::new();
        inventory.add_item(ItemKind::Shovel).unwrap();
        inventory.add_item(ItemKind::Shovel).unwrap();
        inventory.add_item(ItemKind::Flashlight).unwrap();

        let grouped_items = inventory.counts();
        assert_eq!(grouped_items.len(), 2);
        assert_eq!(grouped_items.get(&ItemKind::Shovel), Some(&2));
        assert_eq!(grouped_items.get(&ItemKind::Flashlight), Some(&1));
    }

    #[test]
    fn counts_empty_inventory_returns_empty_group() {
        let inventory = Inventory::new();
        let grouped_items = inventory.counts();
        assert_eq!(grouped_items.len(), 0);
    }

    #[test]
    fn from_str_shovel_returns_shovel_item_kind() {
        assert_eq!(ItemKind::from_str("shovel"), Ok(ItemKind::Shovel));
    }

    #[test]
    fn from_str_unknown_returns_err() {
        assert_eq!(
            ItemKind::from_str("unknown"),
            Err(ParseItemKindError("unknown".to_string()))
        );
    }
}

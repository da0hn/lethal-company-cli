use crate::inventory::ItemKind;
use std::fmt::{Display, Formatter};

const CATALOG: [StoreItem; 10] = [
    StoreItem {
        kind: ItemKind::Boombox,
        price: 60,
    },
    StoreItem {
        kind: ItemKind::ExtensionLadder,
        price: 60,
    },
    StoreItem {
        kind: ItemKind::Flashlight,
        price: 15,
    },
    StoreItem {
        kind: ItemKind::Jetpack,
        price: 900,
    },
    StoreItem {
        kind: ItemKind::Lockpicker,
        price: 20,
    },
    StoreItem {
        kind: ItemKind::RadarBooster,
        price: 60,
    },
    StoreItem {
        kind: ItemKind::Shovel,
        price: 30,
    },
    StoreItem {
        kind: ItemKind::ZapGun,
        price: 400,
    },
    StoreItem {
        kind: ItemKind::StunGrenade,
        price: 30,
    },
    StoreItem {
        kind: ItemKind::WalkieTalkie,
        price: 10,
    },
];

#[derive(Debug, PartialEq)]
pub struct StoreItem {
    kind: ItemKind,
    price: u32,
}

impl StoreItem {
    pub fn new(kind: ItemKind, price: u32) -> Self {
        Self { kind, price }
    }

    pub fn kind(&self) -> ItemKind {
        self.kind
    }

    pub fn price(&self) -> u32 {
        self.price
    }
}

impl Display for StoreItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.<20}{:>4} CR", self.kind, self.price)
    }
}

pub fn catalog() -> &'static [StoreItem] {
    &CATALOG
}

pub fn price_of(kind: ItemKind) -> Option<u32> {
    catalog()
        .iter()
        .find(|item| item.kind() == kind)
        .map(|item| item.price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_of_shovel_returns_shovel_price() {
        assert_eq!(price_of(ItemKind::Shovel), Some(30));
    }
}

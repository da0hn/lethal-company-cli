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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item {
    kind: ItemKind,
}

impl Item {
    pub fn kind(&self) -> ItemKind {
        self.kind
    }
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

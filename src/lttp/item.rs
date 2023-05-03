use anyhow::anyhow;
use serde::{
    Deserialize,
    Serialize,
};
use std::convert::TryFrom;
use ts_rs::TS;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/BigKey.ts")]
#[serde(rename_all = "camelCase")]
pub struct BigKey {
    // Light world
    pub eastern_palace:     bool,
    pub desert_palace:      bool,
    pub tower_of_hera:      bool,
    // Dark world
    pub palace_of_darkness: bool,
    pub swamp_palace:       bool,
    pub skull_woods:        bool,
    pub thieves_town:       bool,
    pub ice_palace:         bool,
    pub misery_mire:        bool,
    pub turtle_rock:        bool,
    pub gannons_tower:      bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Pendant.ts")]
#[serde(rename_all = "camelCase")]
pub struct Pendant {
    pub red:   bool,
    pub blue:  bool,
    pub green: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Crystal.ts")]
#[serde(rename_all = "camelCase")]
pub struct Crystal {
    pub one:   bool,
    pub two:   bool,
    pub three: bool,
    pub four:  bool,
    pub five:  bool,
    pub six:   bool,
    pub seven: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Bow.ts")]
#[derive(Default)]
pub enum Bow {
    #[default]
    None,
    Wood,
    Silver,
    WoodAndSilver,
}

impl TryFrom<u8> for Bow {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Bow, Self::Error> {
        if number & 0b1100_0000 == 0b1100_0000 {
            Ok(Bow::WoodAndSilver)
        } else if number & 0b1000_0000 == 0b1000_0000 {
            Ok(Bow::Wood)
        } else if number & 0b0100_0000 == 0b0100_0000 {
            Ok(Bow::Silver)
        } else {
            Ok(Bow::None)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Boomerang.ts")]
#[derive(Default)]
pub enum Boomerang {
    #[default]
    None,
    Blue,
    Red,
    Both,
}

impl TryFrom<u8> for Boomerang {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Boomerang, Self::Error> {
        if number & 0b1100_0000 == 0b1100_0000 {
            Ok(Boomerang::Both)
        } else if number & 0b1000_0000 == 0b1000_0000 {
            Ok(Boomerang::Blue)
        } else if number & 0b0100_0000 == 0b0100_0000 {
            Ok(Boomerang::Red)
        } else {
            Ok(Boomerang::None)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Shroom {
    #[default]
    None,
    Available,
    Used,
}

impl TryFrom<u8> for Shroom {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Shroom, Self::Error> {
        if number & 0b10_1000 == 0b10_1000 {
            Ok(Shroom::Available)
        } else if number & 0b00_1000 == 0b00_1000 {
            Ok(Shroom::Used)
        } else {
            Ok(Shroom::None)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Powder {
    #[default]
    None,
    Available,
}

impl TryFrom<u8> for Powder {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Powder, Self::Error> {
        if number & 0b01_0000 == 0b01_0000 {
            Ok(Powder::Available)
        } else {
            Ok(Powder::None)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ShroomPowder {
    #[default]
    None   = 0,
    Shroom = 1,
    Powder = 2,
}

impl TryFrom<u8> for ShroomPowder {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<ShroomPowder, Self::Error> {
        match number {
            0 => Ok(ShroomPowder::None),
            1 => Ok(ShroomPowder::Shroom),
            2 => Ok(ShroomPowder::Powder),
            _ => Err(anyhow!("Unknown shroom/powder flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Flute {
    #[default]
    None,
    Unactivated,
    Activated,
}

impl TryFrom<u8> for Flute {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Flute, Self::Error> {
        if number & 0b01 == 0b01 {
            Ok(Flute::Activated)
        } else if number & 0b10 == 0b10 {
            Ok(Flute::Unactivated)
        } else {
            Ok(Flute::None)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Shovel {
    #[default]
    None,
    Acquired,
}

impl TryFrom<u8> for Shovel {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Shovel, Self::Error> {
        if number & 0b100 == 0b100 {
            Ok(Shovel::Acquired)
        } else {
            Ok(Shovel::None)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Gloves.ts")]
#[derive(Default)]
pub enum Gloves {
    #[default]
    None       = 0,
    PowerGlove = 1,
    TitansMitt = 2,
}

impl TryFrom<u8> for Gloves {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Gloves, Self::Error> {
        match number {
            0 => Ok(Gloves::None),
            1 => Ok(Gloves::PowerGlove),
            2 => Ok(Gloves::TitansMitt),
            _ => Err(anyhow!("Unknown gloves flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Sword.ts")]
#[allow(clippy::enum_variant_names)]
#[derive(Default)]
pub enum Sword {
    #[default]
    None          = 0,
    FightersSword = 1,
    MasterSword   = 2,
    TemperedSword = 3,
    GoldenSword   = 4,
}

impl TryFrom<u8> for Sword {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Sword, Self::Error> {
        match number {
            0 => Ok(Sword::None),
            1 => Ok(Sword::FightersSword),
            2 => Ok(Sword::MasterSword),
            3 => Ok(Sword::TemperedSword),
            4 => Ok(Sword::GoldenSword),
            _ => Err(anyhow!("Unknown sword flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Shield.ts")]
#[allow(clippy::enum_variant_names)]
#[derive(Default)]
pub enum Shield {
    #[default]
    None           = 0,
    FightersShield = 1,
    RedShield      = 2,
    MirrorShield   = 3,
}

impl TryFrom<u8> for Shield {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Shield, Self::Error> {
        match number {
            0 => Ok(Shield::None),
            1 => Ok(Shield::FightersShield),
            2 => Ok(Shield::RedShield),
            3 => Ok(Shield::MirrorShield),
            _ => Err(anyhow!("Unknown shield flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Armor.ts")]
#[allow(clippy::enum_variant_names)]
#[derive(Default)]
pub enum Armor {
    #[default]
    GreenMail = 0,
    BlueMail  = 1,
    RedMail   = 2,
}

impl TryFrom<u8> for Armor {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Armor, Self::Error> {
        match number {
            0 => Ok(Armor::GreenMail),
            1 => Ok(Armor::BlueMail),
            2 => Ok(Armor::RedMail),
            _ => Err(anyhow!("Unknown armor flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Bottle.ts")]
#[allow(clippy::enum_variant_names)]
#[derive(Default)]
pub enum Bottle {
    #[default]
    NoBottle    = 0x00,
    Mushroom    = 0x01,
    Empty       = 0x02,
    RedPotion   = 0x03,
    GreenPotion = 0x04,
    BluePotion  = 0x05,
    Fairy       = 0x06,
    Bee         = 0x07,
    MagicBee    = 0x08,
}

impl TryFrom<u8> for Bottle {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Bottle, Self::Error> {
        match number {
            0x00 => Ok(Bottle::NoBottle),
            0x01 => Ok(Bottle::Mushroom),
            0x02 => Ok(Bottle::Empty),
            0x03 => Ok(Bottle::RedPotion),
            0x04 => Ok(Bottle::GreenPotion),
            0x05 => Ok(Bottle::BluePotion),
            0x06 => Ok(Bottle::Fairy),
            0x07 => Ok(Bottle::Bee),
            0x08 => Ok(Bottle::MagicBee),
            _ => Err(anyhow!("Unknown bottle flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Magic.ts")]
#[derive(Default)]
pub enum Magic {
    #[default]
    Normal  = 0,
    Half    = 1,
    Quarter = 2,
}

impl TryFrom<u8> for Magic {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Magic, Self::Error> {
        match number {
            0 => Ok(Magic::Normal),
            1 => Ok(Magic::Half),
            2 => Ok(Magic::Quarter),
            _ => Err(anyhow!("Unknown magic flag: 0x{:X}", number)),
        }
    }
}

use anyhow::anyhow;
use serde::{
    Deserialize,
    Serialize,
};
use std::convert::TryFrom;
use ts_rs::TS;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Pendant.ts")]
#[serde(rename_all = "camelCase")]
pub struct Pendant {
    pub red:   bool,
    pub blue:  bool,
    pub green: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Bow.ts")]
pub enum Bow {
    None             = 0,
    Wood             = 1,
    WoodWithArrows   = 2,
    Silver           = 3,
    SilverWithArrows = 4,
}

impl Default for Bow {
    fn default() -> Bow { Bow::None }
}

impl TryFrom<u8> for Bow {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Bow, Self::Error> {
        match number {
            0 => Ok(Bow::None),
            1 => Ok(Bow::Wood),
            2 => Ok(Bow::WoodWithArrows),
            3 => Ok(Bow::Silver),
            4 => Ok(Bow::SilverWithArrows),
            _ => Err(anyhow!("Unknown bow flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Boomerang.ts")]
pub enum Boomerang {
    None = 0,
    Blue = 1,
    Red  = 2,
}

impl Default for Boomerang {
    fn default() -> Boomerang { Boomerang::None }
}

impl TryFrom<u8> for Boomerang {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<Boomerang, Self::Error> {
        match number {
            0 => Ok(Boomerang::None),
            1 => Ok(Boomerang::Blue),
            2 => Ok(Boomerang::Red),
            _ => Err(anyhow!("Unknown boomerang flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShroomPowder {
    None   = 0,
    Shroom = 1,
    Powder = 2,
}

impl Default for ShroomPowder {
    fn default() -> ShroomPowder { ShroomPowder::None }
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum FluteShovel {
    None         = 0,
    Shovel       = 1,
    Flute        = 2,
    FluteAndBird = 3,
}

impl Default for FluteShovel {
    fn default() -> FluteShovel { FluteShovel::None }
}

impl TryFrom<u8> for FluteShovel {
    type Error = anyhow::Error;

    fn try_from(number: u8) -> Result<FluteShovel, Self::Error> {
        match number {
            0 => Ok(FluteShovel::None),
            1 => Ok(FluteShovel::Shovel),
            2 => Ok(FluteShovel::Flute),
            3 => Ok(FluteShovel::FluteAndBird),
            _ => Err(anyhow!("Unknown flute/shovel flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Gloves.ts")]
pub enum Gloves {
    None       = 0,
    PowerGlove = 1,
    TitansMitt = 2,
}

impl Default for Gloves {
    fn default() -> Gloves { Gloves::None }
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Sword.ts")]
#[allow(clippy::enum_variant_names)]
pub enum Sword {
    None          = 0,
    FightersSword = 1,
    MasterSword   = 2,
    TemperedSword = 3,
    GoldenSword   = 4,
}

impl Default for Sword {
    fn default() -> Sword { Sword::None }
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Shield.ts")]
#[allow(clippy::enum_variant_names)]
pub enum Shield {
    None           = 0,
    FightersShield = 1,
    RedShield      = 2,
    MirrorShield   = 3,
}

impl Default for Shield {
    fn default() -> Shield { Shield::None }
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Armor.ts")]
#[allow(clippy::enum_variant_names)]
pub enum Armor {
    GreenMail = 0,
    BlueMail  = 1,
    RedMail   = 2,
}

impl Default for Armor {
    fn default() -> Armor { Armor::GreenMail }
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Bottle.ts")]
#[allow(clippy::enum_variant_names)]
pub enum Bottle {
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

impl Default for Bottle {
    fn default() -> Bottle { Bottle::NoBottle }
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Magic.ts")]
pub enum Magic {
    Normal  = 0,
    Half    = 1,
    Quarter = 2,
}

impl Default for Magic {
    fn default() -> Magic { Magic::Normal }
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

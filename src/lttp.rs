pub mod app_state;
pub mod item;
pub mod logic;
pub mod server_config;

pub use self::{
    app_state::AppState,
    server_config::ServerConfig,
};

use crate::lttp::{
    item::{
        Armor,
        BigKey,
        Boomerang,
        Bottle,
        Bow,
        Crystal,
        Flute,
        Gloves,
        Magic,
        Pendant,
        Powder,
        Shield,
        Shovel,
        Shroom,
        Sword,
    },
    logic::{
        Availability,
        DungeonAvailability,
        LocationAvailability,
        RandoLogic,
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use std::convert::TryFrom;
use ts_rs::TS;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/GameState.ts")]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    // Items
    pub bow:              bool,
    pub blue_boomerang:   bool,
    pub red_boomerang:    bool,
    pub hook_shot:        bool,
    pub bomb:             u8,
    pub mushroom:         bool,
    pub mushroom_used:    bool,
    pub powder:           bool,
    pub fire_rod:         bool,
    pub ice_rod:          bool,
    pub bombos_medallion: bool,
    pub ether_medallion:  bool,
    pub quake_medallion:  bool,
    pub lantern:          bool,
    pub hammer:           bool,
    pub flute:            bool,
    pub flute_activated:  bool,
    pub shovel:           bool,
    pub net:              bool,
    pub book:             bool,
    pub bottle:           bool,
    pub bottle_count:     u8,
    pub cane_somaria:     bool,
    pub cane_byrna:       bool,
    pub cape:             bool,
    pub mirror:           bool,
    pub silvers:          bool,
    // Abilities
    pub gloves:           Gloves,
    pub boots:            bool,
    pub flippers:         bool,
    pub moon_pearl:       bool,
    // Weapon & Armor Progression
    pub sword_level:      Sword,
    pub shield_level:     Shield,
    pub armor_level:      Armor,
    // Bottle content
    pub bottle_content1:  Bottle,
    pub bottle_content2:  Bottle,
    pub bottle_content3:  Bottle,
    pub bottle_content4:  Bottle,

    pub rupees:         u16,
    pub heart_quarters: u8,
    pub bomb_capacity:  u8,
    pub hearts:         u8,
    pub max_hearts:     u8,

    pub arrows:         u8,
    pub arrow_capacity: u8,

    pub magic_progression: Magic,

    pub small_keys: u8,
    pub big_key:    BigKey,

    pub pendant: Pendant,
    pub crystal: Crystal,
}

impl TryFrom<Vec<u8>> for GameState {
    type Error = anyhow::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(response: Vec<u8>) -> Result<GameState, Self::Error> {
        let bow = Bow::try_from(response[0x4E])?;
        let flute = Flute::try_from(response[0x4C])?;
        let shovel = Shovel::try_from(response[0x4C])?;
        let shroom = Shroom::try_from(response[0x4C])?;
        let powder = Powder::try_from(response[0x4C])?;
        let boomerang = Boomerang::try_from(response[0x4C])?;

        let bottle1 = Bottle::try_from(response[0x1C])?;
        let bottle2 = Bottle::try_from(response[0x1D])?;
        let bottle3 = Bottle::try_from(response[0x1E])?;
        let bottle4 = Bottle::try_from(response[0x1F])?;

        let mut bottle_count = 0;
        if bottle1 != Bottle::NoBottle {
            bottle_count += 1;
        };
        if bottle2 != Bottle::NoBottle {
            bottle_count += 1;
        };
        if bottle3 != Bottle::NoBottle {
            bottle_count += 1;
        };
        if bottle4 != Bottle::NoBottle {
            bottle_count += 1;
        };

        Ok(GameState {
            bow: bow == Bow::Wood || bow == Bow::WoodAndSilver,
            blue_boomerang: boomerang == Boomerang::Blue || boomerang == Boomerang::Both,
            red_boomerang: boomerang == Boomerang::Red || boomerang == Boomerang::Both,
            hook_shot: response[0x02] > 0,
            bomb: response[0x03],
            mushroom: shroom == Shroom::Available || shroom == Shroom::Used,
            mushroom_used: shroom == Shroom::Used,
            powder: powder == Powder::Available,
            fire_rod: response[0x05] > 0,
            ice_rod: response[0x06] > 0,
            bombos_medallion: response[0x07] > 0,
            ether_medallion: response[0x08] > 0,
            quake_medallion: response[0x09] > 0,
            lantern: response[0x0A] > 0,
            hammer: response[0x0B] > 0,
            flute: flute == Flute::Unactivated || flute == Flute::Activated,
            flute_activated: flute == Flute::Activated,
            shovel: shovel == Shovel::Acquired,
            net: response[0x0D] > 0,
            book: response[0x0E] > 0,
            bottle: response[0x0F] > 0,
            bottle_count,
            cane_somaria: response[0x10] > 0,
            cane_byrna: response[0x11] > 0,
            cape: response[0x12] > 0,
            mirror: response[0x13] > 0,
            silvers: bow == Bow::Silver || bow == Bow::WoodAndSilver,

            gloves: Gloves::try_from(response[0x14])?,
            boots: response[0x15] > 0,
            flippers: response[0x16] > 0,
            moon_pearl: response[0x17] > 0,

            sword_level: Sword::try_from(response[0x19])?,
            shield_level: Shield::try_from(response[0x1A])?,
            armor_level: Armor::try_from(response[0x1B])?,

            bottle_content1: bottle1,
            bottle_content2: bottle2,
            bottle_content3: bottle3,
            bottle_content4: bottle4,

            // Rupees are spread across two bytes, as the randomizer lifted the
            // 255 Rupee limit, and it's stored little-endian.
            rupees: ((u16::from(response[0x23])) << 8) + u16::from(response[0x22]),
            heart_quarters: response[0x2B],
            bomb_capacity: response[0x30] + 10,
            hearts: response[0x2D],
            max_hearts: response[0x2C],

            arrows: response[0x37],
            arrow_capacity: response[0x31] + 30,

            magic_progression: Magic::try_from(response[0x3B])?,

            small_keys: if response[0x2F] == 0xFF {
                0
            } else {
                response[0x2F]
            },
            big_key: BigKey {
                // BigKey1: 0x366
                //       Skull Woods
                //       |Ice Palace
                //       ||Tower of Hera
                //       |||Gargoyle's Domain
                //       ||||Turtle Rock
                //       |||||Gannon's Tower
                //       ||||||x
                //       |||||||x
                //       vvvvvvvv
                //      |--------|
                // Bit:  7      0
                gannons_tower: response[0x26] & 0b0000_0100 > 0,
                turtle_rock:   response[0x26] & 0b0000_1000 > 0,
                thieves_town:  response[0x26] & 0b0001_0000 > 0,
                tower_of_hera: response[0x26] & 0b0010_0000 > 0,
                ice_palace:    response[0x26] & 0b0100_0000 > 0,
                skull_woods:   response[0x26] & 0b1000_0000 > 0,

                // BigKey2: 0x367
                //       X
                //       |X
                //       ||Eastern Palace
                //       |||Desert Palace
                //       ||||X
                //       |||||Swamp Palace
                //       ||||||Dark Palace
                //       |||||||Misery Mire
                //       vvvvvvvv
                //      |--------|
                // Bit:  7      0
                misery_mire:        response[0x27] & 0b0000_0001 > 0,
                desert_palace:      response[0x27] & 0b0000_0010 > 0,
                swamp_palace:       response[0x27] & 0b0000_0100 > 0,
                palace_of_darkness: response[0x27] & 0b0001_0000 > 0,
                eastern_palace:     response[0x27] & 0b0010_0000 > 0,
            },

            // 0x374 -> Pendants (Bitmask)
            // 1 - Red
            // 2 - Blue
            // 4 - Green
            pendant: Pendant {
                red:   response[0x34] & 0b0001 > 0,
                blue:  response[0x34] & 0b0010 > 0,
                green: response[0x34] & 0b0100 > 0,
            },

            // 0x37A -> Crystals (Bitmask)
            // 1 - Misery Mire
            // 2 - Dark Palace
            // 4 - Ice Palace
            // 8 - Turtle Rock
            // 16 - Swamp Palace
            // 32 - Gargoyle's Domain
            // 64 - Skull Woods
            crystal: Crystal {
                one:   response[0x3A] & 0b0000_0001 > 0,
                three: response[0x3A] & 0b0000_0010 > 0,
                five:  response[0x3A] & 0b0000_0100 > 0,
                four:  response[0x3A] & 0b0000_1000 > 0,
                two:   response[0x3A] & 0b0001_0000 > 0,
                six:   response[0x3A] & 0b0010_0000 > 0,
                seven: response[0x3A] & 0b0100_0000 > 0,
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/LocationPosition.ts")]
#[serde(rename_all = "camelCase")]
pub struct LocationPosition {
    pub horizontal: LocationCoordinates,
    pub vertical:   LocationCoordinates,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/LocationCoordinates.ts")]
#[serde(rename_all = "camelCase")]
pub struct LocationCoordinates {
    pub left: f32,
    pub top:  f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/Location.ts")]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name:         String,
    pub hover_text:   String,
    pub position:     LocationPosition,
    #[serde(default)]
    pub cleared:      bool,
    #[serde(skip_serializing)]
    pub logic:        LocationAvailability,
    #[serde(default)]
    pub availability: Availability,
}

impl Location {
    pub fn update(&mut self, update: LocationUpdate) {
        if let Some(cleared) = update.cleared {
            self.cleared = cleared;
        }
    }

    pub fn calculate_availability(
        &mut self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: RandoLogic,
    ) {
        self.availability = self.logic.check(state, dungeons, logic);
    }
}

#[derive(Debug, Clone, Copy, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/LocationUpdate.ts")]
#[serde(rename_all = "camelCase")]
pub struct LocationUpdate {
    pub cleared: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/LocationState.ts")]
#[serde(rename_all = "camelCase")]
pub struct LocationState {
    pub locations: Vec<Location>,
}

impl LocationState {
    pub fn get(&self, name: &str) -> Option<Location> {
        self.locations.iter().position(|l| l.name == name).map(|i| self.locations[i].clone())
    }

    pub fn update(&mut self, name: &str, update: LocationUpdate) {
        if let Some(i) = self.locations.iter().position(|l| l.name == name) {
            self.locations[i].update(update);
        }
    }

    pub fn update_availability(
        &mut self,
        game_state: &GameState,
        dungeon_state: &DungeonState,
        logic: RandoLogic,
    ) {
        self.locations
            .iter_mut()
            .for_each(|loc| loc.calculate_availability(game_state, dungeon_state, logic));
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "ui/src/server_types/DungeonBoss.ts")]
#[serde(rename_all = "camelCase")]
pub struct DungeonBoss {
    pub name:         String,
    pub hover_text:   String,
    pub image_number: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/Dungeon.ts")]
#[serde(rename_all = "camelCase")]
pub struct Dungeon {
    pub name:                 String,
    pub dungeon_code:         String,
    pub hover_text:           String,
    pub total_chests:         u8,
    pub cleared_image:        String,
    pub default_image:        String,
    pub has_reward:           bool,
    pub position:             Option<LocationPosition>,
    pub boss:                 Option<DungeonBoss>,
    #[serde(default)]
    pub found_chests:         u8,
    #[serde(default)]
    pub reward:               DungeonReward,
    #[serde(default)]
    pub medallion:            Medallion,
    #[serde(default)]
    pub cleared:              bool,
    #[serde(default)]
    pub dungeon_availability: Availability,
    #[serde(default)]
    pub boss_availability:    Availability,
    #[serde(default)]
    pub logic:                Option<DungeonAvailability>,
}

impl Dungeon {
    pub fn update(&mut self, update: DungeonUpdate) {
        if let Some(chests) = update.found_chests {
            self.found_chests = chests;
        }
        if let Some(reward) = update.reward {
            self.reward = reward;
        }
        if let Some(medallion) = update.medallion {
            self.medallion = medallion;
        }
        if let Some(cleared) = update.cleared {
            self.cleared = cleared;
        }
    }

    pub fn remaining_chests(&self) -> u8 { self.total_chests - self.found_chests }

    pub fn calculate_availability(
        &mut self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: RandoLogic,
    ) {
        if let Some(dungeon_logic) = self.logic {
            self.dungeon_availability = dungeon_logic.can_get_chest(state, dungeons, logic);
            self.boss_availability = dungeon_logic.is_beatable(state, dungeons, logic);
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "ui/src/server_types/DungeonReward.ts")]
#[derive(Default)]
pub enum DungeonReward {
    #[default]
    Unknown,
    GreenPendant,
    RedBluePendant,
    Crystal,
    RedCrystal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "ui/src/server_types/Medallion.ts")]
#[derive(Default)]
pub enum Medallion {
    #[default]
    Unknown,
    Bombos,
    Ether,
    Quake,
}

#[derive(Debug, Clone, Copy, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/DungeonUpdate.ts")]
#[serde(rename_all = "camelCase")]
pub struct DungeonUpdate {
    pub found_chests: Option<u8>,
    pub reward:       Option<DungeonReward>,
    pub medallion:    Option<Medallion>,
    pub cleared:      Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/DungeonState.ts")]
#[serde(rename_all = "camelCase")]
pub struct DungeonState {
    pub dungeons: Vec<Dungeon>,
}

impl DungeonState {
    pub fn get(&self, dungeon_code: &str) -> Option<Dungeon> {
        self.dungeons
            .iter()
            .position(|d| d.dungeon_code == dungeon_code)
            .map(|i| self.dungeons[i].clone())
    }

    pub fn update(&mut self, dungeon_code: &str, update: DungeonUpdate) {
        if let Some(i) = self.dungeons.iter().position(|d| d.dungeon_code == dungeon_code) {
            self.dungeons[i].update(update);
        }
    }

    pub fn update_availability(&mut self, game_state: &GameState, logic: RandoLogic) {
        let dungeon_state = self.clone();
        self.dungeons.iter_mut().for_each(|dungeon| {
            dungeon.calculate_availability(game_state, &dungeon_state, logic);
        });
    }
}

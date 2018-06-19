mod item;

use failure;

use std::collections::HashMap;
use std::convert::TryFrom;

use self::item::{
    Armor,
    BigKey,
    Boomerang,
    Bottle,
    Bow,
    Crystal,
    FluteShovel,
    Gloves,
    Magic,
    Pendant,
    Shield,
    ShroomPowder,
    Sword,
};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct GameState {
    // Items
    pub bow:               bool,
    pub blue_boomerang:    bool,
    pub red_boomerang:     bool,
    pub hook_shot:         bool,
    pub bomb:              u8,
    pub mushroom:          bool,
    pub powder:            bool,
    pub fire_rod:          bool,
    pub ice_rod:           bool,
    pub bombos_medallion:  bool,
    pub ether_medallion:   bool,
    pub quake_medallion:   bool,
    pub lantern:           bool,
    pub hammer:            bool,
    pub flute:             bool,
    pub shovel:            bool,
    pub net:               bool,
    pub book:              bool,
    pub bottle:            bool,
    pub bottle_count:      u8,
    pub cane_somaria:      bool,
    pub cane_byrna:        bool,
    pub cape:              bool,
    pub mirror:            bool,
    pub silvers:           bool,
    // Abilities
    pub gloves:            Gloves,
    pub boots:             bool,
    pub flippers:          bool,
    pub moon_pearl:        bool,
    // Weapon & Armor Progression
    pub sword_level:       Sword,
    pub shield_level:      Shield,
    pub armor_level:       Armor,
    // Bottle content
    pub bottle_content1:   Bottle,
    pub bottle_content2:   Bottle,
    pub bottle_content3:   Bottle,
    pub bottle_content4:   Bottle,

    pub rupees:            u16,
    pub heart_quarters:    u8,
    pub bomb_capacity:     u8,
    pub hearts:            u8,
    pub max_hearts:        u8,

    pub arrows:            u8,
    pub arrow_capacity:    u8,

    pub magic_progression: Magic,

    pub small_keys:        u8,
    pub big_key:           BigKey,

    pub pendant:           Pendant,
    pub crystal:           Crystal,
}

impl TryFrom<Vec<u8>> for GameState {
    type Error = failure::Error;

    fn try_from(response: Vec<u8>) -> Result<GameState, Self::Error> {
        let bottle1 = Bottle::try_from(response[0x1C])?;
        let bottle2 = Bottle::try_from(response[0x1D])?;
        let bottle3 = Bottle::try_from(response[0x1E])?;
        let bottle4 = Bottle::try_from(response[0x1F])?;

        let mut bottle_count = 0;
        if bottle1 != Bottle::NoBottle { bottle_count += 1 };
        if bottle2 != Bottle::NoBottle { bottle_count += 1 };
        if bottle3 != Bottle::NoBottle { bottle_count += 1 };
        if bottle4 != Bottle::NoBottle { bottle_count += 1 };

        Ok(GameState {
            bow:               Bow::try_from(response[0x00])? != Bow::None,
            blue_boomerang:    Boomerang::try_from(response[0x01])? == Boomerang::Blue,
            red_boomerang:     Boomerang::try_from(response[0x01])? == Boomerang::Red,
            hook_shot:         response[0x02] > 0,
            bomb:              response[0x03],
            mushroom:          ShroomPowder::try_from(response[0x04])? == ShroomPowder::Shroom,
            powder:            ShroomPowder::try_from(response[0x04])? == ShroomPowder::Powder,
            fire_rod:          response[0x05] > 0,
            ice_rod:           response[0x06] > 0,
            bombos_medallion:  response[0x07] > 0,
            ether_medallion:   response[0x08] > 0,
            quake_medallion:   response[0x09] > 0,
            lantern:           response[0x0A] > 0,
            hammer:            response[0x0B] > 0,
            flute:             FluteShovel::try_from(response[0x0C])? == FluteShovel::Flute || FluteShovel::try_from(response[0x0C])? == FluteShovel::FluteAndBird,
            shovel:            FluteShovel::try_from(response[0x0C])? == FluteShovel::Shovel,
            net:               response[0x0D] > 0,
            book:              response[0x0E] > 0,
            bottle:            response[0x0F] > 0,
            bottle_count:      bottle_count,
            cane_somaria:      response[0x10] > 0,
            cane_byrna:        response[0x11] > 0,
            cape:              response[0x12] > 0,
            mirror:            response[0x13] > 0,
            silvers:           Bow::try_from(response[0x00])? == Bow::Silver || Bow::try_from(response[0x00])? == Bow::SilverWithArrows,

            gloves:            Gloves::try_from(response[0x14])?,
            boots:             response[0x15] > 0,
            flippers:          response[0x16] > 0,
            moon_pearl:        response[0x17] > 0,

            sword_level:       Sword::try_from(response[0x19])?,
            shield_level:      Shield::try_from(response[0x1A])?,
            armor_level:       Armor::try_from(response[0x1B])?,

            bottle_content1:   bottle1,
            bottle_content2:   bottle2,
            bottle_content3:   bottle3,
            bottle_content4:   bottle4,

            // Rupees are spread across two bytes, as the randomizer lifted the
            // 255 Rupee limit, and it's stored little-endian.
            rupees:            ((response[0x23] as u16) << 8) + response[0x22] as u16,
            heart_quarters:    response[0x2B],
            bomb_capacity:     response[0x30] + 10,
            hearts:            response[0x2D],
            max_hearts:        response[0x2C],

            arrows:            response[0x37],
            arrow_capacity:    response[0x31] + 30,

            magic_progression: Magic::try_from(response[0x3B])?,

            small_keys:        if response[0x2F] == 0xFF { 0 } else { response[0x2F] },
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
                gannons_tower: response[0x26] & 0b00000100 > 0,
                turtle_rock:   response[0x26] & 0b00001000 > 0,
                thieves_town:  response[0x26] & 0b00010000 > 0,
                tower_of_hera: response[0x26] & 0b00100000 > 0,
                ice_palace:    response[0x26] & 0b01000000 > 0,
                skull_woods:   response[0x26] & 0b10000000 > 0,

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
                misery_mire:        response[0x27] & 0b00000001 > 0,
                desert_palace:      response[0x27] & 0b00000010 > 0,
                swamp_palace:       response[0x27] & 0b00000100 > 0,
                palace_of_darkness: response[0x27] & 0b00010000 > 0,
                eastern_palace:     response[0x27] & 0b00100000 > 0,
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
                one:   response[0x3A] & 0b00000001 > 0,
                three: response[0x3A] & 0b00000010 > 0,
                five:  response[0x3A] & 0b00000100 > 0,
                four:  response[0x3A] & 0b00001000 > 0,
                two:   response[0x3A] & 0b00010000 > 0,
                six:   response[0x3A] & 0b00100000 > 0,
                seven: response[0x3A] & 0b01000000 > 0,
            },
        })
    }
}

impl GameState {
    /// Return a new GameState merging in the items from the old GameState.
    /// Things like bomb count, hearts, rupees, etc are taken from self.
    pub fn merge(&self, old: GameState) -> Self {
        GameState {
            bow:               self.bow              || old.bow,
            blue_boomerang:    self.blue_boomerang   || old.blue_boomerang,
            red_boomerang:     self.red_boomerang    || old.red_boomerang,
            hook_shot:         self.hook_shot,
            bomb:              self.bomb,
            mushroom:          self.mushroom         || old.mushroom,
            powder:            self.powder           || old.powder,
            fire_rod:          self.fire_rod,
            ice_rod:           self.ice_rod,
            bombos_medallion:  self.bombos_medallion,
            ether_medallion:   self.ether_medallion,
            quake_medallion:   self.quake_medallion,
            lantern:           self.lantern,
            hammer:            self.hammer,
            flute:             self.flute            || old.flute,
            shovel:            self.shovel           || old.shovel,
            net:               self.net,
            book:              self.book,
            bottle:            self.bottle,
            bottle_count:      self.bottle_count,
            cane_somaria:      self.cane_somaria,
            cane_byrna:        self.cane_byrna,
            cape:              self.cape,
            mirror:            self.mirror,
            silvers:           self.silvers          || old.silvers,

            gloves:            self.gloves,
            boots:             self.boots,
            flippers:          self.flippers,
            moon_pearl:        self.moon_pearl,

            sword_level:       self.sword_level,
            shield_level:      self.shield_level,
            armor_level:       self.armor_level,

            bottle_content1:   self.bottle_content1,
            bottle_content2:   self.bottle_content2,
            bottle_content3:   self.bottle_content3,
            bottle_content4:   self.bottle_content4,

            // Rupees are spread across two bytes, as the randomizer lifted the
            // 255 Rupee limit, and it's stored little-endian.
            rupees:            self.rupees,
            heart_quarters:    self.heart_quarters,
            bomb_capacity:     self.bomb_capacity,
            hearts:            self.hearts,
            max_hearts:        self.max_hearts,

            arrows:            self.arrows,
            arrow_capacity:    self.arrow_capacity,

            magic_progression: self.magic_progression,

            small_keys:        self.small_keys,
            big_key:           self.big_key,

            pendant:           self.pendant,
            crystal:           self.crystal,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Location {
    pub cleared: bool,
}

impl Location {
    pub fn update(&mut self, update: LocationUpdate) {
        if let Some(cleared) = update.cleared {
            self.cleared = cleared
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct LocationUpdate {
    pub cleared: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LocationState {
    pub locations: HashMap<String, Location>,
}

impl LocationState {
    pub fn get(&self, name: String) -> Location {
        if let Some(location) = self.locations.get(&name) {
            return location.clone();
        }

        Location::default()
    }

    pub fn update(&mut self, name: String, update: LocationUpdate) {
        if let Some(location) = self.locations.get_mut(&name) {
            location.update(update);
            return;
        }

        let mut location = Location::default();
        location.update(update);
        self.locations.insert(name, location);
    }
}


#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Dungeon {
    pub found_chests: u8,
    pub reward: DungeonReward,
    pub medallion: Medallion,
    pub cleared: bool,
}

impl Dungeon {
    pub fn update(&mut self, update: DungeonUpdate) {
        if let Some(chests) = update.found_chests {
            self.found_chests = chests
        }
        if let Some(reward) = update.reward {
            self.reward = reward
        }
        if let Some(medallion) = update.medallion {
            self.medallion = medallion
        }
        if let Some(cleared) = update.cleared {
            self.cleared = cleared
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DungeonReward {
    Unknown,
    GreenPendant,
    RedBluePendant,
    Crystal,
    RedCrystal,
}

impl Default for DungeonReward {
    fn default() -> DungeonReward { DungeonReward::Unknown }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Medallion {
    Unknown,
    Bombos,
    Ether,
    Quake,
}

impl Default for Medallion {
    fn default() -> Medallion { Medallion::Unknown }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct DungeonUpdate {
    pub found_chests: Option<u8>,
    pub reward: Option<DungeonReward>,
    pub medallion: Option<Medallion>,
    pub cleared: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DungeonState {
    pub dungeons: HashMap<String, Dungeon>,
}

impl DungeonState {
    pub fn get(&self, name: String) -> Dungeon {
        if let Some(dungeon) = self.dungeons.get(&name) {
            return dungeon.clone();
        }

        Dungeon::default()
    }

    pub fn update(&mut self, name: String, update: DungeonUpdate) {
        if let Some(dungeon) = self.dungeons.get_mut(&name) {
            dungeon.update(update);
            return;
        }

        let mut dungeon = Dungeon::default();
        dungeon.update(update);
        self.dungeons.insert(name, dungeon);
    }
}

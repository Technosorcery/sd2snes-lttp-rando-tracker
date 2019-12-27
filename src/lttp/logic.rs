use crate::{
    lttp::{
        item::{
            Armor,
            Gloves,
            Magic,
            Shield,
            Sword,
        },
        GameState,
        RandoLogic,
    },
    ServerConfig,
};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Availability {
    Unavailable,
    GlitchPossible,
    Possible,
    GlitchAgahnim,
    Agahnim,
    GlitchAvailable,
    Available,
}

impl Default for Availability {
    fn default() -> Availability { Availability::Possible }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Rule {
    Glitchless,
    OverWorldGlitches,
    MajorGlitches,

    BlueBoomerang,
    Bomb,
    BombosMedallion,
    Book,
    Boots,
    Bottle,
    Bow,
    ByrnaCane,
    Cape,
    EtherMedallion,
    FireRod,
    Flippers,
    Flute,
    FluteActivated,
    Hammer,
    HookShot,
    IceRod,
    Lantern,
    Mirror,
    MoonPearl,
    Mushroom,
    Net,
    Powder,
    QuakeMedallion,
    RedBoomerang,
    Shovel,
    Silvers,
    SomariaCane,

    Armor1,
    Armor2,
    Gloves1,
    Gloves2,
    Magic1,
    Magic2,
    Shield1,
    Shield2,
    Shield3,
    Sword1,
    Sword2,
    Sword3,
    Sword4,

    BluePendant,
    GreenPendant,
    RedPendant,

    CanBlockLasers,
    CanEnterEastDarkWorldDeathMountain,
    CanEnterEastDeathMountain,
    CanEnterMireArea,
    CanEnterNorthEastDarkWorld,
    CanEnterNorthWestDarkWorld,
    CanEnterSouthDarkWorld,
    CanEnterWestDeathMountain,
    CanExtendMagic,
    CanFly,
    CanGoBeatAgahnim1,
    BeatAgahnim1,
    CanLiftDarkRocks,
    CanLiftRocks,
    CanLightTorches,
    CanMeltThings,
    CanShootArrows,
    CanSpinSpeed,
    GlitchedLinkInDarkWorld,
    CanEnterDesertPalace,

    Rupees30,
    Rupees80,
    Rupees100,
    Rupees500,

    BothRedCrystals,
}

impl Rule {
    pub fn check(&self, state: &GameState, settings: &ServerConfig) -> bool {
        match self {
            Rule::Glitchless => settings.logic >= RandoLogic::Glitchless,
            Rule::OverWorldGlitches => settings.logic >= RandoLogic::OverWorldGlitches,
            Rule::MajorGlitches => settings.logic >= RandoLogic::MajorGlitches,

            Rule::BlueBoomerang => state.blue_boomerang,
            Rule::Bomb => state.bomb > 0,
            Rule::BombosMedallion => state.bombos_medallion,
            Rule::Book => state.book,
            Rule::Boots => state.boots,
            Rule::Bottle => state.bottle,
            Rule::Bow => state.bow,
            Rule::ByrnaCane => state.cane_byrna,
            Rule::Cape => state.cape,
            Rule::EtherMedallion => state.ether_medallion,
            Rule::FireRod => state.fire_rod,
            Rule::Flippers => state.flippers,
            Rule::Flute => state.flute,
            Rule::FluteActivated => state.flute_activated,
            Rule::Hammer => state.hammer,
            Rule::HookShot => state.hook_shot,
            Rule::IceRod => state.ice_rod,
            Rule::Lantern => state.lantern,
            Rule::Mirror => state.mirror,
            Rule::MoonPearl => state.moon_pearl,
            Rule::Mushroom => state.mushroom,
            Rule::Net => state.net,
            Rule::Powder => state.powder,
            Rule::QuakeMedallion => state.quake_medallion,
            Rule::RedBoomerang => state.red_boomerang,
            Rule::Shovel => state.shovel,
            Rule::Silvers => state.silvers,
            Rule::SomariaCane => state.cane_somaria,

            Rule::Armor1 => state.armor_level >= Armor::BlueMail,
            Rule::Armor2 => state.armor_level >= Armor::RedMail,
            Rule::Gloves1 => state.gloves >= Gloves::PowerGlove,
            Rule::Gloves2 => state.gloves >= Gloves::TitansMitt,
            Rule::Magic1 => state.magic_progression >= Magic::Half,
            Rule::Magic2 => state.magic_progression >= Magic::Quarter,
            Rule::Shield1 => state.shield_level >= Shield::FightersShield,
            Rule::Shield2 => state.shield_level >= Shield::RedShield,
            Rule::Shield3 => state.shield_level >= Shield::MirrorShield,
            Rule::Sword1 => state.sword_level >= Sword::FightersSword,
            Rule::Sword2 => state.sword_level >= Sword::MasterSword,
            Rule::Sword3 => state.sword_level >= Sword::TemperedSword,
            Rule::Sword4 => state.sword_level >= Sword::GoldenSword,

            Rule::BluePendant => state.pendant.blue,
            Rule::GreenPendant => state.pendant.green,
            Rule::RedPendant => state.pendant.red,

            Rule::CanLiftRocks => Rule::Gloves1.check(&state, &settings),
            Rule::CanLiftDarkRocks => Rule::Gloves2.check(&state, &settings),
            Rule::CanLightTorches => {
                Rule::FireRod.check(&state, &settings) || Rule::Lantern.check(&state, &settings)
            }
            Rule::CanMeltThings => {
                Rule::FireRod.check(&state, &settings)
                    || (Rule::BombosMedallion.check(&state, &settings)
                        && Rule::Sword1.check(&state, &settings))
            }
            Rule::CanFly => Rule::Flute.check(&state, &settings),
            Rule::CanSpinSpeed => {
                Rule::Boots.check(&state, &settings)
                    && (Rule::Sword1.check(&state, &settings)
                        || Rule::HookShot.check(&state, &settings))
            }
            Rule::CanShootArrows => Rule::Bow.check(&state, &settings),
            Rule::CanBlockLasers => Rule::Shield3.check(&state, &settings),
            Rule::CanExtendMagic => {
                Rule::Magic1.check(&state, &settings) || Rule::Bottle.check(&state, &settings)
            }
            Rule::GlitchedLinkInDarkWorld => {
                Rule::MoonPearl.check(&state, &settings) || Rule::Bottle.check(&state, &settings)
            }
            // TODO (#420): Logic says we should be tracking if Agahnim 1 has already been beaten or not.
            // TODO: Logic also says that the lantern isn't required if we allow "out of logic" glitches.
            Rule::CanGoBeatAgahnim1 => {
                Rule::Lantern.check(&state, &settings)
                    && (Rule::Cape.check(&state, &settings)
                        || Rule::Sword2.check(&state, &settings))
                    && Rule::Sword1.check(&state, &settings)
            }
            // TODO (#420): Really need to be tracking if Agahnim 1 has already been beaten.
            Rule::BeatAgahnim1 => true,
            Rule::CanEnterNorthEastDarkWorld => {
                Rule::CanGoBeatAgahnim1.check(&state, &settings)
                    || (Rule::Hammer.check(&state, &settings)
                        && Rule::CanLiftRocks.check(&state, &settings)
                        && Rule::MoonPearl.check(&state, &settings))
                    || (Rule::CanLiftDarkRocks.check(&state, &settings)
                        && Rule::Flippers.check(&state, &settings)
                        && Rule::MoonPearl.check(&state, &settings))
            }
            Rule::CanEnterNorthWestDarkWorld => {
                Rule::MoonPearl.check(&state, &settings)
                    && ((Rule::CanEnterNorthEastDarkWorld.check(&state, &settings)
                        && Rule::HookShot.check(&state, &settings)
                        && (Rule::Flippers.check(&state, &settings)
                            || Rule::CanLiftRocks.check(&state, &settings)
                            || Rule::Hammer.check(&state, &settings)))
                        || (Rule::Hammer.check(&state, &settings)
                            && Rule::CanLiftRocks.check(&state, &settings))
                        || Rule::CanLiftDarkRocks.check(&state, &settings))
            }
            Rule::CanEnterSouthDarkWorld => {
                Rule::MoonPearl.check(&state, &settings)
                    && (Rule::CanLiftDarkRocks.check(&state, &settings)
                        || (Rule::Hammer.check(&state, &settings)
                            && Rule::CanLiftRocks.check(&state, &settings))
                        || (Rule::CanEnterNorthEastDarkWorld.check(&state, &settings)
                            && (Rule::Hammer.check(&state, &settings)
                                || (Rule::HookShot.check(&state, &settings)
                                    && (Rule::Flippers.check(&state, &settings)
                                        || Rule::CanLiftRocks.check(&state, &settings))))))
            }
            Rule::CanEnterMireArea => {
                Rule::CanFly.check(&state, &settings)
                    && Rule::CanLiftDarkRocks.check(&state, &settings)
            }
            Rule::CanEnterWestDeathMountain => {
                Rule::CanFly.check(&state, &settings)
                    || (Rule::CanLiftRocks.check(&state, &settings)
                        && Rule::Lantern.check(&state, &settings))
            }
            Rule::CanEnterEastDeathMountain => {
                Rule::CanEnterWestDeathMountain.check(&state, &settings)
                    && (Rule::HookShot.check(&state, &settings)
                        || (Rule::Mirror.check(&state, &settings)
                            && Rule::Hammer.check(&state, &settings)))
            }
            Rule::CanEnterEastDarkWorldDeathMountain => {
                Rule::CanLiftDarkRocks.check(&state, &settings)
                    && Rule::CanEnterEastDeathMountain.check(&state, &settings)
            }
            Rule::CanEnterDesertPalace => true,

            Rule::Rupees30 => state.rupees >= 30,
            Rule::Rupees80 => state.rupees >= 80,
            Rule::Rupees100 => state.rupees >= 100,
            Rule::Rupees500 => state.rupees >= 500,

            Rule::BothRedCrystals => true,
        }
    }
}

#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Logic {
    pub availability: Availability,
    pub rules:        Vec<Rule>,
}

impl Logic {
    pub fn check(&self, state: &GameState, settings: &ServerConfig) -> bool {
        self.rules.iter().all(|&rule| rule.check(state, settings))
    }
}

use crate::{
    lttp::{
        item::{
            Armor,
            Gloves,
            Magic,
            Shield,
            Sword,
        },
        RandoLogic,
    },
    GameState,
};
use serde::Deserialize;
use std::convert::TryInto;
use tracing::error;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Rule {
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
    MayEnterDesertPalace,

    Rupee,

    AllSevenCrystals,
    BothRedCrystals,
}

impl Rule {
    pub fn check(self, state: &GameState) -> bool {
        #[allow(clippy::match_same_arms)]
        match self {
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

            Rule::CanLiftRocks => Rule::Gloves1.check(state),
            Rule::CanLiftDarkRocks => Rule::Gloves2.check(state),
            Rule::CanLightTorches => Rule::FireRod.check(state) || Rule::Lantern.check(state),
            Rule::CanMeltThings => {
                Rule::FireRod.check(state)
                    || (Rule::BombosMedallion.check(state) && Rule::Sword1.check(state))
            }
            Rule::CanFly => Rule::Flute.check(state),
            Rule::CanSpinSpeed => {
                Rule::Boots.check(state)
                    && (Rule::Sword1.check(state) || Rule::HookShot.check(state))
            }
            Rule::CanShootArrows => Rule::Bow.check(state),
            Rule::CanBlockLasers => Rule::Shield3.check(state),
            Rule::CanExtendMagic => Rule::Magic1.check(state) || Rule::Bottle.check(state),
            Rule::GlitchedLinkInDarkWorld => {
                Rule::MoonPearl.check(state) || Rule::Bottle.check(state)
            }
            // TODO (#420): Really need to be tracking if Agahnim 1 has already been beaten.
            Rule::BeatAgahnim1 => false,

            Rule::AllSevenCrystals => {
                state.crystal.one
                    && state.crystal.two
                    && state.crystal.three
                    && state.crystal.four
                    && state.crystal.five
                    && state.crystal.six
                    && state.crystal.seven
            }
            Rule::BothRedCrystals => false,

            x => {
                error!("check has not been implemented for {:?}", x);
                unimplemented!()
            }
        }
    }

    pub fn check_quantity(self, state: &GameState, quantity: u16) -> bool {
        match self {
            Rule::Bomb => state.bomb >= quantity.try_into().unwrap(),
            Rule::Bottle => state.bottle_count >= quantity.try_into().unwrap(),
            Rule::Rupee => state.rupees >= quantity,
            x => {
                error!("check_quantity has not been implemented for {:?}", x);
                unimplemented!();
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn check_with_options(
        self,
        state: &GameState,
        logic: RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match self {
            Rule::CanGoBeatAgahnim1 => {
                Rule::BeatAgahnim1.check(state)
                    || (allow_out_of_logic_glitches || Rule::Lantern.check(state))
                        && (Rule::Cape.check(state) || Rule::Sword2.check(state))
                        && Rule::Sword1.check(state)
            }
            Rule::CanEnterNorthEastDarkWorld => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::BeatAgahnim1.check(state)
                            || (agahnim_check
                                && Rule::CanGoBeatAgahnim1.check_with_options(
                                    state,
                                    logic,
                                    false,
                                    allow_out_of_logic_glitches,
                                ))
                            || (Rule::Hammer.check(state)
                                && Rule::CanLiftRocks.check(state)
                                && Rule::MoonPearl.check(state))
                            || (Rule::CanLiftDarkRocks.check(state)
                                && Rule::Flippers.check(state)
                                && Rule::MoonPearl.check(state))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::BeatAgahnim1.check(state)
                            || (agahnim_check
                                && Rule::CanGoBeatAgahnim1.check_with_options(
                                    state,
                                    logic,
                                    false,
                                    allow_out_of_logic_glitches,
                                ))
                            || (Rule::MoonPearl.check(state)
                                && ((Rule::CanLiftDarkRocks.check(state)
                                    && (Rule::Boots.check(state) || Rule::Flippers.check(state)))
                                    || (Rule::Hammer.check(state)
                                        && Rule::CanLiftRocks.check(state))))
                            || (Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                logic,
                                false,
                                allow_out_of_logic_glitches,
                            ) && ((Rule::Mirror.check(state)
                                && Rule::CanSpinSpeed.check(state))
                                || (Rule::MoonPearl.check(state)
                                    && (Rule::Mirror.check(state) || Rule::Boots.check(state)))))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::BeatAgahnim1.check(state)
                            || (agahnim_check
                                && Rule::CanGoBeatAgahnim1.check_with_options(
                                    state,
                                    logic,
                                    false,
                                    allow_out_of_logic_glitches,
                                ))
                            || (Rule::MoonPearl.check(state)
                                && (Rule::CanLiftDarkRocks.check(state)
                                    && (Rule::Boots.check(state) || Rule::Flippers.check(state)))
                                || (Rule::Hammer.check(state) && Rule::CanLiftRocks.check(state)))
                            || (Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                logic,
                                false,
                                allow_out_of_logic_glitches,
                            ) && (Rule::Bottle.check(state)
                                || (Rule::Mirror.check(state) && Rule::CanSpinSpeed.check(state))
                                || (Rule::MoonPearl.check(state)
                                    && (Rule::Mirror.check(state) || Rule::Boots.check(state)))))
                    }
                }
            }
            Rule::CanEnterNorthWestDarkWorld => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::MoonPearl.check(state)
                            && ((Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                logic,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            ) && Rule::HookShot.check(state)
                                && (Rule::Flippers.check(state)
                                    || Rule::CanLiftRocks.check(state)
                                    || Rule::Hammer.check(state)))
                                || (Rule::Hammer.check(state) && Rule::CanLiftRocks.check(state))
                                || Rule::CanLiftDarkRocks.check(state))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::CanEnterWestDeathMountain.check_with_options(
                            state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) && (Rule::Mirror.check(state)
                            || (Rule::Boots.check(state) && Rule::MoonPearl.check(state)))
                            || (Rule::MoonPearl.check(state)
                                && (Rule::CanLiftDarkRocks.check(state)
                                    || (Rule::Hammer.check(state)
                                        && Rule::CanLiftRocks.check(state))
                                    || ((Rule::BeatAgahnim1.check(state)
                                        || (agahnim_check
                                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                                state,
                                                logic,
                                                agahnim_check,
                                                allow_out_of_logic_glitches,
                                            )))
                                        && Rule::HookShot.check(state)
                                        && (Rule::Hammer.check(state)
                                            || Rule::CanLiftRocks.check(state)
                                            || Rule::Flippers.check(state)))))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::CanEnterWestDeathMountain.check_with_options(
                            state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || (Rule::MoonPearl.check(state)
                            && (Rule::CanLiftDarkRocks.check(state)
                                || (Rule::Hammer.check(state) && Rule::CanLiftRocks.check(state))
                                || ((Rule::BeatAgahnim1.check(state)
                                    || (agahnim_check
                                        && Rule::CanGoBeatAgahnim1.check_with_options(
                                            state,
                                            logic,
                                            agahnim_check,
                                            allow_out_of_logic_glitches,
                                        )))
                                    && Rule::HookShot.check(state)
                                    && (Rule::Hammer.check(state)
                                        || Rule::CanLiftRocks.check(state)
                                        || Rule::Flippers.check(state)))))
                    }
                }
            }
            Rule::CanEnterSouthDarkWorld => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::MoonPearl.check(state)
                            && (Rule::CanLiftDarkRocks.check(state)
                                || (Rule::Hammer.check(state) && Rule::CanLiftRocks.check(state))
                                || (Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                    state,
                                    logic,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ) && (Rule::Hammer.check(state)
                                    || (Rule::HookShot.check(state)
                                        && (Rule::Flippers.check(state)
                                            || Rule::CanLiftRocks.check(state))))))
                    }
                    RandoLogic::OverWorldGlitches => {
                        (Rule::CanEnterWestDeathMountain.check_with_options(
                            state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) && (Rule::Mirror.check(state)
                            || (Rule::Boots.check(state) && Rule::MoonPearl.check(state))))
                            || (Rule::MoonPearl.check(state)
                                && (Rule::CanLiftDarkRocks.check(state)
                                    || (Rule::Hammer.check(state)
                                        && Rule::CanLiftRocks.check(state))
                                    || ((Rule::BeatAgahnim1.check(state)
                                        || (agahnim_check
                                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                                state,
                                                logic,
                                                agahnim_check,
                                                allow_out_of_logic_glitches,
                                            )))
                                        && (Rule::Hammer.check(state)
                                            || Rule::HookShot.check(state)
                                                && (Rule::Flippers.check(state)
                                                    || Rule::CanLiftRocks.check(state))))))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::CanEnterWestDeathMountain.check_with_options(
                            state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || (Rule::MoonPearl.check(state)
                            && (Rule::CanLiftDarkRocks.check(state)
                                || (Rule::Hammer.check(state) && Rule::CanLiftRocks.check(state))
                                || ((Rule::BeatAgahnim1.check(state)
                                    || (agahnim_check
                                        && Rule::CanGoBeatAgahnim1.check_with_options(
                                            state,
                                            logic,
                                            agahnim_check,
                                            allow_out_of_logic_glitches,
                                        )))
                                    && (Rule::Hammer.check(state)
                                        || (Rule::HookShot.check(state)
                                            && (Rule::Flippers.check(state)
                                                || Rule::CanLiftRocks.check(state)))))))
                    }
                }
            }
            Rule::CanEnterMireArea => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::CanFly.check(state) && Rule::CanLiftDarkRocks.check(state)
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::CanLiftDarkRocks.check(state)
                            && (Rule::CanFly.check(state) || Rule::Boots.check(state))
                            || (Rule::MoonPearl.check(state)
                                && Rule::Boots.check(state)
                                && Rule::CanEnterSouthDarkWorld.check_with_options(
                                    state,
                                    logic,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                    RandoLogic::MajorGlitches => {
                        (Rule::Bottle.check(state)
                            && Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                logic,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            ))
                            || (Rule::CanLiftDarkRocks.check(state)
                                && (Rule::CanFly.check(state)
                                    || Rule::Bottle.check(state)
                                    || Rule::Boots.check(state)))
                            || (Rule::GlitchedLinkInDarkWorld.check(state)
                                && Rule::Boots.check(state)
                                && Rule::CanEnterSouthDarkWorld.check_with_options(
                                    state,
                                    logic,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                }
            }
            Rule::CanEnterWestDeathMountain => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::CanFly.check(state)
                            || (Rule::CanLiftRocks.check(state)
                                && (allow_out_of_logic_glitches || Rule::Lantern.check(state)))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::Boots.check(state)
                            || Rule::CanFly.check(state)
                            || (Rule::CanLiftRocks.check(state)
                                && (allow_out_of_logic_glitches || Rule::Lantern.check(state)))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::Boots.check(state)
                            || Rule::Bottle.check(state)
                            || Rule::CanFly.check(state)
                            || (Rule::CanLiftRocks.check(state)
                                && (allow_out_of_logic_glitches || Rule::Lantern.check(state)))
                    }
                }
            }
            Rule::CanEnterEastDeathMountain => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::CanEnterWestDeathMountain.check_with_options(
                            state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) && (Rule::HookShot.check(state)
                            || (Rule::Mirror.check(state) && Rule::Hammer.check(state)))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::Boots.check(state)
                            || (Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                logic,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            ) && (Rule::HookShot.check(state)
                                || (Rule::Mirror.check(state) && Rule::Hammer.check(state))))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::Boots.check(state)
                            || (Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                logic,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            ) && (Rule::HookShot.check(state) || Rule::Mirror.check(state)))
                    }
                }
            }
            Rule::CanEnterEastDarkWorldDeathMountain => {
                match logic {
                    RandoLogic::Glitchless => {
                        Rule::CanLiftDarkRocks.check(state)
                            && Rule::CanEnterEastDeathMountain.check_with_options(
                                state,
                                logic,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                    RandoLogic::OverWorldGlitches => {
                        (Rule::MoonPearl.check(state) && Rule::Book.check(state))
                            || ((Rule::CanLiftRocks.check(state)
                                || (Rule::Hammer.check(state) && Rule::Book.check(state)))
                                && Rule::CanEnterEastDeathMountain.check_with_options(
                                    state,
                                    logic,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::MoonPearl.check(state)
                            || (Rule::Bottle.check(state) && Rule::Boots.check(state))
                            || ((Rule::CanLiftRocks.check(state)
                                || (Rule::Hammer.check(state) && Rule::Boots.check(state)))
                                && Rule::CanEnterEastDeathMountain.check_with_options(
                                    state,
                                    logic,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                            || (Rule::Mirror.check(state)
                                && Rule::CanEnterWestDeathMountain.check_with_options(
                                    state,
                                    logic,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                }
            }

            Rule::CanEnterDesertPalace | Rule::MayEnterDesertPalace => true,

            x => x.check(state),
        }
    }
}

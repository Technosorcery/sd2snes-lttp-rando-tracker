use crate::lttp::{
    logic::{
        Availability,
        DungeonAvailability,
        RandoLogic,
        Rule,
    },
    DungeonState,
    GameState,
};
use serde::{
    Deserialize,
    Serialize,
};
use ts_rs::TS;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "ui/src/server_types/LocationAvailability.ts")]
#[serde(rename_all = "camelCase")]
pub enum LocationAvailability {
    AginahsCave,
    BombableHut,
    BombFairy,
    BombosTablet,
    BottleVendor,
    BugKid,
    BumperCave,
    BuriedItem,
    ByrnaSpikeCave,
    CastleSecretEntrance,
    Catfish,
    CaveUnderRockBottomChest,
    CaveUnderRockThreeTopChests,
    CheckerboardCave,
    ChickenHouse,
    CHouse,
    DarkWorldDeathMountain,
    DeathMountainEast,
    DesertWestLedge,
    DigGame,
    EscapeSewer,
    EtherTablet,
    FloatingIsland,
    ForestHideout,
    FugitiveUnderTheBridge,
    GraveyardCliffCave,
    Hammers,
    HypeCave,
    HyruleCastle,
    IceRodCave,
    KakarikoWell,
    KingsTomb,
    KingZora,
    LakeHyliaIsland,
    Library,
    LightWorldSwamp,
    LinksHouse,
    LostOldMan,
    LumberjackTree,
    MadBatter,
    MasterSwordPedestal,
    MimicCave,
    MinimoldormCave,
    Mushroom,
    PurpleChest,
    Pyramid,
    RaceMinigame,
    Sahasrahla,
    SahasrahlasHut,
    Sanctuary,
    SouthOfGrove,
    SpectacleRock,
    SpectacleRockCave,
    SpiralCave,
    StumpKid,
    TakeTheFrogHome,
    Tavern,
    ThievesHut,
    TreasureChestMinigame,
    WaterfallOfTheWishing,
    WestOfMire,
    WestOfSanctuary,
    Witch,
    ZoraRiverLedge,
}

impl LocationAvailability {
    #[allow(clippy::too_many_lines)]
    pub fn check(
        self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: RandoLogic,
    ) -> Availability {
        #[allow(clippy::match_same_arms)]
        match self {
            LocationAvailability::AginahsCave => {
                if Rule::Bomb.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::BombableHut => {
                if !Rule::Bomb.check(state) {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::MoonPearl.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }

                if Rule::GlitchedLinkInDarkWorld.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::BombFairy => {
                if Rule::BothRedCrystals.check(state) && Rule::MoonPearl.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) && (Rule::Hammer.check(state)
                        || (Rule::Mirror.check(state) && Rule::BeatAgahnim1.check(state)))
                    {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) && (Rule::Hammer.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                state,
                                RandoLogic::Glitchless,
                                false,
                                false,
                            )))
                    {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) && (Rule::Hammer.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                state,
                                RandoLogic::Glitchless,
                                false,
                                true,
                            )))
                    {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Mirror.check(state) && Rule::CanSpinSpeed.check(state) {
                    return Availability::Available;
                }
                if Rule::BothRedCrystals.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && ((Rule::Hammer.check(state) && Rule::MoonPearl.check(state))
                        || (Rule::Mirror.check(state) && Rule::BeatAgahnim1.check(state)))
                    {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) && ((Rule::Hammer.check(state) && Rule::MoonPearl.check(state))
                        || (Rule::Mirror.check(state)
                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                false,
                            )))
                    {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) && ((Rule::Hammer.check(state) && Rule::MoonPearl.check(state))
                        || (Rule::Mirror.check(state)
                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                true,
                            )))
                    {
                        return Availability::GlitchAgahnim;
                    }

                    if logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) && ((Rule::Hammer.check(state)
                        && Rule::GlitchedLinkInDarkWorld.check(state))
                        || (Rule::Mirror.check(state) && Rule::BeatAgahnim1.check(state)))
                    {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) && ((Rule::Hammer.check(state)
                        && Rule::GlitchedLinkInDarkWorld.check(state))
                        || (Rule::Mirror.check(state)
                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                false,
                            )))
                    {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) && ((Rule::Hammer.check(state)
                        && Rule::GlitchedLinkInDarkWorld.check(state))
                        || (Rule::Mirror.check(state)
                            && Rule::CanGoBeatAgahnim1.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                true,
                            )))
                    {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::BombosTablet => {
                if Rule::Book.check(state)
                    && Rule::Mirror.check(state)
                    && Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    )
                {
                    if Rule::Sword2.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::Book.check(state)
                    && Rule::Mirror.check(state)
                    && Rule::Sword2.check(state)
                {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Book.check(state)
                    && (Rule::Boots.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::CanEnterSouthDarkWorld.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                false,
                            )))
                {
                    if Rule::Sword2.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::Book.check(state)
                    && Rule::Mirror.check(state)
                    && Rule::Sword2.check(state)
                {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Book.check(state)
                    && (Rule::Boots.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::CanEnterSouthDarkWorld.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                false,
                            )))
                {
                    if Rule::Sword2.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::Book.check(state)
                    && Rule::Mirror.check(state)
                    && Rule::Sword2.check(state)
                {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::BottleVendor => {
                if Rule::Rupee.check_quantity(state, 100) {
                    return Availability::Available;
                }
            }
            LocationAvailability::BugKid => {
                if Rule::Bottle.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::BumperCave => {
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    if Rule::CanLiftRocks.check(state) && Rule::Cape.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) && Rule::CanLiftRocks.check(state)
                    && Rule::Cape.check(state)
                {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) && Rule::CanLiftRocks.check(state)
                    && Rule::Cape.check(state)
                {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    if Rule::MoonPearl.check(state)
                        && (Rule::Boots.check(state)
                            || (Rule::CanLiftRocks.check(state) && Rule::Cape.check(state)))
                    {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::MoonPearl.check(state)
                    && (Rule::Boots.check(state)
                        || (Rule::CanLiftRocks.check(state) && Rule::Cape.check(state)))
                {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    if Rule::GlitchedLinkInDarkWorld.check(state)
                        && (Rule::Boots.check(state)
                            || (Rule::CanLiftRocks.check(state) && Rule::Cape.check(state)))
                    {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::GlitchedLinkInDarkWorld.check(state)
                    && (Rule::Boots.check(state)
                        || (Rule::CanLiftRocks.check(state) && Rule::Cape.check(state)))
                {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::BuriedItem => {
                if Rule::Shovel.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::ByrnaSpikeCave => {
                if Rule::CanLiftRocks.check(state) && Rule::Hammer.check(state) {
                    if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        true,
                    ) && Rule::MoonPearl.check(state)
                    {
                        if Rule::CanExtendMagic.check(state)
                            && (Rule::Cape.check(state) || Rule::ByrnaCane.check(state))
                        {
                            if Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                RandoLogic::Glitchless,
                                false,
                                false,
                            ) {
                                return Availability::Available;
                            }
                            return Availability::GlitchAvailable;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                    if logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }

                    if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) && Rule::MoonPearl.check(state)
                    {
                        if Rule::CanExtendMagic.check(state)
                            && (Rule::Cape.check(state) || Rule::ByrnaCane.check(state))
                        {
                            if Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                false,
                            ) {
                                return Availability::Available;
                            }
                            return Availability::GlitchAvailable;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                    if logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }

                    if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) && (Rule::MoonPearl.check(state)
                        || (Rule::Bottle.check(state) && Rule::Boots.check(state)))
                    {
                        if Rule::CanExtendMagic.check(state)
                            && (Rule::Cape.check(state) || Rule::ByrnaCane.check(state))
                        {
                            if Rule::CanEnterWestDeathMountain.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                false,
                            ) {
                                return Availability::Available;
                            }
                            return Availability::GlitchAvailable;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                }
            }
            LocationAvailability::CastleSecretEntrance => {
                return Availability::Available;
            }
            LocationAvailability::Catfish => {
                if Rule::MoonPearl.check(state) && Rule::CanLiftRocks.check(state) {
                    if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::MoonPearl.check(state)
                    && (Rule::CanLiftRocks.check(state) || Rule::Boots.check(state))
                {
                    if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::GlitchedLinkInDarkWorld.check(state)
                    && (Rule::CanLiftRocks.check(state) || Rule::Boots.check(state))
                {
                    if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::CaveUnderRockBottomChest => {
                if Rule::MoonPearl.check(state)
                    && (Rule::HookShot.check(state) || Rule::Boots.check(state))
                {
                    if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }

                    if logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if Rule::CanLiftRocks.check(state) {
                        if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            true,
                        ) {
                            return Availability::GlitchAvailable;
                        }

                        if logic < RandoLogic::MajorGlitches {
                            return Availability::Unavailable;
                        }
                        if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            false,
                            true,
                        ) {
                            return Availability::GlitchAvailable;
                        }
                    }
                }
            }
            LocationAvailability::CaveUnderRockThreeTopChests => {
                if Rule::MoonPearl.check(state) && Rule::HookShot.check(state) {
                    if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }

                    if logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if Rule::CanLiftRocks.check(state) {
                        if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            true,
                        ) {
                            return Availability::GlitchAvailable;
                        }

                        if logic < RandoLogic::MajorGlitches {
                            return Availability::Unavailable;
                        }
                        if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            false,
                            true,
                        ) {
                            return Availability::GlitchAvailable;
                        }
                    }
                }
            }
            LocationAvailability::CheckerboardCave => {
                if Rule::CanFly.check(state)
                    && Rule::CanLiftDarkRocks.check(state)
                    && Rule::Mirror.check(state)
                {
                    return Availability::Available;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanLiftRocks.check(state) {
                    if Rule::Boots.check(state) {
                        return Availability::Available;
                    } else if Rule::Mirror.check(state) {
                        if Rule::CanEnterMireArea.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterMireArea.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            true,
                            false,
                        ) {
                            return Availability::Agahnim;
                        } else if Rule::CanEnterMireArea.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            true,
                            true,
                        ) {
                            return Availability::GlitchAgahnim;
                        }

                        if logic < RandoLogic::MajorGlitches {
                            return Availability::Unavailable;
                        }
                        if Rule::CanEnterMireArea.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterMireArea.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            true,
                            false,
                        ) {
                            return Availability::Agahnim;
                        } else if Rule::CanEnterMireArea.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            true,
                            true,
                        ) {
                            return Availability::GlitchAgahnim;
                        }
                    }
                }
            }
            LocationAvailability::ChickenHouse => {
                if Rule::Bomb.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::CHouse => {
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
            }
            LocationAvailability::DarkWorldDeathMountain => {
                if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) && Rule::MoonPearl.check(state)
                    {
                        return Availability::Available;
                    }
                    return Availability::GlitchAvailable;
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchAvailable;
                }
                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchAvailable;
                }
            }
            LocationAvailability::DeathMountainEast => {
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    if Rule::Bomb.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    if Rule::Bomb.check(state) {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    if Rule::Bomb.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Bomb.check(state) {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    if Rule::Bomb.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    true,
                ) {
                    if Rule::Bomb.check(state) {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }
            }
            LocationAvailability::DesertWestLedge => {
                if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Possible;
                }
                if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Possible;
                }
                if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::MajorGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if DungeonAvailability::DesertPalace.can_enter(
                    state,
                    dungeons,
                    RandoLogic::MajorGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
            }
            LocationAvailability::DigGame => {
                if !Rule::Rupee.check_quantity(state, 80) {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::MoonPearl.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::GlitchedLinkInDarkWorld.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::EscapeSewer => {
                return Availability::Available;
            }
            LocationAvailability::EtherTablet => {
                if Rule::Book.check(state)
                    && (Rule::Mirror.check(state)
                        || (Rule::Hammer.check(state) && Rule::HookShot.check(state)))
                {
                    if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        if Rule::Sword2.check(state) {
                            return Availability::Available;
                        }

                        return Availability::Possible;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        if Rule::Sword2.check(state) {
                            return Availability::GlitchAvailable;
                        }

                        return Availability::GlitchPossible;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Book.check(state) {
                    if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && DungeonAvailability::TowerOfHera.can_enter(
                        state,
                        dungeons,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        if Rule::Sword2.check(state) {
                            return Availability::Available;
                        }

                        return Availability::Possible;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) && DungeonAvailability::TowerOfHera.can_enter(
                        state,
                        dungeons,
                        RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        if Rule::Sword2.check(state) {
                            return Availability::GlitchAvailable;
                        }

                        return Availability::GlitchPossible;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Book.check(state) {
                    if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) && DungeonAvailability::TowerOfHera.can_enter(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        if Rule::Sword2.check(state) {
                            return Availability::Available;
                        }

                        return Availability::Possible;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) && DungeonAvailability::TowerOfHera.may_enter(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Possible;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) && DungeonAvailability::TowerOfHera.can_enter(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        if Rule::Sword2.check(state) {
                            return Availability::GlitchAvailable;
                        }

                        return Availability::GlitchPossible;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) && DungeonAvailability::TowerOfHera.may_enter(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchPossible;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) && DungeonAvailability::TowerOfHera.may_enter(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) && Rule::Sword2.check(state)
                    {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterWestDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) && DungeonAvailability::TowerOfHera.may_enter(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) && Rule::Sword2.check(state)
                    {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::FloatingIsland => {
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    if Rule::Mirror.check(state)
                        && Rule::MoonPearl.check(state)
                        && Rule::CanLiftDarkRocks.check(state)
                    {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    if Rule::Mirror.check(state)
                        && Rule::MoonPearl.check(state)
                        && Rule::CanLiftDarkRocks.check(state)
                    {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    if Rule::Boots.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::MoonPearl.check(state)
                            && Rule::CanLiftRocks.check(state)
                            && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                false,
                            ))
                    {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Boots.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::MoonPearl.check(state)
                            && Rule::CanLiftRocks.check(state)
                            && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                true,
                            ))
                    {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    if Rule::Boots.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanLiftRocks.check(state)
                            && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                false,
                            ))
                    {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    true,
                ) {
                    if Rule::Boots.check(state)
                        || (Rule::Mirror.check(state)
                            && Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanLiftRocks.check(state)
                            && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                true,
                            ))
                    {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }
            }
            LocationAvailability::ForestHideout => {
                return Availability::Available;
            }
            LocationAvailability::FugitiveUnderTheBridge => {
                if Rule::Flippers.check(state) || logic >= RandoLogic::OverWorldGlitches {
                    return Availability::Available;
                }

                return Availability::GlitchAvailable;
            }
            LocationAvailability::GraveyardCliffCave => {
                if Rule::Mirror.check(state) && Rule::MoonPearl.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Boots.check(state) {
                    return Availability::Available;
                } else if Rule::Mirror.check(state) && Rule::MoonPearl.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                } else if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                } else if Rule::Mirror.check(state) && Rule::GlitchedLinkInDarkWorld.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::Hammers => {
                if Rule::CanLiftDarkRocks.check(state) && Rule::Hammer.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Hammer.check(state) && Rule::MoonPearl.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && (Rule::CanLiftDarkRocks.check(state)
                        || (Rule::Boots.check(state)
                            && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                false,
                                false,
                            )))
                    {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) && (Rule::CanLiftDarkRocks.check(state)
                        || (Rule::Boots.check(state)
                            && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                true,
                                false,
                            )))
                    {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) && (Rule::CanLiftDarkRocks.check(state)
                        || (Rule::Boots.check(state)
                            && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                RandoLogic::OverWorldGlitches,
                                true,
                                true,
                            )))
                    {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Hammer.check(state) && Rule::GlitchedLinkInDarkWorld.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::HypeCave => {
                if !Rule::Bomb.check(state) {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::MoonPearl.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::GlitchedLinkInDarkWorld.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::HyruleCastle => {
                return Availability::Available;
            }
            LocationAvailability::IceRodCave => {
                if Rule::Bomb.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::KakarikoWell => {
                if Rule::Bomb.check(state) {
                    return Availability::Available;
                }
                return Availability::Possible;
            }
            LocationAvailability::KingsTomb => {
                if Rule::Boots.check(state) && Rule::CanLiftDarkRocks.check(state) {
                    return Availability::Available;
                } else if Rule::Boots.check(state) && Rule::Mirror.check(state) {
                    if Rule::MoonPearl.check(state) {
                        if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                            state,
                            RandoLogic::Glitchless,
                            false,
                            false,
                        ) {
                            return Availability::Available;
                        } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                            state,
                            RandoLogic::Glitchless,
                            true,
                            false,
                        ) {
                            return Availability::Agahnim;
                        } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                            state,
                            RandoLogic::Glitchless,
                            true,
                            true,
                        ) {
                            return Availability::GlitchAgahnim;
                        } else if logic >= RandoLogic::OverWorldGlitches {
                            return Availability::Available;
                        }
                    } else if Rule::GlitchedLinkInDarkWorld.check(state)
                        && logic >= RandoLogic::MajorGlitches
                    {
                        return Availability::Available;
                    }
                }
            }
            LocationAvailability::KingZora => {
                if Rule::Rupee.check_quantity(state, 500) {
                    if logic == RandoLogic::Glitchless {
                        if Rule::Flippers.check(state) || Rule::CanLiftRocks.check(state) {
                            return Availability::Available;
                        }

                        return Availability::GlitchAvailable;
                    }

                    return Availability::Available;
                }
            }
            LocationAvailability::LakeHyliaIsland => {
                if Rule::Flippers.check(state)
                    && Rule::MoonPearl.check(state)
                    && Rule::Mirror.check(state)
                {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Possible;
                }
                if Rule::Boots.check(state) {
                    return Availability::Available;
                }
                if Rule::Flippers.check(state) && Rule::Mirror.check(state) {
                    if (Rule::MoonPearl.check(state)
                        && Rule::CanEnterSouthDarkWorld.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            false,
                        ))
                        || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            false,
                            false,
                        )
                    {
                        return Availability::Available;
                    } else if (Rule::MoonPearl.check(state)
                        && Rule::CanEnterSouthDarkWorld.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            true,
                            false,
                        ))
                        || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            true,
                            false,
                        )
                    {
                        return Availability::Agahnim;
                    } else if (Rule::MoonPearl.check(state)
                        && Rule::CanEnterSouthDarkWorld.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            true,
                            true,
                        ))
                        || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            state,
                            RandoLogic::OverWorldGlitches,
                            true,
                            true,
                        )
                    {
                        return Availability::GlitchAgahnim;
                    }

                    if logic < RandoLogic::MajorGlitches {
                        return Availability::Possible;
                    }
                    if Rule::GlitchedLinkInDarkWorld.check(state)
                        || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            false,
                            false,
                        )
                    {
                        return Availability::Available;
                    } else if Rule::GlitchedLinkInDarkWorld.check(state)
                        || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            true,
                            false,
                        )
                    {
                        return Availability::Agahnim;
                    } else if Rule::GlitchedLinkInDarkWorld.check(state)
                        || Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            state,
                            RandoLogic::MajorGlitches,
                            true,
                            true,
                        )
                    {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::Library => {
                if Rule::Boots.check(state) {
                    return Availability::Available;
                }

                return Availability::Possible;
            }
            LocationAvailability::LightWorldSwamp => {
                return Availability::Available;
            }
            LocationAvailability::LinksHouse => {
                return Availability::Available;
            }
            LocationAvailability::LostOldMan => {
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    if Rule::Lantern.check(state) {
                        return Availability::Available;
                    }

                    return Availability::GlitchAvailable;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Lantern.check(state) {
                        return Availability::Available;
                    }

                    return Availability::GlitchAvailable;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    true,
                ) {
                    if Rule::Lantern.check(state) {
                        return Availability::Available;
                    }

                    return Availability::GlitchAvailable;
                }
            }
            LocationAvailability::LumberjackTree => {
                if Rule::Boots.check(state) {
                    if Rule::BeatAgahnim1.check(state) {
                        return Availability::Available;
                    } else if Rule::CanGoBeatAgahnim1.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanGoBeatAgahnim1.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                return Availability::Possible;
            }
            LocationAvailability::MadBatter => {
                if Rule::Hammer.check(state)
                    || (Rule::MoonPearl.check(state)
                        && Rule::Mirror.check(state)
                        && Rule::CanLiftDarkRocks.check(state))
                {
                    if Rule::Powder.check(state) {
                        return Availability::Available;
                    } else if Rule::SomariaCane.check(state) && Rule::Mushroom.check(state) {
                        return Availability::GlitchAvailable;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Powder.check(state) && Rule::Boots.check(state) {
                    return Availability::Available;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Powder.check(state) && Rule::Mirror.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::MasterSwordPedestal => {
                if Rule::BluePendant.check(state)
                    && Rule::GreenPendant.check(state)
                    && Rule::RedPendant.check(state)
                {
                    return Availability::Available;
                } else if Rule::Book.check(state) {
                    return Availability::Possible;
                }
            }
            LocationAvailability::MimicCave => {
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) && Rule::Mirror.check(state)
                    && Rule::MayEnterDesertPalace.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    )
                {
                    if Rule::FireRod.check(state)
                        && Rule::CanEnterDesertPalace.check_with_options(
                            state,
                            RandoLogic::Glitchless,
                            false,
                            false,
                        )
                    {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) && Rule::Mirror.check(state)
                    && Rule::MayEnterDesertPalace.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        true,
                    )
                {
                    if Rule::FireRod.check(state)
                        && Rule::CanEnterDesertPalace.check_with_options(
                            state,
                            RandoLogic::Glitchless,
                            false,
                            true,
                        )
                    {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::Hammer.check(state) && Rule::Mirror.check(state) {
                    if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }
                    if logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }

                    if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }
                }
            }
            LocationAvailability::MinimoldormCave => {
                if Rule::Bomb.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::Mushroom => {
                return Availability::Available;
            }
            LocationAvailability::PurpleChest => {
                if Rule::CanLiftDarkRocks.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::MoonPearl.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && LocationAvailability::TakeTheFrogHome.check(
                        state,
                        dungeons,
                        RandoLogic::OverWorldGlitches,
                    ) == Availability::Available
                        && (Rule::CanLiftDarkRocks.check(state)
                            || (Rule::Boots.check(state)
                                && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                    state,
                                    RandoLogic::OverWorldGlitches,
                                    false,
                                    false,
                                )))
                    {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) && (LocationAvailability::TakeTheFrogHome.check(
                        state,
                        dungeons,
                        RandoLogic::OverWorldGlitches,
                    ) == Availability::Available
                        || LocationAvailability::TakeTheFrogHome.check(
                            state,
                            dungeons,
                            RandoLogic::OverWorldGlitches,
                        ) == Availability::Agahnim)
                        && (Rule::CanLiftDarkRocks.check(state)
                            || (Rule::Boots.check(state)
                                && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                    state,
                                    RandoLogic::OverWorldGlitches,
                                    true,
                                    false,
                                )))
                    {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) && (LocationAvailability::TakeTheFrogHome.check(
                        state,
                        dungeons,
                        RandoLogic::OverWorldGlitches,
                    ) == Availability::Available
                        || LocationAvailability::TakeTheFrogHome.check(
                            state,
                            dungeons,
                            RandoLogic::OverWorldGlitches,
                        ) == Availability::Agahnim
                        || LocationAvailability::TakeTheFrogHome.check(
                            state,
                            dungeons,
                            RandoLogic::OverWorldGlitches,
                        ) == Availability::GlitchAgahnim)
                        && (Rule::CanLiftDarkRocks.check(state)
                            || (Rule::Boots.check(state)
                                && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                    state,
                                    RandoLogic::OverWorldGlitches,
                                    true,
                                    true,
                                )))
                    {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) && LocationAvailability::TakeTheFrogHome.check(
                    state,
                    dungeons,
                    RandoLogic::MajorGlitches,
                ) == Availability::Available
                    && (Rule::Mirror.check(state)
                        || (Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanLiftDarkRocks.check(state))
                        || (Rule::Boots.check(state)
                            && Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                false,
                                false,
                            )))
                {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    false,
                ) && (LocationAvailability::TakeTheFrogHome.check(
                    state,
                    dungeons,
                    RandoLogic::MajorGlitches,
                ) == Availability::Available
                    || LocationAvailability::TakeTheFrogHome.check(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                    ) == Availability::Agahnim)
                    && (Rule::Mirror.check(state)
                        || (Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanLiftDarkRocks.check(state))
                        || (Rule::Boots.check(state)
                            && Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                true,
                                false,
                            )))
                {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    true,
                ) && (LocationAvailability::TakeTheFrogHome.check(
                    state,
                    dungeons,
                    RandoLogic::MajorGlitches,
                ) == Availability::Available
                    || LocationAvailability::TakeTheFrogHome.check(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                    ) == Availability::Agahnim
                    || LocationAvailability::TakeTheFrogHome.check(
                        state,
                        dungeons,
                        RandoLogic::MajorGlitches,
                    ) == Availability::GlitchAgahnim)
                    && (Rule::Mirror.check(state)
                        || (Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanLiftDarkRocks.check(state))
                        || (Rule::Boots.check(state)
                            && Rule::GlitchedLinkInDarkWorld.check(state)
                            && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                                state,
                                RandoLogic::MajorGlitches,
                                true,
                                true,
                            )))
                {
                    return Availability::GlitchAgahnim;
                }
            }
            LocationAvailability::Pyramid => {
                if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
            }
            LocationAvailability::RaceMinigame => {
                if Rule::Bomb.check(state) || Rule::Boots.check(state) {
                    return Availability::Available;
                }

                return Availability::Possible;
            }
            LocationAvailability::Sahasrahla => {
                if Rule::GreenPendant.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::SahasrahlasHut => {
                if Rule::Bomb.check(state) || Rule::Boots.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::Sanctuary => {
                return Availability::Available;
            }
            LocationAvailability::SouthOfGrove => {
                if Rule::Mirror.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::Boots.check(state) {
                    return Availability::Available;
                } else if Rule::Mirror.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }

                    if logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::SpectacleRock => {
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    if Rule::Mirror.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    if Rule::Mirror.check(state) {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    if Rule::Boots.check(state) || Rule::Mirror.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Boots.check(state) || Rule::Mirror.check(state) {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    if Rule::Boots.check(state) || Rule::Mirror.check(state) {
                        return Availability::Available;
                    }

                    return Availability::Possible;
                } else if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    true,
                ) {
                    if Rule::Boots.check(state) || Rule::Mirror.check(state) {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::GlitchPossible;
                }
            }
            LocationAvailability::SpectacleRockCave => {
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    return Availability::GlitchAvailable;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchAvailable;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterWestDeathMountain.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchAvailable;
                }
            }
            LocationAvailability::SpiralCave => {
                if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterEastDeathMountain.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    true,
                ) {
                    return Availability::GlitchAvailable;
                }

                if logic >= RandoLogic::OverWorldGlitches {
                    if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }
                }

                if logic >= RandoLogic::MajorGlitches {
                    if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterEastDeathMountain.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }
                }
            }
            LocationAvailability::StumpKid => {
                if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::MoonPearl.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::GlitchedLinkInDarkWorld.check(state) {
                    if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterSouthDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::TakeTheFrogHome => {
                if Rule::CanLiftDarkRocks.check(state) {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::Glitchless,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::MoonPearl.check(state)
                    && (Rule::CanLiftDarkRocks.check(state)
                        || (Rule::Boots.check(state) && Rule::Mirror.check(state)))
                {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::GlitchedLinkInDarkWorld.check(state)
                    && (Rule::CanLiftDarkRocks.check(state)
                        || (Rule::Boots.check(state) && Rule::Mirror.check(state)))
                {
                    if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        state,
                        RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            LocationAvailability::Tavern => {
                return Availability::Available;
            }
            LocationAvailability::ThievesHut => {
                if Rule::Bomb.check(state) {
                    return Availability::Available;
                }
                return Availability::Possible;
            }
            LocationAvailability::TreasureChestMinigame => {
                if !Rule::Rupee.check_quantity(state, 30) {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterNorthWestDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterNorthEastDarkWorld.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
            }
            LocationAvailability::WaterfallOfTheWishing => {
                if Rule::Flippers.check(state) {
                    return Availability::Available;
                } else if Rule::MoonPearl.check(state) {
                    if logic < RandoLogic::OverWorldGlitches {
                        return Availability::GlitchAvailable;
                    }

                    return Availability::Available;
                } else if Rule::Boots.check(state) {
                    return Availability::GlitchAvailable;
                }
            }
            LocationAvailability::WestOfMire => {
                if Rule::CanEnterMireArea.check_with_options(
                    state,
                    RandoLogic::Glitchless,
                    false,
                    false,
                ) {
                    if Rule::MoonPearl.check(state) {
                        return Availability::Available;
                    } else if Rule::Mirror.check(state) {
                        return Availability::GlitchAvailable;
                    }
                }
                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }

                if Rule::MoonPearl.check(state) || Rule::Mirror.check(state) {
                    if Rule::CanEnterMireArea.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if Rule::CanEnterMireArea.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if Rule::CanEnterMireArea.check_with_options(
                        state,
                        RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
                if logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }

                if Rule::CanEnterMireArea.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Available;
                } else if Rule::CanEnterMireArea.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if Rule::CanEnterMireArea.check_with_options(
                    state,
                    RandoLogic::MajorGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }
            }
            LocationAvailability::WestOfSanctuary => {
                if Rule::Boots.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::Witch => {
                if Rule::Mushroom.check(state) {
                    return Availability::Available;
                }
            }
            LocationAvailability::ZoraRiverLedge => {
                if Rule::Flippers.check(state) {
                    return Availability::Available;
                } else if Rule::CanLiftRocks.check(state) {
                    return Availability::Possible;
                }

                if logic < RandoLogic::OverWorldGlitches {
                    return Availability::GlitchPossible;
                }
                if Rule::Boots.check(state) && Rule::MoonPearl.check(state) {
                    return Availability::Available;
                }

                return Availability::Possible;
            }
        }

        Availability::Unavailable
    }
}

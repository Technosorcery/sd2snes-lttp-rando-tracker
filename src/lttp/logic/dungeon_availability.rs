use crate::lttp::{
    logic::{
        Availability,
        RandoLogic,
        Rule,
    },
    Dungeon,
    DungeonState,
    GameState,
    Medallion,
};
use serde_derive::{
    Deserialize,
    Serialize,
};
use std::cmp::{
    max,
    min,
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DungeonAvailability {
    DesertPalace,
    EasternPalace,
    GanonsTower,
    IcePalace,
    MiseryMire,
    PalaceOfDarkness,
    SkullWoods,
    SwampPalace,
    ThievesTown,
    TowerOfHera,
    TurtleRock,
}

impl DungeonAvailability {
    pub fn can_enter(
        &self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match self {
            DungeonAvailability::DesertPalace => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        Rule::Book.check(&state)
                            || (Rule::Mirror.check(&state)
                                && Rule::CanLiftDarkRocks.check(&state)
                                && Rule::CanFly.check(&state))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::Book.check(&state)
                            || Rule::Boots.check(&state)
                            || (Rule::Mirror.check(&state)
                                && Rule::CanEnterMireArea.check_with_options(
                                    &state,
                                    &RandoLogic::OverWorldGlitches,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::Book.check(&state)
                            || Rule::Boots.check(&state)
                            || (Rule::Mirror.check(&state)
                                && Rule::CanEnterMireArea.check_with_options(
                                    &state,
                                    &RandoLogic::MajorGlitches,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                }
            }
            DungeonAvailability::GanonsTower => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        Rule::AllSevenCrystals.check(&state)
                            && Rule::MoonPearl.check(&state)
                            && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                                &state,
                                &RandoLogic::Glitchless,
                                false,
                                allow_out_of_logic_glitches,
                            )
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::Boots.check(&state) && Rule::MoonPearl.check(&state)
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::CanEnterWestDeathMountain.check_with_options(
                            &state,
                            &RandoLogic::MajorGlitches,
                            false,
                            allow_out_of_logic_glitches,
                        )
                    }
                }
            }
            DungeonAvailability::IcePalace => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        Rule::CanLiftDarkRocks.check(&state)
                            && Rule::CanMeltThings.check(&state)
                            && (allow_out_of_logic_glitches
                                || (Rule::MoonPearl.check(&state) && Rule::Flippers.check(&state)))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::CanLiftDarkRocks.check(&state) && Rule::CanMeltThings.check(&state)
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::CanLiftDarkRocks.check(&state)
                            || (Rule::Mirror.check(&state)
                                && Rule::GlitchedLinkInDarkWorld.check(&state)
                                && Rule::CanEnterSouthDarkWorld.check_with_options(
                                    &state,
                                    &RandoLogic::MajorGlitches,
                                    agahnim_check,
                                    allow_out_of_logic_glitches,
                                ))
                    }
                }
            }
            DungeonAvailability::MiseryMire => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        self.has_medallion(&state, &dungeons)
                            && Rule::Sword1.check(&state)
                            && Rule::MoonPearl.check(&state)
                            && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                            && Rule::CanEnterMireArea.check_with_options(
                                &state,
                                &RandoLogic::Glitchless,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                    RandoLogic::OverWorldGlitches => {
                        self.has_medallion(&state, &dungeons)
                            && Rule::Sword1.check(&state)
                            && Rule::MoonPearl.check(&state)
                            && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                            && Rule::CanEnterMireArea.check_with_options(
                                &state,
                                &RandoLogic::OverWorldGlitches,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                    RandoLogic::MajorGlitches => {
                        self.has_medallion(&state, &dungeons)
                            && Rule::Sword1.check(&state)
                            && (Rule::MoonPearl.check(&state)
                                || (Rule::Bottle.check(&state) && Rule::Boots.check(&state)))
                            && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                            && Rule::CanEnterMireArea.check_with_options(
                                &state,
                                &RandoLogic::MajorGlitches,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                }
            }
            DungeonAvailability::PalaceOfDarkness => {
                if *logic < RandoLogic::MajorGlitches {
                    return Rule::CanEnterNorthEastDarkWorld.check_with_options(
                        &state,
                        logic,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    ) && Rule::MoonPearl.check(&state);
                } else {
                    return (Rule::GlitchedLinkInDarkWorld.check(&state)
                        && Rule::CanEnterNorthEastDarkWorld.check_with_options(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ))
                        || Rule::CanEnterWestDeathMountain.check_with_options(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        );
                }
            }
            DungeonAvailability::SkullWoods => {
                if *logic == RandoLogic::Glitchless {
                    return Rule::MoonPearl.check(&state)
                        && Rule::CanEnterNorthWestDarkWorld.check_with_options(
                            &state,
                            &RandoLogic::Glitchless,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        );
                } else {
                    return Rule::CanEnterNorthWestDarkWorld.check_with_options(
                        &state,
                        logic,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    );
                }
            }
            DungeonAvailability::SwampPalace => {
                if *logic < RandoLogic::MajorGlitches {
                    return Rule::MoonPearl.check(&state)
                        && Rule::Mirror.check(&state)
                        && Rule::Flippers.check(&state)
                        && Rule::CanEnterSouthDarkWorld.check_with_options(
                            &state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        );
                } else {
                    return DungeonAvailability::MiseryMire.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    ) || (Rule::MoonPearl.check(&state)
                        && Rule::Mirror.check(&state)
                        && Rule::Flippers.check(&state)
                        && Rule::CanEnterSouthDarkWorld.check_with_options(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ));
                }
            }
            DungeonAvailability::ThievesTown => {
                return if *logic == RandoLogic::Glitchless {
                    Rule::GlitchedLinkInDarkWorld.check(&state)
                        && Rule::CanEnterNorthWestDarkWorld.check_with_options(
                            &state,
                            &RandoLogic::Glitchless,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                } else {
                    Rule::MoonPearl.check(&state)
                        && Rule::CanEnterNorthWestDarkWorld.check_with_options(
                            &state,
                            logic,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                };
            }
            DungeonAvailability::TowerOfHera => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        Rule::CanEnterWestDeathMountain.check_with_options(
                            &state,
                            &RandoLogic::Glitchless,
                            false,
                            allow_out_of_logic_glitches,
                        ) && (Rule::Mirror.check(&state)
                            || (Rule::HookShot.check(&state) && Rule::Hammer.check(&state)))
                    }
                    RandoLogic::OverWorldGlitches => {
                        Rule::Boots.check(&state)
                            || (Rule::CanEnterWestDeathMountain.check_with_options(
                                &state,
                                &RandoLogic::OverWorldGlitches,
                                false,
                                allow_out_of_logic_glitches,
                            ) && (Rule::Mirror.check(&state)
                                || (Rule::HookShot.check(&state) && Rule::Hammer.check(&state))))
                    }
                    RandoLogic::MajorGlitches => {
                        Rule::Boots.check(&state)
                            || (Rule::CanEnterWestDeathMountain.check_with_options(
                                &state,
                                &RandoLogic::MajorGlitches,
                                false,
                                allow_out_of_logic_glitches,
                            ) && (Rule::Mirror.check(&state)
                                || (Rule::HookShot.check(&state) && Rule::Hammer.check(&state))))
                            || DungeonAvailability::MiseryMire.can_enter(
                                &state,
                                &dungeons,
                                &RandoLogic::MajorGlitches,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                };
            }
            DungeonAvailability::TurtleRock => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        self.upper_can(
                            &state,
                            &dungeons,
                            &RandoLogic::Glitchless,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                    }
                    RandoLogic::OverWorldGlitches => {
                        self.middle(
                            &state,
                            &RandoLogic::OverWorldGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || self.upper_can(
                            &state,
                            &dungeons,
                            &RandoLogic::OverWorldGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                    }
                    RandoLogic::MajorGlitches => {
                        self.lower(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || self.middle(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || self.upper_can(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                    }
                }
            }

            x => eprintln!("can_enter not yet implemented for {:?}", x),
        }

        false
    }

    pub fn may_enter(
        &self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match self {
            DungeonAvailability::MiseryMire => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        self.may_have_medallion(&state, &dungeons)
                            && Rule::Sword1.check(&state)
                            && Rule::MoonPearl.check(&state)
                            && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                            && Rule::CanEnterMireArea.check_with_options(
                                &state,
                                &RandoLogic::Glitchless,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                    RandoLogic::OverWorldGlitches => {
                        self.may_have_medallion(&state, &dungeons)
                            && Rule::Sword1.check(&state)
                            && Rule::MoonPearl.check(&state)
                            && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                            && Rule::CanEnterMireArea.check_with_options(
                                &state,
                                &RandoLogic::OverWorldGlitches,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                    RandoLogic::MajorGlitches => {
                        self.may_have_medallion(&state, &dungeons)
                            && Rule::Sword1.check(&state)
                            && (Rule::MoonPearl.check(&state)
                                || (Rule::Bottle.check(&state) && Rule::Boots.check(&state)))
                            && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                            && Rule::CanEnterMireArea.check_with_options(
                                &state,
                                &RandoLogic::MajorGlitches,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                }
            }
            DungeonAvailability::TowerOfHera => {
                if *logic < RandoLogic::MajorGlitches {
                    return self.can_enter(&state, &dungeons, &logic, false, false);
                } else {
                    return DungeonAvailability::MiseryMire.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    );
                }
            }
            DungeonAvailability::TurtleRock => {
                return match *logic {
                    RandoLogic::Glitchless => {
                        self.upper_may(
                            &state,
                            &dungeons,
                            &RandoLogic::Glitchless,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                    }
                    RandoLogic::OverWorldGlitches => {
                        self.middle(
                            &state,
                            &RandoLogic::OverWorldGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || self.upper_may(
                            &state,
                            &dungeons,
                            &RandoLogic::OverWorldGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                    }
                    RandoLogic::MajorGlitches => {
                        self.lower(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || self.middle(
                            &state,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        ) || self.upper_may(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            agahnim_check,
                            allow_out_of_logic_glitches,
                        )
                    }
                }
            }

            x => eprintln!("may_enter not yet implemented for {:?}", x),
        }

        false
    }

    pub fn is_beatable(
        &self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: &RandoLogic,
    ) -> Availability {
        match self {
            DungeonAvailability::DesertPalace => {
                if Rule::CanLiftRocks.check(&state) && Rule::CanLightTorches.check(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                        if Rule::Boots.check(&state) {
                            if self.can_hurt_boss(&state) {
                                return Availability::Available;
                            } else {
                                return Availability::GlitchAvailable;
                            }
                        } else if self.can_hurt_boss(&state) {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        if Rule::Boots.check(&state) {
                            if self.can_hurt_boss(&state) {
                                return Availability::Available;
                            } else {
                                return Availability::GlitchAvailable;
                            }
                        } else if self.can_hurt_boss(&state) {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        if self.can_hurt_boss(&state) {
                            return Availability::Agahnim;
                        } else {
                            return Availability::GlitchAgahnim;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                        if Rule::Boots.check(&state) {
                            if self.can_hurt_boss(&state) {
                                return Availability::Available;
                            } else {
                                return Availability::GlitchAvailable;
                            }
                        } else if self.can_hurt_boss(&state) {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        if self.can_hurt_boss(&state) {
                            return Availability::Agahnim;
                        } else {
                            return Availability::GlitchAgahnim;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::EasternPalace => {
                if Rule::Bow.check(&state) {
                    if Rule::Lantern.check(&state) {
                        return Availability::Available;
                    } else {
                        return Availability::GlitchAvailable;
                    }
                }
            }
            DungeonAvailability::GanonsTower => {
                if Rule::HookShot.check(&state)
                    && Rule::Bow.check(&state)
                    && Rule::CanLightTorches.check(&state)
                    && (Rule::Hammer.check(&state) || Rule::Sword1.check(&state))
                {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                        if Rule::Boots.check(&state)
                            && Rule::Hammer.check(&state)
                            && Rule::FireRod.check(&state)
                            && Rule::SomariaCane.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        if Rule::Boots.check(&state)
                            && Rule::Hammer.check(&state)
                            && Rule::FireRod.check(&state)
                            && Rule::SomariaCane.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        if Rule::Boots.check(&state)
                            && Rule::Hammer.check(&state)
                            && Rule::FireRod.check(&state)
                            && Rule::SomariaCane.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                        if Rule::Boots.check(&state)
                            && Rule::Hammer.check(&state)
                            && Rule::FireRod.check(&state)
                            && Rule::SomariaCane.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        if Rule::Boots.check(&state)
                            && Rule::Hammer.check(&state)
                            && Rule::FireRod.check(&state)
                            && Rule::SomariaCane.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                }
            }
            DungeonAvailability::IcePalace => {
                if Rule::CanMeltThings.check(&state) && Rule::CanLiftRocks.check(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false)
                        && Rule::Hammer.check(&state)
                    {
                        if Rule::HookShot.check(&state) && Rule::SomariaCane.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && Rule::Hammer.check(&state)
                    {
                        if Rule::HookShot.check(&state) && Rule::SomariaCane.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false)
                        && Rule::Hammer.check(&state)
                    {
                        if Rule::HookShot.check(&state) && Rule::SomariaCane.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) && Rule::Hammer.check(&state)
                    {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::MiseryMire => {
                if Rule::SomariaCane.check(&state) && self.can_hurt_boss(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false)
                        && Rule::Lantern.check(&state)
                    {
                        if Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false)
                    && Rule::Lantern.check(&state)
                {
                    return Availability::Possible;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    if Rule::CanLightTorches.check(&state)
                        && (Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    return Availability::GlitchPossible;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state) {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) && Rule::Lantern.check(&state)
                {
                    return Availability::Possible;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::CanLightTorches.check(&state)
                        && (Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchPossible;
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false)
                    && Rule::Lantern.check(&state)
                {
                    if Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state) {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::MajorGlitches,
                    false,
                    false,
                ) && Rule::Lantern.check(&state)
                {
                    return Availability::Possible;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::CanLightTorches.check(&state)
                        && (Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    return Availability::GlitchPossible;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::PalaceOfDarkness => {
                if Rule::Hammer.check(&state) && Rule::Bow.check(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false)
                        && Rule::Lantern.check(&state)
                    {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        true,
                        false,
                    ) && Rule::Lantern.check(&state)
                    {
                        return Availability::Agahnim;
                    } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true)
                    {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && Rule::Lantern.check(&state)
                    {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) && Rule::Lantern.check(&state)
                    {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false)
                        && Rule::Lantern.check(&state)
                    {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) && Rule::Lantern.check(&state)
                    {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::SkullWoods => {
                if Rule::MoonPearl.check(&state)
                    && Rule::FireRod.check(&state)
                    && Rule::Sword1.check(&state)
                {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true)
                    {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::SwampPalace => {
                if Rule::Hammer.check(&state) && Rule::HookShot.check(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true)
                    {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if Rule::HookShot.check(&state)
                    && Rule::Flippers.check(&state)
                    && (Rule::Sword1.check(&state)
                        || Rule::Hammer.check(&state)
                        || ((Rule::Bow.check(&state) || Rule::CanExtendMagic.check(&state))
                            && (Rule::FireRod.check(&state) || Rule::IceRod.check(&state))))
                {
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false)
                        && (Rule::Hammer.check(&state)
                            || DungeonAvailability::MiseryMire.can_enter(
                                &state,
                                &dungeons,
                                &RandoLogic::MajorGlitches,
                                false,
                                false,
                            ))
                    {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) && (Rule::Hammer.check(&state)
                        || DungeonAvailability::MiseryMire.can_enter(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            false,
                            true,
                        ))
                    {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) && (Rule::Hammer.check(&state)
                        || DungeonAvailability::MiseryMire.can_enter(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            true,
                            false,
                        ))
                    {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) && (Rule::Hammer.check(&state)
                        || DungeonAvailability::MiseryMire.can_enter(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            true,
                            true,
                        ))
                    {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::ThievesTown => {
                if self.can_hurt_boss(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true)
                    {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                        return Availability::Available;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchAvailable;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::TowerOfHera => {
                if Rule::Sword1.check(&state) || Rule::Hammer.check(&state) {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                        if Rule::CanLightTorches.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        if Rule::CanLightTorches.check(&state) {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        if Rule::CanLightTorches.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        if Rule::CanLightTorches.check(&state) {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                        if Rule::CanLightTorches.check(&state)
                            || DungeonAvailability::MiseryMire.can_enter(
                                &state,
                                &dungeons,
                                &RandoLogic::MajorGlitches,
                                false,
                                false,
                            )
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        if Rule::CanLightTorches.check(&state)
                            || DungeonAvailability::MiseryMire.can_enter(
                                &state,
                                &dungeons,
                                &RandoLogic::MajorGlitches,
                                false,
                                true,
                            )
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Possible;
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchPossible;
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        false,
                    ) {
                        return Availability::Agahnim;
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        true,
                        true,
                    ) {
                        return Availability::GlitchAgahnim;
                    }
                }
            }
            DungeonAvailability::TurtleRock => {
                if Rule::FireRod.check(&state)
                    && Rule::IceRod.check(&state)
                    && Rule::SomariaCane.check(&state)
                {
                    if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false)
                        && Rule::Lantern.check(&state)
                        && (Rule::Hammer.check(&state) || Rule::Sword2.check(&state))
                    {
                        if Rule::Cape.check(&state)
                            || Rule::ByrnaCane.check(&state)
                            || Rule::CanBlockLasers.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        false,
                    ) {
                        return Availability::Possible;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        if Rule::Cape.check(&state)
                            || Rule::ByrnaCane.check(&state)
                            || Rule::CanBlockLasers.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::Glitchless,
                        false,
                        true,
                    ) {
                        return Availability::GlitchPossible;
                    }

                    if *logic < RandoLogic::OverWorldGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) && Rule::Lantern.check(&state)
                        && (Rule::Hammer.check(&state) || Rule::Sword2.check(&state))
                    {
                        if Rule::Cape.check(&state)
                            || Rule::ByrnaCane.check(&state)
                            || Rule::CanBlockLasers.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Possible;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        if Rule::Cape.check(&state)
                            || Rule::ByrnaCane.check(&state)
                            || Rule::CanBlockLasers.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::OverWorldGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchPossible;
                    }

                    if *logic < RandoLogic::MajorGlitches {
                        return Availability::Unavailable;
                    }
                    if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false)
                        && Rule::Lantern.check(&state)
                        && (Rule::Hammer.check(&state) || Rule::Sword2.check(&state))
                    {
                        if Rule::Cape.check(&state)
                            || Rule::ByrnaCane.check(&state)
                            || Rule::CanBlockLasers.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        false,
                    ) {
                        return Availability::Possible;
                    } else if self.can_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        if Rule::Cape.check(&state)
                            || Rule::ByrnaCane.check(&state)
                            || Rule::CanBlockLasers.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else if self.may_enter(
                        &state,
                        &dungeons,
                        &RandoLogic::MajorGlitches,
                        false,
                        true,
                    ) {
                        return Availability::GlitchPossible;
                    }
                }
            }
        }

        Availability::Unavailable
    }

    pub fn can_hurt_boss(&self, state: &GameState) -> bool {
        return match self {
            DungeonAvailability::DesertPalace => {
                Rule::Sword1.check(&state)
                    || Rule::Hammer.check(&state)
                    || Rule::Bow.check(&state)
                    || Rule::FireRod.check(&state)
                    || Rule::IceRod.check(&state)
                    || Rule::ByrnaCane.check(&state)
                    || Rule::SomariaCane.check(&state)
            }
            DungeonAvailability::ThievesTown => {
                Rule::Sword1.check(&state)
                    || Rule::Hammer.check(&state)
                    || Rule::SomariaCane.check(&state)
                    || Rule::ByrnaCane.check(&state)
            }
            DungeonAvailability::MiseryMire => {
                Rule::Sword1.check(&state) || Rule::Hammer.check(&state) || Rule::Bow.check(&state)
            }
            x => {
                eprintln!("can_hurt_boss not yet implemented for {:?}", x);
                false
            }
        };
    }

    pub fn can_get_chest(
        &self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: &RandoLogic,
    ) -> Availability {
        match self {
            DungeonAvailability::DesertPalace => {
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::Boots.check(&state)
                        && (self.dungeon_from_state(&dungeons).remaining_chests() == 2
                            || (self.can_hurt_boss(&state)
                                && Rule::CanLightTorches.check(&state)
                                && Rule::CanLiftRocks.check(&state)))
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::Boots.check(&state)
                        && (self.dungeon_from_state(&dungeons).remaining_chests() == 2
                            || (self.can_hurt_boss(&state)
                                && Rule::CanLightTorches.check(&state)
                                && Rule::CanLiftRocks.check(&state)))
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::Boots.check(&state)
                        && (self.dungeon_from_state(&dungeons).remaining_chests() == 2
                            || (self.can_hurt_boss(&state)
                                && Rule::CanLightTorches.check(&state)
                                && Rule::CanLiftRocks.check(&state)))
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::EasternPalace => {
                if Rule::Lantern.check(&state) {
                    if Rule::Bow.check(&state) {
                        return Availability::Available;
                    } else if self.dungeon_from_state(&dungeons).remaining_chests() >= 2 {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.dungeon_from_state(&dungeons).remaining_chests() == 3 {
                    return Availability::Available;
                } else {
                    return Availability::Possible;
                }
            }
            DungeonAvailability::GanonsTower => {
                let mut small_keys_needed = 0;
                let mut big_key_needed = 0;
                let mut big_key_guaranteed = false;
                // Hope Room x2
                let mut min_available_chests: u8 = 2;
                let mut max_available_chests = 2;
                // Bob's Torch
                if Rule::Boots.check(&state) {
                    min_available_chests += 1;
                    max_available_chests += 1;
                }
                // DMs Room x4 + Randomizer Room x4 + Firesnake Room
                if Rule::Hammer.check(&state) && Rule::HookShot.check(&state) {
                    min_available_chests += 9;
                    max_available_chests += 9;
                    small_keys_needed = 4;
                }
                // Map Chest
                if Rule::Hammer.check(&state)
                    && (Rule::Boots.check(&state) || Rule::HookShot.check(&state))
                {
                    min_available_chests += 1;
                    max_available_chests += 1;
                }
                // Bob's Chest + Big Key Room x3
                if (Rule::Hammer.check(&state) && Rule::HookShot.check(&state))
                    || (Rule::FireRod.check(&state) && Rule::SomariaCane.check(&state))
                {
                    min_available_chests += 4;
                    max_available_chests += 4;
                    small_keys_needed = max(3, small_keys_needed);
                }
                // Tile Room
                if Rule::SomariaCane.check(&state) {
                    min_available_chests += 1;
                    max_available_chests += 1;
                }
                // Compass Room x4
                if Rule::FireRod.check(&state) && Rule::SomariaCane.check(&state) {
                    min_available_chests += 4;
                    max_available_chests += 4;
                    small_keys_needed = max(4, small_keys_needed);
                }
                // Big Chest
                if Rule::Hammer.check(&state)
                    && Rule::Boots.check(&state)
                    && Rule::HookShot.check(&state)
                    && Rule::SomariaCane.check(&state)
                    && Rule::FireRod.check(&state)
                {
                    min_available_chests += 1;
                    max_available_chests += 1;
                    big_key_needed = 1;
                    big_key_guaranteed = true;
                } // Mini Helmasaur Room x2 + Pre-Moldorm Chest if Rule::Bow.check(&state) && Rule::CanLightTorches.check(&state) { if big_key_guaranteed { min_available_chests += 3; } max_available_chests += 3; small_keys_needed = max(3, small_keys_needed); big_key_needed = 1; }
                  // Moldorm Chest
                if Rule::HookShot.check(&state)
                    && Rule::Bow.check(&state)
                    && Rule::CanLightTorches.check(&state)
                    && (Rule::Hammer.check(&state) || Rule::Sword1.check(&state))
                {
                    if big_key_guaranteed {
                        min_available_chests += 1;
                    }
                    max_available_chests += 1;
                    small_keys_needed = max(4, small_keys_needed);
                    big_key_needed = 1;
                }
                let max_items_available =
                    min(20, max_available_chests - small_keys_needed - big_key_needed);
                // 4 keys + big key + map + compass
                let min_items_available = min_available_chests.checked_sub(7).unwrap_or(0);

                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if this_dungeon.remaining_chests() > (20 - min_items_available) {
                        return Availability::Available;
                    } else if this_dungeon.remaining_chests() > (20 - max_items_available) {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    if this_dungeon.remaining_chests() > (20 - min_items_available) {
                        return Availability::GlitchAvailable;
                    } else if this_dungeon.remaining_chests() > (20 - max_items_available) {
                        return Availability::GlitchPossible;
                    }
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if this_dungeon.remaining_chests() > (20 - min_items_available) {
                        return Availability::Available;
                    } else if this_dungeon.remaining_chests() > (20 - max_items_available) {
                        return Availability::Possible;
                    }
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if this_dungeon.remaining_chests() > (20 - min_items_available) {
                        return Availability::Available;
                    } else if this_dungeon.remaining_chests() > (20 - max_items_available) {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if this_dungeon.remaining_chests() > (20 - min_items_available) {
                        return Availability::GlitchAvailable;
                    } else if this_dungeon.remaining_chests() > (20 - max_items_available) {
                        return Availability::GlitchPossible;
                    }
                }
            }
            DungeonAvailability::IcePalace => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::Hammer.check(&state) && Rule::CanLiftRocks.check(&state) {
                        if Rule::HookShot.check(&state) {
                            return Availability::Available;
                        } else if Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state) {
                            if this_dungeon.remaining_chests() >= 2 {
                                return Availability::Available;
                            } else {
                                return Availability::Possible;
                            }
                        } else {
                            return Availability::Possible;
                        }
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    if Rule::Hammer.check(&state) && Rule::CanLiftRocks.check(&state) {
                        if Rule::HookShot.check(&state) {
                            return Availability::GlitchAvailable;
                        } else {
                            if this_dungeon.remaining_chests() >= 2 {
                                return Availability::GlitchAvailable;
                            } else {
                                return Availability::GlitchPossible;
                            }
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::Hammer.check(&state) && Rule::CanLiftRocks.check(&state) {
                        if Rule::HookShot.check(&state) {
                            return Availability::Available;
                        } else if Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state) {
                            if this_dungeon.remaining_chests() >= 2 {
                                return Availability::Available;
                            } else {
                                return Availability::Possible;
                            }
                        } else {
                            return Availability::Possible;
                        }
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Hammer.check(&state) && Rule::CanLiftRocks.check(&state) {
                        if Rule::HookShot.check(&state) {
                            return Availability::GlitchAvailable;
                        } else {
                            if this_dungeon.remaining_chests() >= 2 {
                                return Availability::GlitchAvailable;
                            } else {
                                return Availability::GlitchPossible;
                            }
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::Hammer.check(&state) && Rule::CanLiftRocks.check(&state) {
                        if Rule::HookShot.check(&state) {
                            return Availability::Available;
                        } else if Rule::ByrnaCane.check(&state) || Rule::Cape.check(&state) {
                            if this_dungeon.remaining_chests() >= 2 {
                                return Availability::Available;
                            } else {
                                return Availability::Possible;
                            }
                        } else {
                            return Availability::Possible;
                        }
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::Hammer.check(&state) && Rule::CanLiftRocks.check(&state) {
                        if Rule::HookShot.check(&state) {
                            return Availability::GlitchAvailable;
                        } else {
                            if this_dungeon.remaining_chests() >= 2 {
                                return Availability::GlitchAvailable;
                            } else {
                                return Availability::GlitchPossible;
                            }
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                }
            }
            DungeonAvailability::MiseryMire => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::CanLightTorches.check(&state) {
                        if this_dungeon.remaining_chests() == 2
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || (Rule::SomariaCane.check(&state) && self.can_hurt_boss(&state)))
                        {
                            return Availability::Available;
                        } else if this_dungeon.remaining_chests() == 1
                            && (Rule::Cape.check(&state) || Rule::ByrnaCane.check(&state))
                            && Rule::SomariaCane.check(&state)
                            && self.can_hurt_boss(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else {
                        return Availability::Possible;
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    return Availability::Possible;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::CanLightTorches.check(&state) {
                        if this_dungeon.remaining_chests() == 2
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || (Rule::SomariaCane.check(&state)
                                    && self.can_hurt_boss(&state)
                                    && Rule::Lantern.check(&state)))
                        {
                            return Availability::Available;
                        } else if this_dungeon.remaining_chests() == 1
                            && (Rule::Cape.check(&state) || Rule::ByrnaCane.check(&state))
                            && Rule::SomariaCane.check(&state)
                            && self.can_hurt_boss(&state)
                            && Rule::Lantern.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else {
                        return Availability::Possible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    return Availability::Possible;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::CanLightTorches.check(&state) {
                        if this_dungeon.remaining_chests() == 2
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || (Rule::SomariaCane.check(&state)
                                    && self.can_hurt_boss(&state)
                                    && Rule::Lantern.check(&state)))
                        {
                            return Availability::GlitchAvailable;
                        } else if this_dungeon.remaining_chests() == 1
                            && (Rule::Cape.check(&state) || Rule::ByrnaCane.check(&state))
                            && Rule::SomariaCane.check(&state)
                            && self.can_hurt_boss(&state)
                            && Rule::Lantern.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchPossible;
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::CanLightTorches.check(&state) {
                        if this_dungeon.remaining_chests() == 2
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || (Rule::SomariaCane.check(&state)
                                    && self.can_hurt_boss(&state)
                                    && Rule::Lantern.check(&state)))
                        {
                            return Availability::Available;
                        } else if this_dungeon.remaining_chests() == 1
                            && (Rule::Cape.check(&state) || Rule::ByrnaCane.check(&state))
                            && Rule::SomariaCane.check(&state)
                            && self.can_hurt_boss(&state)
                            && Rule::Lantern.check(&state)
                        {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else {
                        return Availability::Possible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Possible;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::CanLightTorches.check(&state) {
                        if this_dungeon.remaining_chests() == 2
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || (Rule::SomariaCane.check(&state)
                                    && self.can_hurt_boss(&state)
                                    && Rule::Lantern.check(&state)))
                        {
                            return Availability::GlitchAvailable;
                        } else if this_dungeon.remaining_chests() == 1
                            && (Rule::Cape.check(&state) || Rule::ByrnaCane.check(&state))
                            && Rule::SomariaCane.check(&state)
                            && self.can_hurt_boss(&state)
                            && Rule::Lantern.check(&state)
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    return Availability::GlitchPossible;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::PalaceOfDarkness => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::Bow.check(&state)
                        && (this_dungeon.remaining_chests() >= 2 || Rule::Hammer.check(&state))
                    {
                        if Rule::Lantern.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if Rule::Lantern.check(&state) {
                        return Availability::Possible;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, false) {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::Bow.check(&state)
                        && (this_dungeon.remaining_chests() >= 2 || Rule::Hammer.check(&state))
                    {
                        if Rule::Lantern.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if Rule::Lantern.check(&state) {
                        return Availability::Possible;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Bow.check(&state)
                        && (this_dungeon.remaining_chests() >= 2 || Rule::Hammer.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::Bow.check(&state)
                        && (this_dungeon.remaining_chests() >= 2 || Rule::Hammer.check(&state))
                    {
                        if Rule::Lantern.check(&state) {
                            return Availability::Available;
                        } else {
                            return Availability::Possible;
                        }
                    } else if Rule::Lantern.check(&state) {
                        return Availability::Possible;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::Bow.check(&state)
                        && (this_dungeon.remaining_chests() >= 2 || Rule::Hammer.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::SkullWoods => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::MoonPearl.check(&state)
                        && Rule::FireRod.check(&state)
                        && (Rule::Sword1.check(&state) || this_dungeon.remaining_chests() == 2)
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, false) {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::MoonPearl.check(&state)
                        && Rule::FireRod.check(&state)
                        && (Rule::Sword1.check(&state) || this_dungeon.remaining_chests() == 2)
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::MoonPearl.check(&state)
                        && Rule::FireRod.check(&state)
                        && (Rule::Sword1.check(&state) || this_dungeon.remaining_chests() == 2)
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::MoonPearl.check(&state)
                        && Rule::FireRod.check(&state)
                        && (Rule::Sword1.check(&state) || this_dungeon.remaining_chests() == 2)
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::MoonPearl.check(&state)
                        && Rule::FireRod.check(&state)
                        && (Rule::Sword1.check(&state) || this_dungeon.remaining_chests() == 2)
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::SwampPalace => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::Hammer.check(&state) {
                        if Rule::HookShot.check(&state) || this_dungeon.remaining_chests() >= 5 {
                            return Availability::Available;
                        } else if this_dungeon.remaining_chests() >= 3 {
                            return Availability::Possible;
                        }
                    } else if this_dungeon.remaining_chests() == 6 {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, false) {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::Hammer.check(&state) {
                        if Rule::HookShot.check(&state) || this_dungeon.remaining_chests() >= 5 {
                            return Availability::Available;
                        } else if this_dungeon.remaining_chests() >= 3 {
                            return Availability::Possible;
                        }
                    } else if this_dungeon.remaining_chests() == 6 {
                        return Availability::Possible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Hammer.check(&state) {
                        if Rule::HookShot.check(&state) || this_dungeon.remaining_chests() >= 5 {
                            return Availability::GlitchAvailable;
                        } else if this_dungeon.remaining_chests() >= 3 {
                            return Availability::GlitchPossible;
                        }
                    } else if this_dungeon.remaining_chests() == 6 {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::Flippers.check(&state)
                        && (Rule::Hammer.check(&state)
                            || DungeonAvailability::MiseryMire.can_enter(
                                &state,
                                &dungeons,
                                &RandoLogic::MajorGlitches,
                                false,
                                false,
                            ))
                    {
                        if Rule::HookShot.check(&state) {
                            return Availability::Available;
                        } else if this_dungeon.remaining_chests() >= 3 {
                            return Availability::Possible;
                        }
                    } else if this_dungeon.remaining_chests() >= 5 {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::Flippers.check(&state)
                        && (Rule::Hammer.check(&state)
                            || DungeonAvailability::MiseryMire.can_enter(
                                &state,
                                &dungeons,
                                &RandoLogic::MajorGlitches,
                                false,
                                true,
                            ))
                    {
                        if Rule::HookShot.check(&state) {
                            return Availability::GlitchAvailable;
                        } else if this_dungeon.remaining_chests() >= 3 {
                            return Availability::GlitchPossible;
                        }
                    } else if this_dungeon.remaining_chests() >= 5 {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::ThievesTown => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::Hammer.check(&state)
                        || this_dungeon.remaining_chests() >= 3
                        || (self.can_hurt_boss(&state) && this_dungeon.remaining_chests() >= 2)
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, false) {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, true, true) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::Hammer.check(&state)
                        || this_dungeon.remaining_chests() >= 3
                        || (self.can_hurt_boss(&state) && this_dungeon.remaining_chests() >= 2)
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::Hammer.check(&state)
                        || this_dungeon.remaining_chests() >= 3
                        || (self.can_hurt_boss(&state) && this_dungeon.remaining_chests() >= 2)
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    false,
                ) {
                    return Availability::Agahnim;
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    true,
                    true,
                ) {
                    return Availability::GlitchAgahnim;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::Hammer.check(&state)
                        || this_dungeon.remaining_chests() >= 3
                        || (self.can_hurt_boss(&state) && this_dungeon.remaining_chests() >= 2)
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::Hammer.check(&state)
                        || this_dungeon.remaining_chests() >= 3
                        || (self.can_hurt_boss(&state) && this_dungeon.remaining_chests() >= 2)
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::TowerOfHera => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::CanLightTorches.check(&state)
                        && (this_dungeon.remaining_chests() == 2
                            || Rule::Sword1.check(&state)
                            || Rule::Hammer.check(&state))
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    if Rule::CanLightTorches.check(&state)
                        && (this_dungeon.remaining_chests() == 2
                            || Rule::Sword1.check(&state)
                            || Rule::Hammer.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::CanLightTorches.check(&state)
                        && (this_dungeon.remaining_chests() == 2
                            || Rule::Sword1.check(&state)
                            || Rule::Hammer.check(&state))
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::CanLightTorches.check(&state)
                        && (this_dungeon.remaining_chests() == 2
                            || Rule::Sword1.check(&state)
                            || Rule::Hammer.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if (Rule::CanLightTorches.check(&state)
                        || DungeonAvailability::MiseryMire.can_enter(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            false,
                            false,
                        ))
                        && (this_dungeon.remaining_chests() == 2
                            || Rule::Sword1.check(&state)
                            || Rule::Hammer.check(&state))
                    {
                        return Availability::Available;
                    } else {
                        return Availability::Possible;
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if (Rule::CanLightTorches.check(&state)
                        || DungeonAvailability::MiseryMire.can_enter(
                            &state,
                            &dungeons,
                            &RandoLogic::MajorGlitches,
                            false,
                            false,
                        ))
                        && (this_dungeon.remaining_chests() == 2
                            || Rule::Sword1.check(&state)
                            || Rule::Hammer.check(&state))
                    {
                        return Availability::GlitchAvailable;
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    return Availability::Possible;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    return Availability::GlitchPossible;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, false)
                {
                    return Availability::Agahnim;
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, true, true)
                {
                    return Availability::GlitchAgahnim;
                }
            }
            DungeonAvailability::TurtleRock => {
                let this_dungeon = self.dungeon_from_state(&dungeons);
                if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::FireRod.check(&state) {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            if this_dungeon.remaining_chests() >= 2
                                || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                    == Availability::Available
                            {
                                return Availability::Available;
                            } else {
                                return Availability::Possible;
                            }
                        } else if this_dungeon.remaining_chests() >= 2 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 4 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::Glitchless, false, false) {
                    if Rule::FireRod.check(&state) {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 2 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 4 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    if Rule::FireRod.check(&state) {
                        if this_dungeon.remaining_chests() >= 2
                            || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                == Availability::Available
                            || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                == Availability::GlitchAvailable
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::Glitchless, false, true) {
                    return Availability::GlitchPossible;
                }

                if *logic < RandoLogic::OverWorldGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::OverWorldGlitches, false, false) {
                    if Rule::FireRod.check(&state) {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            if this_dungeon.remaining_chests() >= 2
                                // TODO: Should this actually be checking if the dungeon is beatable under OverWorld Glitches rules?
                                || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless) == Availability::Available
                            {
                                return Availability::Available;
                            } else {
                                return Availability::Possible;
                            }
                        } else if this_dungeon.remaining_chests() >= 2 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 4 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    false,
                ) {
                    if Rule::FireRod.check(&state) {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 2 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 4 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.can_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    if Rule::FireRod.check(&state) {
                        if this_dungeon.remaining_chests() >= 2
                            // TODO: Should these be checking is_beatable with OverWorldGlitches logic?
                            || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                == Availability::Available
                            || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                == Availability::GlitchAvailable
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::OverWorldGlitches,
                    false,
                    true,
                ) {
                    return Availability::GlitchPossible;
                }

                if *logic < RandoLogic::MajorGlitches {
                    return Availability::Unavailable;
                }
                if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, false) {
                    if Rule::FireRod.check(&state) {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            if this_dungeon.remaining_chests() >= 2
                                // TODO: Should this be using MajorGlitches logic to check is_beatable?
                                || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                    == Availability::Available
                            {
                                return Availability::Available;
                            } else {
                                return Availability::Possible;
                            }
                        } else if this_dungeon.remaining_chests() >= 2 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 4 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.may_enter(
                    &state,
                    &dungeons,
                    &RandoLogic::MajorGlitches,
                    false,
                    false,
                ) {
                    if Rule::FireRod.check(&state) {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 2 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        if Rule::Lantern.check(&state)
                            && (Rule::Cape.check(&state)
                                || Rule::ByrnaCane.check(&state)
                                || Rule::CanBlockLasers.check(&state))
                        {
                            return Availability::Possible;
                        } else if this_dungeon.remaining_chests() >= 4 {
                            return Availability::Possible;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    }
                } else if self.can_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    if Rule::FireRod.check(&state) {
                        if this_dungeon.remaining_chests() >= 2
                            // TODO: Should this be using MajorGlitches logic when checking is_beatable?
                            || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                == Availability::Available
                            || self.is_beatable(&state, &dungeons, &RandoLogic::Glitchless)
                                == Availability::GlitchAvailable
                        {
                            return Availability::GlitchAvailable;
                        } else {
                            return Availability::GlitchPossible;
                        }
                    } else {
                        return Availability::GlitchPossible;
                    }
                } else if self.may_enter(&state, &dungeons, &RandoLogic::MajorGlitches, false, true)
                {
                    return Availability::GlitchPossible;
                }
            }
        }

        Availability::Unavailable
    }

    fn has_medallion(&self, state: &GameState, dungeons: &DungeonState) -> bool {
        if *self == DungeonAvailability::MiseryMire || *self == DungeonAvailability::TurtleRock {
            let this_dungeon = self.dungeon_from_state(&dungeons);
            (match this_dungeon.medallion {
                Medallion::Bombos => Rule::BombosMedallion.check(&state),
                Medallion::Ether => Rule::EtherMedallion.check(&state),
                Medallion::Quake => Rule::QuakeMedallion.check(&state),
                Medallion::Unknown => false,
            }) || (Rule::BombosMedallion.check(&state)
                && Rule::EtherMedallion.check(&state)
                && Rule::QuakeMedallion.check(&state))
        } else {
            eprintln!("{:?} is not accessed via medallion", self);
            false
        }
    }

    fn may_have_medallion(&self, state: &GameState, dungeons: &DungeonState) -> bool {
        if *self == DungeonAvailability::MiseryMire || *self == DungeonAvailability::TurtleRock {
            let this_dungeon = self.dungeon_from_state(&dungeons);
            !(match this_dungeon.medallion {
                Medallion::Bombos => !Rule::BombosMedallion.check(&state),
                Medallion::Ether => !Rule::EtherMedallion.check(&state),
                Medallion::Quake => !Rule::QuakeMedallion.check(&state),
                Medallion::Unknown => true,
            }) || (!Rule::BombosMedallion.check(&state)
                && !Rule::EtherMedallion.check(&state)
                && !Rule::QuakeMedallion.check(&state))
        } else {
            eprintln!("{:?} is not accessed via medallion", self);
            false
        }
    }

    fn lower(
        &self,
        state: &GameState,
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        *logic == RandoLogic::MajorGlitches
            && Rule::CanEnterWestDeathMountain.check_with_options(
                &state,
                &RandoLogic::MajorGlitches,
                agahnim_check,
                allow_out_of_logic_glitches,
            )
            && (Rule::MoonPearl.check(&state)
                || (Rule::Bottle.check(&state) && Rule::Boots.check(&state)))
            && Rule::Mirror.check(&state)
    }

    fn middle(
        &self,
        state: &GameState,
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match *logic {
            RandoLogic::Glitchless => false,
            RandoLogic::OverWorldGlitches => {
                (Rule::Mirror.check(&state)
                    || (Rule::MoonPearl.check(&state) && Rule::CanSpinSpeed.check(&state)))
                    && (Rule::Boots.check(&state)
                        || Rule::SomariaCane.check(&state)
                        || Rule::HookShot.check(&state))
                    && Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::OverWorldGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
            RandoLogic::MajorGlitches => {
                (Rule::Mirror.check(&state)
                    || (Rule::GlitchedLinkInDarkWorld.check(&state)
                        && Rule::CanSpinSpeed.check(&state)))
                    && (Rule::Boots.check(&state)
                        || Rule::SomariaCane.check(&state)
                        || Rule::HookShot.check(&state))
                    && (Rule::CanEnterEastDarkWorldDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::MajorGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    ))
            }
        }
    }

    fn upper_can(
        &self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match *logic {
            RandoLogic::Glitchless => {
                DungeonAvailability::TurtleRock.has_medallion(&state, &dungeons)
                    && Rule::Sword1.check(&state)
                    && Rule::MoonPearl.check(&state)
                    && Rule::SomariaCane.check(&state)
                    && Rule::Hammer.check(&state)
                    && Rule::CanLiftRocks.check(&state)
                    && Rule::CanEnterEastDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::Glitchless,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
            RandoLogic::OverWorldGlitches => {
                DungeonAvailability::TurtleRock.has_medallion(&state, &dungeons)
                    && Rule::Sword1.check(&state)
                    && Rule::MoonPearl.check(&state)
                    && Rule::SomariaCane.check(&state)
                    && Rule::Hammer.check(&state)
                    && (Rule::CanLiftDarkRocks.check(&state) || Rule::Boots.check(&state))
                    && Rule::CanEnterEastDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::OverWorldGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
            RandoLogic::MajorGlitches => {
                DungeonAvailability::TurtleRock.has_medallion(&state, &dungeons)
                    && Rule::Sword1.check(&state)
                    && (Rule::MoonPearl.check(&state)
                        || (Rule::Bottle.check(&state) && Rule::Boots.check(&state)))
                    && Rule::SomariaCane.check(&state)
                    && Rule::Hammer.check(&state)
                    && (Rule::CanLiftDarkRocks.check(&state) || Rule::Boots.check(&state))
                    && Rule::CanEnterEastDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::MajorGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
        }
    }

    fn upper_may(
        &self,
        state: &GameState,
        dungeons: &DungeonState,
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match *logic {
            RandoLogic::Glitchless => {
                DungeonAvailability::TurtleRock.may_have_medallion(&state, &dungeons)
                    && Rule::Sword1.check(&state)
                    && Rule::MoonPearl.check(&state)
                    && Rule::SomariaCane.check(&state)
                    && Rule::Hammer.check(&state)
                    && Rule::CanLiftDarkRocks.check(&state)
                    && Rule::CanEnterEastDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::Glitchless,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
            RandoLogic::OverWorldGlitches => {
                DungeonAvailability::TurtleRock.may_have_medallion(&state, &dungeons)
                    && Rule::Sword1.check(&state)
                    && Rule::MoonPearl.check(&state)
                    && Rule::SomariaCane.check(&state)
                    && Rule::Hammer.check(&state)
                    && (Rule::CanLiftDarkRocks.check(&state) || Rule::Boots.check(&state))
                    && Rule::CanEnterEastDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::OverWorldGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
            RandoLogic::MajorGlitches => {
                DungeonAvailability::TurtleRock.may_have_medallion(&state, &dungeons)
                    && Rule::Sword1.check(&state)
                    && (Rule::MoonPearl.check(&state)
                        || (Rule::Bottle.check(&state) && Rule::Boots.check(&state)))
                    && Rule::SomariaCane.check(&state)
                    && Rule::Hammer.check(&state)
                    && (Rule::CanLiftDarkRocks.check(&state) || Rule::Boots.check(&state))
                    && Rule::CanEnterEastDeathMountain.check_with_options(
                        &state,
                        &RandoLogic::MajorGlitches,
                        agahnim_check,
                        allow_out_of_logic_glitches,
                    )
            }
        }
    }

    fn dungeon_from_state(&self, dungeons: &DungeonState) -> Dungeon {
        let dungeon_code = match self {
            DungeonAvailability::DesertPalace => "DP",
            DungeonAvailability::EasternPalace => "EP",
            DungeonAvailability::GanonsTower => "GT",
            DungeonAvailability::IcePalace => "IP",
            DungeonAvailability::MiseryMire => "MM",
            DungeonAvailability::PalaceOfDarkness => "PoD",
            DungeonAvailability::SkullWoods => "SW",
            DungeonAvailability::SwampPalace => "SP",
            DungeonAvailability::ThievesTown => "TT",
            DungeonAvailability::TowerOfHera => "ToH",
            DungeonAvailability::TurtleRock => "TR",
        };

        dungeons.get(dungeon_code).unwrap()
    }
}

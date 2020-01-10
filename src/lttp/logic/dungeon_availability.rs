use crate::lttp::{
    logic::{
        Availability,
        RandoLogic,
        Rule,
    },
    GameState,
};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
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
        logic: &RandoLogic,
        agahnim_check: bool,
        allow_out_of_logic_glitches: bool,
    ) -> bool {
        match self {
            DungeonAvailability::DesertPalace => false,
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
                                &RandoLogic::MajorGlitches,
                                agahnim_check,
                                allow_out_of_logic_glitches,
                            )
                    }
                };
            }

            x => todo!("can_enter not yet implemented for {:?}", x),
        }
    }

    pub fn may_enter(
        &self,
        _state: &GameState,
        _logic: &RandoLogic,
        _agahnim_check: bool,
        _allow_out_of_logic_glitches: bool,
    ) -> bool {
        match self {
            x => todo!("may_enter not yet implemented for {:?}", x),
        }
    }

    pub fn is_beatable(&self, _state: &GameState, _logic: &RandoLogic) -> Availability {
        match self {
            x => todo!("is_beatable not yet implemented for {:?}", x),
        }
    }

    pub fn can_hurt_boss(&self, _state: &GameState) -> bool {
        match self {
            x => todo!("can_hurt_boss not yet implemented for {:?}", x),
        }
    }

    pub fn can_get_chest(&self, _state: &GameState, _logic: &RandoLogic) -> Availability {
        match self {
            x => todo!("can_get_chest not yet implemented for {:?}", x),
        }
    }
}

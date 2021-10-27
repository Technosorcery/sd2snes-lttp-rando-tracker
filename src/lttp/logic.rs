mod availability;
mod dungeon_availability;
mod location_availability;
mod rule;

pub use crate::lttp::logic::{
    availability::Availability,
    dungeon_availability::DungeonAvailability,
    location_availability::LocationAvailability,
    rule::Rule,
};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RandoLogic {
    Glitchless,
    OverWorldGlitches,
    MajorGlitches,
}

impl Default for RandoLogic {
    fn default() -> RandoLogic { RandoLogic::Glitchless }
}

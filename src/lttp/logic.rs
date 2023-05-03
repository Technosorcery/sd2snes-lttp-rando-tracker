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
use serde::{
    Deserialize,
    Serialize,
};
use ts_rs::TS;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Deserialize, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/RandoLogic.ts")]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum RandoLogic {
    #[default]
    Glitchless,
    OverWorldGlitches,
    MajorGlitches,
}

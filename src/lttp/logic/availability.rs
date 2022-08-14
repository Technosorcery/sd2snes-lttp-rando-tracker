use serde::{
    Deserialize,
    Serialize,
};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "ui/src/server_types/Availability.ts")]
#[serde(rename_all = "camelCase")]
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
    fn default() -> Availability { Availability::Unavailable }
}

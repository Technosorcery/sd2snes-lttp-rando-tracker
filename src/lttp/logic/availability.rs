use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
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

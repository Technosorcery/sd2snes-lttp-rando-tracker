use serde_derive::{
    Deserialize,
    Serialize,
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
    fn default() -> Availability { Availability::Unavailable }
}

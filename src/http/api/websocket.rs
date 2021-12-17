use crate::lttp::{
    Dungeon,
    GameState,
    Location,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ServerMessage {
    Item(GameState),
    Dungeon(Vec<Dungeon>),
    Location(Vec<Location>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ClientMessage {
    None,
}

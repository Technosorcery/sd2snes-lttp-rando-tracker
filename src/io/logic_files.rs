use crate::lttp::{
    Dungeon,
    Location,
};

use anyhow::{
    Context,
    Result,
};
use once_cell::sync;
use std::collections::HashMap;

#[iftree::include_file_tree("paths = '/logic/**'")]
#[derive(Debug)]
pub struct LogicAsset {
    relative_path: &'static str,
    contents_str:  &'static str,
}

static LOGIC_ASSET_MAP: sync::Lazy<HashMap<&str, &LogicAsset>> =
    sync::Lazy::new(|| ASSETS.iter().map(|asset| (asset.relative_path, asset)).collect());

pub fn base_dungeon_data() -> Result<Vec<Dungeon>> {
    let dungeon_file = LOGIC_ASSET_MAP
        .get("logic/dungeon_locations.yaml")
        .context("Unable to load base dungeon data")?;

    serde_yaml::from_str::<Vec<Dungeon>>(dungeon_file.contents_str)
        .context("Unable to parse base dungeon data")
}

pub fn base_location_data() -> Result<Vec<Location>> {
    let location_file = LOGIC_ASSET_MAP
        .get("logic/poi_locations.yaml")
        .context("Unable to load base location data")?;

    serde_yaml::from_str::<Vec<Location>>(location_file.contents_str)
        .context("Unable to parse base location data")
}

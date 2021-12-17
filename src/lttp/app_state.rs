use crate::lttp::{
    DungeonState,
    DungeonUpdate,
    GameState,
    LocationState,
    LocationUpdate,
    ServerConfig,
};

use anyhow::{
    bail,
    Result,
};
use std::sync::RwLock;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct AppState {
    pub dungeon_state:  RwLock<DungeonState>,
    pub game_state:     RwLock<GameState>,
    pub location_state: RwLock<LocationState>,
    pub server_config:  RwLock<ServerConfig>,
    pub update_sender:  broadcast::Sender<Update>,
}

#[derive(Debug, Copy, Clone)]
pub enum Update {
    Dungeons,
    Items,
    Locations,
}

impl AppState {
    #[tracing::instrument(skip(self), err)]
    pub fn update_availabilities(&self) -> Result<()> {
        tracing::trace!("Getting ServerConfig");
        let server_config = match self.server_config.read() {
            Ok(sc) => sc.clone(),
            Err(e) => bail!("Failed to get server config to update available locations: {:?}", e),
        };
        tracing::trace!("Getting GameState");
        let game_state = match self.game_state.read() {
            Ok(gs) => gs.clone(),
            Err(e) => bail!("Failed to get game state to update available locations: {:?}", e),
        };
        tracing::trace!("Getting DungeonState");
        let mut dungeon_state = match self.dungeon_state.write() {
            Ok(ds) => ds,
            Err(e) => bail!("Failed to get dungeon state to update available locations: {:?}", e),
        };
        tracing::trace!("Getting LocationState");
        let mut location_state = match self.location_state.write() {
            Ok(ls) => ls,
            Err(e) => bail!("Failed to get location state to update available locations: {:?}", e),
        };

        tracing::trace!("Updating LocationState");
        location_state.update_availability(game_state, &dungeon_state.clone(), server_config.logic);

        tracing::trace!("Updating DungeonState");
        dungeon_state.update_availability(game_state, server_config.logic);

        tracing::trace!("Sending update broadcasts");
        let update_sender = self.update_sender.clone();
        // Can ignore the result, since it's possible that nothing is listening at the moment.
        let _ = update_sender.send(Update::Items);
        let _ = update_sender.send(Update::Locations);
        let _ = update_sender.send(Update::Dungeons);

        Ok(())
    }

    pub fn set_dungeon_state(&self, dungeon: &str, state: DungeonUpdate) -> Result<()> {
        match self.dungeon_state.write() {
            Ok(mut ds) => {
                ds.update(dungeon, state);
                let server_config = match self.server_config.read() {
                    Ok(sc) => sc.clone(),
                    Err(e) => {
                        bail!("Failed to get server config to update dungeon availibility: {:?}", e)
                    }
                };
                let game_state = match self.game_state.read() {
                    Ok(gs) => gs.clone(),
                    Err(e) => {
                        bail!("Failed to get game state to update dungeon availibility: {:?}", e)
                    }
                };
                ds.update_availability(game_state, server_config.logic);

                // We can ignore the result as we don't really care if nobody is listening for updates at the moment.
                let _ = self.update_sender.clone().send(Update::Dungeons);
            }
            Err(e) => bail!("Unable to update dungeon state: {:?}", e),
        }

        Ok(())
    }

    pub fn set_location_state(&self, location: &str, state: LocationUpdate) -> Result<()> {
        match self.location_state.write() {
            Ok(mut ls) => {
                ls.update(location, state);

                // We can ignore the result as we don't really care if nobody is listening for updates at the moment.
                let _ = self.update_sender.clone().send(Update::Locations);
            }
            Err(e) => bail!("Unable to update location state: {:?}", e),
        }

        Ok(())
    }
}

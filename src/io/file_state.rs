use crate::lttp::{
    AppState,
    GameState,
};

use std::{
    fs::File,
    io::Read,
    sync::Arc,
    time::Instant,
};
use tracing::{
    debug,
    info,
    warn,
};

#[tracing::instrument(skip(app_state))]
pub fn poll_status(app_state: Arc<AppState>, source: &str) {
    if source.is_empty() {
        return;
    }

    let loop_start = Instant::now();
    debug!("Starting file update poll cycle");

    let mut f = match File::open(source) {
        Ok(f) => f,
        Err(e) => {
            warn!("Unable to open state file {:?}: {}", source, e);
            return;
        }
    };
    let mut state_json = String::new();
    if let Err(e) = f.read_to_string(&mut state_json) {
        warn!("Unable to read state file {:?}: {}", source, e);
        return;
    };

    let prev_game_state = match app_state.game_state.read() {
        Ok(pgs) => pgs.clone(),
        Err(e) => {
            warn!("Unable to get current game state: {}", e);
            return;
        }
    };
    match serde_json::from_str::<GameState>(&state_json) {
        Ok(new_gs) => {
            let should_broadcast_update = new_gs != prev_game_state;

            match app_state.game_state.write() {
                Ok(mut gs) => *gs = new_gs,
                Err(e) => {
                    warn!("Unable to lock game state for update: {}", e);
                    return;
                }
            }
            if should_broadcast_update {
                info!("Updating location & dungeon availabilities");
                if let Err(e) = app_state.update_availabilities() {
                    warn!("Unable to update dungeon & location availabilities: {}", e);
                    return;
                };
            }
        }
        Err(e) => {
            warn!("Unable to parse game state file: {}", e);
            return;
        }
    }

    debug!("File update poll cycle completed in: {:?}", loop_start.elapsed());
}

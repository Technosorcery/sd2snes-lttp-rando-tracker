use crate::lttp::{
    AppState,
    GameState,
};

use anyhow::{
    anyhow,
    Context,
    Result,
};
use std::{
    convert::TryFrom,
    sync::Arc,
};
use tracing::{
    debug,
    info,
};

#[tracing::instrument(skip(app_state), err)]
pub async fn poll_status(app_state: Arc<AppState>, device: &str) -> Result<()> {
    if device.is_empty() {
        return Ok(());
    }

    let mut client = qusb2snes_client::Client::new().await?;
    debug!("Attaching to {}", device);
    client.attach(device).await?;

    let region_start = qusb2snes_client::offsets::WRAM + 0x0000_F340;
    let region_length = 0x0200;
    let snes_state = client.get_address(region_start, region_length).await?;
    let prev_game_state = match app_state.game_state.read() {
        Ok(pgs) => pgs.clone(),
        Err(e) => return Err(anyhow!("Unable to get current game state: {}", e)),
    };

    let new_game_state =
        GameState::try_from(snes_state).context("Unable to parse game state from WRAM")?;
    let should_broadcast_update = new_game_state != prev_game_state;

    match app_state.game_state.write() {
        Ok(mut gs) => *gs = new_game_state,
        Err(e) => return Err(anyhow!("Unable to lock game state for update: {}", e)),
    }

    if should_broadcast_update {
        info!("Updating location & dungeon availabilities");
        app_state.update_availabilities()?;
    };

    Ok(())
}

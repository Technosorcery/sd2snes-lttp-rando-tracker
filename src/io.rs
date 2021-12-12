pub mod file_state;
pub mod logic_files;
pub mod snes_state;

use crate::lttp::{
    app_config::DataSource,
    AppState,
};

use std::{
    sync::Arc,
    thread,
    time::{
        Duration,
        Instant,
    },
};
use tracing::{
    debug,
    error,
};

#[tracing::instrument(skip(app_state))]
pub async fn game_state_poller(app_state: Arc<AppState>) {
    loop {
        let loop_start = Instant::now();
        debug!("Starting file update poll cycle");

        let sleep_duration = match app_state.app_config.read() {
            Ok(ac) => ac.data_poll_rate,
            Err(e) => {
                error!("Unable to get data polling rate from app config: {:?}", e);
                continue;
            }
        };
        let data_poll_source = match app_state.app_config.read() {
            Ok(ac) => ac.data_source.clone(),
            Err(e) => {
                error!("Unable to get data polling source from app config: {:?}", e);
                continue;
            }
        };

        match data_poll_source {
            DataSource::LocalFile(file_name) => {
                file_state::poll_status(app_state.clone(), &file_name);
            }
            DataSource::Qusb2snes(config) => {
                snes_state::poll_status(app_state.clone(), config.clone()).await;
            }
        }

        debug!("File update poll cycle completed in: {:?}", loop_start.elapsed());
        thread::sleep(Duration::from_millis(sleep_duration));
    }
}

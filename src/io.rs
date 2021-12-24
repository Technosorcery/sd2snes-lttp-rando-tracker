pub mod file_state;
pub mod logic_files;
pub mod snes_state;

use crate::lttp::{
    server_config::DataSourceType,
    AppState,
};

use qusb2snes_client::results;
use std::{
    sync::Arc,
    time::{
        Duration,
        Instant,
    },
};
use tokio::time::{
    self,
    sleep,
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

        let server_config = match app_state.server_config.read() {
            Ok(sc) => sc.clone(),
            Err(e) => {
                error!("Unable to get server config for game state polling: {:?}", e);
                continue;
            }
        };

        match server_config.source_type {
            DataSourceType::LocalFile => {
                file_state::poll_status(app_state.clone(), &server_config.data_source);
            }
            DataSourceType::QUsb2snes => {
                if let Err(e) =
                    snes_state::poll_status(app_state.clone(), &server_config.data_source).await
                {
                    error!("Problem fetching SNES state: {:?}", e);
                };
            }
        }

        debug!("File update poll cycle completed in: {:?}", loop_start.elapsed());
        sleep(Duration::from_millis(server_config.data_poll_rate.into())).await;
    }
}

#[tracing::instrument(skip(app_state))]
pub async fn device_list_poller(app_state: Arc<AppState>) {
    let mut timer = time::interval(time::Duration::from_millis(1_000));
    timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        timer.tick().await;

        let mut client = match qusb2snes_client::Client::new().await {
            Ok(c) => c,
            Err(e) => {
                debug!("Error connecting to QUsb2snes server: {:?}", e);
                continue;
            }
        };

        if let Ok(results::Result {
            results: results::ResultData::Text(dev_list),
        }) = client.device_list().await
        {
            let mut server_config = match app_state.server_config.write() {
                Ok(sc) => sc,
                Err(e) => {
                    error!("Unable to get server config to update device list: {:?}", e);
                    continue;
                }
            };
            server_config.qusb_devices = dev_list;
        } else {
            debug!("Unable to get device list from QUsb2snes server.");
            continue;
        };
    }
}

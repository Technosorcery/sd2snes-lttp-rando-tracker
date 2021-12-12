use crate::lttp::logic::RandoLogic;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Polling rate of the selected data source in milliseconds.
    pub data_poll_rate: u64,
    pub data_source:    DataSource,
    pub logic:          RandoLogic,
}

#[derive(Debug, Clone, Serialize)]
pub enum DataSource {
    LocalFile(String),
    Qusb2snes(Qusb2snesConfig),
}

impl Default for DataSource {
    fn default() -> DataSource { DataSource::LocalFile("example-data.json".to_string()) }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Qusb2snesConfig {
    pub available_devices: Vec<String>,
    pub selected_device:   String,
}

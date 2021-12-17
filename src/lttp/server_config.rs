use crate::lttp::logic::RandoLogic;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/ServerConfig.ts")]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    /// Polling rate of the selected data source in milliseconds.
    pub data_poll_rate: u32,
    pub data_source:    DataSource,
    pub logic:          RandoLogic,
    pub api_port:       u16,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/DataSource.ts")]
#[serde(tag = "type")]
pub enum DataSource {
    LocalFile(LocalFileConfig),
    Qusb2snes(Qusb2snesConfig),
}

impl Default for DataSource {
    fn default() -> DataSource { DataSource::LocalFile(LocalFileConfig { source: "example-data.json".to_string() }) }
}

#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/Qusb2snesConfig.ts")]
pub struct Qusb2snesConfig {
    pub available_devices: Vec<String>,
    pub selected_device:   String,
}

#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/LocalFileConfig.ts")]
pub struct LocalFileConfig {
    pub source: String,
}

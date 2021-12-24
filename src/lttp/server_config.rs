use crate::lttp::logic::RandoLogic;
use serde::{
    Deserialize,
    Serialize,
};
use ts_rs::TS;

#[derive(Debug, Clone, Default, PartialEq, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/ServerConfig.ts")]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    /// Polling rate of the selected data source in milliseconds.
    pub api_port:       u16,
    pub data_poll_rate: u32,
    pub data_source:    String,
    pub logic:          RandoLogic,
    pub qusb_devices:   Vec<String>,
    pub source_type:    DataSourceType,
}

impl ServerConfig {
    pub fn update(&mut self, update: ServerConfigUpdate) {
        if let Some(new_poll_rate) = update.data_poll_rate {
            self.data_poll_rate = new_poll_rate;
        }
        if let Some(new_source_type) = update.source_type {
            self.source_type = new_source_type;
        }
        if let Some(new_source) = update.data_source {
            self.data_source = new_source;
        }
        if let Some(new_logic) = update.logic {
            self.logic = new_logic;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, TS)]
#[ts(export, export_to = "ui/src/server_types/DataSourceType.ts")]
pub enum DataSourceType {
    LocalFile,
    QUsb2snes,
}

impl Default for DataSourceType {
    fn default() -> DataSourceType { DataSourceType::QUsb2snes }
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "ui/src/server_types/ServerConfigUpdate.ts")]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ServerConfigUpdate {
    #[serde(default)]
    pub data_poll_rate: Option<u32>,
    #[serde(default)]
    pub data_source:    Option<String>,
    #[serde(default)]
    pub logic:          Option<RandoLogic>,
    #[serde(default)]
    pub source_type:    Option<DataSourceType>,
}

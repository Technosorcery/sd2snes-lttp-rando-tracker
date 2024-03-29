#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

mod http;
mod io;
mod lttp;

use crate::{
    io::logic_files,
    lttp::{
        AppState,
        DungeonState,
        GameState,
        LocationState,
        ServerConfig,
    },
};

use anyhow::{
    bail,
    Result,
};
use clap::{
    crate_authors,
    crate_description,
    value_parser,
    Arg,
    ArgAction,
    ArgGroup,
    Command,
};
use std::{
    env,
    sync::{
        Arc,
        RwLock,
    },
};
use tokio::sync::broadcast;
use tracing::Level;
use tracing_subscriber::{
    filter::EnvFilter,
    prelude::*,
};

#[allow(clippy::too_many_lines)]
#[tokio::main]
async fn main() -> Result<()> {
    let version_string = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or(env!("CARGO_PKG_VERSION"));

    let matches = Command::new("SD2SNES LttP Randomizer Tracker")
        .version(version_string)
        .author(crate_authors!(", "))
        .about(crate_description!())
        .arg(
            Arg::new("device")
                .help("The SD2SNES device to use in the Qusb2snes server.")
                .short('d')
                .long("device")
                .num_args(1),
        )
        .arg(
            Arg::new("file")
                .help("JSON file to read game state from")
                .short('f')
                .long("file")
                .num_args(1),
        )
        .group(ArgGroup::new("source").args(["device", "file"]))
        .arg(
            Arg::new("verbose")
                .help("Enable more verbose output (can be provided multiple times to increase verbosity)")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count),
        )
        .arg(
            Arg::new("port")
                .help("Port number to run the web UI server on")
                .short('p')
                .long("port")
                .num_args(1)
                .default_value("8000")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("server-address")
                .help("Address to bind the UI & websocket server to")
                .short('a')
                .long("address")
                .num_args(1)
                .default_value("0.0.0.0")
                .value_parser(value_parser!(std::net::IpAddr)),
        )
        .get_matches();

    let log_level = match matches.get_one::<u8>("verbose").copied() {
        None | Some(0) => Level::ERROR,
        Some(1) => Level::WARN,
        Some(2) => Level::INFO,
        Some(3) => Level::DEBUG,
        Some(_) => Level::TRACE,
    };

    let server_port: u16 = match matches.try_get_one("port") {
        Ok(p) => {
            match p {
                Some(i) => *i,
                None => panic!("No port number provided."),
            }
        }
        Err(e) => panic!("Invalid port number: {e:?}"),
    };
    let server_address: std::net::IpAddr = match matches.try_get_one("server-address") {
        Ok(a) => {
            match a {
                Some(a) => *a,
                None => panic!("No address provided."),
            }
        }
        Err(e) => panic!("Invalid address: {e:?}"),
    };

    if std::env::var_os("RUST_LOG").is_none() {
        use std::ops::Add;
        let mut rust_log_setting = "sd2snes_lttp_rando_tracker=info,tower_http=debug".to_owned();
        if log_level == Level::TRACE {
            rust_log_setting = rust_log_setting.add(",tokio=trace,runtime=trace");
        };
        std::env::set_var("RUST_LOG", rust_log_setting);
    }
    let console_layer = console_subscriber::spawn();
    let filter = EnvFilter::from_default_env().add_directive(log_level.into());
    if let Err(e) = tracing_subscriber::registry()
        .with(console_layer)
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .try_init()
    {
        bail!("Unable to initialize logging: {:?}", e);
    }

    let (source_type, data_source) = if let Some(file_name) = matches.get_one::<String>("file") {
        (lttp::server_config::DataSourceType::LocalFile, file_name.to_string())
    } else if let Some(device_name) = matches.get_one::<String>("device") {
        (lttp::server_config::DataSourceType::QUsb2snes, device_name.to_string())
    } else {
        (lttp::server_config::DataSourceType::default(), String::default())
    };

    let server_config = ServerConfig {
        data_poll_rate: 1_000,
        source_type,
        data_source,
        api_port: server_port,
        ..ServerConfig::default()
    };

    let dungeons = logic_files::base_dungeon_data()?;
    let locations = logic_files::base_location_data()?;

    let (sender, _) = broadcast::channel(16);
    let app_state = Arc::new(AppState {
        dungeon_state:  RwLock::new(DungeonState {
            dungeons,
        }),
        game_state:     RwLock::new(GameState::default()),
        location_state: RwLock::new(LocationState {
            locations,
        }),
        server_config:  RwLock::new(server_config),
        update_sender:  sender,
    });
    let app = http::build(app_state.clone());

    let game_state_poller_app_state = app_state.clone();
    tokio::spawn(async move {
        io::game_state_poller(game_state_poller_app_state).await;
    });
    tokio::spawn(async move {
        io::device_list_poller(app_state).await;
    });

    axum::Server::bind(&format!("{server_address}:{server_port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

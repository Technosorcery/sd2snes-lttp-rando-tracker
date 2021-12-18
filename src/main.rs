#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

mod http;
mod io;
mod lttp;

use crate::lttp::{
    AppState,
    DungeonState,
    GameState,
    LocationState,
    ServerConfig,
};

use anyhow::{
    bail,
    Result,
};
use clap::{
    crate_authors,
    crate_description,
    App,
    Arg,
    ArgGroup,
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
    let version_string = match option_env!("VERGEN_GIT_SEMVER") {
        Some(v) => v,
        None => env!("VERGEN_BUILD_SEMVER"),
    };

    let matches = App::new("SD2SNES LttP Randomizer Tracker")
        .version(version_string)
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("device")
                .help("The SD2SNES device to use in the Qusb2snes server.")
                .short("d")
                .long("device")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .help("JSON file to read game state from")
                .short("f")
                .long("file")
                .takes_value(true),
        )
        .group(ArgGroup::with_name("source").required(true).args(&["device", "file"]))
        .arg(
            Arg::with_name("verbose")
                .help("Enable more verbose output")
                .short("v")
                .long("verbose")
                .multiple(true),
        )
        .arg(
            Arg::with_name("port")
                .help("Port number to run the web UI server on")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8000"),
        )
        .arg(
            Arg::with_name("server-address")
                .help("Address to bind the UI & websocket server to")
                .short("a")
                .long("address")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let server_port = match matches.value_of("port").unwrap().parse::<u16>() {
        Ok(i) => i,
        Err(e) => panic!("Invalid port number: {:?}", e),
    };
    let server_address =
        match matches.value_of("server-address").unwrap().parse::<std::net::IpAddr>() {
            Ok(a) => a,
            Err(e) => panic!("Invalid address: {}", e),
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

    let data_source = if let Some(file_name) = matches.value_of("file") {
        lttp::server_config::DataSource::LocalFile(lttp::server_config::LocalFileConfig {
            source: file_name.to_string(),
        })
    } else if let Some(device_name) = matches.value_of("device") {
        lttp::server_config::DataSource::Qusb2snes(lttp::server_config::Qusb2snesConfig {
            selected_device: device_name.to_string(),
            ..lttp::server_config::Qusb2snesConfig::default()
        })
    } else {
        lttp::server_config::DataSource::default()
    };

    let server_config = ServerConfig {
        data_poll_rate: 1_000,
        data_source,
        api_port: server_port,
        ..ServerConfig::default()
    };

    let dungeons = io::logic_files::base_dungeon_data()?;
    let locations = io::logic_files::base_location_data()?;

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

    tokio::spawn(async move {
        io::game_state_poller(app_state).await;
    });

    axum::Server::bind(&format!("{}:{}", server_address, server_port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

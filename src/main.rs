#![
deny(
    // warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    // missing_docs,
)
]
#![feature(try_from, plugin)]
#![plugin(rocket_codegen)]

extern crate bus;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate includedir;
#[macro_use]
extern crate lazy_panic;
#[macro_use]
extern crate lazy_static;
extern crate phf;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate serial;
extern crate tokio_core;
extern crate tokio;
extern crate unicase;
extern crate websocket;

mod lttp;

use bus::{Bus, BusReader};

use clap::{
    App,
    Arg,
    ArgGroup,
};

use futures::stream::{SplitSink, SplitStream};
use futures::prelude::Poll;

use hyper::method::Method as hMethod;
use rocket::config::{Config, Environment};
use rocket::http::{
    ContentType,
    Status,
};
use rocket::http::hyper::header::{
    AccessControlAllowHeaders,
    AccessControlAllowMethods,
    AccessControlAllowOrigin,
};
use rocket::Response;
use rocket_contrib::Json;

use std::convert::TryFrom;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use std::io::prelude::*;
use serial::prelude::*;

use serial::PortSettings;

use tokio_core::reactor::{Core, Handle};
use tokio::prelude::task;

use unicase::UniCase;

use websocket::async::Server;
use websocket::futures::{Async, Future, Sink, Stream};
use websocket::message::OwnedMessage;

use lttp::{
    Dungeon,
    DungeonState,
    DungeonUpdate,
    GameState,
    Location,
    LocationState,
    LocationUpdate,
};

include!(concat!(env!("OUT_DIR"), "/ui_files.rs"));
include!(concat!(env!("OUT_DIR"), "/logic_files.rs"));

#[derive(Debug, Copy, Clone)]
pub enum ServerOpcode {
        // address space operations
        Get = 0,
        Put,
        VGet,
        VPut,

        // file system operations
        LS,
        MKDIR,
        RM,
        MV,

        // special operations
        Reset,
        Boot,
        PowerCycle,
        Info,
        MenuReset,
        Stream,
        Time,

        // response
        Response,
}

#[derive(Debug, Copy, Clone)]
pub enum ServerSpace {
    File = 0,
    SNES,
    MSU,
    Cmd,
    Config,
}

#[derive(Debug, Copy, Clone)]
pub enum ServerFlag {
    None = 0,
    SkipReset = 1,
    OnlyReset = 2,
    ClrX = 4,
    SetX = 8,
    StreamBurst = 16,
    NoResp = 64,
    Data64B = 128,
}

lazy_static! {
    static ref UPDATE_BUS: Mutex<Bus<Update>> = Mutex::new(Bus::new(10));
    static ref DUNGEON_STATE: Mutex<DungeonState> = Mutex::new(DungeonState::default());
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::default());
    static ref LOCATION_STATE: Mutex<LocationState> = Mutex::new(LocationState::default());
    static ref SERVER_CONFIG: Mutex<ServerConfig> = Mutex::new(ServerConfig::default());
}

fn update_tracker_serial_data(serial_port: &str) {
    let mut port = match serial::open(&serial_port) {
        Ok(p)  => p,
        Err(e) => {
            println!("Unable to open '{}': {}", &serial_port, &e);
            ::std::process::exit(1);
        },
    };

    if let Err(e) = port.configure(&PortSettings {
        baud_rate:    serial::Baud9600,
        char_size:    serial::Bits8,
        parity:       serial::ParityNone,
        stop_bits:    serial::Stop1,
        flow_control: serial::FlowNone
    }) {
        println!("Unable to configure '{}': {}", &serial_port, &e);
        ::std::process::exit(1);
    };

    if let Err(e) = port.set_timeout(Duration::from_millis(5000)) {
        println!("Unable to set serial port timeout ({}): {}", &serial_port, &e);
        ::std::process::exit(1);
    };

    let one_second = time::Duration::from_millis(1_000);
    loop {
        thread::sleep(one_second);

                        // 0xF50000 <-- WRAM start on SD2SNES
                        // 0x00F340 <-- Offset in WRAM to the items & other
                        //              things we're interested in tracking.
        let mem_offset: u32 = 0xF5F340;
        let mem_size:   u32 = 0x000200;
        // Handy if we want to look at more of the WRAM, so we don't need to
        // manually update the offset into our WRAM chunk of every item.
        let item_start = 0x0000;

        // println!("Querying SD2SNES...");
        let response = match read_wram(&mut port, mem_offset, mem_size) {
            Ok(r)  => r,
            Err(e) => {
                println!("Error reading response: {}", e);
                continue;
            },
        };

        // Write out the raw response memory for debugging.
        //let mut buffer = File::create("raw_response.txt").unwrap();
        //buffer.write(&response[..]).unwrap();

        let prev_game_state = *GAME_STATE.lock().unwrap();
        match GameState::try_from(response[(item_start as usize)..(mem_size as usize)].to_vec()) {
            Ok(gs) => {
                let new_gs = gs.merge(prev_game_state);
                let should_update_bus = new_gs != prev_game_state;

                *GAME_STATE.lock().unwrap() = new_gs;
                if should_update_bus {
                    UPDATE_BUS.lock().unwrap().broadcast(Update::Items);
                }
            }
            Err(e) => {
                println!("Unable to parse game state: {}", e);
                continue;
            }
        };
    }
}

#[derive(Debug, Copy, Clone)]
enum Update {
    Dungeons,
    Items,
    Locations,
}

fn update_tracker_file_data(file_path: &str) {
    let one_second = time::Duration::from_millis(1_000);
    loop {
        thread::sleep(one_second);

        let mut f = match File::open(&file_path) {
            Ok(f)  => f,
            Err(e) => {
                println!("Unable to open file {:?}: {}", &file_path, e);
                continue;
            }
        };
        let mut state_json = String::new();
        if let Err(e) = f.read_to_string(&mut state_json) {
            println!("Unable to read file {:?}: {}", &file_path, e);
            continue;
        };

        let prev_game_state = *GAME_STATE.lock().unwrap();
        match serde_json::from_str::<GameState>(&state_json) {
            Ok(gs) => {
                let new_gs = gs.merge(prev_game_state);
                let should_update_bus = new_gs != prev_game_state;

                *GAME_STATE.lock().unwrap() = new_gs;
                if should_update_bus {
                    UPDATE_BUS.lock().unwrap().broadcast(Update::Items);
                }
            }
            Err(e) => {
                println!("Unable to parse game state {:?}: {}", &file_path, e);
                continue;
            },
        };
    }
}

fn read_wram<T: SerialPort>(port: &mut T, mem_offset: u32, mem_size: u32) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::with_capacity(512);
    buf.extend_from_slice("USBA".as_bytes());
    buf.resize(512, 0);
    buf[4] = ServerOpcode::Get as u8; // opcode
    buf[5] = ServerSpace::SNES as u8; // space
    buf[6] = ServerFlag::None  as u8; // flags
    // offset is big endian, and starts at index 256
    buf[256] = ((mem_offset >> 24) & 0xFF) as u8;
    buf[257] = ((mem_offset >> 16) & 0xFF) as u8;
    buf[258] = ((mem_offset >> 8)  & 0xFF) as u8;
    buf[259] = ((mem_offset >> 0)  & 0xFF) as u8;
    // size is big endian, and starts at index 252
    buf[252] = ((mem_size >> 24) & 0xFF) as u8;
    buf[253] = ((mem_size >> 16) & 0xFF) as u8;
    buf[254] = ((mem_size >>  8) & 0xFF) as u8;
    buf[255] = ((mem_size >>  0) & 0xFF) as u8;

    port.write_all(&buf[..])?;

    let mut resp_buf: [u8; 512] = [0; 512];
    let mut read_bytes: u32 = 0;
    let mut result = vec!();
    // Read in an extra "block" as the first one will be the header for the response.
    while read_bytes < mem_size + 512 {
        let resp_size = port.read(&mut resp_buf)?;
        read_bytes += resp_size as u32;
        result.extend_from_slice(&resp_buf[..resp_size]);
    }

    if result[0] != "U".as_bytes()[0] ||
       result[1] != "S".as_bytes()[0] ||
       result[2] != "B".as_bytes()[0] ||
       result[3] != "A".as_bytes()[0] ||
       result[4] != ServerOpcode::Response as u8 {

        let timeout = port.timeout();
        port.set_timeout(Duration::from_millis(50))?;
        let mut buf: [u8; 64] = [0; 64];
        while let Ok(_) = port.read(&mut buf) { }
        port.set_timeout(timeout)?;
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Malformed response"));
    }

    // Drop the first "block" as it's just the header.
    Ok(result[512..].to_vec())
}

fn state_response<'r>() -> Response<'r> {
    let mut response = Response::new();
    response.set_header(ContentType::JSON);
    response.set_status(Status::Ok);
    response.set_header(AccessControlAllowOrigin::Any);
    response.set_header(AccessControlAllowMethods(vec![hMethod::Get]));
    response.set_header(AccessControlAllowHeaders(vec![
        UniCase("content-type".to_owned()),
        UniCase("accept".to_owned()),
    ]));

    response
}

#[options("/game_state")]
fn get_game_state_options<'r>() -> Response<'r> { state_response() }

#[get("/game_state", format = "application/json")]
fn get_game_state<'r>() -> Option<Response<'r>> {
    let game_state = *GAME_STATE.lock().unwrap();
    let mut response = state_response();
    let json = match serde_json::to_string(&game_state) {
        Ok(j) => j,
        Err(e) => {
            println!("Could not serialize game state: {:?}", e);
            return None;
        }
    };
    response.set_sized_body(Cursor::new(json));

    Some(response)
}

#[options("/location_state")]
fn get_location_state_options<'r>() -> Response<'r> { state_response() }

#[get("/location_state", format = "application/json")]
fn get_location_state<'r>() -> Option<Response<'r>> {
    let location_state = LOCATION_STATE.lock().unwrap().clone();
    let mut response = state_response();
    let json = match serde_json::to_string(&location_state.locations) {
        Ok(j) => j,
        Err(e) => {
            println!("Could not serialize location state: {:?}", e);
            return None;
        }
    };
    response.set_sized_body(Cursor::new(json));

    Some(response)
}

#[options("/location_state/<_location>")]
fn set_location_state_options<'r>(_location: String) -> Response<'r> { state_response() }

#[post("/location_state/<location>", data = "<state>", format = "application/json")]
fn set_location_state<'r>(location: String, state: Json<LocationUpdate>) -> Option<Response<'r>> {
    let location_update = state.into_inner();
    let state;
    {
        let mut location_state = LOCATION_STATE.lock().unwrap();
        location_state.update(location.clone(), location_update);
        state = location_state.get(location).clone();
        UPDATE_BUS.lock().unwrap().broadcast(Update::Locations);
    }

    let mut response = state_response();
    let json = match serde_json::to_string(&state) {
        Ok(j) => j,
        Err(e) => {
            println!("Could not serialize dungeon state: {:?}", e);
            return None;
        }
    };
    response.set_sized_body(Cursor::new(json));

    Some(response)
}

#[options("/dungeon_state")]
fn get_dungeon_state_options<'r>() -> Response<'r> { state_response() }

#[get("/dungeon_state", format = "application/json")]
fn get_dungeon_state<'r>() -> Option<Response<'r>> {
    let dungeon_state = DUNGEON_STATE.lock().unwrap().clone();
    let mut response = state_response();
    let json = match serde_json::to_string(&dungeon_state.dungeons) {
        Ok(j) => j,
        Err(e) => {
            println!("Could not serialize dungeon state: {:?}", e);
            return None;
        }
    };
    response.set_sized_body(Cursor::new(json));

    Some(response)
}

#[options("/dungeon_state/<_dungeon>")]
fn set_dungeon_state_options<'r>(_dungeon: String) -> Response<'r> { state_response() }

#[post("/dungeon_state/<dungeon>", data = "<state>", format = "application/json")]
fn set_dungeon_state<'r>(dungeon: String, state: Json<DungeonUpdate>) -> Option<Response<'r>> {
    let dungeon_update = state.into_inner();
    let state;
    {
        let mut dungeon_state = DUNGEON_STATE.lock().unwrap();
        dungeon_state.update(dungeon.clone(), dungeon_update);
        state = dungeon_state.get(dungeon).clone();
        UPDATE_BUS.lock().unwrap().broadcast(Update::Dungeons);
    }

    let mut response = state_response();
    let json = match serde_json::to_string(&state) {
        Ok(j) => j,
        Err(e) => {
            println!("Could not serialize dungeon state: {:?}", e);
            return None;
        }
    };
    response.set_sized_body(Cursor::new(json));

    Some(response)
}

#[get("/<file..>")]
fn files<'r>(file: PathBuf) -> Option<Response<'r>> {
    let mut path = Path::new("ui/dist/").join(file);
    // If the path somehow isn't representable as a str, it's not one we want to
    // attempt to serve.
    let mut path_str = match path.to_str() {
        Some(s) => s.to_string(),
        None    => return None,
    };

    let file = match UI_FILES.get(&path_str) {
        // Check if we were given a file path, or a directory. If the file can't
        // be found, assume it was supposed to be a directory, and try grabbing
        // the index.html from it.
        Err(_) => {
            path = path.join("index.html");
            path_str = match path.to_str() {
                Some(s) => s.to_string(),
                None    => return None,
            };
            // If we still can't find the file, there isn't one to find.
            match UI_FILES.get(&path_str) {
                Err(_) => return None,
                Ok(f)  => f,
            }
        },
        Ok(f) => f,
    };

    let mut response = Response::new();

    if let Some(ext) = path.extension() {
        if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
            response.set_header(ct);
        }
    }
    response.set_status(Status::Ok);
    response.set_sized_body(Cursor::new(file));

    Some(response)
}

#[get("/")]
fn root<'r>() -> Option<Response<'r>> { files(PathBuf::from("")) }

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct ServerConfig {
    pub api_port: u16,
    pub websocket_port: u16,
}

#[options("/config")]
fn get_config_options<'r>() -> Response<'r> { state_response() }

#[get("/config", format = "application/json")]
fn get_config<'r>() -> Option<Response<'r>> {
    let server_config = { *SERVER_CONFIG.lock().unwrap() };
    let mut response = state_response();
    let json = match serde_json::to_string(&server_config) {
        Ok(j) => j,
        Err(e) => {
            println!("Could not serialize server config: {:?}", e);
            return None;
        }
    };
    response.set_sized_body(Cursor::new(json));

    Some(response)
}

fn set_base_location_data() {
    let path = Path::new("logic/poi_locations.yaml").to_str().unwrap();
    let file = match String::from_utf8(LOGIC_FILES.get(&path).unwrap().to_vec()) {
        Ok(s) => s,
        Err(e) => panic!("Unable to read POI Location data: {:?}", e),
    };

    match serde_yaml::from_str::<Vec<Location>>(&file) {
        Ok(locations) => {
            *LOCATION_STATE.lock().unwrap() = LocationState { locations };
            UPDATE_BUS.lock().unwrap().broadcast(Update::Locations);
        }
        Err(e) => {
            panic!("Unable to parse location state {:?}: {}", &path, e);
        },
    };
}

fn set_base_dungeon_data() {
    let path = Path::new("logic/dungeon_locations.yaml").to_str().unwrap();
    let file = match String::from_utf8(LOGIC_FILES.get(&path).unwrap().to_vec()) {
        Ok(s) => s,
        Err(e) => panic!("Unable to read Dungeon data: {:?}", e),
    };

    match serde_yaml::from_str::<Vec<Dungeon>>(&file) {
        Ok(dungeons) => {
            *DUNGEON_STATE.lock().unwrap() = DungeonState { dungeons };
            UPDATE_BUS.lock().unwrap().broadcast(Update::Dungeons);
        }
        Err(e) => {
            panic!("Unable to parse dungeon state {:?}: {}", &path, e);
        },
    };
}

fn main() {
    UI_FILES.set_passthrough(env::var_os("UI_FILES_PASSTHROUGH").is_some());
    LOGIC_FILES.set_passthrough(env::var_os("LOGIC_FILES_PASSTHROUGH").is_some());

    let matches = App::new("SD2SNES LttP Randomizer Tracker")
                          .version(crate_version!())
                          .author(crate_authors!())
                          .about(crate_description!())
                          .arg(Arg::with_name("serial")
                               .help("The SD2SNES serial port to use.")
                               .short("s")
                               .long("serial")
                               .takes_value(true))
                          .arg(Arg::with_name("file")
                               .help("JSON file to read game state from")
                               .short("f")
                               .long("file")
                               .takes_value(true))
                          .group(ArgGroup::with_name("source")
                                 .required(true)
                                 .args(&["serial", "file"]))
                          .arg(Arg::with_name("verbose")
                               .help("Enable more verbose output")
                               .short("v")
                               .long("verbose")
                               .multiple(true))
                          .arg(Arg::with_name("port")
                               .help("Port number to run the web UI server on")
                               .short("p")
                               .long("port")
                               .takes_value(true)
                               .default_value("8000"))
                          .arg(Arg::with_name("websocket-port")
                               .help("Port number to run the websocket server on [default: <port> + 1]")
                               .short("w")
                               .long("websocket-port")
                               .takes_value(true))
                          .arg(Arg::with_name("server-address")
                               .help("Address to bind the UI & websocket server to")
                               .short("a")
                               .long("address")
                               .takes_value(true)
                               .default_value("0.0.0.0"))
                          .get_matches();

    let verbose_level = matches.occurrences_of("verbose");

    match verbose_level {
        0 => set_panic_message!(lazy_panic::formatter::JustError),
        1 => set_panic_message!(lazy_panic::formatter::Simple),
        _ => set_panic_message!(lazy_panic::formatter::Debug),
    }

    let server_port = match matches.value_of("port").unwrap().parse::<u16>() {
        Ok(i) => i,
        Err(e) => panic!("Invalid port number: {:?}", e),
    };
    let websocket_port = match matches.value_of("websocket-port") {
        Some(i) => i.parse::<u16>().unwrap_or_else(|e| panic!("Invalid websocket port number: {:?}", e)),
        None => server_port + 1,
    };
    let server_address = match matches.value_of("server-address").unwrap().parse::<std::net::IpAddr>() {
        Ok(a) => a,
        Err(e) => panic!("Invalid address: {}", e),
    };

    {
        *SERVER_CONFIG.lock().unwrap() = ServerConfig {
            api_port: server_port,
            websocket_port,
        };
    }

    set_base_location_data();
    set_base_dungeon_data();

    thread::spawn(move || {
        if let Some(serial) = matches.value_of("serial") {
            let serial_port = serial.to_string();
            println!("Using serial port: {}", &serial_port);
            update_tracker_serial_data(&serial_port);
        } else if let Some(file) = matches.value_of("file") {
            let file_source = file.to_string();
            println!("Using file: {}", &file_source);
            update_tracker_file_data(&file_source);
        }
    });

    let websocket_bind_addr = format!("{}:{}", &server_address, websocket_port).parse::<std::net::SocketAddr>().unwrap_or_else(|e| panic!("Invalid IP/Port combination: {:?}", e));
    thread::spawn(move || {
        websocket_server(websocket_bind_addr);
    });

    let rocket_env = if verbose_level > 0 { Environment::Development } else { Environment::Production };
    let rocket_config = Config::build(rocket_env)
        .address(format!("{}", &server_address))
        .port(server_port)
        // We don't actually use the secret key, so we don't really care what it is.
        .secret_key("8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=")
        .finalize().unwrap();

    rocket::custom(rocket_config, true)
        .mount(
            "/",
            routes![
                files,
                get_config_options,
                get_config,
                get_dungeon_state_options,
                get_dungeon_state,
                get_game_state_options,
                get_game_state,
                get_location_state_options,
                get_location_state,
                root,
                set_dungeon_state_options,
                set_dungeon_state,
                set_location_state_options,
                set_location_state,
            ]
        )
        .launch();
}

#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[derive(Debug, Clone, Serialize)]
enum WebSocketPayload {
    Item(GameState),
    Dungeon(Vec<Dungeon>),
    Location(Vec<Location>),
}

impl WebSocketPayload {
    pub fn to_json_string(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(j) => j,
            Err(e) => panic!("Could not serialize WebSocketPayload: {:?}", e),
        }
    }
}

fn websocket_state_message(state_kind: Update) -> OwnedMessage {
    let payload = match state_kind {
        Update::Items     => { WebSocketPayload::Item(*GAME_STATE.lock().unwrap()) },
        Update::Dungeons  => { WebSocketPayload::Dungeon(DUNGEON_STATE.lock().unwrap().clone().dungeons) },
        Update::Locations => { WebSocketPayload::Location(LOCATION_STATE.lock().unwrap().clone().locations) },
    };

    let json_payload = payload.to_json_string();
    // println!("Message payload: {:?}", &json_payload);

    OwnedMessage::Text(json_payload)
}

fn spawn_future<F, I, E>(f: F, desc: &'static str, handle: &Handle)
    where F: Future<Item = I, Error = E> + 'static,
          E: Debug
{
    handle.spawn(f.map_err(move |e| println!("{}: '{:?}'", desc, e))
                  .map(move |_| println!("{}: Finished.", desc)));
}

#[allow(deprecated)]
struct Peer {
    bus: BusReader<Update>,
    sink: Box<futures::sink::Wait<SplitSink<websocket::client::async::Framed<tokio_core::net::TcpStream,websocket::async::MessageCodec<OwnedMessage>>>>>,
    stream: Box<SplitStream<websocket::client::async::Framed<tokio_core::net::TcpStream,websocket::async::MessageCodec<OwnedMessage>>>>,
}

impl Peer {
    #[allow(deprecated)]
    pub fn new(
        bus: BusReader<Update>,
        sink: Box<futures::sink::Wait<SplitSink<websocket::client::async::Framed<tokio_core::net::TcpStream,websocket::async::MessageCodec<OwnedMessage>>>>>,
        stream: Box<SplitStream<websocket::client::async::Framed<tokio_core::net::TcpStream,websocket::async::MessageCodec<OwnedMessage>>>>,
    ) -> Peer {
        Peer { bus, sink, stream }
    }
}

impl Future for Peer {
    type Item = ();
    type Error = websocket::WebSocketError;

    fn poll(&mut self) -> Poll<(), websocket::WebSocketError> {
        if let Ok(state_update) = self.bus.recv_timeout(Duration::from_millis(10)) {
            println!("Sending update for: {:?}", state_update);
            if self.sink.send(websocket_state_message(state_update)).is_err() {
                return Ok(Async::Ready(()));
            }
            if self.sink.flush().is_err() {
                return Ok(Async::Ready(()));
            }
        };

        while let Async::Ready(message) = self.stream.poll()? {
            match message {
                Some(OwnedMessage::Ping(p)) => {
                    if self.sink.send(OwnedMessage::Pong(p)).is_err() {
                        return Ok(Async::Ready(()));
                    };
                    if self.sink.flush().is_err() {
                        return Ok(Async::Ready(()));
                    };
                },
                Some(OwnedMessage::Close(_)) => return Ok(Async::Ready(())),
                Some(OwnedMessage::Text(text)) => {
                    if text == "HELLO" {
                        println!("Sending initial state");
                        if self.sink.send(websocket_state_message(Update::Items)).is_err() {
                            return Ok(Async::Ready(()));
                        };
                        if self.sink.send(websocket_state_message(Update::Locations)).is_err() {
                            return Ok(Async::Ready(()));
                        };
                        if self.sink.send(websocket_state_message(Update::Dungeons)).is_err() {
                            return Ok(Async::Ready(()));
                        };
                        if self.sink.flush().is_err() {
                            return Ok(Async::Ready(()));
                        };
                    } // else {
                    //     println!("Got message: {:?}", text);
                    // }
                },
                _ => {},
            }
        }

        task::current().notify();
        Ok(Async::NotReady)
    }
}

fn websocket_server(bind_addr: std::net::SocketAddr) {
    println!("Opening websocket server: {}", &bind_addr);
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let server = Server::bind(bind_addr, &handle).unwrap();

    let f = server.incoming().for_each(move |(upgrade, addr)| {
        println!("Incoming connection from {:?}", addr);
        let f = upgrade.accept().and_then(|(s, _)| {
            let (sink, stream) = s.split();

            println!("Creating peer");

            Peer::new(
                UPDATE_BUS.lock().unwrap().add_rx(),
                Box::new(sink.wait()),
                Box::new(stream)
            )
        });

        spawn_future(f, "Client Status", &handle);
        Ok(())
    }).map_err(|_| {});

    core.run(f).unwrap();
    println!("Websocket server closed");
}

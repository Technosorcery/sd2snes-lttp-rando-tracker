#![feature(try_from,plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serial;

mod lttp;

use clap::{App, Arg, ArgGroup};

use rocket::response::NamedFile;
use rocket_contrib::Json;

use std::convert::TryFrom;
use std::io;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use std::io::prelude::*;
use serial::prelude::*;

use serial::PortSettings;

use lttp::GameState;

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
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::default());
}

fn update_tracker_serial_data(serial_port: &str) {
    let mut port = match serial::open(&serial_port) {
        Ok(p) => p,
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
            Ok(r) => r,
            Err(e) => {
                println!("Error reading response: {}", e);
                continue;
            },
        };

        // Write out the raw response memory for debugging.
        let mut buffer = File::create("raw_response.txt").unwrap();
        buffer.write(&response[..]).unwrap();

        let game_state = match GameState::try_from(response[(item_start as usize)..(mem_size as usize)].to_vec()) {
            Ok(gs) => gs,
            Err(e) => {
                println!("Unable to parse game state: {}", e);
                continue;
            }
        };

        // println!("Game State: {:#?}", &game_state);
        { *GAME_STATE.lock().unwrap() = game_state; }
    }
}

fn update_tracker_file_data(file_path: &str) {
    let one_second = time::Duration::from_millis(1_000);
    loop {
        thread::sleep(one_second);

        let mut f = match File::open(&file_path) {
            Ok(f) => f,
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

        let game_state: GameState = match serde_json::from_str(&state_json) {
            Ok(gs) => gs,
            Err(e) => {
                println!("Unable to parse game state {:?}: {}", &file_path, e);
                continue;
            }
        };

        { *GAME_STATE.lock().unwrap() = game_state; }
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

    port.write(&buf[..])?;

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

#[get("/game_state", format = "application/json")]
fn get_game_state() -> Json<GameState> {
    let game_state = GAME_STATE.lock().unwrap().clone();
    Json(game_state)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new("static/").join(file);
    if path.is_dir() { path = path.join("index.html") }

    println!("Attempting to find static file at: {:?}", &path);

    NamedFile::open(path).ok()
}

#[get("/")]
fn root() -> Option<NamedFile> { files(PathBuf::from("")) }

fn main() {
    let matches = App::new("SD2SNES LttP Randomizer Tracker")
                          .version(crate_version!())
                          .author(crate_authors!())
                          .about("Automatically track progress in a Link to the Past randomizer run using a USB2SNES modified SD2SNES.")
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
                          .get_matches();

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

    rocket::ignite().mount(
        "/",
        routes![get_game_state,files,root]
    ).launch();
}

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
extern crate serial;

use clap::{Arg, App};

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

#[derive(Debug, Default, Clone, Copy, Serialize)]
struct BigKey {
    // Light world
    eastern_palace: bool,
    desert_palace: bool,
    tower_of_hera: bool,
    // Dark world
    palace_of_darkness: bool,
    swamp_palace: bool,
    skull_woods: bool,
    thieves_town: bool,
    ice_palace: bool,
    misery_mire: bool,
    turtle_rock: bool,
    gannons_tower: bool,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
struct Pendant {
    red: bool,
    blue: bool,
    green: bool,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
struct Crystal {
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    five: bool,
    six: bool,
    seven: bool,
}

#[derive(Debug, Copy, Clone, Serialize)]
enum BowFlags {
    None             = 0,
    Wood             = 1,
    WoodWithArrows   = 2,
    Silver           = 3,
    SilverWithArrows = 4,
}

impl Default for BowFlags {
    fn default() -> BowFlags { BowFlags::None }
}

impl TryFrom<u8> for BowFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<BowFlags, Self::Error> {
        match number {
            0 => Ok(BowFlags::None),
            1 => Ok(BowFlags::Wood),
            2 => Ok(BowFlags::WoodWithArrows),
            3 => Ok(BowFlags::Silver),
            4 => Ok(BowFlags::SilverWithArrows),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum BoomerangFlags {
    None = 0,
    Blue = 1,
    Red  = 2,
}

impl Default for BoomerangFlags {
    fn default() -> BoomerangFlags { BoomerangFlags::None }
}

impl TryFrom<u8> for BoomerangFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<BoomerangFlags, Self::Error> {
        match number {
            0 => Ok(BoomerangFlags::None),
            1 => Ok(BoomerangFlags::Blue),
            2 => Ok(BoomerangFlags::Red),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum ShroomPowderFlags {
    None   = 0,
    Shroom = 1,
    Powder = 2,
}

impl Default for ShroomPowderFlags {
    fn default() -> ShroomPowderFlags { ShroomPowderFlags::None }
}

impl TryFrom<u8> for ShroomPowderFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<ShroomPowderFlags, Self::Error> {
        match number {
            0 => Ok(ShroomPowderFlags::None),
            1 => Ok(ShroomPowderFlags::Shroom),
            2 => Ok(ShroomPowderFlags::Powder),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum FluteShovelFlags {
    None         = 0,
    Shovel       = 1,
    Flute        = 2,
    FluteAndBird = 3,
}

impl Default for FluteShovelFlags {
    fn default() -> FluteShovelFlags { FluteShovelFlags::None }
}

impl TryFrom<u8> for FluteShovelFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<FluteShovelFlags, Self::Error> {
        match number {
            0 => Ok(FluteShovelFlags::None),
            1 => Ok(FluteShovelFlags::Shovel),
            2 => Ok(FluteShovelFlags::Flute),
            3 => Ok(FluteShovelFlags::FluteAndBird),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum GlovesFlags {
    None       = 0,
    PowerGlove = 1,
    TitansMitt = 2,
}

impl Default for GlovesFlags {
    fn default() -> GlovesFlags { GlovesFlags::None }
}

impl TryFrom<u8> for GlovesFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<GlovesFlags, Self::Error> {
        match number {
            0 => Ok(GlovesFlags::None),
            1 => Ok(GlovesFlags::PowerGlove),
            2 => Ok(GlovesFlags::TitansMitt),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum SwordFlags {
    None          = 0,
    FightersSword = 1,
    MasterSword   = 2,
    TemperedSword = 3,
    GoldenSword   = 4,
}

impl Default for SwordFlags {
    fn default() -> SwordFlags { SwordFlags::None }
}

impl TryFrom<u8> for SwordFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<SwordFlags, Self::Error> {
        match number {
            0 => Ok(SwordFlags::None),
            1 => Ok(SwordFlags::FightersSword),
            2 => Ok(SwordFlags::MasterSword),
            3 => Ok(SwordFlags::TemperedSword),
            4 => Ok(SwordFlags::GoldenSword),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum ShieldFlags {
    None           = 0,
    FightersShield = 1,
    RedShield      = 2,
    MirrorShield   = 3,
}

impl Default for ShieldFlags {
    fn default() -> ShieldFlags { ShieldFlags::None }
}

impl TryFrom<u8> for ShieldFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<ShieldFlags, Self::Error> {
        match number {
            0 => Ok(ShieldFlags::None),
            1 => Ok(ShieldFlags::FightersShield),
            2 => Ok(ShieldFlags::RedShield),
            3 => Ok(ShieldFlags::MirrorShield),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum ArmorFlags {
    GreenMail = 0,
    BlueMail  = 1,
    RedMail   = 2,
}

impl Default for ArmorFlags {
    fn default() -> ArmorFlags { ArmorFlags::GreenMail }
}

impl TryFrom<u8> for ArmorFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<ArmorFlags, Self::Error> {
        match number {
            0 => Ok(ArmorFlags::GreenMail),
            1 => Ok(ArmorFlags::BlueMail),
            2 => Ok(ArmorFlags::RedMail),
            _ => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum BottleFlags {
    NoBottle    = 0x00,
    Mushroom    = 0x01,
    Empty       = 0x02,
    RedPotion   = 0x03,
    GreenPotion = 0x04,
    BluePotion  = 0x05,
    Fairy       = 0x06,
    Bee         = 0x07,
    MagicBee    = 0x08,
}

impl Default for BottleFlags {
    fn default() -> BottleFlags { BottleFlags::NoBottle }
}

impl TryFrom<u8> for BottleFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<BottleFlags, Self::Error> {
        match number {
            0x00 => Ok(BottleFlags::NoBottle),
            0x01 => Ok(BottleFlags::Mushroom),
            0x02 => Ok(BottleFlags::Empty),
            0x03 => Ok(BottleFlags::RedPotion),
            0x04 => Ok(BottleFlags::GreenPotion),
            0x05 => Ok(BottleFlags::BluePotion),
            0x06 => Ok(BottleFlags::Fairy),
            0x07 => Ok(BottleFlags::Bee),
            0x08 => Ok(BottleFlags::MagicBee),
            _    => Err(format_err!("Unknown item flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
enum MagicFlags {
    Normal  = 0,
    Half    = 1,
    Quarter = 2,
}

impl Default for MagicFlags {
    fn default() -> MagicFlags { MagicFlags::Normal }
}

impl TryFrom<u8> for MagicFlags {
    type Error = failure::Error;

    fn try_from(number: u8) -> Result<MagicFlags, Self::Error> {
        match number {
            0 => Ok(MagicFlags::Normal),
            1 => Ok(MagicFlags::Half),
            2 => Ok(MagicFlags::Quarter),
            _ => Err(format_err!("Unknown magic flag: 0x{:X}", number)),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Serialize)]
struct GameState {
    // Items
    pub bow:               BowFlags,
    pub boomerang:         BoomerangFlags,
    pub hook_shot:         bool,
    pub bomb:              u8,
    pub shroom_powder:     ShroomPowderFlags,
    pub fire_rod:          bool,
    pub ice_rod:           bool,
    pub bombos_medallion:  bool,
    pub ether_medallion:   bool,
    pub quake_medallion:   bool,
    pub lantern:           bool,
    pub hammer:            bool,
    pub flute_shovel:      FluteShovelFlags,
    pub net:               bool,
    pub book:              bool,
    pub bottle:            bool,
    pub cane_somaria:      bool,
    pub cane_byrna:        bool,
    pub cape:              bool,
    pub mirror:            bool,
    // Abilities
    pub gloves:            GlovesFlags,
    pub boots:             bool,
    pub flippers:          bool,
    pub moon_pearl:        bool,
    // Weapon & Armor Progression
    pub sword_level:       SwordFlags,
    pub shield_level:      ShieldFlags,
    pub armor_level:       ArmorFlags,
    // Bottle content
    pub bottle_content1:   BottleFlags,
    pub bottle_content2:   BottleFlags,
    pub bottle_content3:   BottleFlags,
    pub bottle_content4:   BottleFlags,

    pub rupees:            u16,
    pub heart_quarters:    u8,
    pub bomb_capacity:     u8,
    pub hearts:            u8,
    pub max_hearts:        u8,

    pub arrows:            u8,
    pub arrow_capacity:    u8,

    pub magic_progression: MagicFlags,

    pub small_keys:        u8,
    pub big_key:           BigKey,

    pub pendant:           Pendant,
    pub crystal:           Crystal,
}

impl TryFrom<Vec<u8>> for GameState {
    type Error = failure::Error;

    fn try_from(response: Vec<u8>) -> Result<GameState, Self::Error> {
        Ok(GameState {
            bow:               BowFlags::try_from(response[0x00])?,
            boomerang:         BoomerangFlags::try_from(response[0x01])?,
            hook_shot:         response[0x02] > 0,
            bomb:              response[0x03],
            shroom_powder:     ShroomPowderFlags::try_from(response[0x04])?,
            fire_rod:          response[0x05] > 0,
            ice_rod:           response[0x06] > 0,
            bombos_medallion:  response[0x07] > 0,
            ether_medallion:   response[0x08] > 0,
            quake_medallion:   response[0x09] > 0,
            lantern:           response[0x0A] > 0,
            hammer:            response[0x0B] > 0,
            flute_shovel:      FluteShovelFlags::try_from(response[0x0C])?,
            net:               response[0x0D] > 0,
            book:              response[0x0E] > 0,
            bottle:            response[0x0F] > 0,
            cane_somaria:      response[0x10] > 0,
            cane_byrna:        response[0x11] > 0,
            cape:              response[0x12] > 0,
            mirror:            response[0x13] > 0,

            gloves:            GlovesFlags::try_from(response[0x14])?,
            boots:             response[0x15] > 0,
            flippers:          response[0x16] > 0,
            moon_pearl:        response[0x17] > 0,

            sword_level:       SwordFlags::try_from(response[0x19])?,
            shield_level:      ShieldFlags::try_from(response[0x1A])?,
            armor_level:       ArmorFlags::try_from(response[0x1B])?,

            bottle_content1:   BottleFlags::try_from(response[0x1C])?,
            bottle_content2:   BottleFlags::try_from(response[0x1D])?,
            bottle_content3:   BottleFlags::try_from(response[0x1E])?,
            bottle_content4:   BottleFlags::try_from(response[0x1F])?,

            // Rupees are spread across two bytes, as the randomizer lifted the
            // 255 Rupee limit, and it's stored little-endian.
            rupees:            ((response[0x23] as u16) << 8) + response[0x22] as u16,
            heart_quarters:    response[0x2B],
            bomb_capacity:     response[0x30] + 10,
            hearts:            response[0x2D],
            max_hearts:        response[0x2C],

            arrows:            response[0x37],
            arrow_capacity:    response[0x31] + 30,

            magic_progression: MagicFlags::try_from(response[0x3B])?,

            small_keys:        if response[0x2F] == 0xFF { 0 } else { response[0x2F] },
            big_key: BigKey {
                // BigKey1: 0x366
                //       Skull Woods
                //       |Ice Palace
                //       ||Tower of Hera
                //       |||Gargoyle's Domain
                //       ||||Turtle Rock
                //       |||||Gannon's Tower
                //       ||||||x
                //       |||||||x
                //       vvvvvvvv
                //      |--------|
                // Bit:  7      0
                gannons_tower: response[0x26] & 0b00000100 > 0,
                turtle_rock:   response[0x26] & 0b00001000 > 0,
                thieves_town:  response[0x26] & 0b00010000 > 0,
                tower_of_hera: response[0x26] & 0b00100000 > 0,
                ice_palace:    response[0x26] & 0b01000000 > 0,
                skull_woods:   response[0x26] & 0b10000000 > 0,

                // BigKey2: 0x367
                //       X
                //       |X
                //       ||Eastern Palace
                //       |||Desert Palace
                //       ||||X
                //       |||||Swamp Palace
                //       ||||||Dark Palace
                //       |||||||Misery Mire
                //       vvvvvvvv
                //      |--------|
                // Bit:  7      0
                misery_mire:        response[0x27] & 0b00000001 > 0,
                desert_palace:      response[0x27] & 0b00000010 > 0,
                swamp_palace:       response[0x27] & 0b00000100 > 0,
                palace_of_darkness: response[0x27] & 0b00010000 > 0,
                eastern_palace:     response[0x27] & 0b00100000 > 0,
            },

            // 0x374 -> Pendants (Bitmask)
            // 1 - Red
            // 2 - Blue
            // 4 - Green
            pendant: Pendant {
                red:   response[0x34] & 0b0001 > 0,
                blue:  response[0x34] & 0b0010 > 0,
                green: response[0x34] & 0b0100 > 0,
            },

            // 0x37A -> Crystals (Bitmask)
            // 1 - Misery Mire
            // 2 - Dark Palace
            // 4 - Ice Palace
            // 8 - Turtle Rock
            // 16 - Swamp Palace
            // 32 - Gargoyle's Domain
            // 64 - Skull Woods
            crystal: Crystal {
                one:   response[0x3A] & 0b00000001 > 0,
                three: response[0x3A] & 0b00000010 > 0,
                five:  response[0x3A] & 0b00000100 > 0,
                four:  response[0x3A] & 0b00001000 > 0,
                two:   response[0x3A] & 0b00010000 > 0,
                six:   response[0x3A] & 0b00100000 > 0,
                seven: response[0x3A] & 0b01000000 > 0,
            },
        })
    }
}


lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::default());
}

fn update_tracker_data(serial_port: &str) {
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
                          .arg(Arg::with_name("SERIAL")
                               .help("The SD2SNES serial port to use.")
                               .required(true)
                               .index(1))
                          .get_matches();

    let serial_port = matches.value_of("SERIAL").unwrap().to_string();
    println!("Using serial port: {}", &serial_port);

    thread::spawn(move || { update_tracker_data(&serial_port) });

    rocket::ignite().mount(
        "/",
        routes![get_game_state,files,root]
    ).launch();
}

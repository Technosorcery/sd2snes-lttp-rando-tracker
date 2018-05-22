#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate serial;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

use clap::{Arg, App};

use std::io;
use std::fs::File;
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use std::io::prelude::*;
use serial::prelude::*;

use num_traits::{FromPrimitive, ToPrimitive};
use serial::PortSettings;

#[derive(Debug, Copy, Clone)]
pub enum ServerOpcode {
        // address space operations
        GET = 0,
        PUT,
        VGET,
        VPUT,

        // file system operations
        LS,
        MKDIR,
        RM,
        MV,

        // special operations
        RESET,
        BOOT,
        POWER_CYCLE,
        INFO,
        MENU_RESET,
        STREAM,
        TIME,

        // response
        RESPONSE,
}

#[derive(Debug, Copy, Clone)]
pub enum ServerSpace {
    FILE = 0,
    SNES,
    MSU,
    CMD,
    CONFIG,
}

#[derive(Debug, Copy, Clone)]
pub enum ServerFlag {
    NONE = 0,
    SKIPRESET = 1,
    ONLYRESET = 2,
    CLRX = 4,
    SETX = 8,
    STREAM_BURST = 16,
    NORESP = 64,
    DATA64B = 128,
}

#[derive(Debug, Default)]
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

bitflags! {
    struct BowFlags: u8 {
        const NONE = 0;
        const SILVER = 4;
    }
}

impl Default for BowFlags {
    fn default() -> BowFlags { BowFlags::NONE }
}

bitflags! {
    struct BoomerangFlags: u8 {
        const NONE = 0;
        const BLUE = 1;
        const RED  = 2;
        const BOTH = Self::BLUE.bits | Self::RED.bits;
    }
}

impl Default for BoomerangFlags {
    fn default() -> BoomerangFlags { BoomerangFlags::NONE }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum GenericItemFlags {
    None = 0,
    Have = 1,
}

impl Default for GenericItemFlags {
    fn default() -> GenericItemFlags { GenericItemFlags::None }
}

bitflags! {
    struct ShroomPowderFlags: u8 {
        const NONE   = 0;
        const SHROOM = 1;
        const POWDER = 2;
        const BOTH   = Self::SHROOM.bits | Self::POWDER.bits;
    }
}

impl Default for ShroomPowderFlags {
    fn default() -> ShroomPowderFlags { ShroomPowderFlags::NONE }
}

bitflags! {
    struct FluteShovelFlags: u8 {
        const NONE   = 0;
        const SHOVEL = 1;
        const FLUTE  = 2;
        const BOTH   = Self::SHOVEL.bits | Self::FLUTE.bits;
    }
}

impl Default for FluteShovelFlags {
    fn default() -> FluteShovelFlags { FluteShovelFlags::NONE }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum MirrorFlags {
    None = 0,
    Have = 2,
}

impl Default for MirrorFlags {
    fn default() -> MirrorFlags { MirrorFlags::None }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum GlovesFlags {
    None       = 0,
    PowerGlove = 1,
    TitansMitt = 2,
}

impl Default for GlovesFlags {
    fn default() -> GlovesFlags { GlovesFlags::None }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum ShieldFlags {
    None           = 0,
    FightersShield = 1,
    RedShield      = 2,
    MirrorShield   = 3,
}

impl Default for ShieldFlags {
    fn default() -> ShieldFlags { ShieldFlags::None }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum ArmorFlags {
    GreenMail = 0,
    BlueMail  = 1,
    RedMail   = 2,
}

impl Default for ArmorFlags {
    fn default() -> ArmorFlags { ArmorFlags::GreenMail }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum BottleFlags {
    NoBottle   = 0x00,
    Empty      = 0x02,
    BluePotion = 0x05,
    // RedPotion
    // GreenPotion
    // Bee
    // MagicBee
}

impl Default for BottleFlags {
    fn default() -> BottleFlags { BottleFlags::NoBottle }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum MagicFlags {
    Normal  = 0,
    Half    = 1,
    Quarter = 2,
}

impl Default for MagicFlags {
    fn default() -> MagicFlags { MagicFlags::Normal }
}

#[derive(Debug, Default)]
struct GameState {
    // Items
    pub bow:               BowFlags,
    pub boomerang:         BoomerangFlags,
    pub hook_shot:         GenericItemFlags,
    pub bomb:              u8,
    pub shroom_powder:     ShroomPowderFlags,
    pub fire_rod:          GenericItemFlags,
    pub ice_rod:           GenericItemFlags,
    pub bombos_medallion:  GenericItemFlags,
    pub ether_medallion:   GenericItemFlags,
    pub quake_medallion:   GenericItemFlags,
    pub lantern:           GenericItemFlags,
    pub hammer:            GenericItemFlags,
    pub flute_shovel:      FluteShovelFlags,
    pub net:               GenericItemFlags,
    pub book:              GenericItemFlags,
    pub bottle:            u8,
    pub cane_somaria:      GenericItemFlags,
    pub cane_byrna:        GenericItemFlags,
    pub cape:              GenericItemFlags,
    pub mirror:            MirrorFlags,
    // Abilities
    pub gloves:            GlovesFlags,
    pub boots:             GenericItemFlags,
    pub flippers:          GenericItemFlags,
    pub moon_pearl:        GenericItemFlags,
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

    pub small_keys:        u8,
    pub big_key:           BigKey,

    pub magic_progression: MagicFlags,
}
lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::default());
}

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

    loop {}
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
        let mem_size:   u32 = 0x000040;
        // Handy if we want to look at more of the WRAM, so we don't need to
        // manually update the offset into our WRAM chunk of every item.
        let item_start = 0x00;

        println!("Querying SD2SNES...");
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

        let game_state = GameState {
            bow:               BowFlags::from_bits(response[item_start + 0x00]).unwrap(),
            boomerang:         BoomerangFlags::from_bits(response[item_start + 0x01]).unwrap(),
            hook_shot:         num_traits::FromPrimitive::from_u8(response[item_start + 0x02]).unwrap(),
            bomb:              response[item_start + 0x03],
            shroom_powder:     ShroomPowderFlags::from_bits(response[item_start + 0x04]).unwrap(),
            fire_rod:          num_traits::FromPrimitive::from_u8(response[item_start + 0x05]).unwrap(),
            ice_rod:           num_traits::FromPrimitive::from_u8(response[item_start + 0x06]).unwrap(),
            bombos_medallion:  num_traits::FromPrimitive::from_u8(response[item_start + 0x07]).unwrap(),
            ether_medallion:   num_traits::FromPrimitive::from_u8(response[item_start + 0x08]).unwrap(),
            quake_medallion:   num_traits::FromPrimitive::from_u8(response[item_start + 0x09]).unwrap(),
            lantern:           num_traits::FromPrimitive::from_u8(response[item_start + 0x0A]).unwrap(),
            hammer:            num_traits::FromPrimitive::from_u8(response[item_start + 0x0B]).unwrap(),
            flute_shovel:      FluteShovelFlags::from_bits(response[item_start + 0x0C]).unwrap(),
            net:               num_traits::FromPrimitive::from_u8(response[item_start + 0x0D]).unwrap(),
            book:              num_traits::FromPrimitive::from_u8(response[item_start + 0x0E]).unwrap(),
            bottle:            response[item_start + 0x0F],
            cane_somaria:      num_traits::FromPrimitive::from_u8(response[item_start + 0x10]).unwrap(),
            cane_byrna:        num_traits::FromPrimitive::from_u8(response[item_start + 0x11]).unwrap(),
            cape:              num_traits::FromPrimitive::from_u8(response[item_start + 0x12]).unwrap(),
            mirror:            num_traits::FromPrimitive::from_u8(response[item_start + 0x13]).unwrap(),

            gloves:            num_traits::FromPrimitive::from_u8(response[item_start + 0x14]).unwrap(),
            boots:             num_traits::FromPrimitive::from_u8(response[item_start + 0x15]).unwrap(),
            flippers:          num_traits::FromPrimitive::from_u8(response[item_start + 0x16]).unwrap(),
            moon_pearl:        num_traits::FromPrimitive::from_u8(response[item_start + 0x17]).unwrap(),

            sword_level:       num_traits::FromPrimitive::from_u8(response[item_start + 0x19]).unwrap(),
            shield_level:      num_traits::FromPrimitive::from_u8(response[item_start + 0x1A]).unwrap(),
            armor_level:       num_traits::FromPrimitive::from_u8(response[item_start + 0x1B]).unwrap(),

            bottle_content1:   num_traits::FromPrimitive::from_u8(response[item_start + 0x1C]).unwrap(),
            bottle_content2:   num_traits::FromPrimitive::from_u8(response[item_start + 0x1D]).unwrap(),
            bottle_content3:   num_traits::FromPrimitive::from_u8(response[item_start + 0x1E]).unwrap(),
            bottle_content4:   num_traits::FromPrimitive::from_u8(response[item_start + 0x1F]).unwrap(),

            // Rupees are spread across two bytes, as the randomizer lifted the
            // 255 Rupee limit, and it's stored little-endian.
            rupees:            ((response[item_start + 0x21] as u16) << 8) + response[item_start + 0x20] as u16,
            heart_quarters:    response[item_start + 0x2B],
            bomb_capacity:     response[item_start + 0x30] + 10,
            hearts:            response[item_start + 0x2D],
            max_hearts:        response[item_start + 0x2C],

            arrows:            response[item_start + 0x37],
            arrow_capacity:    response[item_start + 0x31] + 30,

            small_keys:        if response[item_start + 0x2F] == 0xFF {     0 } else { response[item_start + 0x2F] },
            big_key:           BigKey::default(),
            //if response[item_start + 0x2F] == 0xFF { false } else { response[item_start + 0x26] > 0 };
            // 0x27 -> PoD Big Key

            magic_progression: num_traits::FromPrimitive::from_u8(response[item_start + 0x3B]).unwrap(),
        };

        println!("Game State: {:#?}", &game_state);

        // println!("Items: {:02X} {:02X} {:02X} {:02X} {:02X}", &game_state.bow, &game_state.boomerang, &game_state.hook_shot, &game_state.bomb, &game_state.shroom_powder);
        // println!("       {:02X} {:02X} {:02X} {:02X} {:02X}", &game_state.fire_rod, &game_state.ice_rod, &game_state.bombos_medallion, &game_state.ether_medallion, &game_state.quake_medallion);
        // println!("       {:02X} {:02X} {:02X} {:02X} {:02X}", &game_state.lantern, &game_state.hammer, &game_state.flute_shovel, &game_state.net, &game_state.book);
        // println!("       {:02X} {:02X} {:02X} {:02X} {:02X}", &game_state.bottle, &game_state.cane_somaria, &game_state.cane_byrna, &game_state.cape, &game_state.mirror);
        // println!("\nAbilities: {:02X} {:02X} {:02X} {:02X}", &game_state.boots, &game_state.gloves, &game_state.flippers, &game_state.moon_pearl);
        // println!("\nProgression: {:02X} {:02X} {:02X}", &game_state.sword_level, &game_state.shield_level, &game_state.armor_level);
        // println!("\nBottles: {:02X} {:02X} {:02X} {:02X}", &game_state.bottle_content1, &game_state.bottle_content2, &game_state.bottle_content3, &game_state.bottle_content4);
        // println!("\nRupees: {}", &game_state.rupees);
        // println!("Bombs: {}/{}", &game_state.bomb, &game_state.bomb_capacity);
        // println!("Arrows: {}/{}", &game_state.arrows, &game_state.arrow_capacity);
        // println!("Hearts: {:01.2}/{}", (game_state.hearts as f32) / (8 as f32), game_state.max_hearts / 8);
        // println!("Heart quarters: {}", &game_state.heart_quarters);
        // println!("Magic: {}", if game_state.magic_progression == 0 { "Normal" } else
        //                       if game_state.magic_progression == 1 { "1/2"    } else
        //                       if game_state.magic_progression == 2 { "1/4"    } else
        //                                                            { "WTF"    });
        // println!("\nKeys: small: {}", &game_state.small_keys);
        // println!("");

        { *GAME_STATE.lock().unwrap() = game_state; }
    }
}

fn read_wram<T: SerialPort>(port: &mut T, mem_offset: u32, mem_size: u32) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::with_capacity(512);
    buf.extend_from_slice("USBA".as_bytes());
    buf.resize(512, 0);
    buf[4] = ServerOpcode::GET as u8; // opcode
    buf[5] = ServerSpace::SNES as u8; // space
    buf[6] = ServerFlag::NONE  as u8; // flags
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
       result[4] != ServerOpcode::RESPONSE as u8 {

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

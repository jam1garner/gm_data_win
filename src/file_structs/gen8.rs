use binrw::{derive_binread, BinReaderExt};
use nom::{IResult, error::ErrorKind};
use super::{PosSlice, PosCursor};
use modular_bitfield::prelude::*;
use chrono::naive::NaiveDateTime;

#[derive_binread]
#[derive(Debug, Clone)]
pub struct Gen8 {
    pub unk: u32,
    pub internal_name_offset: u32,
    pub config_name_offset: u32,
    pub unk2: u32,
    pub unk3: u32, // 10000000 by default
    pub unk4: u32,
    pub unk5: [u8; 0x10], // always 0'd out?
    pub internal_name_offset2: u32,
    pub unk6: u32, // gamemaker major version?
    pub unk7: [u32; 3],
    pub window_size: (u32, u32),
    pub game_options: GameOptions,
    pub unk8: u32, // crc32?
    pub unk9: u32,
    pub unk10: [u32; 3],

    #[br(map = |time: u64| NaiveDateTime::from_timestamp(time as i64, 0))]
    pub build_time: NaiveDateTime,

    // display name
    pub game_name_offset: u32,
    pub unk12: u32,
    pub unk13: u32,

    // more crcs?
    pub unk14: [u32; 3],
    pub server_port: u32,

    #[br(temp)]
    pub number_count: u32,

    #[br(count = number_count)]
    pub numbers: Vec<u32>, // ????

    pub nonsense: [u8; 0x40],
}

// 0x0001 - start fullscreen
// 0x0002 - use synchronization to avoid tearing (vsync)
// 0x0004 - ?
// 0x0008 - interpolate colors between pixels 
// 0x0010 - keep aspect ratio
// 0x0020 - display cursor
// 0x0040 - allow window resize
// 0x0080 - allow fullscreen switching
// 0x0100 - ?
// 0x0200 - ?
// 0x0400 - ?
// 0x0800 - ?
// 0x1000 - ?
// 0x2000 - save location (0 - local appdata, 1 - appdata)
// 0x4000 - borderless window

#[derive_binread]
#[br(map = Self::from_bytes)]
#[bitfield]
#[derive(Debug, Clone)]
pub struct GameOptions {
    pub start_fullscreen: bool,
    pub use_vsync: bool,
    #[skip] __: bool,
    pub interpolate_colors: bool,
    pub keep_aspect_ratio: bool,
    pub display_cursor: bool,
    pub allow_window_resize: bool,
    pub allow_fullscreen_switching: bool,
    #[skip] __: B5,
    pub save_location: SaveLocation,
    pub borderless_window: bool,
    #[skip] __: B17,
}

#[derive(Debug, BitfieldSpecifier, Clone, Copy)]
pub enum SaveLocation {
    LocalAppData,
    AppData,
}

impl super::ParseSection for Gen8 {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let mut cursor = PosCursor::from(input.clone());

        if let Ok(gen8) = cursor.read_le() {
            Ok((input, gen8))
        } else {
            Err(nom::Err::Error((input, ErrorKind::ParseTo)))
        }
    }
}

use binrw::{derive_binread, BinRead, BinReaderExt, io::SeekFrom};
use nom::{IResult, error::ErrorKind};
use super::{PosSlice, PosCursor, ptr_list};

impl super::ParseSection for Room {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let mut cursor = PosCursor::from(input.clone());

        if let Ok(room) = dbg!(cursor.read_le()) {
            Ok((input, room))
        } else {
            Err(nom::Err::Error((input, ErrorKind::ParseTo)))
        }
    }
}

#[derive(BinRead, Debug, Clone)]
pub struct Room {
    #[br(parse_with = ptr_list)]
    pub entries: Vec<RoomEntry>,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct RoomEntry {
    pub name_offset: u32,
    pub caption_offset: u32,
    pub width: u32,
    pub height: u32,
    pub speed: u32,
    #[br(map = gm_bool)]
    pub persistent: bool, //boolean
    pub argb: u32,
    #[br(map = gm_bool)]
    pub draw_bg_color: bool, //boolean
    pub _unk1: u32,
    pub flags: u32, // Room Entry Flags (enableViews=1, ShowColor=2, ClearDisplayBuffer=4)

    #[br(temp)]
    pub bgs_offset: u32, //Offsets to the list<t> later

    #[br(temp)]
    pub views_offset: u32,

    #[br(temp)]
    pub objs_offset: u32,

    #[br(temp)]
    pub tiles_offset: u32,

    pub world: u32,
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub meters_per_pixel: f32,

    #[br(temp)]
    pub unk_pointer: u32,

    //#[br(temp)]
    pub unk_pointer2: u32,

    #[br(seek_before = SeekFrom::Start(bgs_offset as u64), parse_with = ptr_list)]
    pub backgrounds: Vec<Background>,

    #[br(seek_before = SeekFrom::Start(views_offset as u64), parse_with = ptr_list)]
    pub views: Vec<View>,

    #[br(seek_before = SeekFrom::Start(objs_offset as u64), parse_with = ptr_list)]
    pub game_objects: Vec<GameObject>,

    #[br(seek_before = SeekFrom::Start(tiles_offset as u64), parse_with = ptr_list)]
    pub tiles: Vec<Tile>,

    #[br(seek_before = SeekFrom::Start(unk_pointer as u64), parse_with = ptr_list)]
    pub unk: Vec<Unk>,

    //#[br(seek_before = SeekFrom::Start(unk_pointer2 as u64), parse_with = ptr_list)]
    //pub unk2: Vec<Unk2>,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct Unk {
    pub name_offset: u32,
    pub unks: [u32; 9]
}

//#[derive_binread]
//#[derive(Debug, Clone)]
//pub struct Unk2 {
//}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct Background {
    #[br(map = gm_bool)]
    pub enabled: bool,
    #[br(map = gm_bool)]
    pub foreground: bool,
    pub bg_def_index: u32,
    pub x: u32,
    pub y: u32,
    #[br(map = gm_bool)]
    pub tile_x: bool,
    #[br(map = gm_bool)]
    pub tile_y: bool,
    pub speed_x: u32,
    pub speed_y: u32,
    pub object_id: i32,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct View {
    #[br(map = gm_bool)]
    pub enabled: bool,
    pub view_x: i32,
    pub view_y: i32,
    pub port_x: i32,
    pub port_y: i32,
    pub port_width: i32,
    pub port_height: i32,
    pub border_x: u32,
    pub border_y: i32,
    pub speed_x: u32,
    pub speed_y: u32,
    pub object_id: i32,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct GameObject {
    pub x: i32,
    pub y: i32,
    pub bg_def_index: i32,
    pub instance_id: i32,
    pub creation_code_id: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub argb_tint: u32,
    pub rotation: f32,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub bg_def_index: i32,
    pub source_x: i32,
    pub source_y: i32,  
    pub width: u32,
    pub height: u32,
    pub tile_depth: i32,
    pub instance_id: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub argb_tint: u32,
}

fn gm_bool(var: u32) -> bool {
    var != 0
}

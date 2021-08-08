use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Room {
    pub rooms: Vec<RoomEntry>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_f32};

fn get_room_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, RoomEntry> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    RoomEntry::take(input)
}

impl super::ParseSection for Room {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let rooms =
            offsets.iter()
            .map(|offset|{
                get_room_entry_at_offset(input, *offset)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|a| a.1)
            .collect::<Vec<_>>();

        let locations = offsets.iter()
            .enumerate()
            .map(|(a,b)| (*b, a))
            .collect::<HashMap<u32, usize>>();

        Ok((input, Self {
            rooms, locations
        }))
    }
}

type Rect32 = (Point32, Point32);
type Point32 = (u32, u32);
type Bool32 = (u32);

#[derive(Debug, Clone)]
pub struct Backgrounds {
    pub enabled: Bool32,
    pub foreground: Bool32,
    pub bg_def_index: u32,
    pub position: Point32,
    pub tile_x: Bool32,
    pub tile_y: Bool32,
    pub speed_x: u32,
    pub speed_y: u32,
    pub object_id: i32,
}

#[derive(Debug, Clone)]
pub struct Views {
    pub enabled: Bool32,
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

#[derive(Debug, Clone)]
pub struct GameObjects {
    pub x: i32,
    pub y: i32,
    pub bg_def_index: i32,
    pub instance_id: i32,
    pub creation_code_id: i32,  // to CODE (-1 for none) -> gml_RoomCC_<name>_<CreationCodeID>
    pub scale_x: f32,
    pub scale_y: f32,
    pub argb_tint: u32,
    pub rotation: f32,
}

#[derive(Debug, Clone)]
pub struct Tiles {
    pub x: i32,
    pub y: i32,
    pub bg_def_index: i32,
    pub source_x: i32,
    pub source_y: i32,  
    pub size: Point32,
    pub tile_depth: i32,
    pub instance_id: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub argb_tint: u32,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub name_offset: u32,
    pub caption_offset: u32,
    pub size: Point32,
    pub speed: u32,
    pub persistent: Bool32, //boolean
    pub argb: u32,
    pub draw_bg_color: Bool32, //boolean
    pub _unk1: u32,
    pub flags: u32, // Room Entry Flags (enableViews=1, ShowColor=2, ClearDisplayBuffer=4)
    pub bg_offset: u32, //Offsets to the list<t> later
    pub obj_offset: u32,
    pub view_offset: u32,
    pub tile_offset: u32,
    pub world: u32,
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
    pub gravity_x: f32,
    pub gravity_y: f32,
    pub meters_per_pixel: f32,
    pub backgrounds: Vec<Backgrounds>,
    pub views: Vec<Views>,
    pub game_objects: Vec<GameObjects>,
    pub tiles: Vec<Tiles>,
}

pub fn take_point32(input: PosSlice) -> IResult<PosSlice, Point32> {
    tuple((le_u32, le_u32))(input)
}

pub fn take_rect32(input: PosSlice) -> IResult<PosSlice, Rect32> {
    tuple((take_point32, take_point32))(input)
}

impl RoomEntry {
    pub fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, (
            name_offset,
            size,
            bounds,
            _unk6,
            origin,
            unk3,
            _unk7,
            some_float_count,
        )) = tuple((
            le_u32,
            take_point32,
            take_rect32,
            count(le_u32, 5),
            take_point32,
            le_u32,
            le_u32,
            le_u32,
        ))(input)?;

        let (input, unk_floats) = count(le_f32, some_float_count as _)(input)?;

        let (input, (
            opacity_maybe,
            unk5,
            frame_count,
        ))= tuple((
            le_f32,
            le_u32,
            le_u32
        ))(input)?;

        let (input, tpag_offsets) = count(le_u32, frame_count as _)(input)?;
        //let (input, trans_count) = le_u32(input)?;
        //let trans_count = trans_count & 0xffff;

        //let data_size = (((size.0 * size.1) + 7) / 8) as usize;
        //let transparencies =
        //    if some_float_count == 0 {
        //        Vec::from(&input.1[..data_size * trans_count as usize])
        //            .chunks(data_size)
        //            .map(Vec::from)
        //            .collect::<Vec<_>>()
        //    } else {
        //        Vec::new()
        //    };

        Ok((
            input,
            RoomEntry {
                name_offset,
                size,
                bounds,
                origin,
                unk3: unk3 as i32,
                opacity_maybe,
                unk5,
                tpag_offsets,
                //transparencies,
                unk_floats,
            }
        ))
    }
}

impl Room {
    pub fn get(&self, loc: u32) -> Option<&RoomEntry> {
        Some(&self.rooms[*self.locations.get(&loc)?])
    }
}

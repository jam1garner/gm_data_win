use binrw::{derive_binread, BinRead, BinReaderExt};
use nom::{IResult, error::ErrorKind};
use super::{PosSlice, PosCursor, ptr_list};

use std::fmt;

impl super::ParseSection for Objt {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let mut cursor = PosCursor::from(input.clone());

        if let Ok(objt) = cursor.read_le() {
            Ok((input, objt))
        } else {
            Err(nom::Err::Error((input, ErrorKind::ParseTo)))
        }
    }
}

#[derive(BinRead, Debug, Clone)]
pub struct Objt {
    #[br(parse_with = ptr_list)]
    pub entries: Vec<ObjtEntry>,
}

#[derive(BinRead, Debug, Clone)]
pub struct ObjtEntry {
    pub name_offset: u32,
    pub sprite_index: i32,

    #[br(map = gm_bool)]
    pub is_visible: bool,
    
    #[br(map = gm_bool)]
    pub is_solid: bool,

    pub depth: i32,

    #[br(map = gm_bool)]
    pub is_persistent: bool,

    pub parent_index: i32,
    pub texture_mask_index: i32,

    #[br(map = gm_bool)]
    pub uses_physics: bool,

    #[br(map = gm_bool)]
    pub is_sensor: bool,

    pub physics: ObjtPhysics,

    #[br(parse_with = ptr_list)]
    pub unk_list: Vec<Unk>,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct ObjtPhysics {
    pub shape: CollisionShape,
    pub density: f32,
    pub restitution: f32,
    pub collision_group: u32,
    pub linear_damping: f32,
    pub angular_damping: f32,

    #[br(temp)]
    pub collision_point_count: u32,

    pub friction: f32,

    #[br(map = gm_bool)]
    pub start_awake: bool,

    #[br(map = gm_bool)]
    pub kinematic: bool,

    #[br(count = collision_point_count)]
    pub collision_points: Vec<Vec2>,
}

#[derive(Debug, BinRead, Clone)]
pub struct Unk {
    #[br(parse_with = ptr_list)]
    unk_list: Vec<Unk2>,
}

#[derive(Debug, BinRead, Clone)]
pub struct Unk2 {
    unk: u32,

    #[br(parse_with = ptr_list)]
    unk_list: Vec<Unk3>,
}

#[derive_binread]
#[derive(Debug, Clone)]
pub struct Unk3 {
    #[br(temp)]
    mask_count: u32,

    #[br(count = mask_count)]
    mask_offets: Vec<u32>,

    #[br(temp)]
    unk_count: u32,

    #[br(count = unk_count)]
    unk_indices: Vec<u32>,

    unk_index: i32,
    unk: u32,
    unk2: u32,
    unk3: u32,
}

#[derive(BinRead, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec2")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

#[derive(BinRead, Clone, Debug)]
#[br(repr(u32))]
pub enum CollisionShape {
    Circle = 0,
    Box = 1,
    ConvexShape = 2,
}

fn gm_bool(var: u32) -> bool {
    var != 0
}

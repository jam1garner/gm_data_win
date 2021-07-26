use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sprt {
    pub sprites: Vec<SprtEntry>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_f32};

fn get_sprt_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, SprtEntry> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    SprtEntry::take(input)
}

impl super::ParseSection for Sprt {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let sprites =
            offsets.iter()
            .map(|offset|{
                get_sprt_entry_at_offset(input, *offset)
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
            sprites, locations
        }))
    }
}

type Rect32 = (Point32, Point32);
type Point32 = (u32, u32);

#[derive(Debug, Clone)]
pub struct SprtEntry {
    pub name_offset: u32,
    pub size: Point32,
    pub bounds: Rect32,
    //pub unk2: [u32; 5],
    pub origin: Point32,
    pub unk3: i32,
    //pub unk4: [u32; 2],
    pub opacity_maybe: f32,
    pub unk5: u32,
    pub tpag_offsets: Vec<u32>,
    //pub transparencies: Vec<Vec<u8>>,
    pub unk_floats: Vec<f32>,
}

pub fn take_point32(input: PosSlice) -> IResult<PosSlice, Point32> {
    tuple((le_u32, le_u32))(input)
}

pub fn take_rect32(input: PosSlice) -> IResult<PosSlice, Rect32> {
    tuple((take_point32, take_point32))(input)
}

impl SprtEntry {
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
            SprtEntry {
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

impl Sprt {
    pub fn get(&self, loc: u32) -> Option<&SprtEntry> {
        Some(&self.sprites[*self.locations.get(&loc)?])
    }
}

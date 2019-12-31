use std::collections::HashMap;

#[derive(Debug)]
pub struct Tpag {
    pub texture_pages: Vec<TpagEntry>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_u16};

fn get_tpag_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, TpagEntry> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    TpagEntry::take(input)
}

impl super::ParseSection for Tpag {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let texture_pages =
            offsets.iter()
            .map(|offset|{
                get_tpag_entry_at_offset(input, *offset)
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
            texture_pages, locations
        }))
    }
}

type Rect16 = (Point16, Point16);
type Point16 = (u16, u16);

#[derive(Debug)]
pub struct TpagEntry {
    pub sprite_bounds: Rect16,
    pub unk2: Rect16,
    pub size: Point16,
    pub texture_index: u16,
}

pub fn take_point16(input: PosSlice) -> IResult<PosSlice, Point16> {
    tuple((le_u16, le_u16))(input)
}

pub fn take_rect16(input: PosSlice) -> IResult<PosSlice, Rect16> {
    tuple((take_point16, take_point16))(input)
}

impl TpagEntry {
    pub fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, (
            sprite_bounds,
            unk2,
            size,
            texture_index
        )) = tuple((
            take_rect16,
            take_rect16,
            take_point16,
            le_u16
        ))(input)?;

        Ok((
            input,
            TpagEntry {
                sprite_bounds,
                unk2,
                size,
                texture_index
            }
        ))
    }
}

impl Tpag {
    pub fn get(&self, loc: u32) -> Option<&TpagEntry> {
        Some(&self.texture_pages[*self.locations.get(&loc)?])
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Font {
    pub fonts: Vec<FontEntry>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_f32, le_u16};

fn get_font_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, FontEntry> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    let (input, (
        name,
        font_name,
        size,
        unk,
        entire_font_tpag,
        unk2,
        unk3,
        char_count
    )) = tuple((
        le_u32,
        le_u32,
        le_f32,
        tuple((le_u32, le_u32, le_u32, le_u32)),
        le_u32,
        tuple((le_f32, le_f32)),
        le_u32,
        le_u32,
    ))(input)?;

    let (input, offsets) = count(le_u32, char_count as usize)(input)?;

    let chars =
        offsets.iter()
        .map(|offset|{
            get_char_entry_at_offset(input, *offset)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|(_, a)| a)
        .collect::<Vec<_>>();

    Ok((input, FontEntry {
        name,
        font_name,
        size,
        unk,
        entire_font_tpag,
        unk2,
        unk3,
        chars
    }))
}

fn get_char_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, FontChar> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    let (input, (
        character,
        bounds,
        origin
    )) = tuple((
        le_u16,
        tuple((
            tuple((le_u16, le_u16)),
            tuple((le_u16, le_u16)),
        )),
        tuple((le_u16, le_u16)),
    ))(input)?;

    let character: char = std::char::decode_utf16(std::iter::once(character))
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap()[0];

    Ok((input, FontChar{
        character,
        bounds,
        origin
    }))
}

type Rect16 = (Point16, Point16);
type Point16 = (u16, u16);

#[derive(Debug, Clone)]
pub struct FontChar {
    pub character: char,
    pub bounds: Rect16,
    pub origin: Point16,
}

#[derive(Debug, Clone)]
pub struct FontEntry {
    pub name: u32,
    pub font_name: u32,
    pub size: f32,
    pub unk: (u32, u32, u32, u32),
    pub entire_font_tpag: u32,
    pub unk2: (f32, f32),
    pub unk3: u32,
    pub chars: Vec<FontChar>,
}

impl super::ParseSection for Font {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let fonts =
            offsets.iter()
            .map(|offset|{
                get_font_entry_at_offset(input, *offset)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(_, a)| a)
            .collect::<Vec<_>>();

        let locations = offsets.iter()
            .enumerate()
            .map(|(a,b)| (*b, a))
            .collect::<HashMap<u32, usize>>();

        Ok((input, Self {
            fonts, locations
        }))
    }
}

impl Font {
    pub fn get(&self, loc: u32) -> Option<&FontEntry> {
        Some(&self.fonts[*self.locations.get(&loc)?])
    }
}

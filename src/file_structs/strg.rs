use std::collections::HashMap;

#[derive(Debug)]
pub struct Strg {
    pub strings: Vec<String>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_u8};

fn get_strg_string_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, String> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);
    let (input, char_count) = le_u32(input)?;
    let (input, chars) = count(le_u8, char_count as _)(input)?;

    Ok((input, std::str::from_utf8(&chars).unwrap().to_string()))
}

impl super::ParseSection for Strg {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let strings =
            offsets.iter()
            .map(|offset|{
                get_strg_string_at_offset(input, *offset)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|a| a.1)
            .collect::<Vec<_>>();

        let locations = offsets.iter()
            .enumerate()
            .map(|(a,b)| (b + 4, a))
            .collect::<HashMap<u32, usize>>();

        Ok((input, Strg {
            strings, locations
        }))
    }
}

impl Strg {
    pub fn get(&self, loc: u32) -> Option<&String> {
        Some(&self.strings[*self.locations.get(&loc)?])
    }
}

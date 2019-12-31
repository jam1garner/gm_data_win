use std::collections::HashMap;

#[derive(Debug)]
pub struct Audo {
    pub files: Vec<Vec<u8>>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_u8};

fn get_sond_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, Vec<u8>> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    take_audo_entry(input)
}

impl super::ParseSection for Audo {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let files =
            offsets.iter()
            .map(|offset|{
                get_sond_entry_at_offset(input, *offset)
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
            files, locations
        }))
    }
}

pub fn take_audo_entry(input: PosSlice) -> IResult<PosSlice, Vec<u8>> {
    let (input, size) = le_u32(input)?;
    count(le_u8, size as _)(input)
}

impl Audo {
    pub fn get(&self, loc: u32) -> Option<&Vec<u8>> {
        Some(&self.files[*self.locations.get(&loc)?])
    }
}

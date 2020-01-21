use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Agrp {
    pub audio_groups: Vec<u32>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count};
use super::PosSlice;
use super::byte_parsers::le_u32;

fn get_agrp_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, u32> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    le_u32(input)
}

impl super::ParseSection for Agrp {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let audio_groups =
            offsets.iter()
            .map(|offset|{
                get_agrp_entry_at_offset(input, *offset)
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
            audio_groups, locations
        }))
    }
}

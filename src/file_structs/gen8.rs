#[derive(Debug, Clone)]
pub struct Gen8 {
    window_size: (u32, u32),
    game_name_offset: u32,
}

use nom::{IResult, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::le_u32;

impl super::ParseSection for Gen8 {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (_, window_size) = tuple((le_u32, le_u32))(input.slice(0x3C, input.1.len()))?;
        let (_, game_name_offset) = le_u32(input.slice(0x64, input.1.len()))?;
        
        Ok((input, Gen8 {
            window_size, game_name_offset
        }))
    }
}

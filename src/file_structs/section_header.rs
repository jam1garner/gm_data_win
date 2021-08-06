use nom::{
    IResult,
    bytes::complete::{tag, take},
    sequence::tuple,
};

use super::PosSlice;
use super::byte_parsers::le_u32;

pub fn take_section(magic: &'static [u8]) -> impl Fn(PosSlice) -> IResult<PosSlice, PosSlice> {
    move |input: PosSlice| {
        let (input, (tag0, file_size)) = tuple((tag(magic), le_u32))(input)?;
        let magic0 = std::str::from_utf8(tag0.1).unwrap();
        //dbg!(magic0);
        Ok(take(file_size)(input)?)
    }
}

pub trait ParseSection: Sized {
    fn take<'a>(input: PosSlice) -> IResult<PosSlice, Self>;
}

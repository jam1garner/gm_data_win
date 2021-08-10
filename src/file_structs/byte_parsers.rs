mod nom_impl {
    pub use nom::number::complete::*;
}

use super::PosSlice;
use nom::IResult;
use nom::error::*;
use nom::Err;

pub fn le_u32(i: PosSlice) -> IResult<PosSlice, u32> {
    let input = i.1;
    nom_impl::le_u32(input)
        .map(|(_, val)| (i.slice(4, i.1.len()), val))
        .map_err(|_: nom::Err<()>|{
            Err::Error(make_error(i, ErrorKind::Eof))}
        )
}

pub fn le_f32(i: PosSlice) -> IResult<PosSlice, f32> {
    let input = i.1;
    nom_impl::le_f32(input)
        .map(|(_, val)| (i.slice(4, i.1.len()), val))
        .map_err(|_: nom::Err<()>|{
            Err::Error(make_error(i, ErrorKind::Eof))}
        )
}

pub fn le_u16(i: PosSlice) -> IResult<PosSlice, u16> {
    let input = i.1;
    nom_impl::le_u16(input)
        .map(|(_, val)| (i.slice(2, i.1.len()), val))
        .map_err(|_: nom::Err<()>|{
            Err::Error(make_error(i, ErrorKind::Eof))}
        )
}

pub fn le_u8(i: PosSlice) -> IResult<PosSlice, u8> {
    let input = i.1;
    nom_impl::le_u8(input)
        .map(|(_, val)| (i.slice(1, i.1.len()), val))
        .map_err(|_: nom::Err<()>|{
            Err::Error(make_error(i, ErrorKind::Eof))}
        )
}

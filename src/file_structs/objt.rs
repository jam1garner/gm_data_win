use binrw::{BinRead, BinReaderExt};

use nom::{IResult, error::ErrorKind};
use super::{PosSlice, PosCursor, ptr_list};

impl super::ParseSection for Objt {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let mut cursor = PosCursor::from(input.clone());

        if let Ok(objt) = cursor.read_le() {
            //dbg!(&objt);
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
    pub sprite_index: u32,
}

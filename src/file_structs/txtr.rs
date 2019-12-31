use std::collections::HashMap;

#[derive(Debug)]
pub struct Txtr {
    pub files: Vec<TxtrEntry>,
    pub locations: HashMap<u32, usize>,
}

use std::iter;
use nom::{IResult, multi::count, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::{le_u32};

fn get_txtr_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, (u32, u32, u32)> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    tuple((le_u32, le_u32, le_u32))(input)
}

impl super::ParseSection for Txtr {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let files =
            offsets.iter()
            .map(|offset|{
                get_txtr_entry_at_offset(input, *offset)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|a| a.1)
            .collect::<Vec<_>>();

        let pos = input.pos();
        let png_offsets = files.iter()
            .map(|a| a.2 as usize - pos)
            .chain(iter::once(input.len()))
            .collect::<Vec<_>>();

        let pngs = png_offsets[..files.len()]
            .iter()
            .zip(png_offsets[1..].iter())
            .map(|(&start, &end)| Vec::from(&input.1[start..end]))
            .collect::<Vec<_>>();
        
        let files = files.into_iter()
            .zip(pngs.into_iter())
            .map(|((unk1, unk2, _), png)| {
                TxtrEntry {
                    unk1, unk2, png
                }
            })
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

impl Txtr {
    pub fn get(&self, loc: u32) -> Option<&Vec<u8>> {
        Some(&self.files[*self.locations.get(&loc)?].png)
    }
}

#[derive(Debug, Clone)]
pub struct TxtrEntry {
    pub unk1: u32, // either 0 or 1
    pub unk2: u32, // always 0
    pub png: Vec<u8>,
}

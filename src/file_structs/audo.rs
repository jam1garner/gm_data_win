use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Audo {
    pub offset: u32,
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
        let offset = input.pos() as u32 - 8;
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
            offset, files, locations
        }))
    }
}

pub fn take_audo_entry(input: PosSlice) -> IResult<PosSlice, Vec<u8>> {
    let (input, size) = le_u32(input)?;
    count(le_u8, size as _)(input)
}

use std::io::prelude::*;

impl Audo {
    pub fn get(&self, loc: u32) -> Option<&Vec<u8>> {
        Some(&self.files[*self.locations.get(&loc)?])
    }

    pub fn write_to<W: Write>(&self, f: &mut W, mut pos: u32) -> std::io::Result<()> {
        f.write_all(b"AUDO")?;
        let count = self.files.len() as u32;
        let files_size: u32 = self.files.iter().map(|a| ((a.len() as u32 + 3) & !3) + 4).sum();
        let section_size: u32 = 4 + (4 * count) + files_size;
        f.write_all(&section_size.to_le_bytes())?;
        f.write_all(&count.to_le_bytes())?;
        pos += 0xC + (4 * count);
        let padding_amounts: Vec<_> = self.files.iter().map(|file|{
            f.write_all(&pos.to_le_bytes());
            let file_size = file.len() as u32;
            pos += ((file_size + 3) & !3) + 4;
            (((file_size + 3) & !3) - file_size) as usize
        }).collect();
        for (file, padding_amount) in self.files.iter().zip(padding_amounts.into_iter()) {
            f.write_all(&(file.len() as u32).to_le_bytes())?;
            f.write_all(file)?;
            f.write_all(&vec![0; padding_amount])?;
        }

        Ok(())
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Sond {
    pub sounds: Vec<SondEntry>,
    pub locations: HashMap<u32, usize>,
}

use nom::{IResult, multi::count, sequence::tuple};
use super::PosSlice;
use super::byte_parsers::{le_u32, le_f32};

fn get_sond_entry_at_offset(input: PosSlice, offset: u32) -> IResult<PosSlice, SondEntry> {
    let off = (offset as usize) - input.pos();
    let input = input.offset(off);

    SondEntry::take(input)
}

impl super::ParseSection for Sond {
    fn take(input: PosSlice) -> IResult<PosSlice, Self> {
        let (input, index_count) = le_u32(input)?;
        let (input, offsets) = count(le_u32, index_count as _)(input)?;

        let sounds =
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
            sounds, locations
        }))
    }
}

#[derive(Debug)]
pub struct SondEntry {
    pub name_offset: u32,
    pub unk1: u32, // Always zero
    pub unk2: u32, // Always zero
    pub name_offset2: u32,
    pub unk3: u32, // Always zero
    pub play_speed: f32,
    pub unk4: u32, // Always zero
    pub audiogroup_index: u32,
    pub index_in_audiogroup: u32,
}

impl SondEntry {
    pub fn take(input: PosSlice) -> IResult<PosSlice, SondEntry> {
        let (input, (
            name_offset,
            unk1,
            unk2,
            name_offset2,
            unk3,
            play_speed,
            unk4,
            audiogroup_index,
            index_in_audiogroup,
        )) = tuple((
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_f32,
            le_u32,
            le_u32,
            le_u32,
        ))(input)?;

        Ok((
            input,
            SondEntry {
                name_offset,
                unk1,
                unk2,
                name_offset2,
                unk3,
                play_speed,
                unk4,
                audiogroup_index,
                index_in_audiogroup,
            }
        ))
    }
}

impl Sond {
    pub fn get(&self, loc: u32) -> Option<&SondEntry> {
        Some(&self.sounds[*self.locations.get(&loc)?])
    }
}

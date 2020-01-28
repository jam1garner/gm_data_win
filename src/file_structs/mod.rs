mod track_slice;
mod section_header;
mod gen8;
mod strg;
mod sond;
mod audo;
mod txtr;
mod tpag;
mod sprt;
mod font;
mod agrp;
mod byte_parsers;

use section_header::{take_section, ParseSection};
use nom::IResult;
use track_slice::PosSlice;
use byte_parsers::le_u32;
use std::path::Path;

macro_rules! define_sections {
    ($enum_name:ident,
        {
            $(
                ($magic:expr, $section:ident, $section_inner:ident, $take_section:ident)
            ),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $enum_name {
            $(
                $section($section_inner),
            )*
            Unk {
                name: String,
                data: Vec<u8>
            }
        }

        use nom::{
            sequence::tuple,
            branch::alt,
            bytes::complete::take,
        };

        impl $enum_name {
            fn _take_unk(input: PosSlice) -> IResult<PosSlice, Self> {
                let (input, (name, size)) = tuple((take(4usize), le_u32))(input)?;

                let name = std::str::from_utf8(name.1).unwrap().to_string();

                let (input, data) = take(size as usize)(input)?;
                let data = data.1.to_vec();

                Ok((
                    input,
                    Self::Unk {
                        name,
                        data
                    }
                ))
            }

            $(
                fn $take_section(input: PosSlice) -> IResult<PosSlice, Self>{
                    let (remains, input) = take_section($magic)(input)?;
                    let (_, section) = $section_inner::take(input)?;
                    Ok((
                        remains,
                        Self::$section(
                            section
                        )
                    ))
                }
            )*

            pub fn take(input: PosSlice) -> IResult<PosSlice, Self> {
                Ok(alt((
                    $(
                        Self::$take_section,
                    )*
                    Self::_take_unk
                ))(input)?)
            }
        }
    }
}

pub use gen8::*;
pub use strg::*;
pub use sond::*;
pub use audo::*;
pub use txtr::*;
pub use tpag::*;
pub use sprt::*;
pub use font::*;
pub use agrp::*;

define_sections!{
    Section,
    {
        (b"GEN8", Gen8, Gen8, _gen8),
        (b"STRG", Strg, Strg, _strg),
        (b"SOND", Sond, Sond, _sond),
        (b"AUDO", Audo, Audo, _audo),
        (b"TXTR", Txtr, Txtr, _txtr),
        (b"TPAG", Tpag, Tpag, _tpag),
        (b"SPRT", Sprt, Sprt, _sprt),
        (b"FONT", Font, Font, _font),
        (b"AGRP", Agrp, Agrp, _agrp),
    }
}

pub fn take_data_win_file(input: &[u8]) -> Vec<Section> {
    let input = PosSlice::from_slice(input);

    let (_, input) = take_section(b"FORM")(input).unwrap();
    

    nom::multi::many0(Section::take)(input).unwrap().1
}

//#[cfg(textures)]
use {
    image::{
        DynamicImage,
        GenericImageView
    },
    std::sync::Arc,
    lazy_init::Lazy,
};

#[derive(Default)]
pub struct FormFile {
    pub audos: Vec<Audo>,
    pub strg: Option<Strg>,
    pub sond: Option<Sond>,
    pub txtr: Option<Txtr>,
    pub tpag: Option<Tpag>,
    pub sprt: Option<Sprt>,
    pub font: Option<Font>,
    pub agrp: Option<Agrp>,
    //#[cfg(textures)]
    pub textures: Vec<Lazy<Arc<DynamicImage>>>,
}

impl FormFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let data = std::fs::read(path.as_ref()).unwrap();
        FormFile::from_sections(take_data_win_file(&data))
    }

    pub fn from_sections(sections: Vec<Section>) -> Self {
        let mut file = FormFile::default();

        for section in sections {
            match section {
                Section::Audo(audo) => {
                    file.audos.push(audo)
                }
                Section::Strg(strg) => {
                    file.strg = Some(strg)
                }
                Section::Sond(sond) => {
                    file.sond = Some(sond)
                }
                Section::Txtr(txtr) => {
                    file.textures = (0..txtr.files.len()).map(|_| Lazy::new()).collect();
                    file.txtr = Some(txtr);
                }
                Section::Tpag(tpag) => {
                    file.tpag = Some(tpag);
                }
                Section::Sprt(sprt) => {
                    file.sprt = Some(sprt);
                }
                Section::Font(font) => {
                    file.font = Some(font);
                }
                _ => {}
            }
        }

        file
    }
    
    //#[cfg(textures)]
    pub fn get_texture(&self, index: usize) -> Arc<DynamicImage> {
        Arc::clone(self.textures[index].get_or_create(||{
            println!("Loading texture {}...", index);
            Arc::new(image::load_from_memory_with_format(
                &self.txtr.as_ref().unwrap().files[index].png,
                image::ImageFormat::PNG
            ).unwrap())
        }))
    }

    pub fn get_tpag_info(&self, loc: u32) -> (((u16, u16), (u16, u16)), usize) {
        let tpag = self.tpag.as_ref().unwrap().get(loc).unwrap();
        (tpag.sprite_bounds, tpag.texture_index as usize)
    }

    pub fn get_tpag_subimage(&self, loc: u32) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
        let (((x, y), (w, h)), texture_index) = self.get_tpag_info(loc);
        self.get_texture(texture_index).view(
            x as u32, y as u32, w as u32, h as u32
        ).to_image()
    }

    pub fn get_tpag_from_name_and_frame(&self, sprite_name: &str, frame: usize) -> Option<(((u16, u16), (u16, u16)), usize)> {
        let offset = self.name_to_offset(sprite_name)? as u32;

        Some(self.get_tpag_info(
            self.sprt.as_ref().unwrap().sprites.iter().find(
                |sprite| sprite.name_offset == offset
            )?.tpag_offsets.get(frame).copied()?
        ))
    }

    pub fn name_to_offset(&self, name: &str) -> Option<u32> {
        let strg = self.strg.as_ref().unwrap();
        let index = strg.strings.iter().position(|string| string == name)?;
        strg.locations.get_by_right(&index).copied()
    }
}

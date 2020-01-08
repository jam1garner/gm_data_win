use std::fs;
use gm_data_win::take_data_win_file;
use gm_data_win::file_structs::Section;
use gm_data_win::file_structs::FormFile;
use image::GenericImage;

fn extension_from_magic(magic: &[u8]) -> &'static str {
    match magic {
        b"RIFF" => {
            "wav"
        }
        b"OggS" => {
            "ogg"
        }
        _ => {
            "bin"
        }
    }
}

fn main() {
    #[cfg(feature = "test")]
    let files = vec![
        &include_bytes!("/home/jam/a/re/rivals/data.win")[..],
        &include_bytes!("/home/jam/a/re/rivals/audiogroup1.dat")[..],
        &include_bytes!("/home/jam/a/re/rivals/audiogroup2.dat")[..],
        &include_bytes!("/home/jam/a/re/rivals/audiogroup3.dat")[..],
        &include_bytes!("/home/jam/a/re/rivals/audiogroup4.dat")[..],
    ];
    #[cfg(not(feature = "test"))]
    let files = vec![];

    let a = files.into_iter()
        .map(|bytes| take_data_win_file(bytes).into_iter())
        .flatten()
        .collect::<Vec<_>>();

    for i in &a {
        match i {
            Section::Unk {
                name, ..
            } => {
                dbg!(name);
            }

            Section::Strg(_) => {
                dbg!("STRG");
            }
            Section::Audo(_) => {
                dbg!("AUDO");
            }
            Section::Sond(_) => {
                dbg!("SOND");
            }
            Section::Tpag(_) => {
                dbg!("TPAG");
            }
            Section::Sprt(_) => {
                dbg!("SPRT");
            }

            Section::Txtr(txtr) => {
                dbg!("TXTR");
                //println!("Txtr = {:?}", txtr.files.iter().map(|a| (a.unk1, a.unk2)).collect::<Vec<_>>())
            }

            a => {dbg!(a);}
        }
    }

    let file = FormFile::from_sections(a);
    if cfg!(feature = "sounds") {
        let strg = file.strg.as_ref().unwrap();
        let sond = file.sond.as_ref().unwrap();
        let _ = fs::create_dir("sounds");
        for sound in &sond.sounds {
            let name = sound.name_offset;
            if sound.index_in_audiogroup == 0xFFFF_FFFF {
                continue
            }
            let file = &file.audos[sound.audiogroup_index as usize].files[sound.index_in_audiogroup as usize];
            
            let name = strg.get(name).unwrap();
            
            std::fs::write(
                &format!(
                    "sounds/{}.{}",
                    name,
                    extension_from_magic(&file[..4])
                ),
                file
            ).unwrap();
        }
    }

    if cfg!(feature = "textures") {
        let txtr = file.txtr.as_ref().unwrap();
        let _ = fs::create_dir("textures");
        for (i, texture) in txtr.files.iter().enumerate() {
            let _ = std::fs::write(&format!("textures/{}_{}.png", i, texture.unk1), &texture.png);
        }
    }

    if cfg!(feature = "sprites") {
        let sprt = file.sprt.as_ref().unwrap();
        let strg = file.strg.as_ref().unwrap();
        let _ = fs::create_dir("sprites");
        for sprite in &sprt.sprites {
            let name = strg.get(sprite.name_offset).unwrap();
            println!("Saving '{}'...", name);
            let _ = fs::create_dir(&format!("sprites/{}", name));
            for (i, &tpag) in sprite.tpag_offsets.iter().enumerate() {
                file.get_tpag_subimage(tpag)
                    .save_with_format(
                        &format!("sprites/{}/{}_{}.png", name, name, i),
                        image::ImageFormat::PNG
                    ).unwrap();
            }
        }
    }

    if cfg!(feature = "font") {
        let strg = file.strg.as_ref().unwrap();
        let font = file.font.as_ref().unwrap();
        let _ = fs::create_dir("fonts");
        for font in &font.fonts {
            let name = strg.get(font.font_name).unwrap();
            let alias = strg.get(font.name).unwrap();
            println!("Saving font '{}' (alias: '{}')", name, alias);
            let _ = fs::create_dir(&format!("fonts/{}", name));
            let mut font_sheet = file.get_tpag_subimage(font.entire_font_tpag);
            for font_char in &font.chars {
                let ((x, y), (w, h)) = font_char.bounds;
                if w == 0 {
                    println!("Warning: font '{}', character '{}' is zero-width", name, font_char.character);
                    continue
                }
                font_sheet.sub_image(
                    x as _, y as _, w as _, h as _
                ).to_image().save_with_format(
                    &format!("fonts/{}/{}.png", name, font_char.character),
                    image::ImageFormat::PNG
                ).unwrap();
            }
        }
    }
}



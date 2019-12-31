use std::fs;
use gm_data_win::take_data_win_file;
use gm_data_win::file_structs::Section;
use gm_data_win::file_structs::FormFile;

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

            a @ _ => {dbg!(a);}
        }
    }

    let file = FormFile::from_sections(a);
    let strg = file.strg.as_ref().unwrap();
    let sond = file.sond.as_ref().unwrap();
    let txtr = file.txtr.as_ref().unwrap();
    let sprt = file.sprt.as_ref().unwrap();
    let tpag = file.tpag.as_ref().unwrap();
    use std::collections::HashSet;
    let mut values: HashSet<&[u8]> = HashSet::new();
    //fs::create_dir("sounds");
    for sound in &sond.sounds {
        let name = sound.name_offset;
        /*if sound.index_in_audiogroup == 0xFFFF_FFFF {
            continue
        }
        let file = &file.audos[sound.audiogroup_index as usize].files[sound.index_in_audiogroup as usize];
        */
        let name = strg.get(name).unwrap();
        
        //std::fs::write(&format!("sounds/{}.{}", name, extension_from_magic(&file[..4])), file);
    }
    dbg!(values);

    let mut textures = vec![];

    //fs::create_dir("textures");
    for (i, texture) in txtr.files.iter().enumerate() {
        //std::fs::write(&format!("textures/{}_{}.png", i, texture.unk1), &texture.png);
        println!("Loading texture {}...", i);
        textures.push(
            image::load_from_memory_with_format(
                &texture.png,
                image::ImageFormat::PNG
            ).unwrap()
        );
    }

    //fs::create_dir("sprites");
    for sprite in &sprt.sprites {
        let name = strg.get(sprite.name_offset).unwrap();
        println!("Saving '{}'...", name);
        let tpags = sprite.tpag_offsets.iter()
                        .map(|&off| tpag.get(off).unwrap())
                        .collect::<Vec<_>>();
        fs::create_dir(&format!("sprites/{}", name));
        for (i, tpag) in tpags.iter().enumerate() {
            let ((x, y), (w, h)) = tpag.sprite_bounds;
            let sprite = textures[tpag.texture_index as usize].crop(
                x as u32, y as u32, w as u32, h as u32
            );
            sprite.save_with_format(&format!("sprites/{}/{}_{}.png", name, name, i), image::ImageFormat::PNG).unwrap();
        }
    }
}



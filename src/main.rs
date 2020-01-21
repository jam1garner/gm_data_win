use std::io::{prelude::*, SeekFrom};
use image::{GenericImage, GenericImageView};
use rayon::prelude::*;
use std::fs;
use std::iter;
use std::collections::{BTreeSet, BTreeMap};
use gm_data_win::take_data_win_file;
use gm_data_win::file_structs::{FormFile, Txtr, TxtrEntry, SondEntry};

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
    let mut args = Args::from_args();

    args.audio_groups.get_or_insert_with(|| vec![
        "audiogroup1.dat".into(),
        "audiogroup2.dat".into(),
        "audiogroup3.dat".into(),
        "audiogroup4.dat".into(),
    ]);

    if !args.extract_sprites && !args.extract_textures && !args.extract_fonts &&
        !args.extract_audio && !args.mod_sprites && !args.mod_audio {
        args.mod_audio = true;
        args.mod_sprites = true;
    }

    let files: Vec<Vec<u8>> = iter::once(&args.data_win)
        .chain(args.audio_groups.as_ref().unwrap().iter())
        .map(|path| fs::read(path).unwrap_or_else(|_| panic!("Failed to read file '{}'", path)))
        .collect();

    let path = &args.data_win;
    let data_win = take_data_win_file(&fs::read(path).unwrap_or_else(
            |_| panic!("Failed to read file '{}'", path)
        ));

    let sections = files
        .into_iter()
        .map(|bytes| take_data_win_file(&bytes).into_iter())
        .chain(iter::once(data_win.clone().into_iter()))
        .flatten()
        .collect::<Vec<_>>();

    let mut file = FormFile::from_sections(sections);

    if args.extract_audio {
        let strg = file.strg.as_ref().unwrap();
        let sond = file.sond.as_ref().unwrap();
        let sounds_folder = format!("{}/sounds", args.originals_folder);
        let _ = fs::create_dir_all(&sounds_folder);
        for sound in &sond.sounds {
            let name = sound.name_offset;
            if sound.index_in_audiogroup == 0xFFFF_FFFF {
                continue
            }
            let file = &file.audos[sound.audiogroup_index as usize].files[sound.index_in_audiogroup as usize];
            
            let name = strg.get(name).unwrap();
            
            std::fs::write(
                &format!(
                    "{}/{}.{}",
                    sounds_folder,
                    name,
                    extension_from_magic(&file[..4])
                ),
                file
            ).unwrap();
        }
    }

    if args.extract_textures {
        let txtr = file.txtr.as_ref().unwrap();
        let textures_folder = format!("{}/sprites", args.originals_folder);
        let _ = fs::create_dir_all(&textures_folder);
        let textures = txtr.files.iter().enumerate().collect::<Vec<_>>();
        textures.par_iter().for_each(|(i, texture)| {
            let _ = std::fs::write(
                &format!("{}/{}_{}.png", textures_folder, i, texture.unk1),
                &texture.png
            );
        });
    }

    if args.extract_sprites {
        let sprt = file.sprt.as_ref().unwrap();
        let strg = file.strg.as_ref().unwrap();
        let sprites_folder = format!("{}/sprites", args.originals_folder);
        let _ = fs::create_dir(&sprites_folder);
        sprt.sprites.par_iter().for_each(|sprite| {
            let name = strg.get(sprite.name_offset).unwrap();
            println!("Saving '{}'...", name);
            let _ = fs::create_dir_all(&format!("{}/{}", sprites_folder, name));
            let tpags = sprite.tpag_offsets.iter().enumerate().collect::<Vec<_>>();
            tpags.par_iter().for_each(|(i, &tpag)| {
                file.get_tpag_subimage(tpag)
                    .save_with_format(
                        &format!("{}/{}/{}.png", sprites_folder, name, i),
                        image::ImageFormat::PNG
                    ).unwrap();
            });
        });
    }

    if args.extract_fonts {
        let strg = file.strg.as_ref().unwrap();
        let font = file.font.as_ref().unwrap();
        let fonts_folder = format!("{}/fonts", args.originals_folder);
        let _ = fs::create_dir_all(&fonts_folder);
        for font in &font.fonts {
            let name = strg.get(font.font_name).unwrap();
            let alias = strg.get(font.name).unwrap();
            println!("Saving font '{}' (alias: '{}')", name, alias);
            let _ = fs::create_dir(&format!("{}/{}", fonts_folder, name));
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
                    &format!("{}/{}/{}.png", fonts_folder, name, font_char.character),
                    image::ImageFormat::PNG
                ).unwrap();
            }
        }
    }

    let mut f = fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&args.data_win)
                    .unwrap();

    if args.mod_sprites {
        let sprites_folder = format!("{}/sprites", args.mod_folder);

        if let Ok(sprites_dir) = fs::read_dir(&sprites_folder) {
            let sprites_dir = sprites_dir.collect::<Result<Vec<_>, _>>().unwrap();

            let anims = sprites_dir.par_iter().filter_map(|anim| {
                let anim_dir = anim.path();
                if anim_dir.is_dir() {
                    let anim_name = anim.file_name();
                    let frames = fs::read_dir(anim_dir).unwrap().collect::<Result<Vec<_>, _>>()
                        .unwrap()
                        .par_iter()
                        .map(|frame|{
                            let filename = frame.file_name();
                            let filename = filename.to_str().unwrap();
                            let frame_num: usize = filename.parse()
                                .unwrap_or_else(|_| -> usize {
                                    filename[anim_name.len() + 1..filename.len() - 4].parse()
                                        .unwrap_or_else(|_| panic!(
                                            "Invalid filename: '{:?}', use format [num].png",
                                            frame.path()
                                        ))
                                });
                                
                            (frame_num, image::open(frame.path()).unwrap())
                        })
                        .collect::<Vec<_>>();
                    Some((anim_name, frames))
                } else {
                    None
                }
            }).collect::<Vec<_>>();

            let needed_edits = anims.par_iter().map(|(anim_name, frames)| {
                let anim_name = anim_name.to_str().unwrap().to_string();
                frames.par_iter()
                    .filter_map(|(frame_num, image)|{
                        let name = format!("Sprite '{}' frame {}", anim_name, frame_num);
                        match file.get_tpag_from_name_and_frame(&anim_name, *frame_num) {
                            Some(tpag_data) => Some((tpag_data, image, name)),
                            None => {
                                println!(
                                    "Warning: {} not found in data.win but exists in mods folder",
                                    name
                                );
                                None
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_par_iter()
            }).flatten().collect::<Vec<_>>();

            let needed_textures: BTreeSet<usize> = needed_edits.iter().map(|edit| (edit.0).1).collect();

            let mut textures_to_edit = needed_textures
                .into_par_iter()
                .map(|texture_num|{
                    let txtr = file.txtr.as_ref().unwrap();

                    let texture = image::load_from_memory_with_format(
                                        &txtr.files[texture_num].png,
                                        image::ImageFormat::PNG
                                    ).unwrap_or_else(|_| {
                                        panic!("Texture {} failed to load.", )
                                    });
                    
                    (texture_num, texture)
                })
                .collect::<BTreeMap<usize, _>>();

            // Apply edits to textures
            for ((((x, y), (w, h)), texture_num), sprite, name) in needed_edits {
                let ((x, y), (w, h)) = ((x as u32, y as u32), (w as u32, h as u32));
                if sprite.dimensions() != (w, h) {
                    println!(
                        "Warning: {} incorrect size. Should be ({}, {}), is ({}, {})",
                        name,
                        w, h, sprite.width(), sprite.height()
                    );
                }
                textures_to_edit
                    .get_mut(&texture_num)
                    .unwrap()
                    .copy_from(
                        &sprite.view(0, 0, w, h),
                        x,
                        y
                    );
            }

            if let FormFile { txtr: Some(Txtr { ref offset, ref files, .. }), .. } = file {
                let new_files: Vec<_> = files
                    .into_par_iter()
                    .enumerate()
                    .map(|(i, &TxtrEntry { unk1, unk2, ref png })|{
                        let png = textures_to_edit
                            .get(&i)
                            .map(|texture| {
                                let mut buffer = Vec::with_capacity(0x8_0000);
                                texture.write_to(&mut buffer, image::ImageFormat::PNG).unwrap();
                                buffer
                            })
                            .unwrap_or_else(|| remove_padding(png.clone()));
                        TxtrEntry {
                            unk1, unk2, png
                        }
                    })
                    .collect();

                let loc = f.seek(SeekFrom::Start((offset + 0xC + (4 * new_files.len())) as u64)).unwrap();
                let mut file_pos = (loc as usize + (0xC * new_files.len())) as u32;
                for &TxtrEntry { ref unk1, ref unk2, ref png } in &new_files {
                    f.write(&unk1.to_le_bytes()).unwrap();
                    f.write(&unk2.to_le_bytes()).unwrap();
                    f.write(&file_pos.to_le_bytes()).unwrap();
                    file_pos += png.len() as u32;
                }
                for TxtrEntry { png, .. } in new_files {
                    f.write(&png).unwrap();
                }
                let padding = ((file_pos + 0x1f) & !0x1f) - file_pos;
                // Rewrite TXTR size
                let loc = f.seek(SeekFrom::Start((offset + 4) as u64)).unwrap() as u32;
                f.write(&((file_pos + padding) - (loc + 4)).to_le_bytes()).unwrap();
                // Write TXTR padding
                f.seek(SeekFrom::Start(file_pos as u64)).unwrap() as u32;
                f.write(&vec![0; padding as usize]).unwrap();
            }
        }

    } /*else if args.mod_textures {
        todo!()
    }

    if args.mod_fonts {
        todo!()
    }*/

    if args.mod_audio {
        let audio_folder = format!("{}/sounds", args.mod_folder);
        if let Ok(audio_dir) = fs::read_dir(&audio_folder) {
            let audio_dir = audio_dir.collect::<Result<Vec<_>, _>>().unwrap();
            let sounds = audio_dir.par_iter().filter_map(|sound| {
                let sound_dir = sound.path();
                if !sound_dir.is_dir() {
                    let name_offset = file.name_to_offset(sound.path().file_stem()?.to_str()?)?;
                    let &SondEntry {
                        audiogroup_index, index_in_audiogroup, ..
                    } = file.sond.as_ref().unwrap().sounds
                        .iter()
                        .find(|entry| entry.name_offset == name_offset)?;
                    let data = fs::read(sound.path()).ok()?;
                    Some(((audiogroup_index as usize, index_in_audiogroup as usize), data))
                } else {
                    None
                }
            }).collect::<Vec<_>>();
            
            for ((audiogroup_index, index_in_audiogroup), data) in sounds {
                *file.audos[audiogroup_index].files.get_mut(index_in_audiogroup).unwrap() = data;
            }

            for (file, audo) in args.audio_groups.unwrap().iter().zip(file.audos[1..].iter()) {
                let mut audio = fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(file)
                        .unwrap();
                audio.seek(SeekFrom::Start(8)).unwrap();
                audo.write_to(&mut audio, 8).unwrap();
                let file_size = audio.seek(SeekFrom::Current(0)).unwrap();
                audio.set_len(file_size).unwrap();
                audio.seek(SeekFrom::Start(4)).unwrap();
                audio.write(&(file_size as u32 - 8).to_le_bytes()).unwrap();
            }
            if !args.mod_sprites {
                f.seek(SeekFrom::Start(file.audos[0].offset as u64)).unwrap();
            }
        }
    }

    // NOTE: must be seeked to where you want the AUDO section written beforehand!!
    if args.mod_audio | args.mod_sprites {
        let pos = f.seek(SeekFrom::Current(0)).unwrap() as u32;
        // Write AUDO section back (since it's been pushed back)
        file.audos[0].write_to(&mut f, pos).unwrap();
        // Rewrite FORM size
        let form_size = f.seek(SeekFrom::Current(0)).unwrap() as u32 - 8;
        f.seek(SeekFrom::Start(4)).unwrap();
        f.write_all(&form_size.to_le_bytes()).unwrap();
        f.set_len(form_size as u64 + 8).unwrap();
    }

    let mut stdout = std::io::stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    std::io::stdin().read(&mut [0]).unwrap();
}

fn remove_padding(mut data: Vec<u8>) -> Vec<u8> {
    if !data.is_empty() {
        for i in (0..data.len()).rev() {
            if data[i] != 0u8 {
                data.truncate(i+1);
                return data;
            }
        }
    }
    vec![]
}

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(short = "a", long)]
    extract_audio: bool,

    #[structopt(short = "s", long)]
    extract_sprites: bool,

    #[structopt(short = "f", long)]
    extract_fonts: bool,

    #[structopt(short = "t", long)]
    extract_textures: bool,

    #[structopt(short = "A", long)]
    mod_audio: bool,

    #[structopt(short = "S", long)]
    mod_sprites: bool,

    //#[structopt(short = "F", long)]
    //mod_fonts: bool,

    //#[structopt(short = "T", long)]
    //mod_textures: bool,

    #[structopt(short, long, long, default_value = "mods")]
    mod_folder: String,

    #[structopt(short, long, default_value = "files")]
    originals_folder: String,

    #[structopt(default_value = "data.win")]
    data_win: String,

    #[structopt(long)]
    audio_groups: Option<Vec<String>>
}

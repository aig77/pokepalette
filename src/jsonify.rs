use rand::Rng;
use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use serde_json;
use std::path::Path;
use std::collections::HashMap;
use crate::sprite::{Sprite, RegionalVariant};
use crate::color::Rgb;

use super::Flags;

pub fn read_sprites_json(filename: &str, flags: &Flags) -> Result<Vec<Sprite>, Box<dyn Error>>{
    let mut file = fs::File::open(filename)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let sprites: Vec<Sprite> = serde_json::from_str::<Vec<Sprite>>(&json_data)?
        .into_iter()
        .filter(|sprite| {
            let include_sprite = 
            (!flags.no_shiny || !sprite.shiny) &&
            (!flags.no_female || !sprite.female) &&
            (!flags.no_mega || !sprite.mega) &&
            (!flags.no_regional_variant || {
                match sprite.regional_variant {
                    RegionalVariant::Regular => true,
                    _ => false
                }
            });

            include_sprite
        }).collect();

    Ok(sprites)
}

pub fn generate_sprites_json(sprite_dir: &str, json_name: &str, size: usize) {

    // let mut rng = rand::thread_rng();

    let sprites = get_sprites(Path::new(sprite_dir))
        .expect("error reading sprites");

    // add random color to sprite's Palette to pad up to size of wallpaper palette
    // let color_counts = get_color_counts(&sprites);
    // let color_counts_len = color_counts.len();

    // for sprite in &mut sprites {
    //     let palette = &mut sprite.palette;
    //     while palette.len() < size {
    //         let r = rng.gen_range(0..color_counts_len);
    //         palette.colors.push(color_counts[r].0.clone());
    //     }
    // }

    let json_data = match serde_json::to_string_pretty(&sprites) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("error serializing sprites to JSON: {}", err);
            return;
        }
    };

    let mut file = match fs::File::create(Path::new(json_name)) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error creating file: {}", err);
            return;
        }
    };

    if let Err(err) = file.write_all(json_data.as_bytes()) {
        eprintln!("error writing JSON to file: {}", err);
    }
}

fn get_sprites(dir: &Path) -> Result<Vec<Sprite>, Box<dyn Error>> {
    let mut sprites = vec![];

    fn visit_dirs(dir: &Path, sprites: &mut Vec<Sprite>) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, sprites)?;
            } else if let Some(extension) = path.extension() {
                if extension == "png" {
                    sprites.push(Sprite::new(&path))
                }
            }
        }
        
        Ok(())
    }

    visit_dirs(dir, &mut sprites)?;

    Ok(sprites)
}

fn get_color_counts(sprites: &Vec<Sprite>) -> Vec<(Rgb<u8>, u32)> {
    let mut colormap: HashMap<Rgb<u8>, u32> = HashMap::new();

    for sprite in sprites {
        for color in &sprite.palette.colors {
            let count = colormap.entry(*color).or_insert(0);
            *count += 1;
        }
    }

    let mut entries: Vec<(Rgb<u8>, u32)> = colormap.into_iter().collect();
    entries.sort_by_key(|&(_, count)| count);

    entries
}

fn get_most_common_colors(color_counts: Vec<(Rgb<u8>, u32)>) -> Vec<(Rgb<u8>, u32)>  {

    let mut median_counts = vec![];
    let mid = color_counts.len() / 2;
    let median_value = color_counts[mid].1;

    median_counts.push(color_counts[mid].clone());
    let mut i = mid as isize - 1;
    while i >= 0 && color_counts[i as usize].1 == median_value {
        median_counts.push(color_counts[i as usize].clone());
        i -= 1;
    }

    median_counts
}

// let sums = self.palette.iter()
    //     .fold((0, 0, 0), |acc, color| {
    //         (acc.0 + color.r as u32, acc.1 + color.g as u32, acc.2 + color.b as u32)
    //     });

    // Rgb {
    //     r: (sums.0 / self.len() as u32) as u8,
    //     g: (sums.1 / self.len() as u32) as u8,
    //     b: (sums.2 / self.len() as u32) as u8
    // }

    // let colors: Vec<Rgb<u8>> = sprites.iter()
    //     .flat_map(|s| s.palette.palette.clone().into_iter())
    //     .collect();

    // let total_colors = sprites.clone()
    //     .into_iter()
    //     .fold(0, |acc, sprite| acc + sprite.palette.palette.len());
use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use serde_json;
use std::path::Path;
use crate::sprite::{Sprite, RegionalVariant};

use super::Flags;

const JSON_PATH: &str = "./data/sprites.json";
const SPRITES_PATH: &str = "./data/pokemon-gen8";

pub fn read_sprites_json(flags: &Flags) -> Result<Vec<Sprite>, Box<dyn Error>>{
    let mut file = fs::File::open(JSON_PATH)?;
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

pub fn generate_sprites_json() {

    let sprites = get_sprites(Path::new(SPRITES_PATH))
        .expect("error reading sprites");

    let json_data = match serde_json::to_string_pretty(&sprites) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("error serializing sprites to JSON: {}", err);
            return;
        }
    };

    let mut file = match fs::File::create(Path::new(JSON_PATH)) {
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
mod rgb;
mod sprite;
mod jsonify;
mod wal;

use std::fs;
use std::path::Path;
use std::collections::HashMap;
// use rgb::Rgb;

use rgb::Rgb;
use sprite::Sprite;
use wal::get_wal_scheme;
// use jsonify::generate_pokeschemes_json;

fn main() {
    // get wal scheme

    // get pokemon scheme map

    // calc distances from wal scheme to all instances in pokemon scheme map

    // grab k smallest distances

    // return them

    // TODO:
    // create file with pokemon schemes stored for faster access
    // create distance function
    // create cli tools

    let wal = get_wal_scheme(8);

    println!("wal: {}\n", generate_scheme_blocks(&wal));

    let sprites = get_sprites(Path::new("./data/pokemon-gen8/regular/"))
        .expect("unable to get pokemon from dir");

    let sprite_map: HashMap<String, &Sprite> = sprites
        .iter()
        .map(|sprite| (sprite.name.clone(), sprite))
        .collect();

    if let Some(sprite) = sprite_map.get("Pikachu") {
        println!("{}", sprite);
    }

    if let Some(sprite) = sprite_map.get("Treecko") {
        println!("{}", sprite);
    }
}

pub fn generate_scheme_blocks(scheme: &Vec<Rgb<u8>>) -> String {
    let mut result = String::new();

    for color in scheme {
        let escape_code: String = color.ansi_color();
        result.push_str(&format!("{}   \x1b[0m", escape_code)); // Add color block with reset ANSI colors
    }

    result
}

fn get_sprites(dir: &Path) -> Result<Vec<Sprite>, std::io::Error> {
    let mut sprites = vec![];

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == "png" {
                sprites.push(Sprite::new(&path));
            }
        }
    }

    Ok(sprites)
}









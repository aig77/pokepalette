#[allow(dead_code)]
mod sprite;

#[allow(dead_code)]
mod quantize;

use serde_json;
use sprite::Sprite;
use std::fs;

const DB_PATH: &str = "assets/pokemon.json";

fn main() {
    let file = fs::File::open(DB_PATH).expect("Failed to open pokemon.json");

    let _sprites: Vec<Sprite> =
        serde_json::from_reader(file).expect("Failed to parse pokemon.json");

    // for sprite in sprites {
    // println!("{}", sprite);
    // }
}

#[allow(dead_code)]
mod sprite;

#[allow(dead_code)]
mod quantize;

use image;
use image::Pixel;
use serde_json;
use sprite::Sprite;
use std::fs;

const DB_PATH: &str = "pokemon.json";

fn main() {
    let file = fs::File::open(DB_PATH).expect("Failed to open pokemon.json");

    let _sprites: Vec<Sprite> =
        serde_json::from_reader(file).expect("Failed to parse pokemon.json");

    let image_path_str = "pikachu.png";
    let rgb = image::open(image_path_str).unwrap().to_rgb8();
    let pixels = rgb.pixels();

    let colors = pixels
        .into_iter()
        .map(|pixel| {
            let color = pixel.to_rgb();
            [color[0] as u8, color[1] as u8, color[2] as u8]
        })
        .collect();

    let palette = quantize::get_palette(&colors, 5, 4, true);

    for weighted_color in palette {
        println!(
            "\x1b[48;2;{};{};{}m   \x1b[0m RGB({:>3}, {:>3}, {:>3}). Freq: {}",
            weighted_color.color[0],
            weighted_color.color[1],
            weighted_color.color[2],
            weighted_color.color[0],
            weighted_color.color[1],
            weighted_color.color[2],
            weighted_color.freq,
        );
    }
}

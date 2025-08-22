#[allow(dead_code)]
mod sprite;

#[allow(dead_code)]
mod quantize;

#[allow(dead_code)]
mod distance;

use pokepalette::{DEFAULT_IGNORE_BLACK, DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};

use image;
use image::Pixel;
use serde_json;
use sprite::Sprite;
use std::fs;

const DB_PATH: &str = "pokemon.json";

fn main() {
    let top_k = 10;

    let file = fs::File::open(DB_PATH).expect("Failed to open pokemon.json");

    let sprites: Vec<Sprite> = serde_json::from_reader(file).expect("Failed to parse pokemon.json");

    let image_path_str = "black-hole.png";
    let rgb = image::open(image_path_str).unwrap().to_rgb8();
    let pixels = rgb.pixels();

    let colors = pixels
        .into_iter()
        .map(|pixel| {
            let color = pixel.to_rgb();
            [color[0] as u8, color[1] as u8, color[2] as u8]
        })
        .collect();

    let image_palette = quantize::get_palette(
        &colors,
        DEFAULT_PALETTE_SIZE,
        DEFAULT_LEVELS,
        DEFAULT_IGNORE_BLACK,
    );

    for weighted_color in &image_palette {
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

    let mut distances: Vec<(Sprite, f32)> = sprites
        .into_iter()
        .map(|sprite| {
            let dist = distance::palette_distance(&sprite.palette, &image_palette);
            (sprite, dist)
        })
        .collect();

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let top: Vec<_> = distances.into_iter().take(top_k).collect();

    for (sprite, distance) in top {
        println!("{} {}\n", sprite, distance);
    }
}

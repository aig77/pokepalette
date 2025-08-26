mod cli;
mod distance;
mod quantize;
mod sprite;

use pokepalette::{DB_PATH, DEFAULT_IGNORE_BLACK, DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};

use anyhow::Result;
use image;
use image::Pixel;
use quantize::{get_palette, WeightedColor};
use serde_json;
use sprite::Sprite;
use std::fs;

fn main() -> Result<()> {
    // Load database
    let file = fs::File::open(DB_PATH).expect("Failed to open pokemon.json");
    let sprites: Vec<Sprite> = serde_json::from_reader(file).expect("Failed to parse pokemon.json");

    // Load CLI config and filters
    let (config, filtered) = cli::get_config_and_filter_sprites(sprites);

    // Generate image palette
    let image_palette = get_image_palette(&config.image)?;

    // Get sprites sorted by distance to image
    let ranked = get_pokemon_ranked(&image_palette, &filtered);

    // Get top k
    let top: Vec<(&Sprite, f32)> = ranked.into_iter().take(config.top_k).collect();

    // Print results
    if config.verbose {
        print_image_information(image_palette);
        print_top_information(&top);
    } else {
        print_result(&top);
    }

    Ok(())
}

fn get_image_palette(path: &str) -> Result<Vec<WeightedColor>> {
    let rgb = image::open(path)?.to_rgb8();
    let pixels = rgb.pixels();

    let colors = pixels
        .into_iter()
        .map(|pixel| {
            let color = pixel.to_rgb();
            [color[0] as u8, color[1] as u8, color[2] as u8]
        })
        .collect();

    Ok(get_palette(
        &colors,
        DEFAULT_PALETTE_SIZE,
        DEFAULT_LEVELS,
        false,
    ))
}

fn get_pokemon_ranked<'a>(
    image_palette: &Vec<quantize::WeightedColor>,
    sprites: &'a Vec<Sprite>,
) -> Vec<(&'a Sprite, f32)> {
    let mut distances: Vec<(&'a Sprite, f32)> = sprites
        .iter()
        .map(|sprite| {
            let dist = distance::palette_distance(&sprite.palette, &image_palette);
            (sprite, dist)
        })
        .collect();

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    distances
}

fn print_result(top: &Vec<(&Sprite, f32)>) {
    for (sprite, _) in top {
        let shiny = if sprite.shiny {
            " (shiny)".to_string()
        } else {
            "".to_string()
        };
        println!("{}{}", sprite.name, shiny);
    }
}

fn print_image_information(image_palette: Vec<WeightedColor>) {
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

    println!("");
}

fn print_top_information(top: &Vec<(&Sprite, f32)>) {
    for (sprite, distance) in top {
        println!("{}\nScore: {}\n", sprite, distance);
    }
}

pub mod distance;
pub mod quantize;

pub use distance::palette_distance;
pub use quantize::{get_palette, WeightedColor};

use crate::sprite::Sprite;
use crate::{DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};
use anyhow::Result;
use image;

pub fn get_image_palette(path: &str) -> Result<Vec<WeightedColor>> {
    // Convert to rgba first to filter transparent pixels
    let rgba = image::open(path)?.to_rgba8();

    let colors: Vec<[u8; 3]> = rgba
        .pixels()
        .filter_map(|pixel| {
            if pixel[3] == 0 {
                // Skip fully transparent pixels
                None
            } else {
                Some([pixel[0], pixel[1], pixel[2]])
            }
        })
        .collect();

    Ok(quantize::get_palette(
        &colors,
        DEFAULT_PALETTE_SIZE,
        DEFAULT_LEVELS,
        false,
    ))
}

pub fn get_pokemon_ranked<'a>(
    image_palette: &Vec<WeightedColor>,
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

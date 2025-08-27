use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct WeightedColor {
    pub color: [u8; 3],
    pub freq: f32,
}

pub fn get_palette(
    pixels: &Vec<[u8; 3]>,
    palette_size: usize,
    levels: usize,
    ignore_black: bool,
) -> Vec<WeightedColor> {
    // Sensible ranges
    if levels < 2 || levels > 16 {
        panic!("levels must be between 2 and 16");
    }

    if palette_size < 1 || palette_size > 10 {
        panic!("palette_size must be between 1 and 10");
    }

    // Check for logical conflicts
    let max_possible_colors = levels * levels * levels;
    if palette_size > max_possible_colors {
        panic!(
            "palette_size ({}) cannot exceed maximum possible quantized colors ({})",
            palette_size, max_possible_colors
        );
    }

    // Check for empty input
    if pixels.is_empty() {
        panic!("cannot generate palette from empty pixel array");
    }

    let bucket_size = (256 / levels) as u8;

    let mut quantized_counts = HashMap::new();
    for pixel in pixels {
        if ignore_black && *pixel == [0, 0, 0] {
            continue;
        }
        let qcolor = quantize_color(pixel, bucket_size);
        *quantized_counts.entry(qcolor).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = quantized_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    sorted
        .into_iter()
        .take(palette_size)
        .map(|(color, count)| WeightedColor {
            color: color,
            freq: count as f32 / pixels.len() as f32,
        })
        .collect()
}

fn quantize_color(color: &[u8; 3], bucket_size: u8) -> [u8; 3] {
    let channel0 = (color[0] / bucket_size) * bucket_size + (bucket_size / 2);
    let channel1 = (color[1] / bucket_size) * bucket_size + (bucket_size / 2);
    let channel2 = (color[2] / bucket_size) * bucket_size + (bucket_size / 2);
    [channel0, channel1, channel2]
}

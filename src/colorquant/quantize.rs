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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantize_color_center_of_bucket() {
        // With bucket_size=64 (4 levels), colors should snap to bucket centers
        let color = [0, 64, 128];
        let quantized = quantize_color(&color, 64);
        // 0 -> bucket 0 -> center at 32
        // 64 -> bucket 1 -> center at 96
        // 128 -> bucket 2 -> center at 160
        assert_eq!(quantized, [32, 96, 160]);
    }

    #[test]
    fn test_quantize_color_same_bucket() {
        // Colors in the same bucket should quantize to the same value
        let color1 = [10, 10, 10];
        let color2 = [50, 50, 50];
        let bucket_size = 64;
        assert_eq!(
            quantize_color(&color1, bucket_size),
            quantize_color(&color2, bucket_size)
        );
    }

    #[test]
    fn test_quantize_color_different_buckets() {
        let color1 = [0, 0, 0];
        let color2 = [255, 255, 255];
        let bucket_size = 64;
        assert_ne!(
            quantize_color(&color1, bucket_size),
            quantize_color(&color2, bucket_size)
        );
    }

    #[test]
    fn test_get_palette_single_color() {
        let pixels = vec![[100, 100, 100]; 100];
        let palette = get_palette(&pixels, 1, 4, false);
        assert_eq!(palette.len(), 1);
        assert_eq!(palette[0].freq, 1.0);
    }

    #[test]
    fn test_get_palette_two_colors_equal_freq() {
        let mut pixels = vec![[0, 0, 0]; 50];
        pixels.extend(vec![[255, 255, 255]; 50]);
        let palette = get_palette(&pixels, 2, 4, false);
        assert_eq!(palette.len(), 2);
        // Both should have ~0.5 frequency
        assert!((palette[0].freq - 0.5).abs() < 0.01);
        assert!((palette[1].freq - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_get_palette_respects_palette_size() {
        let mut pixels = vec![[0, 0, 0]; 40];
        pixels.extend(vec![[128, 128, 128]; 30]);
        pixels.extend(vec![[255, 255, 255]; 30]);
        let palette = get_palette(&pixels, 2, 4, false);
        // Should only return top 2 even though 3 colors exist
        assert_eq!(palette.len(), 2);
    }

    #[test]
    fn test_get_palette_ignore_black() {
        let mut pixels = vec![[0, 0, 0]; 50];
        pixels.extend(vec![[255, 0, 0]; 50]);
        let palette = get_palette(&pixels, 2, 4, true);
        // Black should be ignored, only red remains
        assert_eq!(palette.len(), 1);
        assert_eq!(palette[0].freq, 0.5); // 50 red out of 100 total
    }

    #[test]
    fn test_get_palette_sorted_by_frequency() {
        let mut pixels = vec![[255, 0, 0]; 60]; // 60% red
        pixels.extend(vec![[0, 255, 0]; 40]); // 40% green
        let palette = get_palette(&pixels, 2, 4, false);
        // Red should be first (higher frequency)
        assert!(palette[0].freq > palette[1].freq);
    }

    #[test]
    #[should_panic(expected = "levels must be between 2 and 16")]
    fn test_get_palette_invalid_levels_low() {
        let pixels = vec![[100, 100, 100]];
        get_palette(&pixels, 1, 1, false);
    }

    #[test]
    #[should_panic(expected = "levels must be between 2 and 16")]
    fn test_get_palette_invalid_levels_high() {
        let pixels = vec![[100, 100, 100]];
        get_palette(&pixels, 1, 17, false);
    }

    #[test]
    #[should_panic(expected = "cannot generate palette from empty pixel array")]
    fn test_get_palette_empty_pixels() {
        let pixels: Vec<[u8; 3]> = vec![];
        get_palette(&pixels, 1, 4, false);
    }
}

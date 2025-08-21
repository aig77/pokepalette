use std::collections::HashMap;

pub fn get_palette(
    pixels: &Vec<[u8; 3]>,
    levels: usize,
    ignore_black: bool,
    palette_size: usize,
) -> Vec<[u8; 3]> {
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

    let mut color_counts = HashMap::new();
    for pixel in pixels {
        if ignore_black && *pixel == [0, 0, 0] {
            continue;
        }
        *color_counts.entry(*pixel).or_insert(0) += 1
    }

    let bucket_size = (256 / levels) as u8;

    let mut quantized_counts = HashMap::new();
    for (color, count) in color_counts.iter() {
        let qcolor = quantize_color(color, bucket_size);
        *quantized_counts.entry(qcolor).or_insert(0) += count;
    }

    let mut sorted: Vec<_> = quantized_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    sorted
        .into_iter()
        .take(palette_size)
        .map(|(color, _)| color)
        .collect()
}

fn quantize_color(color: &[u8; 3], bucket_size: u8) -> [u8; 3] {
    let channel0 = (color[0] / bucket_size) * bucket_size + (bucket_size / 2);
    let channel1 = (color[1] / bucket_size) * bucket_size + (bucket_size / 2);
    let channel2 = (color[2] / bucket_size) * bucket_size + (bucket_size / 2);
    [channel0, channel1, channel2]
}

use crate::quantize::WeightedColor;

pub fn palette_distance(palette_a: &Vec<WeightedColor>, palette_b: &Vec<WeightedColor>) -> f32 {
    let distance_a_to_b = directional_distance(palette_a, palette_b);
    let distance_b_to_a = directional_distance(palette_b, palette_a);

    (distance_a_to_b + distance_b_to_a) / 2.0
}

fn directional_distance(from_palette: &Vec<WeightedColor>, to_palette: &Vec<WeightedColor>) -> f32 {
    let mut total_weighted_distance = 0.0;

    for wc_from in from_palette {
        let min_distance = to_palette
            .iter()
            .map(|wc_to| euclidean_distance(&wc_from.color, &wc_to.color))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f32::MAX);

        total_weighted_distance += min_distance * wc_from.freq;
    }

    total_weighted_distance
}

fn euclidean_distance(color1: &[u8; 3], color2: &[u8; 3]) -> f32 {
    let dr = color1[0] as f32 - color2[0] as f32;
    let dg = color1[1] as f32 - color2[1] as f32;
    let db = color1[2] as f32 - color2[2] as f32;
    (dr * dr + dg * dg + db * db).sqrt()
}

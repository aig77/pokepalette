use super::quantize::WeightedColor;
use lab::Lab;

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
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(f32::MAX);

        total_weighted_distance += min_distance * wc_from.freq;
    }

    total_weighted_distance
}

fn euclidean_distance(color1: &[u8; 3], color2: &[u8; 3]) -> f32 {
    // Convert RGB to Lab for more accurate distance calculation
    let lab1 = Lab::from_rgb(color1);
    let lab2 = Lab::from_rgb(color2);
    let dl = lab1.l - lab2.l;
    let da = lab1.a - lab2.a;
    let db = lab1.b - lab2.b;
    (dl * dl + da * da + db * db).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance_identical_colors() {
        let color = [128, 128, 128];
        let distance = euclidean_distance(&color, &color);
        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_euclidean_distance_black_white() {
        let black = [0, 0, 0];
        let white = [255, 255, 255];
        let distance = euclidean_distance(&black, &white);
        // In LAB space, black to white should be ~100 (L channel range)
        assert!(distance > 90.0 && distance < 110.0);
    }

    #[test]
    fn test_euclidean_distance_symmetry() {
        let red = [255, 0, 0];
        let blue = [0, 0, 255];
        let d1 = euclidean_distance(&red, &blue);
        let d2 = euclidean_distance(&blue, &red);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_euclidean_distance_similar_colors_small() {
        let color1 = [100, 100, 100];
        let color2 = [105, 100, 100];
        let distance = euclidean_distance(&color1, &color2);
        // Similar colors should have small distance
        assert!(distance < 5.0);
    }

    #[test]
    fn test_directional_distance_identical_palettes() {
        let palette = vec![
            WeightedColor { color: [255, 0, 0], freq: 0.5 },
            WeightedColor { color: [0, 255, 0], freq: 0.5 },
        ];
        let distance = directional_distance(&palette, &palette);
        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_directional_distance_weights_matter() {
        let palette_a = vec![
            WeightedColor { color: [255, 0, 0], freq: 1.0 },
        ];
        let palette_b = vec![
            WeightedColor { color: [0, 0, 255], freq: 1.0 },
        ];
        let d1 = directional_distance(&palette_a, &palette_b);

        let palette_a_low_weight = vec![
            WeightedColor { color: [255, 0, 0], freq: 0.1 },
        ];
        let d2 = directional_distance(&palette_a_low_weight, &palette_b);

        // Lower weight should result in lower total distance
        assert!(d2 < d1);
    }

    #[test]
    fn test_palette_distance_symmetry() {
        let palette_a = vec![
            WeightedColor { color: [255, 0, 0], freq: 0.7 },
            WeightedColor { color: [0, 255, 0], freq: 0.3 },
        ];
        let palette_b = vec![
            WeightedColor { color: [0, 0, 255], freq: 0.5 },
            WeightedColor { color: [255, 255, 0], freq: 0.5 },
        ];
        let d1 = palette_distance(&palette_a, &palette_b);
        let d2 = palette_distance(&palette_b, &palette_a);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_palette_distance_identical() {
        let palette = vec![
            WeightedColor { color: [128, 64, 32], freq: 0.6 },
            WeightedColor { color: [200, 100, 50], freq: 0.4 },
        ];
        let distance = palette_distance(&palette, &palette);
        assert_eq!(distance, 0.0);
    }
}

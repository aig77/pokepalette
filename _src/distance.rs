use crate::color::Rgb;
use crate::palette::Palette;
use delta_e::*;


pub trait DistanceFn<T> {
    fn color_distance(&self, color1: &Rgb<u8>, color2: &Rgb<u8>) -> T;
    fn palette_distance(&self, palette1: &Palette<Rgb<u8>>, palette2: &Palette<Rgb<u8>>) -> T;
}

#[derive(Clone)]
pub struct EuclideanDistanceFn;
impl DistanceFn<f64> for EuclideanDistanceFn {
    fn color_distance(&self, color1: &Rgb<u8>, color2: &Rgb<u8>) -> f64 {
        let norm1 = color1.normalize();
        let norm2 = color2.normalize();
        let diff_r = (norm1.r - norm2.r).powi(2);
        let diff_g = (norm1.g - norm2.g).powi(2);
        let diff_b = (norm1.b - norm2.b).powi(2);
        diff_r + diff_g + diff_b
    }

    fn palette_distance(&self, palette1: &Palette<Rgb<u8>>, palette2: &Palette<Rgb<u8>>) -> f64 {
        if palette1.len() != palette2.len() {
            panic!("color palettes must be the same length");
        }

        let sum_squared_diff: f64 = palette1.colors.iter().enumerate()
            .fold(0.0, | acc, (i, color1) | {
                let diff = self.color_distance(color1, &palette2.colors[i]);
                acc + diff
            });

            sum_squared_diff.sqrt()
    }

    
}

#[derive(Clone)]
pub struct DE2000DistanceFn;
impl DistanceFn<f64> for DE2000DistanceFn {
    fn color_distance(&self, color1: &Rgb<u8>, color2: &Rgb<u8>) -> f64 {
        let a = &[color1.r, color1.b, color1.g];
        let b = &[color2.r, color2.g, color2.b];
        DE2000::from_rgb(a, b) as f64
    }

    fn palette_distance(&self, palette1: &Palette<Rgb<u8>>, palette2: &Palette<Rgb<u8>>) -> f64 {
        if palette1.len() != palette2.len() {
            panic!("Palettes must be the same length");
        }

        let sum = palette1.colors.iter().enumerate()
            .fold(0.0, |acc, (i, color1)| {
                let color2 = palette2.colors[i];
                acc + DE2000::from_rgb(&[color1.r, color1.b, color1.g], &[color2.r, color2.g, color2.b]) as f64
            });

        sum / palette1.len() as f64
    }
}

#[derive(Clone)]
pub struct MSSDDistanceFn;
impl DistanceFn<f64> for MSSDDistanceFn {
    fn color_distance(&self, color1: &Rgb<u8>, color2: &Rgb<u8>) -> f64 {
        EuclideanDistanceFn.color_distance(color1, color2)
    }

    fn palette_distance(&self, palette1: &Palette<Rgb<u8>>, palette2: &Palette<Rgb<u8>>) -> f64 {
        let n = palette1.len();
        let m = palette2.len();

        let dist_matrix: Vec<Vec<f64>> = palette1.colors.iter()
            .map(|color1| {
                palette2.colors.iter()
                    .map(|color2| {
                        DE2000DistanceFn.color_distance(color1, color2)
                }).collect()
            }).collect();

        let mut min_sum = f64::INFINITY;
        // let mut best_pairing = (0, 0);

        for i in 0..n {
            for j in 0..m {
                let mut total_dist = 0.0;
                for k in 0..std::cmp::min(i+1, j+1) {
                    total_dist += dist_matrix[i-k][j-k];
                }

                if total_dist < min_sum {
                    min_sum = total_dist;
                    // best_pairing = (i, j);
                }
            }
        }

        // (min_sum, best_pairing)
        min_sum
    }
}

// pub fn mssd<D>(palette1: &Palette<Rgb<u8>>, palette2: &Palette<Rgb<u8>>, distance_fn: D) -> (f64, (usize, usize))
// where 
//     D: DistanceFn<f64> + Clone
// {
//     let n = palette1.len();
//     let m = palette2.len();

//     // if n != m {
//     //     panic!("color palettes must be the same length");
//     // }

//     // let dist_matrix = vec![vec![0.0; m+1]; n+1];

//     let dist_matrix: Vec<Vec<f64>> = palette1.colors.iter()
//         .map(|color1| {
//             palette2.colors.iter()
//                 .map(|color2| {
//                     distance_fn.color_distance(color1, color2)
//             }).collect()
//         }).collect();

//     let mut min_sum = f64::INFINITY;
//     let mut best_pairing = (0, 0);

//     for i in 0..n {
//         for j in 0..m {
//             let mut total_dist = 0.0;
//             for k in 0..std::cmp::min(i+1, j+1) {
//                 total_dist += dist_matrix[i-k][j-k];
//             }

//             if total_dist < min_sum {
//                 min_sum = total_dist;
//                 best_pairing = (i, j);
//             }
//         }
//     }

//     (min_sum, best_pairing)
// }

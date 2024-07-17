use std::fmt;
use std::path::Path;
use std::io::Error;
use serde::{Serialize, Deserialize};
use image::{ColorType, DynamicImage};
use color_thief::ColorFormat;
use crate::color::Rgb;

// use lab::Lab;
use delta_e::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette<T> {
    pub colors: Vec<T>
}

impl<T> Palette<T> {
    pub fn new(colors: Vec<T>) -> Self {
        Palette::<T> { colors }
    }
}

impl Palette<Rgb<u8>> {
    pub fn from_img_path(path: &Path) -> Self {
        let img: DynamicImage = image::open(path).unwrap();
        let img = img.to_rgb8();
        let buffer = img.into_raw();
        let color_type = color_thief::ColorFormat::Rgb;
        // let (buffer, color_type) = get_image_buffer(img);

        let mut colors: Vec<Rgb<u8>> = color_thief::get_palette(&buffer, color_type, 10, 9)
            .unwrap()
            .iter()
            .map(|color| Rgb {
                r: color.r,
                g: color.g,
                b: color.b
            })
            .collect();
        
        // fill remaining palettes
        let last = colors[colors.len()-1];
        while colors.len() < 8 {
            colors.push(last);
        }

        Palette::new(colors)
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn euclidean_distance_with_weights(&self, other: &Palette<Rgb<u8>>, weights: &[f64]) -> f64 {
        let sum_squared_diff: f64 = self.colors.iter().enumerate()
            .fold(0.0, | acc, (i, self_color) | {
                let self_norm = self_color.normalize();
                let other_norm = other.colors[i].normalize();
                let diff_r = (self_norm.r - other_norm.r).powi(2) * weights[i];
                let diff_g = (self_norm.g - other_norm.g).powi(2) * weights[i];
                let diff_b = (self_norm.b - other_norm.b).powi(2) * weights[i];
                acc + diff_r + diff_g + diff_b
            });

            sum_squared_diff.sqrt()
    }

    pub fn euclidean_distance(&self, other: &Palette<Rgb<u8>>) -> f64 {
        self.euclidean_distance_with_weights(other, &vec![1.0; self.len()])
    }

    // pub fn to_lab(&self) -> Palette<Lab> {
    //     let mut rgbs = vec![];
    //     for color in &self.colors {
    //         rgbs.push([color.r, color.g, color.b]);
    //     }
    //     Palette::new(lab::rgbs_to_labs(&rgbs))
    // }

    pub fn de2000_distance(&self, other: &Palette<Rgb<u8>>) -> f64 {
        let sum = self.colors.iter().enumerate()
            .fold(0.0, |acc, (i, color)| {
                let other_color = other.colors[i];
                let color1 = [color.r, color.b, color.g];
                let color2 = [other_color.r, other_color.g, other_color.b];
                acc + DE2000::from_rgb(&color1, &color2) as f64
            });
        
        sum / self.len() as f64
    }
}

impl fmt::Display for Palette<Rgb<u8>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for color in &self.colors {
            let escape_code = color.ansi_color();
            result.push_str(&format!("{}   \x1b[0m", escape_code));
        }

        write!(f, "{}", result)?;

        Ok(())
    }   
}

// impl Palette<Lab> {
//     pub fn len(&self) -> usize {
//         self.colors.len()
//     }

//     pub fn ciede2000_distance(&self, other: &Palette<Lab>) -> f64 {
//         if self.len() != other.len() {
//             panic!("palettes must have the same length");
//         }

//         let mut total_distance = 0.0;

//         for i in 0..self.len() {
//             let color1 = self.colors[i];
//             let color2 = other.colors[i];
//             total_distance += DeltaE::new(color1, color2, DE2000);
//         }

//         total_distance / self.len() as f64
//     }
// }

// fn get_image_buffer(img: DynamicImage) -> (Vec<u8>, ColorFormat) {
//     match img.color() {
//         ColorType::Rgb8 => {
//             let buffer = img.to_rgb8();
//             (buffer.to_vec(), ColorFormat::Rgb)
//         }
//         ColorType::Rgba8 => {
//             let buffer = img.to_rgba8();
//             (buffer.to_vec(), ColorFormat::Rgba)
//         }
//         ColorType::L8 => {
//             let buffer = img.to_luma8();
//             let rgba_buffer = buffer
//                 .pixels()
//                 .flat_map(|&pixel| vec![pixel[0], pixel[0], pixel[0], 255])
//                 .collect();
//             (rgba_buffer, ColorFormat::Rgba)
//         }
//         ColorType::La8 => {
//             let buffer = img.to_luma_alpha8();
//             let rgba_buffer = buffer
//                 .pixels()
//                 .flat_map(|pixel| {
//                     let gray = pixel[0];
//                     let alpha = pixel[1];
//                     vec![gray, gray, gray, alpha]
//                 })
//                 .collect();
//             (rgba_buffer, ColorFormat::Rgba)
//         }
//         _ => panic!("Unsupported image type"),
//     }
// }
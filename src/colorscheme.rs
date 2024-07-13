use std::fmt;
use std::path::Path;
use serde::{Serialize, Deserialize};
use color_thief::ColorFormat;
use image::{DynamicImage, ColorType};
use crate::rgb::Rgb;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorScheme {
    scheme: Vec<Rgb<u8>>
}

impl ColorScheme {
    pub fn new(scheme: Vec<Rgb<u8>>) -> Self {
        ColorScheme { scheme }
    }

    pub fn from_img_path(path: &Path) -> Self {
        let img = image::open(path).unwrap();
        let (buffer, color_type) = get_image_buffer(img);

        let mut colors: Vec<Rgb<u8>> = color_thief::get_palette(&buffer, color_type, 10, 9)
            .unwrap()
            .iter()
            .map(|color| Rgb {
                r: color.r,
                g: color.g,
                b: color.b
            })
            .collect();
        
        // pad with black so all schemes are size 8
        while colors.len() < 8 {
            colors.push( Rgb { r: 0, g: 0, b: 0 } );
        }

        ColorScheme::new(colors)
    }

    pub fn len(&self) -> usize {
        self.scheme.len()
    }

    pub fn euclidean_distance_with_weights(&self, other: &ColorScheme, weights: &[f64]) -> f64 {
        let sum_squared_diff = self.scheme.iter().enumerate()
            .fold(0.0, | acc, (i, self_color) | {
                let self_norm = Rgb {
                    r: self_color.r as f64 / 255.0,
                    g: self_color.g as f64 / 255.0,
                    b: self_color.b as f64 / 255.0
                };

                let other_color = &other.scheme[i];
                let other_norm = Rgb {
                    r: other_color.r as f64 / 255.0,
                    g: other_color.g as f64 / 255.0,
                    b: other_color.b as f64 / 255.0
                };

                let diff_r = (self_norm.r - other_norm.r).powi(2) * weights[i];
                let diff_g = (self_norm.g - other_norm.g).powi(2) * weights[i];
                let diff_b = (self_norm.b - other_norm.b).powi(2) * weights[i];

                acc + diff_r + diff_g + diff_b
            });
        
        sum_squared_diff.sqrt()
        
    }

    pub fn euclidean_distance(&self, other: &ColorScheme) -> f64 {
        self.euclidean_distance_with_weights(other, &vec![1.0; self.len()])
    }
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for color in &self.scheme {
            let escape_code = color.ansi_color();
            result.push_str(&format!("{}   \x1b[0m", escape_code));
        }

        write!(f, "{}", result)?;

        Ok(())
    }   
}

fn get_image_buffer(img: DynamicImage) -> (Vec<u8>, ColorFormat) {
    match img.color() {
        ColorType::Rgb8 => {
            let buffer = img.to_rgb8();
            (buffer.to_vec(), ColorFormat::Rgb)
        }
        ColorType::Rgba8 => {
            let buffer = img.to_rgba8();
            (buffer.to_vec(), ColorFormat::Rgba)
        }
        ColorType::L8 => {
            let buffer = img.to_luma8();
            let rgba_buffer = buffer
                .pixels()
                .flat_map(|&pixel| vec![pixel[0], pixel[0], pixel[0], 255])
                .collect();
            (rgba_buffer, ColorFormat::Rgba)
        }
        ColorType::La8 => {
            let buffer = img.to_luma_alpha8();
            let rgba_buffer = buffer
                .pixels()
                .flat_map(|pixel| {
                    let gray = pixel[0];
                    let alpha = pixel[1];
                    vec![gray, gray, gray, alpha]
                })
                .collect();
            (rgba_buffer, ColorFormat::Rgba)
        }
        _ => panic!("Unsupported image type"),
    }
}
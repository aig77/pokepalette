// use super::generate_scheme_blocks;

use std::fmt;
use std::io::Error;
use std::path::Path;
use image::{DynamicImage, ColorType};
use color_thief::ColorFormat;
use serde::{Serialize, Deserialize};

use crate::rgb::Rgb;
use crate::colorscheme::ColorScheme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub scheme: ColorScheme,
    pub shiny: bool,
    pub female: bool,
    pub regional_variant: RegionalVariant
}

struct PathDetails {
    name: String,
    shiny: bool,
    female: bool,
    regional_variant: RegionalVariant
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Serialize, Deserialize)]
pub enum RegionalVariant {
    Regular,
    Alola,
    Galar
}

impl Eq for RegionalVariant {}

impl Sprite {
    pub fn new(path: &Path) -> Sprite {
        let path_details = PathDetails::new(path);

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
            colors.push( Rgb { r: 0, g: 0, b: 0 });
        }

        let scheme = ColorScheme::new(colors);

        Sprite {
            name: path_details.name,
            scheme: scheme,
            shiny: path_details.shiny,
            female: path_details.female,
            regional_variant: path_details.regional_variant,
        }
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "------------------------")?;
        writeln!(f, "scheme:  {}", self.scheme)?;
        writeln!(f, "shiny:   {}", self.shiny)?;
        writeln!(f, "female:  {}", self.female)?;
        writeln!(f, "variant: {:?}", self.regional_variant)?;
        Ok(())
    }   
}

impl PathDetails {
    fn new(path: &Path) -> PathDetails {
        let name = get_name_from_file_stem(path)
            .expect("unable to get name from file stem in path");
    
        let shiny: bool = match path.to_str() {
            Some(path_str) => path_str.contains("shiny"),
            _ => false
        };
    
        let female: bool = match path.to_str() {
            Some(path_str) => path_str.contains("female"),
            _ => false
        };
    
        let regional_variant: RegionalVariant = match path.to_str() {
            Some(path_str) if path_str.contains("alola") => RegionalVariant::Alola,
            Some(path_str) if path_str.contains("galar") => RegionalVariant::Galar,
            _ => RegionalVariant::Regular
        };
    
        PathDetails {
            name: name,
            shiny: shiny,
            female: female,
            regional_variant: regional_variant
        }
    }
}

fn get_name_from_file_stem(path: &Path) -> Result<String, Error> {
    let file_stem = path
        .file_stem()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;

    let file_name_str = file_stem.to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "filename is not valid UTF-8"))?;

    fn capitalize_first_letter(s: &str) -> String {
        if let Some(c) = s.chars().next() {
            c.to_uppercase().chain(s.chars().skip(1)).collect()
        } else {
            String::new()
        }
    }

    let formatted: Vec<String> = file_name_str.split('-')
        .map(|s| capitalize_first_letter(s))
        .collect();

    Ok(formatted.join("-"))
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

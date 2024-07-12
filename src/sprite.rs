use super::generate_scheme_blocks;

use std::fmt;
use std::io::Error;
use std::path::Path;
use rgb::Rgb;
use image::{DynamicImage, ColorType};
use color_thief::ColorFormat;

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub scheme: Vec<Rgb<u8>>,
    pub shiny: bool,
    pub female: bool,
    pub region: Region
}

struct PathDetails {
    name: String,
    shiny: bool,
    female: bool,
    region: Region
}

#[derive(Debug)]
pub enum Region {
    Regular,
    Alola,
    Galar
}

impl Sprite {
    pub fn new(path: &Path) -> Sprite {
        let path_details = get_path_details(path);

        let img = image::open(path).unwrap();
        let (buffer, color_type) = get_image_buffer(img);
        let colors = color_thief::get_palette(&buffer, color_type, 10, 9).unwrap();

        Sprite {
            name: path_details.name,
            scheme: colors,
            shiny: path_details.shiny,
            female: path_details.female,
            region: path_details.region,
        }
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "------------------------")?;
        writeln!(f, "scheme: {}", generate_scheme_blocks(&self.scheme))?;
        writeln!(f, "shiny:  {}", self.shiny)?;
        writeln!(f, "female: {}", self.female)?;
        writeln!(f, "region: {:?}", self.region)?;
        Ok(())
    }   
}

fn get_path_details(path: &Path) -> PathDetails {
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

    let region: Region = match path.to_str() {
        Some(path_str) if path_str.contains("alola") => Region::Alola,
        Some(path_str) if path_str.contains("galar") => Region::Galar,
        _ => Region::Regular
    };

    PathDetails {
        name: name,
        female: female,
        shiny: shiny,
        region: region
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

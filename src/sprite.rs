use std::fmt;
use std::io::Error;
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::colorscheme::ColorScheme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub scheme: ColorScheme,
    pub shiny: bool,
    pub female: bool,
    pub mega: bool,
    pub regional_variant: RegionalVariant
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
        let scheme = ColorScheme::from_img_path(path);

        Sprite {
            name: path_details.name,
            scheme: scheme,
            shiny: path_details.shiny,
            female: path_details.female,
            mega: path_details.mega,
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
        writeln!(f, "mega:    {}", self.mega)?;
        write!(f, "variant: {:?}", self.regional_variant)?;
        Ok(())
    }   
}

struct PathDetails {
    name: String,
    shiny: bool,
    female: bool,
    mega: bool,
    regional_variant: RegionalVariant
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

        let mega: bool = match path.to_str() {
            Some(path_str) => path_str.contains("mega"),
            _ => false,
        };
    
        let regional_variant: RegionalVariant = match path.to_str() {
            Some(path_str) if path_str.contains("alola") => RegionalVariant::Alola,
            Some(path_str) if path_str.contains("galar") => RegionalVariant::Galar,
            _ => RegionalVariant::Regular
        };
    
        PathDetails {
            name,
            shiny,
            female,
            mega,
            regional_variant
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

#![allow(dead_code)]

use crate::quantize::{get_palette, WeightedColor};
use crate::{DEFAULT_IGNORE_BLACK, DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub shiny: bool,
    pub mega: bool,
    pub gmax: bool,
    pub region: Option<String>,
    pub palette: Vec<WeightedColor>,
}

#[derive(Debug)]
enum SpriteError {}

impl Sprite {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .expect("No filename found")
            .to_string();

        let path_str = path.to_string_lossy();

        let shiny = path_str.contains("shiny");

        let mega = path_str.ends_with("mega")
            || path_str.ends_with("mega-x")
            || path_str.ends_with("mega-y")
            || path_str.ends_with("primal");

        let gmax = path_str.ends_with("gmax");

        let region = match path_str {
            s if s.ends_with("alola") => Some("alola".to_string()),
            s if s.ends_with("galar") => Some("galar".to_string()),
            s if s.ends_with("hisui") => Some("hisui".to_string()),
            _ => None,
        };

        let contents = fs::read_to_string(&path).expect("Should have been able to read the file");

        let colors = extract_colors(&contents)?;
        let palette = get_palette(
            &colors,
            DEFAULT_PALETTE_SIZE,
            DEFAULT_LEVELS,
            DEFAULT_IGNORE_BLACK,
        );

        Ok(Sprite {
            name,
            shiny,
            mega,
            gmax,
            region,
            palette,
        })
    }

    pub fn from_content(content: &str, name: &str, shiny: bool) -> Result<Self> {
        let mega = name.ends_with("mega")
            || name.ends_with("mega-x")
            || name.ends_with("mega-y")
            || name.ends_with("primal");

        let gmax = name.ends_with("gmax");

        let region = match name {
            s if s.ends_with("alola") => Some("alola".to_string()),
            s if s.ends_with("galar") => Some("galar".to_string()),
            s if s.ends_with("hisui") => Some("hisui".to_string()),
            _ => None,
        };

        let colors = extract_colors(&content)?;
        let palette = get_palette(
            &colors,
            DEFAULT_PALETTE_SIZE,
            DEFAULT_LEVELS,
            DEFAULT_IGNORE_BLACK,
        );

        Ok(Sprite {
            name: name.to_string(),
            shiny,
            mega,
            gmax,
            region,
            palette,
        })
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pokemon: {}", self.name)?;

        if self.shiny {
            writeln!(f, "  Shiny variant")?;
        }
        if self.mega {
            writeln!(f, "  Mega evolution")?;
        }
        if self.gmax {
            writeln!(f, "  Gigantamax form")?;
        }
        if let Some(region) = &self.region {
            writeln!(f, "  {} variant", region)?;
        }

        writeln!(f, "  Top Colors:")?;
        for (i, weighted_color) in self.palette.iter().enumerate() {
            writeln!(
                f,
                "    {}. \x1b[48;2;{};{};{}m   \x1b[0m RGB({:>3}, {:>3}, {:>3}). Freq: {}",
                i + 1,
                weighted_color.color[0],
                weighted_color.color[1],
                weighted_color.color[2],
                weighted_color.color[0],
                weighted_color.color[1],
                weighted_color.color[2],
                weighted_color.freq,
            )?;
        }

        Ok(())
    }
}

fn extract_colors(content: &str) -> Result<Vec<[u8; 3]>> {
    let re = Regex::new(r"\[(?:38|48);2;(\d+);(\d+);(\d+)m")?;

    re.captures_iter(content)
        .map(|cap| {
            let r = cap[1].parse::<u8>()?;
            let g = cap[2].parse::<u8>()?;
            let b = cap[3].parse::<u8>()?;
            Ok([r, g, b])
        })
        .collect()
}

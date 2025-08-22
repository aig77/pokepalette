#![allow(dead_code)]

use crate::quantize::{get_palette, WeightedColor};
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
    pub fn new(
        path: PathBuf,
        palette_size: usize,
        levels: usize,
        ignore_black: bool,
    ) -> Result<Self> {
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
        let palette = get_palette(&colors, palette_size, levels, ignore_black);

        Ok(Sprite {
            name,
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

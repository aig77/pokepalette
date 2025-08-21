use crate::quantize::get_palette;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    name: String,
    shiny: bool,
    mega: bool,
    gmax: bool,
    region: Option<String>,
    palette: Vec<[u8; 3]>,
}

impl Sprite {
    pub fn new(path: PathBuf, palette_size: usize, ignore_black: bool) -> Self {
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

        let colors = extract_colors(&contents);
        let palette = get_palette(&colors, 4, ignore_black, palette_size);

        Sprite {
            name,
            shiny,
            mega,
            gmax,
            region,
            palette,
        }
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
        for (i, color) in self.palette.iter().enumerate() {
            writeln!(
                f,
                "    {}. \x1b[48;2;{};{};{}m   \x1b[0m RGB({:>3}, {:>3}, {:>3})",
                i + 1,
                color[0],
                color[1],
                color[2],
                color[0],
                color[1],
                color[2],
            )?;
        }

        Ok(())
    }
}

fn extract_colors(content: &str) -> Vec<[u8; 3]> {
    let re = Regex::new(r"\[(?:38|48);2;(\d+);(\d+);(\d+)m").unwrap();

    re.captures_iter(content)
        .map(|cap| {
            let r = cap[1].parse::<u8>().unwrap();
            let g = cap[2].parse::<u8>().unwrap();
            let b = cap[3].parse::<u8>().unwrap();
            [r, g, b]
        })
        .collect()
}

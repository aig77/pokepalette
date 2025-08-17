use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ColorRank {
    color: [u8; 3],
    count: usize,
}

#[derive(Serialize)]
pub struct RankedSprite {
    name: String,
    shiny: bool,
    mega: bool,
    gmax: bool,
    region: Option<String>,
    top_colors: Vec<ColorRank>,
}

impl RankedSprite {
    pub fn new(path: PathBuf, top_n: usize, ignore_black: bool) -> Self {
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
        let top_colors = get_top_colors(&colors, top_n, ignore_black);

        RankedSprite {
            name,
            shiny,
            mega,
            gmax,
            region,
            top_colors,
        }
    }
}

impl fmt::Display for RankedSprite {
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
        for (i, color_rank) in self.top_colors.iter().enumerate() {
            writeln!(
                f,
                "    {}. \x1b[48;2;{};{};{}m   \x1b[0m RGB({:>3}, {:>3}, {:>3}) - {} pixels",
                i + 1,
                color_rank.color[0],
                color_rank.color[1],
                color_rank.color[2],
                color_rank.color[0],
                color_rank.color[1],
                color_rank.color[2],
                color_rank.count
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

fn get_top_colors(colors: &[[u8; 3]], top_n: usize, ignore_black: bool) -> Vec<ColorRank> {
    let mut color_counts = HashMap::new();

    for color in colors {
        *color_counts.entry(*color).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = color_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    if ignore_black {
        sorted.retain(|&(x, _)| x != [0, 0, 0]);
    }

    sorted
        .into_iter()
        .take(top_n)
        .map(|(color, count)| ColorRank { color, count })
        .collect()
}

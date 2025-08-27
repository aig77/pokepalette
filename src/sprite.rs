#![allow(dead_code)]

use crate::quantize::{get_palette, WeightedColor};
use crate::{DEFAULT_IGNORE_BLACK, DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Form {
    Regular,
    Mega(MegaType),
    Gmax,
    Regional(Region),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MegaType {
    Base,   // "mega"
    X,      // "mega-x"
    Y,      // "mega-y"
    Primal, // "primal"
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Region {
    Alola, // "alola"
    Galar, // "galar"
    Hisui, // "hisui"
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Form::Regular => write!(f, "regular"),
            Form::Mega(mega_type) => write!(f, "{}", mega_type),
            Form::Gmax => write!(f, "gmax"),
            Form::Regional(region) => write!(f, "{}", region),
        }
    }
}

impl fmt::Display for MegaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MegaType::Base => write!(f, "mega"),
            MegaType::X => write!(f, "mega-x"),
            MegaType::Y => write!(f, "mega-y"),
            MegaType::Primal => write!(f, "primal"),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Region::Alola => write!(f, "alola"),
            Region::Galar => write!(f, "galar"),
            Region::Hisui => write!(f, "hisui"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub shiny: bool,
    pub form: Form,
    pub palette: Vec<WeightedColor>,
}

#[derive(Debug)]
enum SpriteError {}

impl Sprite {
    pub fn from_content(content: &str, name: &str, shiny: bool) -> Result<Self> {
        let (clean_name, form) = Sprite::parse_name_and_form(&name);

        let colors = extract_colors(&content)?;
        let palette = get_palette(
            &colors,
            DEFAULT_PALETTE_SIZE,
            DEFAULT_LEVELS,
            DEFAULT_IGNORE_BLACK,
        );

        Ok(Sprite {
            name: clean_name,
            shiny,
            form,
            palette,
        })
    }

    fn parse_name_and_form(name: &str) -> (String, Form) {
        match name {
            s if s.ends_with("-mega-x") => match s.strip_suffix("-mega-x") {
                Some(clean) => (clean.to_string(), Form::Mega(MegaType::X)),
                None => panic!("Failed to strip '-mega-x' from '{}'", s),
            },
            s if s.ends_with("-mega-y") => match s.strip_suffix("-mega-y") {
                Some(clean) => (clean.to_string(), Form::Mega(MegaType::Y)),
                None => panic!("Failed to strip '-mega-y' from '{}'", s),
            },
            s if s.ends_with("-mega") => match s.strip_suffix("-mega") {
                Some(clean) => (clean.to_string(), Form::Mega(MegaType::Base)),
                None => panic!("Failed to strip '-mega' from '{}'", s),
            },
            s if s.ends_with("-primal") => match s.strip_suffix("-primal") {
                Some(clean) => (clean.to_string(), Form::Mega(MegaType::Primal)),
                None => panic!("Failed to strip '-primal' from '{}'", s),
            },
            s if s.ends_with("-gmax") => match s.strip_suffix("-gmax") {
                Some(clean) => (clean.to_string(), Form::Gmax),
                None => panic!("Failed to strip '-gmax' from '{}'", s),
            },
            s if s.ends_with("-alola") => match s.strip_suffix("-alola") {
                Some(clean) => (clean.to_string(), Form::Regional(Region::Alola)),
                None => panic!("Failed to strip '-alola' from '{}'", s),
            },
            s if s.ends_with("-galar") => match s.strip_suffix("-galar") {
                Some(clean) => (clean.to_string(), Form::Regional(Region::Galar)),
                None => panic!("Failed to strip '-galar' from '{}'", s),
            },
            s if s.ends_with("-hisui") => match s.strip_suffix("-hisui") {
                Some(clean) => (clean.to_string(), Form::Regional(Region::Hisui)),
                None => panic!("Failed to strip '-hisui' from '{}'", s),
            },
            _ => (name.to_string(), Form::Regular),
        }
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pokemon: {}", self.name)?;

        if self.shiny {
            writeln!(f, "  Shiny variant")?;
        }
        if self.form != Form::Regular {
            writeln!(f, "  Form: {}", self.form)?;
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

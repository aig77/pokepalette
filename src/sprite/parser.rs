use super::form::{Form, MegaType, Region};
use crate::colorquant::get_palette;
use crate::{DEFAULT_IGNORE_BLACK, DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};
use anyhow::Result;
use regex::Regex;

impl super::Sprite {
    pub fn from_content(content: &str, name: &str, gen: u8, shiny: bool) -> Result<Self> {
        let (clean_name, form) = Self::parse_name_and_form(&name);

        let colors = Self::extract_colors(&content)?;
        let palette = get_palette(
            &colors,
            DEFAULT_PALETTE_SIZE,
            DEFAULT_LEVELS,
            DEFAULT_IGNORE_BLACK,
        );

        Ok(Self {
            name: clean_name,
            gen,
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
                Some(clean) => (clean.to_string(), Form::Mega(MegaType::Mega)),
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
            s if s.ends_with("-paldea") => match s.strip_suffix("-paldea") {
                Some(clean) => (clean.to_string(), Form::Regional(Region::Paldea)),
                None => panic!("Failed to strip '-paldea' from '{}'", s),
            },
            _ => (name.to_string(), Form::Regular),
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
}

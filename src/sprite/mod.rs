pub mod form;
pub mod parser;

use crate::colorquant::WeightedColor;
pub use form::{Form, MegaType, Region};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub shiny: bool,
    pub form: Form,
    pub palette: Vec<WeightedColor>,
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

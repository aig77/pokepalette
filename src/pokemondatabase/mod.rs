use crate::cli::args::Args;
use crate::sprite::{Form, Sprite};
use crate::DB_FILE_NAME;
use anyhow::Result;
use bincode;

pub struct PokemonDatabase {
    sprites: Vec<Sprite>,
}

impl PokemonDatabase {
    pub fn load() -> Result<Self> {
        let binary_data = std::fs::read(DB_FILE_NAME)?;
        let (sprites, _): (Vec<Sprite>, usize) =
            bincode::serde::decode_from_slice(&binary_data, bincode::config::standard())?;
        Ok(Self { sprites })
    }

    pub fn filtered(self, args: &Args) -> Vec<Sprite> {
        self.sprites
            .into_iter()
            .filter(|sprite| {
                Self::filter_shiny(&sprite, &args)
                    && Self::filter_mega(&sprite, &args)
                    && Self::filter_gmax(&sprite, &args)
                    && Self::filter_regional(&sprite, &args)
            })
            .collect()
    }

    fn filter_shiny(sprite: &Sprite, args: &Args) -> bool {
        (!args.no_shiny || !sprite.shiny) && (!args.all_shiny || sprite.shiny)
    }

    fn filter_mega(sprite: &Sprite, args: &Args) -> bool {
        let is_mega = matches!(sprite.form, Form::Mega(_));
        (!args.no_mega || !is_mega) && (!args.all_mega || is_mega)
    }

    fn filter_gmax(sprite: &Sprite, args: &Args) -> bool {
        let is_gmax = matches!(sprite.form, Form::Gmax);
        (!args.no_gmax || !is_gmax) && (!args.all_gmax || is_gmax)
    }

    fn filter_regional(sprite: &Sprite, args: &Args) -> bool {
        let is_regional = matches!(sprite.form, Form::Regional(_));
        !args.no_regional || !is_regional
    }
}

use crate::cli::args::Args;
use crate::sprite::{Form, Sprite};
use crate::DB_FILE_NAME;
use anyhow::{anyhow, Result};
use bincode;

const MIN_GEN: u8 = 1;
const MAX_GEN: u8 = 9;

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

    pub fn filtered(self, args: &Args) -> Result<Vec<Sprite>> {
        let mut result = Vec::new();

        for sprite in self.sprites {
            if Self::filter_generation(&sprite, &args)?
                && Self::filter_shiny(&sprite, &args)
                && Self::filter_mega(&sprite, &args)
                && Self::filter_gmax(&sprite, &args)
                && Self::filter_regional(&sprite, &args)
            {
                result.push(sprite);
            }
        }

        Ok(result)
    }

    fn parse_generation(gen_str: &str) -> Result<Vec<u8>> {
        let mut gens: Vec<u8> = Vec::new();
        if gen_str.contains('-') {
            // Range: "1-3" -> [1, 2, 3]
            let parts: Vec<&str> = gen_str.split('-').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid range format: too many parts"));
            }
            let start: u8 = parts[0]
                .parse()
                .map_err(|_| anyhow!("Invalid start number: {}", parts[0]))?;
            let end: u8 = parts[1]
                .parse()
                .map_err(|_| anyhow!("Invalid end number: {}", parts[1]))?;
            if start >= end {
                return Err(anyhow!(
                    "Invalid range format: start cannot be greater than or equal to end"
                ));
            }
            gens.extend(start..=end);
        } else if gen_str.contains(',') {
            // Multiple: "1,2,4" -> [1, 2, 4]
            for part in gen_str.split(',') {
                let gen: u8 = part
                    .trim()
                    .parse()
                    .map_err(|_| anyhow!("Invalid number: {part}"))?;
                gens.push(gen);
            }
        } else {
            // Single "3" -> [3]
            let gen: u8 = gen_str
                .parse()
                .map_err(|_| anyhow!("Invalid number: {gen_str})"))?;
            gens.push(gen);
        }

        // Validate all generations are in valid range
        for &gen in &gens {
            if gen < MIN_GEN || gen > MAX_GEN {
                return Err(anyhow!("Generation {} is out of range (must be 1-9)", gen));
            }
        }

        Ok(gens)
    }

    fn filter_generation(sprite: &Sprite, args: &Args) -> Result<bool> {
        let Some(gen_str) = &args.gen else {
            return Ok(true);
        };

        let allowed_gens = Self::parse_generation(gen_str)?;
        Ok(allowed_gens.contains(&sprite.gen))
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

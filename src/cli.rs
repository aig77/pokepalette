#![allow(dead_code)]

use pokepalette::DEFAULT_TOP_K;

use crate::sprite::Sprite;
use clap::{ArgGroup, Parser};

/// Match pokemon color palettes to your images
#[derive(Parser, Debug)]
#[command(version, about, long_about = None,
    group(ArgGroup::new("shiny-filter")
        .args(["no_shiny", "all_shiny"])
        .multiple(false)),
    group(ArgGroup::new("mega-filter")
        .args(["no_mega", "all_mega"])
        .multiple(false)),
    group(ArgGroup::new("gmax-filter")
        .args(["no_gmax", "all_gmax"])
        .multiple(false)),
)]
struct Args {
    /// Path to the image to find pokemon palettes
    pub image: String,

    /// Number of pokemon palettes provided
    #[arg(short, long, default_value_t = DEFAULT_TOP_K)]
    pub top_k: usize,

    /// Whether to print additional information about the pokemon
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Filter out shinies
    #[arg(long, default_value_t = false)]
    no_shiny: bool,

    /// Filter out non-shinies
    #[arg(long, default_value_t = false)]
    all_shiny: bool,

    /// Filter out mega
    #[arg(long, default_value_t = false)]
    no_mega: bool,

    /// Filter out non-mega
    #[arg(long, default_value_t = false)]
    all_mega: bool,

    /// Filter out gmax
    #[arg(long, default_value_t = false)]
    no_gmax: bool,

    /// Filter out non-gmax
    #[arg(long, default_value_t = false)]
    all_gmax: bool,

    /// Filter out regional variants
    #[arg(long, default_value_t = false)]
    no_regional: bool,
}

pub struct Config {
    pub image: String,
    pub top_k: usize,
    pub verbose: bool,
}

impl Config {
    fn new(args: &Args) -> Self {
        Config {
            image: args.image.clone(),
            top_k: args.top_k,
            verbose: args.verbose,
        }
    }
}

pub fn get_config_and_filter_sprites(sprites: Vec<Sprite>) -> (Config, Vec<Sprite>) {
    let args = Args::parse();
    let config = Config::new(&args);
    let filtered = filter_sprites(sprites, &args);
    (config, filtered)
}

fn filter_sprites(sprites: Vec<Sprite>, args: &Args) -> Vec<Sprite> {
    sprites
        .into_iter()
        .filter(|sprite| {
            let shiny_condition =
                (!args.no_shiny || !sprite.shiny) && (!args.all_shiny || sprite.shiny);
            let mega_condition = (!args.no_mega || !sprite.mega) && (!args.all_mega || sprite.mega);
            let gmax_condition = (!args.no_gmax || !sprite.gmax) && (!args.all_gmax || sprite.gmax);
            let regional_condition = !args.no_regional || sprite.region.is_none();
            shiny_condition && mega_condition && gmax_condition && regional_condition
        })
        .collect()
}

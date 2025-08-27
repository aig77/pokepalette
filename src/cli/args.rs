use crate::sprite::{Form, Sprite};
use crate::DEFAULT_TOP_K;
use clap::{ArgGroup, Parser};

/// Find pokemon color palettes that are similar to your image
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
    /// Path of the image
    image: String,

    /// Number of pokemon returned
    #[arg(short, long, default_value_t = DEFAULT_TOP_K)]
    top_k: usize,

    /// Print additional information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

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

            let mega_condition = {
                let is_mega = matches!(sprite.form, Form::Mega(_));
                (!args.no_mega || !is_mega) && (!args.all_mega || is_mega)
            };

            let gmax_condition = {
                let is_gmax = matches!(sprite.form, Form::Gmax);
                (!args.no_gmax || !is_gmax) && (!args.all_gmax || is_gmax)
            };

            let regional_condition = {
                let is_regional = matches!(sprite.form, Form::Regional(_));
                !args.no_regional || !is_regional
            };

            shiny_condition && mega_condition && gmax_condition && regional_condition
        })
        .collect()
}

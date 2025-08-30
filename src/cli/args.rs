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
pub struct Args {
    /// Path of the image
    pub image: String,

    /// Number of pokemon returned
    #[arg(short, long, default_value_t = DEFAULT_TOP_K)]
    pub top_k: usize,

    /// Print additional information
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(short, long)]
    pub gen: Option<String>,

    /// Filter out shinies
    #[arg(long, default_value_t = false)]
    pub no_shiny: bool,

    /// Filter out non-shinies
    #[arg(long, default_value_t = false)]
    pub all_shiny: bool,

    /// Filter out mega
    #[arg(long, default_value_t = false)]
    pub no_mega: bool,

    /// Filter out non-mega
    #[arg(long, default_value_t = false)]
    pub all_mega: bool,

    /// Filter out gmax
    #[arg(long, default_value_t = false)]
    pub no_gmax: bool,

    /// Filter out non-gmax
    #[arg(long, default_value_t = false)]
    pub all_gmax: bool,

    /// Filter out regional variants
    #[arg(long, default_value_t = false)]
    pub no_regional: bool,
}

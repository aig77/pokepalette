pub mod cli;
pub mod colorquant;
pub mod pokemondatabase;
pub mod sprite;

pub use cli::output::{print_image_information, print_result, print_top_information};
pub use colorquant::{get_palette, palette_distance, WeightedColor};

pub const KRABBY_BASE_URL: &str = "https://raw.githubusercontent.com/yannjor/krabby/main/";
pub const DB_FILE_NAME: &str = "pokemon.bin";
pub const DEFAULT_PALETTE_SIZE: usize = 5;
pub const DEFAULT_LEVELS: usize = 8;
pub const DEFAULT_IGNORE_BLACK: bool = true;
pub const DEFAULT_TOP_K: usize = 10;

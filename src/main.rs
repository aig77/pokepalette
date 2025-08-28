use anyhow::Result;
use bincode;

use pokepalette::colorquant::{get_image_palette, get_pokemon_ranked};
use pokepalette::sprite::Sprite;
use pokepalette::DB_FILE_NAME;
use pokepalette::{
    get_config_and_filter_sprites, print_image_information, print_result, print_top_information,
};

fn main() -> Result<()> {
    // Load database
    let binary_data = std::fs::read(DB_FILE_NAME)?;
    let (sprites, _): (Vec<Sprite>, usize) =
        bincode::serde::decode_from_slice(&binary_data, bincode::config::standard())?;

    // Load CLI config and filters
    let (config, filtered) = get_config_and_filter_sprites(sprites);

    // Generate image palette
    let image_palette = get_image_palette(&config.image)?;

    // Get sprites sorted by distance to image
    let ranked = get_pokemon_ranked(&image_palette, &filtered);

    // Get top k
    let top: Vec<(&Sprite, f32)> = ranked.into_iter().take(config.top_k).collect();

    // Print results
    if config.verbose {
        print_image_information(image_palette);
        print_top_information(&top);
    } else {
        print_result(&top);
    }

    Ok(())
}

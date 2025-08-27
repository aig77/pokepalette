use anyhow::Result;
use serde_json;
use std::fs;

use pokepalette::colorquant::{get_image_palette, get_pokemon_ranked};
use pokepalette::sprite::Sprite;
use pokepalette::DB_PATH;
use pokepalette::{
    get_config_and_filter_sprites, print_image_information, print_result, print_top_information,
};

fn main() -> Result<()> {
    // Load database
    let file = fs::File::open(DB_PATH).expect("Failed to open pokemon.json");
    let sprites: Vec<Sprite> = serde_json::from_reader(file).expect("Failed to parse pokemon.json");

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

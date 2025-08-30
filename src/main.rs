use anyhow::Result;
use clap::Parser;
use pokepalette::cli::args::Args;
use pokepalette::colorquant::{get_image_palette, get_pokemon_ranked};
use pokepalette::pokemondatabase::PokemonDatabase;
use pokepalette::sprite::Sprite;
use pokepalette::{print_image_information, print_result, print_top_information};

fn main() -> Result<()> {
    let args = Args::parse();

    // Load database and filter
    let database = PokemonDatabase::load()?;
    let sprites = database.filtered(&args)?;

    // Generate image palette
    let image_palette = get_image_palette(&args.image)?;

    // Get sprites sorted by distance to image
    let ranked = get_pokemon_ranked(&image_palette, &sprites);

    // Get top k
    let top: Vec<(&Sprite, f32)> = ranked.into_iter().take(args.top_k).collect();

    // Print results
    if args.verbose {
        print_image_information(image_palette);
        print_top_information(&top);
    } else {
        print_result(&top);
    }

    Ok(())
}

use pokepalette::sprite::Sprite;
use pokepalette::{DEFAULT_IGNORE_BLACK, DEFAULT_LEVELS, DEFAULT_PALETTE_SIZE};

use std::env;
use std::fs;
use std::path::PathBuf;

const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const OUTPUT_FILE_PATH: &str = "pokemon.json";

fn main() -> std::io::Result<()> {
    let verbose = true;
    let mut sprites: Vec<Sprite> = Vec::new();

    let regular = PathBuf::from(PROJECT_ROOT).join("assets/colorscripts/regular");
    let shiny = PathBuf::from(PROJECT_ROOT).join("assets/colorscripts/shiny");

    let entries = fs::read_dir(&regular)?.chain(fs::read_dir(&shiny)?);

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        let sprite = Sprite::new(
            path,
            DEFAULT_PALETTE_SIZE,
            DEFAULT_LEVELS,
            DEFAULT_IGNORE_BLACK,
        );

        if verbose {
            println!("{sprite}");
        }

        sprites.push(sprite);
    }

    let full_output_path = PathBuf::from(PROJECT_ROOT).join(OUTPUT_FILE_PATH);

    let mut file = fs::File::create(full_output_path)?;
    serde_json::to_writer_pretty(&mut file, &sprites)?;

    Ok(())
}

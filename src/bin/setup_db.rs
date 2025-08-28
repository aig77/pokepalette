use anyhow::Result;
use bincode;
use indicatif::{ProgressBar, ProgressStyle};
use pokepalette::sprite::Sprite;
use pokepalette::DB_FILE_NAME;
use pokepalette::KRABBY_BASE_URL;
use reqwest;
use serde_json::Value;
use std::env;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

// Create DB in root dir
const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

#[tokio::main]
async fn main() -> Result<()> {
    let mut sprites: Vec<Sprite> = Vec::new();

    let pokemon_names = get_pokemon_list().await?;
    let total_downloads = pokemon_names.len() * 2; // 2x because of shinies

    println!("Downloading {} sprites from krabby", total_downloads);

    // Create progress bar
    let bar = ProgressBar::new(total_downloads as u64);
    bar.set_style(ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
    )?);

    // Create all urls
    let mut all_urls = Vec::new();
    for name in pokemon_names {
        for variant in ["regular", "shiny"] {
            let url = format!(
                "{}/assets/colorscripts/{}/{}",
                KRABBY_BASE_URL, variant, name
            );
            all_urls.push((url, name.clone(), variant == "shiny"));
        }
    }

    // Process in batchces
    let batch_size = 10;
    for batch in all_urls.chunks(batch_size) {
        let mut tasks = Vec::new();

        for (url, name, is_shiny) in batch {
            let url = url.clone();
            let name_clone = name.clone();
            let is_shiny = *is_shiny;

            let task = tokio::spawn(async move {
                sleep(Duration::from_millis(100)).await;

                match download_file(&url).await {
                    Ok(content) => Sprite::from_content(&content, &name_clone, is_shiny),
                    Err(_) => Err(anyhow::anyhow!("Download failed")),
                }
            });
            tasks.push(task);
        }

        // Wait for all downloads to complete
        for task in tasks {
            if let Ok(Ok(sprite)) = task.await {
                sprites.push(sprite);
            }

            bar.inc(1);
        }

        // Pause between batches
        sleep(Duration::from_millis(200)).await;
    }

    bar.finish_with_message("Downloads complete!");
    println!("Created {} sprites", sprites.len());

    sprites.sort_by(|a, b| a.name.cmp(&b.name));

    println!("Creating bin");

    let db_path = PathBuf::from(PROJECT_ROOT).join(DB_FILE_NAME);
    let binary_data = bincode::serde::encode_to_vec(&sprites, bincode::config::standard())?;
    std::fs::write(db_path, &binary_data)?;

    println!("Done");

    Ok(())
}

async fn get_pokemon_list() -> Result<Vec<String>> {
    let list_url = format!("{}/assets/pokemon.json", KRABBY_BASE_URL);
    let response = download_file(&list_url).await?;
    let json: Value = serde_json::from_str(&response)?;
    let mut names = Vec::new();
    if let Some(array) = json.as_array() {
        for pokemon in array {
            names.extend(get_variants(pokemon));
        }
    }
    Ok(names)
}

async fn download_file(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

fn get_variants(pokemon: &Value) -> Vec<String> {
    let Some(slug) = pokemon["slug"].as_str() else {
        return vec![];
    };

    let mut variants = vec![slug.to_string()];

    if let Some(forms) = pokemon["forms"].as_array() {
        variants.extend(
            forms
                .iter()
                .filter_map(|form| form.as_str())
                .map(|form| format!("{}-{}", slug, form)),
        );
    }

    variants
}

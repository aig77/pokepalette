mod flags;
mod sprite;
mod palette;
mod color;
mod jsonify;
mod wal;
mod distance;

use std::env;
use std::cmp::Ordering;
use rand::Rng;
use std::error::Error;
use std::collections::HashMap;


use flags::Flags;
use color::Rgb;
use palette::Palette;
use sprite::Sprite;
use wal::get_wal_palette;
use jsonify::read_sprites_json;
use jsonify::generate_sprites_json;
use distance::*;


const JSON_PATH: &str = "./data/sprites.json";
const SPRITES_PATH: &str = "./data/pokemon-gen8";
const WAL_palette_SIZE: usize = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let flags =  Flags::new();

    let wal = get_wal_palette(WAL_palette_SIZE);

    // generate_sprites_json(SPRITES_PATH, JSON_PATH, WAL_palette_SIZE);

    let sprites = read_sprites_json(JSON_PATH, &flags)?;

    let k = flags.k;
    let top_k_e = get_k_nearest_sprites(&sprites, &wal, k, distance::EuclideanDistanceFn);
    let top_k_d = get_k_nearest_sprites(&sprites, &wal, k, distance::DE2000DistanceFn);
    let top_k_m = get_k_nearest_sprites(&sprites, &wal, k, distance::MSSDDistanceFn);

    if flags.verbose {
        println!("wal:  {}\n", wal);
        println!("----------------------------");
        println!("********** Euclidean **********\n");

        for (dist, sprite) in &top_k_e {
            println!("{}", sprite);
            println!("distance: {:.4}\n", dist);
        }

        println!("********** DE2000 **********\n");

        for (dist, sprite) in &top_k_d {
            println!("{}", sprite);
            println!("distance: {:.4}\n", dist);
        }

        println!("********** MSSD **********\n");

        for (dist, sprite) in &top_k_m {
            println!("{}", sprite);
            println!("distance: {:.4}\n", dist);
        }
    }

    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..k);
    println!("{}", top_k_e[rand].1.name);

    Ok(())
}

fn get_k_nearest_sprites<'a, D>(
    sprites: &'a [Sprite], 
    palette: &Palette<Rgb<u8>>, 
    k: usize, 
    distance_fn: D,
) -> Vec<(f64, &'a Sprite)> 
where 
    D: distance::DistanceFn<f64> + Clone
{
    let mut distances: Vec<(f64, &'a Sprite)> = sprites
        .iter()
        .map(|sprite| {
            let distance = distance_fn.palette_distance(&sprite.palette, palette);
            (distance, sprite)
        })
        .collect();

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    distances
        .iter()
        .take(k)
        .map(|&(dist, sprite)| (dist, sprite))
        .collect()
}









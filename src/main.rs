mod sprite;
mod colorscheme;
mod rgb;
mod jsonify;
mod wal;

use std::cmp::Ordering;

use rand::Rng;
use colorscheme::ColorScheme;
use sprite::Sprite;
use wal::get_wal_scheme;
use jsonify::read_sprites_json;
use jsonify::generate_sprites_json;

fn main() {
    // get wal scheme

    // get pokemon scheme map

    // calc distances from wal scheme to all instances in pokemon scheme map

    // grab k smallest distances

    // return them

    // TODO:
    // create file with pokemon schemes stored for faster access
    // create distance function
    // create cli tools

    // generate_sprites_json();

    let mut rng = rand::thread_rng();

    let k = 10;

    let weights = vec![0.6, 1.0, 1.0, 1.0, 1.0, 0.6, 0.6, 0.2];

    let wal = get_wal_scheme(8);

    println!("{}", wal);

    let sprites = match read_sprites_json() {
        Ok(vec) => vec,
        Err(err) => {
            eprintln!("error reading sprite data from json: {}", err);
            return;
        }
    };
    
    let top_k = get_k_nearest_sprites(&wal, &sprites, k, &weights);

    // for (dist, sprite) in &top_k {
    //     println!("{}\n{}\n", sprite, dist);
    // }

    let rand = rng.gen_range(0..k);

    println!("{}", top_k[rand].1.name);
}

fn get_k_nearest_sprites<'a>(scheme: &ColorScheme, sprites: &'a [Sprite], k: usize, weights: &[f64]) -> Vec<(f64, &'a Sprite)> {
    let mut distances: Vec<(f64, &'a Sprite)> = sprites
        .iter()
        .map(|sprite| {
            let distance = sprite.scheme.euclidean_distance_with_weights(scheme, weights);
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









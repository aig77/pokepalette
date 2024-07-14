mod sprite;
mod colorscheme;
mod rgb;
mod jsonify;
mod wal;

use std::env;
use std::cmp::Ordering;
use rand::Rng;
use colorscheme::ColorScheme;
use sprite::Sprite;
use wal::get_wal_scheme;
use jsonify::read_sprites_json;
use jsonify::generate_sprites_json;

pub struct Flags {
    k: usize,
    no_shiny: bool,
    no_female: bool,
    no_mega: bool,
    no_regional_variant: bool,
    verbose: bool
}

enum FlagEnum {
    K,
    Default
}

impl Flags {
    fn new(args: &[String]) -> Flags {
        let mut flags = Flags {
            k: 5,
            no_shiny: false,
            no_female: false,
            no_mega: false,
            no_regional_variant: false,
            verbose: false
        };
        
        let mut prev: FlagEnum = FlagEnum::Default;

        for arg in args {
            match arg.as_str() {
                "--no-shiny" => flags.no_shiny = true,
                "--no-female" => flags.no_female = true,
                "--no-mega" => flags.no_mega = true,
                "--no-regional" => flags.no_regional_variant = true,
                "-k" => prev = FlagEnum::K,
                "--verbose" => flags.verbose = true,
                val => {
                    match prev {
                        FlagEnum::K => {
                            flags.k = val.parse().expect("invalid value for k");
                            prev = FlagEnum::Default;
                        },
                        _ => continue,
                    }
                }
            }
        }
        flags
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let args: Vec<String> = env::args().collect();

    let flags =  Flags::new(&args);

    let k = flags.k;

    let weights = vec![0.6, 1.0, 1.0, 1.0, 1.0, 0.6, 0.6, 0.2];

    let wal = get_wal_scheme(8);

    let sprites = match read_sprites_json(&flags) {
        Ok(vec) => vec,
        Err(err) => {
            eprintln!("error reading sprite data from json: {}", err);
            return;
        }
    };
    
    let top_k = get_k_nearest_sprites(&wal, &sprites, k, &weights);

    if flags.verbose {
        println!("wal:  {}\n", wal);

        for (dist, sprite) in &top_k {
            println!("{}", sprite);
            println!("distance: {:.4}\n", dist);
        }
    }

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









mod sprite;
mod colorscheme;
mod color;
mod jsonify;
mod wal;
mod distance;

use std::env;
use std::cmp::Ordering;
use rand::Rng;
use color::Rgb;
use colorscheme::ColorScheme;
use sprite::Sprite;
use wal::get_wal_scheme;
use jsonify::read_sprites_json;
use jsonify::generate_sprites_json;
use distance::*;

use std::collections::HashMap;
// use rgb::Rgb;

const JSON_PATH: &str = "./data/sprites.json";
const SPRITES_PATH: &str = "./data/pokemon-gen8";
const WAL_SCHEME_SIZE: usize = 8;

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

    // let weights = vec![0.1, 0.6, 1.0, 1.0, 1.0, 1.0, 0.6, 0.2];
    let weights = vec![0.1, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.1];

    let wal = get_wal_scheme(WAL_SCHEME_SIZE);

    // generate_sprites_json(SPRITES_PATH, JSON_PATH, WAL_SCHEME_SIZE);

    let sprites = match read_sprites_json(JSON_PATH, &flags) {
        Ok(vec) => vec,
        Err(err) => {
            eprintln!("error reading sprite data from json: {}", err);
            return;
        }
    };

    // let mut scheme_count = HashMap::new();

    // for sprite in &sprites {
    //     let count = scheme_count.entry(sprite.scheme.len()).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", scheme_count);

    // for sprite in &sprites {
    //     println!("{}", sprite.scheme.len());
    // }
    
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
    let rand = rng.gen_range(0..k);

    println!("{}", top_k_e[rand].1.name);
}

fn get_k_nearest_sprites<'a, D>(
    sprites: &'a [Sprite], 
    scheme: &ColorScheme<Rgb<u8>>, 
    k: usize, 
    distance_fn: D,
) -> Vec<(f64, &'a Sprite)> 
where 
    D: distance::DistanceFn<f64> + Clone
{
    let mut distances: Vec<(f64, &'a Sprite)> = sprites
        .iter()
        .map(|sprite| {
            let distance = distance_fn.scheme_distance(&sprite.scheme, scheme);
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









use std::env;

pub struct Flags {
    pub k: usize,
    pub no_shiny: bool,
    pub no_female: bool,
    pub no_mega: bool,
    pub no_regional_variant: bool,
    pub verbose: bool
}

pub enum FlagEnum {
    K,
    Default
}

impl Flags {
    pub fn new() -> Flags {
        let args: Vec<String> = env::args().collect();

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
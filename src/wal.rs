use std::fs;
use homedir::my_home;
use std::path::PathBuf;
use crate::rgb::Rgb;
use crate::colorscheme::ColorScheme;

const WAL_RGB: &str = ".cache/wal/colors-rgb";

pub fn get_wal_scheme(n: usize) -> ColorScheme {
    let mut scheme = vec![];

    let wal_path: PathBuf = match my_home().expect("unable to find home directory") {
        Some(mut home) => {
            home.push(WAL_RGB);
            home
        }
        _ => unreachable!()
    };

    match read_first_n_lines(wal_path, n) {
        Ok(lines) => {
            for line in lines {
                let colors: Vec<&str> = line.split(',')
                    .map(|s| s.trim())
                    .collect();
        
                let r: u8 = colors[0].parse().expect("failed to parse red value");
                let g: u8 = colors[1].parse().expect("failed to parse green value");
                let b: u8 = colors[2].parse().expect("failed to parse blue value");

                scheme.push(Rgb { r: r, g: g, b: b });
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e)
    }

    ColorScheme::new(scheme)
}

fn read_first_n_lines(filename: PathBuf, n: usize) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<String> = contents.lines().take(n).map(|s| s.to_string()).collect();
    Ok(lines)
}


use std::fs;
use homedir::my_home;
use std::path::{PathBuf, Path};
use crate::rgb::Rgb;
use crate::colorscheme::ColorScheme;

const WAL_RGB: &str = ".cache/wal/colors-rgb";

pub fn get_wal_scheme(n: usize) -> ColorScheme {
    // try pywal
    match my_home() {
        Ok(buf) => {
            if let Some(mut wal_path) = buf.clone() {
                wal_path.push(WAL_RGB);
                if wal_path.exists() {
                    match read_first_n_lines(wal_path, n) {
                        Ok(lines) => {
                            let scheme = lines.iter().filter_map(|line| {
                                let colors: Vec<&str> = line.split(',')
                                    .map(|s| s.trim())
                                    .collect();
                                let r: u8 = colors[0].parse().unwrap_or_default();
                                let g: u8 = colors[1].parse().unwrap_or_default();
                                let b: u8 = colors[2].parse().unwrap_or_default();
                                Some(Rgb { r, g, b })
                            }).collect();

                            return ColorScheme::new(scheme);
                        },
                        Err(_) => println!("error reading pywal file, trying by OS")
                    }
                }
            }
        },
        Err(_) => println!("couldn't find home directory while looking for pywal cache, trying by OS")
    }

    // try os
    match wallpaper::get() {
        Ok(path) => ColorScheme::from_img_path(Path::new(&path)),
        Err(_) => panic!("
Error getting wallpaper to generate colorscheme. 
Using pywal or pywal16 is recommended since it has already generated the colorscheme for your wallpaper rather 
than having to do it again. 
  * https://github.com/dylanaraps/pywal
  * https://github.com/eylles/pywal16
Make sure pywal is generating a colors-rgb template for Pokescheme to use.

If installing pywal is not an option, Pokescheme will use the rust wallpaper crate to grab your desktop wallpaper.
wallpaper.rs supports the following OS:
  * Windows
  * macOS
  * GNOME
  * KDE
  * Cinnamon
  * Unity
  * Budgie
  * XFCE
  * LXDE
  * MATE
  * Deepin
  * Most Wayland compositors (set only, requires swaybg)
  * i3 (set only, requires feh)
"
        )
    }
} 

fn read_first_n_lines(filename: PathBuf, n: usize) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<String> = contents.lines().take(n).map(|s| s.to_string()).collect();
    Ok(lines)
}


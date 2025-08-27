use crate::colorquant::WeightedColor;
use crate::sprite::{Form, Sprite};

pub fn print_result(top: &Vec<(&Sprite, f32)>) {
    for (sprite, _) in top {
        print!("{}", sprite.name);

        if sprite.shiny {
            print!(" -s");
        }

        if sprite.form != Form::Regular {
            print!(" -f {}", sprite.form);
        }

        println!();
    }
}

pub fn print_image_information(image_palette: Vec<WeightedColor>) {
    for weighted_color in &image_palette {
        println!(
            "\x1b[48;2;{};{};{}m   \x1b[0m RGB({:>3}, {:>3}, {:>3}). Freq: {}",
            weighted_color.color[0],
            weighted_color.color[1],
            weighted_color.color[2],
            weighted_color.color[0],
            weighted_color.color[1],
            weighted_color.color[2],
            weighted_color.freq,
        );
    }

    println!("");
}

pub fn print_top_information(top: &Vec<(&Sprite, f32)>) {
    for (sprite, distance) in top {
        println!("{}\nScore: {}\n", sprite, distance);
    }
}

use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T
}

impl<T> Rgb<T> {
    pub fn ansi_color(&self) -> String 
    where 
        T: fmt::Display
    {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

impl Rgb<u8> {
    pub fn normalize(&self) -> Rgb<f64> {
        Rgb {
            r: self.r as f64 / 255.0,
            g: self.g as f64 / 255.0,
            b: self.b as f64 / 255.0
        }
    }
}





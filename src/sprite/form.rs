use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Form {
    Regular,
    Mega(MegaType),
    Gmax,
    Regional(Region),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MegaType {
    Mega,   // "mega"
    X,      // "mega-x"
    Y,      // "mega-y"
    Primal, // "primal"
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Region {
    Alola, // "alola"
    Galar, // "galar"
    Hisui, // "hisui"
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Form::Regular => write!(f, "regular"),
            Form::Mega(mega_type) => write!(f, "{}", mega_type),
            Form::Gmax => write!(f, "gmax"),
            Form::Regional(region) => write!(f, "{}", region),
        }
    }
}

impl fmt::Display for MegaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MegaType::Mega => write!(f, "mega"),
            MegaType::X => write!(f, "mega-x"),
            MegaType::Y => write!(f, "mega-y"),
            MegaType::Primal => write!(f, "primal"),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Region::Alola => write!(f, "alola"),
            Region::Galar => write!(f, "galar"),
            Region::Hisui => write!(f, "hisui"),
        }
    }
}

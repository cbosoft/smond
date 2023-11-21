use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use crate::sensor::{Sensor, Measurement};

pub struct Battery {
    idx: u8
}

impl Battery {
    pub fn new(idx: u8) -> Self {
        Battery { idx }
    }

    pub fn get_idx(&self) -> u8 {
        self.idx
    }
}

impl Sensor for Battery {
    fn measure(&self) -> Measurement {
        let path = PathBuf::from(format!("/sys/class/power_supply/BAT{}/capacity", self.idx));
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        let _ = f.read_to_string(&mut contents).unwrap();
        let cap = i64::from_str(contents.trim()).unwrap();
        Measurement::new(cap)
    }

    fn ident(&self) -> String {
        format!("battery{}", self.idx)
    }
}

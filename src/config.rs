use std::fs::File;
use std::io::Read;
use std::env;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    poll_period: u64
}

impl Config {

    pub fn load() -> Self {
        let home = env::var("HOME").unwrap();
        let path = PathBuf::from(format!("{home}/.config/smond/config.yaml"));
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        let _ = f.read_to_string(&mut contents);
        serde_yaml::from_str(contents.as_str()).unwrap()
    }

    pub fn get_poll_period(&self) -> u64 {
        self.poll_period
    }
}

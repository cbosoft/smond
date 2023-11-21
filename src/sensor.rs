use std::time::{SystemTime, UNIX_EPOCH};

pub struct Measurement(u64, i64);

impl Measurement {
    pub fn new(v: i64) -> Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self(time, v)
    }

    pub fn as_csv_line(&self) -> String {
        format!("{},{}\n", self.0, self.1)
    }
}

pub trait Sensor {
    fn measure(&self) -> Measurement;
    fn ident(&self) -> String;
}

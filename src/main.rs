pub mod sensor;
mod battery;
mod cpu;
mod config;

use std::time::Duration;
use std::thread::sleep;
use std::path::PathBuf;
use std::env;
use std::fs::{File, create_dir};
use std::io::Write;

use battery::Battery;
use cpu::CPU;
use sensor::Sensor;
use config::Config;

fn main() {
    let config = Config::load();
    let poll_period = Duration::new(config.get_poll_period(), 0);
    let mut sensors: Vec<Box<dyn Sensor>> = Vec::new();
    sensors.push(Box::new(Battery::new(0)));
    sensors.push(Box::new(CPU));
    let home = env::var("HOME").unwrap();

    // Mark start of logging with zero values
    // Useful if logging is interrupted e.g. by reboot
    for sensor in &sensors {
        let ident = sensor.ident();
        let path = PathBuf::from(format!("{home}/.smond_logs/{ident}.csv"));
        let log_dir = path.parent().unwrap();
        if !log_dir.exists() {
            create_dir(log_dir).unwrap();
        }

        let mut f = File::options().create(true).append(true).open(path).unwrap();
        let _ = f.write_fmt(format_args!("0,0\n"));
    }

    // Main logging loop: go through each sensor in turn and record measurements
    loop {
        for sensor in &sensors {
            let ident = sensor.ident();
            let meas = sensor.measure();
            let path = PathBuf::from(format!("{home}/.smond_logs/{ident}.csv"));

            let mut f = File::options().write(true).append(true).open(path).unwrap();
            let _ = f.write_fmt(format_args!("{}", meas.as_csv_line()));
        }
        sleep(poll_period);
    }
}

use crate::sensor::{Sensor, Measurement};

use std::time::Duration;
use std::thread::sleep;

use sysinfo::{System, CpuExt, SystemExt, RefreshKind, CpuRefreshKind};

pub struct CPU;

impl Sensor for CPU {
    fn measure(&self) -> Measurement {
        let mut sys = System::new();
        sys.refresh_cpu();
        sleep(Duration::new(2, 0));
        sys.refresh_cpu();

        Measurement::new(sys.global_cpu_info().cpu_usage() as i64)
    }

    fn ident(&self) -> String {
        format!("cpu")
    }
}

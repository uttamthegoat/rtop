use std::fs;

#[derive(Debug, Clone, Default)]
pub struct BatteryInfo {
    pub present: bool,
    pub capacity: f64,
    pub status: String,
    pub voltage: u64,
    pub energy_full: u64,
    pub energy_now: u64,
}

pub struct BatteryCollector {
    battery: BatteryInfo,
}

impl BatteryCollector {
    pub fn new() -> Self {
        Self {
            battery: BatteryInfo::default(),
        }
    }

    pub async fn refresh(&mut self) {
        self.battery = Self::read_battery();
    }

    pub fn info(&self) -> &BatteryInfo {
        &self.battery
    }

    fn read_battery() -> BatteryInfo {
        let base = "/sys/class/power_supply";
        let dirs = match fs::read_dir(base) {
            Ok(d) => d,
            Err(_) => return BatteryInfo::default(),
        };

        for entry in dirs.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with("BAT") {
                continue;
            }
            let path = entry.path();
            let present = fs::read_to_string(path.join("present"))
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
                .map(|v| v == 1)
                .unwrap_or(false);

            let capacity = fs::read_to_string(path.join("capacity"))
                .ok()
                .and_then(|s| s.trim().parse::<f64>().ok())
                .unwrap_or(0.0);

            let status = fs::read_to_string(path.join("status"))
                .ok()
                .unwrap_or_default()
                .trim()
                .to_string();

            let voltage = fs::read_to_string(path.join("voltage_now"))
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .unwrap_or(0);

            let energy_full = fs::read_to_string(path.join("energy_full"))
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .unwrap_or(0);

            let energy_now = fs::read_to_string(path.join("energy_now"))
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .unwrap_or(0);

            return BatteryInfo {
                present,
                capacity,
                status,
                voltage,
                energy_full,
                energy_now,
            };
        }

        BatteryInfo::default()
    }
}

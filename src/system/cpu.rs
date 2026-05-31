use std::fs;

#[derive(Debug, Clone, Default)]
struct CpuData {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
}

impl CpuData {
    fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal
    }

    fn idle_total(&self) -> u64 {
        self.idle + self.iowait
    }
}

pub struct CpuCollector {
    prev: Vec<CpuData>,
    usage: f64,
    per_core: Vec<f64>,
    temps: Vec<f64>,
    num_cores: usize,
}

impl CpuCollector {
    pub fn new() -> Self {
        let num_cores = Self::detect_num_cores();
        Self {
            prev: vec![CpuData::default(); num_cores + 1],
            usage: 0.0,
            per_core: vec![0.0; num_cores],
            temps: Vec::new(),
            num_cores,
        }
    }

    pub async fn refresh(&mut self) {
        let raw = Self::read_stat();
        let data = Self::parse_stat(&raw);
        self.calculate(&data);
        self.temps = Self::read_cpu_temps();
    }

    pub fn usage(&self) -> f64 {
        self.usage
    }

    pub fn per_core_usage(&self) -> &[f64] {
        &self.per_core
    }

    pub fn temperatures(&self) -> &[f64] {
        &self.temps
    }

    fn detect_num_cores() -> usize {
        let raw = Self::read_stat();
        raw.lines().filter(|l| l.starts_with("cpu") && l.as_bytes().get(3).map_or(false, |c| c.is_ascii_digit())).count().max(1)
    }

    fn read_stat() -> String {
        fs::read_to_string("/proc/stat").unwrap_or_default()
    }

    fn parse_stat(raw: &str) -> Vec<CpuData> {
        let mut data = Vec::new();
        for line in raw.lines() {
            if line.starts_with("cpu") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let nums: Vec<u64> = parts[1..]
                        .iter()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    if nums.len() >= 5 {
                        data.push(CpuData {
                            user: nums[0],
                            nice: nums[1],
                            system: nums[2],
                            idle: nums[3],
                            iowait: nums.get(4).copied().unwrap_or(0),
                            irq: nums.get(5).copied().unwrap_or(0),
                            softirq: nums.get(6).copied().unwrap_or(0),
                            steal: nums.get(7).copied().unwrap_or(0),
                        });
                    }
                }
            }
        }
        data
    }

    fn calculate(&mut self, current: &[CpuData]) {
        let len = current.len().min(self.prev.len());
        self.usage = Self::calc_one(&self.prev[0], &current[0]);

        for i in 0..self.num_cores {
            if i + 1 < len {
                self.per_core[i] = Self::calc_one(&self.prev[i + 1], &current[i + 1]);
            }
        }

        for (i, d) in current.iter().enumerate().take(len) {
            self.prev[i] = d.clone();
        }
    }

    fn calc_one(prev: &CpuData, curr: &CpuData) -> f64 {
        let prev_total = prev.total();
        let curr_total = curr.total();
        let prev_idle = prev.idle_total();
        let curr_idle = curr.idle_total();

        let total_delta = curr_total.saturating_sub(prev_total);
        let idle_delta = curr_idle.saturating_sub(prev_idle);

        if total_delta == 0 {
            return 0.0;
        }
        (total_delta - idle_delta) as f64 / total_delta as f64 * 100.0
    }

    fn read_cpu_temps() -> Vec<f64> {
        let mut temps = Vec::new();

        let base = "/sys/devices/platform/coretemp.0/hwmon";
        if let Ok(dirs) = std::fs::read_dir(base) {
            for dir in dirs.flatten() {
                for i in 1..=128 {
                    let label_path = dir.path().join(format!("temp{}_label", i));
                    let input_path = dir.path().join(format!("temp{}_input", i));
                    if !label_path.exists() || !input_path.exists() {
                        break;
                    }
                    if let Ok(label) = std::fs::read_to_string(&label_path) {
                        if label.trim().starts_with("Core") {
                            if let Ok(val) = std::fs::read_to_string(&input_path) {
                                if let Ok(mc) = val.trim().parse::<f64>() {
                                    temps.push(mc / 1000.0);
                                }
                            }
                        }
                    }
                }
            }
        }

        if temps.is_empty() {
            if let Ok(zones) = std::fs::read_dir("/sys/class/thermal") {
                for zone in zones.flatten() {
                    let name = zone.file_name().to_string_lossy().to_string();
                    if name.starts_with("thermal_zone") {
                        let typ = std::fs::read_to_string(zone.path().join("type")).unwrap_or_default();
                        if typ.trim().contains("cpu") || typ.trim().contains("x86") {
                            if let Ok(val) = std::fs::read_to_string(zone.path().join("temp")) {
                                if let Ok(mc) = val.trim().parse::<f64>() {
                                    temps.push(mc / 1000.0);
                                }
                            }
                        }
                    }
                }
            }
        }

        temps
    }
}

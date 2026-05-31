use std::fs;

pub fn read_u64_from_file(path: &str) -> Option<u64> {
    fs::read_to_string(path).ok()?.trim().parse::<u64>().ok()
}

pub fn read_string_from_file(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

pub fn get_load_average() -> (f64, f64, f64) {
    let raw = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() >= 3 {
        let la1 = parts[0].parse::<f64>().unwrap_or(0.0);
        let la5 = parts[1].parse::<f64>().unwrap_or(0.0);
        let la15 = parts[2].parse::<f64>().unwrap_or(0.0);
        (la1, la5, la15)
    } else {
        (0.0, 0.0, 0.0)
    }
}

pub fn get_uptime() -> u64 {
    let raw = fs::read_to_string("/proc/uptime").unwrap_or_default();
    raw.split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|s| s as u64)
        .unwrap_or(0)
}

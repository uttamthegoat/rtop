pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[(&str, u64)] = &[
        ("TiB", 1u64 << 40),
        ("GiB", 1u64 << 30),
        ("MiB", 1u64 << 20),
        ("KiB", 1u64 << 10),
    ];

    for &(unit, divisor) in UNITS {
        if bytes >= divisor {
            let value = bytes as f64 / divisor as f64;
            return format!("{:.1} {}", value, unit);
        }
    }
    format!("{} B", bytes)
}

pub fn format_percent(value: f64) -> String {
    if value >= 99.9 {
        format!("{:5.0}%", value)
    } else {
        format!("{:5.1}%", value)
    }
}

pub fn format_cpu_usage(value: f64) -> String {
    format!("{:6.1}%", value)
}

pub fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {:02}:{:02}:{:02}", days, hours, mins, secs)
    } else {
        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    }
}

use std::fs;

#[derive(Debug, Default)]
struct MemInfo {
    mem_total: u64,
    mem_available: u64,
    mem_free: u64,
    buffers: u64,
    cached: u64,
    swap_total: u64,
    swap_free: u64,
}

pub struct MemoryCollector {
    data: MemInfo,
}

impl MemoryCollector {
    pub fn new() -> Self {
        Self {
            data: MemInfo::default(),
        }
    }

    pub async fn refresh(&mut self) {
        self.data = Self::read_meminfo();
    }

    pub fn total(&self) -> u64 {
        self.data.mem_total
    }

    pub fn used(&self) -> u64 {
        self.data.mem_total.saturating_sub(self.data.mem_available)
    }

    pub fn percent(&self) -> f64 {
        if self.data.mem_total == 0 {
            return 0.0;
        }
        self.used() as f64 / self.data.mem_total as f64 * 100.0
    }

    pub fn swap_total(&self) -> u64 {
        self.data.swap_total
    }

    pub fn swap_used(&self) -> u64 {
        self.data.swap_total.saturating_sub(self.data.swap_free)
    }

    pub fn available(&self) -> u64 {
        self.data.mem_available
    }

    pub fn free(&self) -> u64 {
        self.data.mem_free
    }

    pub fn cached(&self) -> u64 {
        self.data.cached
    }

    pub fn swap_percent(&self) -> f64 {
        if self.data.swap_total == 0 {
            return 0.0;
        }
        self.swap_used() as f64 / self.data.swap_total as f64 * 100.0
    }

    fn read_meminfo() -> MemInfo {
        let raw = fs::read_to_string("/proc/meminfo").unwrap_or_default();
        let mut info = MemInfo::default();

        for line in raw.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            let val = parts[1].parse::<u64>().unwrap_or(0);
            match parts[0].trim_end_matches(':') {
                "MemTotal" => info.mem_total = val * 1024,
                "MemAvailable" => info.mem_available = val * 1024,
                "MemFree" => info.mem_free = val * 1024,
                "Buffers" => info.buffers = val * 1024,
                "Cached" => info.cached = val * 1024,
                "SwapTotal" => info.swap_total = val * 1024,
                "SwapFree" => info.swap_free = val * 1024,
                _ => {}
            }
        }

        if info.mem_available == 0 {
            info.mem_available = info.mem_free + info.buffers + info.cached;
        }

        info
    }
}

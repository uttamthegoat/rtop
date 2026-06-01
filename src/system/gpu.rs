use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct GpuInfo {
    pub present: bool,
    pub name: String,
    pub temperature: f64,
    pub utilization: f64,
    pub memory_used: u64,
    pub memory_total: u64,
}

pub struct GpuCollector {
    gpu: GpuInfo,
    prev_busy: Option<u64>,
    prev_time: Option<std::time::Instant>,
}

impl GpuCollector {
    pub fn new() -> Self {
        Self {
            gpu: GpuInfo::default(),
            prev_busy: None,
            prev_time: None,
        }
    }

    pub async fn refresh(&mut self) {
        self.gpu = Self::read_gpu_info(&mut self.prev_busy, &mut self.prev_time);
    }

    pub fn info(&self) -> &GpuInfo {
        &self.gpu
    }

    fn read_u64(path: &Path) -> Option<u64> {
        fs::read_to_string(path).ok()?.trim().parse::<u64>().ok()
    }

    fn find_hwmon_temp(device: &Path) -> Option<f64> {
        let hwmon_dir = device.join("hwmon");
        let hwmon_entries = fs::read_dir(&hwmon_dir).ok()?;
        for entry in hwmon_entries.flatten() {
            let temp_input = entry.path().join("temp1_input");
            let val = Self::read_u64(&temp_input)?;
            return Some(val as f64 / 1000.0);
        }
        None
    }

    fn find_drm_card() -> Option<std::path::PathBuf> {
        let base = "/sys/class/drm";
        let dirs = fs::read_dir(base).ok()?;
        for entry in dirs.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.contains("card") && !name.contains("-") {
                return Some(entry.path());
            }
        }
        None
    }

    fn read_gpu_name(device: &Path) -> String {
        if let Ok(uevent) = fs::read_to_string(device.join("uevent")) {
            for line in uevent.lines() {
                if let Some(id) = line.strip_prefix("PCI_ID=") {
                    return format!("Intel GPU ({})", id);
                }
            }
            if let Some(driver) = uevent
                .lines()
                .find(|l| l.starts_with("DRIVER="))
                .map(|l| l.trim_start_matches("DRIVER="))
            {
                if driver == "i915" {
                    return "Intel GPU".to_string();
                }
                return driver.to_string();
            }
        }
        "GPU".to_string()
    }

    fn read_gpu_info(
        prev_busy: &mut Option<u64>,
        prev_time: &mut Option<std::time::Instant>,
    ) -> GpuInfo {
        let card = match Self::find_drm_card() {
            Some(c) => c,
            None => return GpuInfo::default(),
        };

        let device = card.join("device");

        let gpu_name = Self::read_gpu_name(&device);
        let temp = Self::find_hwmon_temp(&device).unwrap_or(0.0);

        let now = std::time::Instant::now();

        let utilization = if let Some(current_busy) = Self::read_u64(&device.join("gpu_busy")) {
            if let (Some(prev), Some(t)) = (*prev_busy, *prev_time) {
                let dt = now.saturating_duration_since(t).as_nanos() as u64;
                if dt > 0 && current_busy >= prev {
                    let busy_delta = current_busy - prev;
                    let pct = (busy_delta as f64 / dt as f64) * 100.0;
                    pct.min(100.0).max(0.0)
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            Self::read_u64(&device.join("gpu_busy_percent"))
                .map(|v| v as f64)
                .unwrap_or(0.0)
        };

        if let Some(busy) = Self::read_u64(&device.join("gpu_busy")) {
            *prev_busy = Some(busy);
            *prev_time = Some(now);
        }

        let mem_total = Self::read_u64(&device.join("mem_info_vram_total")).unwrap_or(0);
        let mem_used = Self::read_u64(&device.join("mem_info_vram_used")).unwrap_or(0);

        GpuInfo {
            present: true,
            name: gpu_name,
            temperature: temp,
            utilization,
            memory_used: mem_used,
            memory_total: mem_total,
        }
    }
}

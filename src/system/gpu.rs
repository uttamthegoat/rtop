use std::fs;

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
}

impl GpuCollector {
    pub fn new() -> Self {
        Self {
            gpu: GpuInfo::default(),
        }
    }

    pub async fn refresh(&mut self) {
        self.gpu = Self::read_gpu();
    }

    pub fn info(&self) -> &GpuInfo {
        &self.gpu
    }

    fn read_gpu() -> GpuInfo {
        let base = "/sys/class/drm";
        let dirs = match fs::read_dir(base) {
            Ok(d) => d,
            Err(_) => return GpuInfo::default(),
        };

        for entry in dirs.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.contains("card") && !name.contains("-") {
                let path = entry.path();
                let device = path.join("device");

                let temp = fs::read_to_string(device.join("temp"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f64>().ok())
                    .unwrap_or(0.0);

                let gpu_name = fs::read_to_string(device.join("uevent"))
                    .ok()
                    .map(|s| {
                        s.lines()
                            .find(|l| l.starts_with("DRIVER="))
                            .map(|l| l.trim_start_matches("DRIVER=").to_string())
                            .unwrap_or_default()
                    })
                    .unwrap_or_default();

                return GpuInfo {
                    present: true,
                    name: gpu_name,
                    temperature: temp / 1000.0,
                    utilization: 0.0,
                    memory_used: 0,
                    memory_total: 0,
                };
            }
        }

        GpuInfo::default()
    }
}

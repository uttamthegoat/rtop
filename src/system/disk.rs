use std::ffi::CString;
use std::fs;

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub percent: f64,
    pub filesystem: String,
}

pub struct DiskCollector {
    disks: Vec<DiskInfo>,
}

impl DiskCollector {
    pub fn new() -> Self {
        Self { disks: Vec::new() }
    }

    pub async fn refresh(&mut self) {
        self.disks = Self::read_disks();
    }

    pub fn disks(&self) -> &[DiskInfo] {
        &self.disks
    }

    fn read_disks() -> Vec<DiskInfo> {
        let raw = fs::read_to_string("/proc/mounts").unwrap_or_default();
        let mut disks = Vec::new();

        for line in raw.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            if parts[0].starts_with("/dev/") && !parts[0].contains("/loop") {
                if let Ok(stat) = fs::metadata(parts[1]) {
                    if stat.is_dir() {
                        let mount = parts[1].to_string();
                        let fs_type = parts[0].to_string();
                        if let Some((total, used, available, percent)) = get_statvfs(parts[1]) {
                            disks.push(DiskInfo {
                                mount_point: mount,
                                total,
                                used,
                                available,
                                percent,
                                filesystem: fs_type,
                            });
                        }
                    }
                }
            }
        }

        disks
    }
}

fn get_statvfs(path: &str) -> Option<(u64, u64, u64, f64)> {
    let mount_c = CString::new(path).ok()?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let ret = unsafe { libc::statvfs(mount_c.as_ptr(), &mut stat) };
    if ret != 0 {
        return None;
    }
    let total = (stat.f_blocks as u64).saturating_mul(stat.f_frsize as u64);
    let available = (stat.f_bavail as u64).saturating_mul(stat.f_frsize as u64);
    let free = (stat.f_bfree as u64).saturating_mul(stat.f_frsize as u64);
    let used = total.saturating_sub(free);
    let percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    Some((total, used, available, percent))
}

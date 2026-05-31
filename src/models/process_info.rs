#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub command: String,
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub memory_rss: u64,
    pub memory_vms: u64,
    pub user: String,
    pub state: String,
    pub priority: i32,
    pub threads: u64,
    pub cpu_time: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
}

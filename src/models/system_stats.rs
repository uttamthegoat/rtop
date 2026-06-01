use crate::system::disk::DiskInfo;
use crate::system::gpu::GpuInfo;
use crate::system::network::NetworkInfo;
use crate::utils::ring_buffer::RingBuffer;

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu_usage: f64,
    pub cpu_per_core: Vec<f64>,
    pub cpu_temperatures: Vec<f64>,
    pub memory_total: u64,
    pub memory_used: u64,
    pub memory_available: u64,
    pub memory_cached: u64,
    pub memory_free: u64,
    pub memory_percent: f64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percent: f64,
    pub cpu_history: RingBuffer<f64>,
    pub memory_history: RingBuffer<f64>,
    pub uptime: u64,
    pub processes_total: u32,
    pub processes_running: u32,
    pub load_average_1: f64,
    pub load_average_5: f64,
    pub load_average_15: f64,
    pub disks: Vec<DiskInfo>,
    pub network_info: Vec<NetworkInfo>,
    pub gpu: GpuInfo,
}

impl Default for SystemStats {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            cpu_per_core: Vec::new(),
            cpu_temperatures: Vec::new(),
            memory_total: 0,
            memory_used: 0,
            memory_available: 0,
            memory_cached: 0,
            memory_free: 0,
            memory_percent: 0.0,
            swap_total: 0,
            swap_used: 0,
            swap_percent: 0.0,
            cpu_history: RingBuffer::new(120),
            memory_history: RingBuffer::new(120),
            uptime: 0,
            processes_total: 0,
            processes_running: 0,
            load_average_1: 0.0,
            load_average_5: 0.0,
            load_average_15: 0.0,
            disks: Vec::new(),
            network_info: Vec::new(),
            gpu: GpuInfo::default(),
        }
    }
}

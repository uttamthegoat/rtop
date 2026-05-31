use crate::models::system_stats::SystemStats;
use crate::system::cpu::CpuCollector;
use crate::system::memory::MemoryCollector;

pub struct MetricsService {
    cpu: CpuCollector,
    memory: MemoryCollector,
}

impl MetricsService {
    pub fn new() -> Self {
        Self {
            cpu: CpuCollector::new(),
            memory: MemoryCollector::new(),
        }
    }

    pub async fn refresh(&mut self, stats: &mut SystemStats) {
        self.cpu.refresh().await;
        self.memory.refresh().await;

        stats.cpu_usage = self.cpu.usage();
        stats.cpu_per_core = self.cpu.per_core_usage().to_vec();
        stats.cpu_history.push(stats.cpu_usage);

        stats.memory_total = self.memory.total();
        stats.memory_used = self.memory.used();
        stats.memory_percent = self.memory.percent();
        stats.memory_history.push(stats.memory_percent);

        stats.swap_total = self.memory.swap_total();
        stats.swap_used = self.memory.swap_used();
        stats.swap_percent = self.memory.swap_percent();
    }
}

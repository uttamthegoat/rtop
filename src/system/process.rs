use crate::models::process_info::ProcessInfo;
use procfs::process::Process;
use std::collections::HashMap;

pub struct ProcessCollector {
    prev_cpu_times: HashMap<u32, (u64, u64)>,
    prev_total_cpu_time: u64,
    processes: Vec<ProcessInfo>,
}

impl ProcessCollector {
    pub fn new() -> Self {
        Self {
            prev_cpu_times: HashMap::new(),
            prev_total_cpu_time: 0,
            processes: Vec::new(),
        }
    }

    pub async fn refresh(&mut self) {
        let mut new_processes = Vec::new();
        let mut new_cpu_times: HashMap<u32, (u64, u64)> = HashMap::new();
        let current_total_cpu_time = Self::total_cpu_time();
        let total_cpu_delta = current_total_cpu_time - self.prev_total_cpu_time;
        let total_memory = Self::total_memory();

        if let Ok(all_proc) = procfs::process::all_processes() {
            for proc in all_proc.flatten() {
                if let Some(info) = Self::process_info(&proc, &self.prev_cpu_times, total_cpu_delta, total_memory) {
                    if let Ok(stat) = proc.stat() {
                        new_cpu_times.insert(info.pid, (stat.utime + stat.stime, stat.starttime));
                    }
                    new_processes.push(info);
                }
            }
        }

        self.processes = new_processes;
        self.prev_cpu_times = new_cpu_times;
        self.prev_total_cpu_time = current_total_cpu_time;
    }

    pub fn processes(&self) -> Vec<ProcessInfo> {
        self.processes.clone()
    }

    fn process_info(
        proc: &Process,
        prev_times: &HashMap<u32, (u64, u64)>,
        total_cpu_time: u64,
        total_memory: u64,
    ) -> Option<ProcessInfo> {
        let stat = proc.stat().ok()?;
        let pid = stat.pid as u32;
        let ppid = stat.ppid as u32;
        let name = stat.comm.clone();
        let state = match stat.state {
            'R' => "R",
            'S' => "S",
            'D' => "D",
            'Z' => "Z",
            'T' => "T",
            'X' => "X",
            _ => "?",
        };

        let cpu_percent = Self::calc_cpu_percent(pid, stat.utime + stat.stime, stat.starttime, prev_times, total_cpu_time);

        let mem_info = proc.statm().ok();
        let (rss, vms) = mem_info.map(|m| {
            let page_size = procfs::page_size();
            (m.resident * page_size, m.size * page_size)
        }).unwrap_or((0, 0));

        let user = proc.status()
            .ok()
            .map(|s| format!("{}", s.euid))
            .unwrap_or_else(|| "?".to_string());

        let priority = stat.priority;
        let threads = stat.num_threads as u64;

        let cmdline = proc.cmdline().ok().map(|args| args.join(" ")).unwrap_or_else(|| name.clone());
        Some(ProcessInfo {
            pid,
            ppid,
            name,
            command: cmdline,
            cpu_percent,
            memory_percent: if total_memory > 0 { rss as f64 / total_memory as f64 * 100.0 } else { 0.0 },
            memory_rss: rss,
            memory_vms: vms,
            user,
            state: state.to_string(),
            priority: priority as i32,
            threads,
            cpu_time: stat.utime + stat.stime,
            io_read_bytes: 0,
            io_write_bytes: 0,
        })
    }

    fn calc_cpu_percent(
        pid: u32,
        current_time: u64,
        _starttime: u64,
        prev_times: &HashMap<u32, (u64, u64)>,
        total_cpu_time: u64,
    ) -> f64 {
        if let Some(&(prev_proc_time, _)) = prev_times.get(&pid) {
            let proc_delta = current_time.saturating_sub(prev_proc_time);
            if total_cpu_time > 0 && proc_delta > 0 {
                return proc_delta as f64 / total_cpu_time as f64 * 100.0;
            }
        }
        0.0
    }

    fn total_cpu_time() -> u64 {
        let stat_content = std::fs::read_to_string("/proc/stat").unwrap_or_default();
        if let Some(line) = stat_content.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                return parts[1..].iter().filter_map(|s| s.parse::<u64>().ok()).sum();
            }
        }
        0
    }

    fn total_memory() -> u64 {
        let mem_content = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        for line in mem_content.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        return kb * 1024;
                    }
                }
            }
        }
        0
    }
}

pub fn kill_process(pid: u32) -> Result<(), std::io::Error> {
    unsafe {
        let result = libc::kill(pid as i32, libc::SIGTERM);
        if result == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

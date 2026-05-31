use crate::models::process_info::ProcessInfo;
use crate::system::process::ProcessCollector;

pub struct ProcessService {
    collector: ProcessCollector,
}

impl ProcessService {
    pub fn new() -> Self {
        Self {
            collector: ProcessCollector::new(),
        }
    }

    pub async fn refresh(&mut self) -> Vec<ProcessInfo> {
        self.collector.refresh().await;
        self.collector.processes()
    }

    pub fn kill(pid: u32) -> Result<(), std::io::Error> {
        crate::system::process::kill_process(pid)
    }
}

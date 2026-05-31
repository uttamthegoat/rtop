use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub current_rx: u64,
    pub current_tx: u64,
    pub top_rx: u64,
    pub top_tx: u64,
    pub total_rx: u64,
    pub total_tx: u64,
}

pub struct NetworkCollector {
    prev: Vec<NetworkStats>,
    current: Vec<NetworkStats>,
    top_rx: HashMap<String, u64>,
    top_tx: HashMap<String, u64>,
    total_rx: HashMap<String, u64>,
    total_tx: HashMap<String, u64>,
}

impl NetworkCollector {
    pub fn new() -> Self {
        Self {
            prev: Vec::new(),
            current: Vec::new(),
            top_rx: HashMap::new(),
            top_tx: HashMap::new(),
            total_rx: HashMap::new(),
            total_tx: HashMap::new(),
        }
    }

    pub async fn refresh(&mut self) {
        self.prev = std::mem::take(&mut self.current);
        self.current = Self::read_net_dev();

        for (prev, curr) in self.prev.iter().zip(self.current.iter()) {
            let rx_delta = curr.rx_bytes.saturating_sub(prev.rx_bytes);
            let tx_delta = curr.tx_bytes.saturating_sub(prev.tx_bytes);
            let name = &curr.interface;

            let top_rx = self.top_rx.entry(name.clone()).or_insert(0);
            *top_rx = (*top_rx).max(rx_delta);
            let top_tx = self.top_tx.entry(name.clone()).or_insert(0);
            *top_tx = (*top_tx).max(tx_delta);
            *self.total_rx.entry(name.clone()).or_insert(0) += rx_delta;
            *self.total_tx.entry(name.clone()).or_insert(0) += tx_delta;
        }
    }

    pub fn interfaces(&self) -> &[NetworkStats] {
        &self.current
    }
    
    pub fn delta(&self) -> Vec<(u64, u64)> {
        let mut deltas = Vec::new();
        for (prev, curr) in self.prev.iter().zip(self.current.iter()) {
            deltas.push((
                curr.rx_bytes.saturating_sub(prev.rx_bytes),
                curr.tx_bytes.saturating_sub(prev.tx_bytes),
            ));
        }
        deltas
    }

    pub fn extended_info(&self) -> Vec<NetworkInfo> {
        let deltas = self.delta();
        self.current
            .iter()
            .zip(deltas.iter())
            .map(|(n, d)| NetworkInfo {
                interface: n.interface.clone(),
                current_rx: d.0,
                current_tx: d.1,
                top_rx: self.top_rx.get(&n.interface).copied().unwrap_or(0),
                top_tx: self.top_tx.get(&n.interface).copied().unwrap_or(0),
                total_rx: self.total_rx.get(&n.interface).copied().unwrap_or(0),
                total_tx: self.total_tx.get(&n.interface).copied().unwrap_or(0),
            })
            .collect()
    }

    fn read_net_dev() -> Vec<NetworkStats> {
        let raw = fs::read_to_string("/proc/net/dev").unwrap_or_default();
        let mut interfaces = Vec::new();

        for line in raw.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 10 {
                continue;
            }
            let iface = parts[0].trim_end_matches(':').to_string();
            let rx_bytes = parts[1].parse().unwrap_or(0);
            let rx_packets = parts[2].parse().unwrap_or(0);
            let rx_errors = parts[3].parse().unwrap_or(0);
            let tx_bytes = parts[9].parse().unwrap_or(0);
            let tx_packets = parts[10].parse().unwrap_or(0);
            let tx_errors = parts[11].parse().unwrap_or(0);

            interfaces.push(NetworkStats {
                interface: iface,
                rx_bytes,
                tx_bytes,
                rx_packets,
                tx_packets,
                rx_errors,
                tx_errors,
            });
        }

        interfaces
    }
}

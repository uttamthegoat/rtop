use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn is_wireless(iface: &str) -> bool {
    let path = format!("/sys/class/net/{}/wireless", iface);
    std::path::Path::new(&path).exists()
}

fn get_wifi_ssid(iface: &str) -> Option<String> {
    if let Ok(out) = std::process::Command::new("nmcli")
        .args(["-t", "-f", "NAME,DEVICE", "connection", "show", "--active"])
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let parts: Vec<&str> = line.rsplitn(2, ':').collect();
                if parts.len() == 2 && parts[0] == iface {
                    let name = parts[1].trim().to_string();
                    if !name.is_empty() {
                        return Some(name);
                    }
                }
            }
        }
    }
    if let Ok(out) = std::process::Command::new("iwgetid")
        .arg("-r")
        .arg(iface)
        .output()
    {
        if out.status.success() {
            let ssid = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !ssid.is_empty() {
                return Some(ssid);
            }
        }
    }
    if let Ok(out) = std::process::Command::new("iw")
        .args(["dev", iface, "link"])
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let line = line.trim();
                if let Some(ssid) = line.strip_prefix("SSID:") {
                    let s = ssid.trim().to_string();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
        }
    }
    None
}

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
    pub display_name: String,
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
    first_tick: bool,
    ssid_cache: HashMap<String, (String, Instant)>,
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
            first_tick: true,
            ssid_cache: HashMap::new(),
        }
    }

    pub async fn refresh(&mut self) {
        self.current = Self::read_net_dev();

        if self.first_tick || self.prev.is_empty() {
            self.first_tick = false;
            self.prev = self.current.clone();
            return;
        }

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

    pub fn extended_info(&mut self) -> Vec<NetworkInfo> {
        let deltas = self.delta();
        const SSID_TTL: std::time::Duration = std::time::Duration::from_secs(30);
        self.current
            .iter()
            .zip(deltas.iter())
            .map(|(n, d)| {
                let display_name = if is_wireless(&n.interface) {
                    if let Some(cached) = self.ssid_cache.get(&n.interface) {
                        if cached.1.elapsed() < SSID_TTL {
                            cached.0.clone()
                        } else {
                            let ssid = get_wifi_ssid(&n.interface).unwrap_or_else(|| n.interface.clone());
                            self.ssid_cache.insert(n.interface.clone(), (ssid.clone(), Instant::now()));
                            ssid
                        }
                    } else {
                        let ssid = get_wifi_ssid(&n.interface).unwrap_or_else(|| n.interface.clone());
                        self.ssid_cache.insert(n.interface.clone(), (ssid.clone(), Instant::now()));
                        ssid
                    }
                } else {
                    n.interface.clone()
                };
                NetworkInfo {
                    interface: n.interface.clone(),
                    display_name,
                    current_rx: d.0,
                    current_tx: d.1,
                    top_rx: self.top_rx.get(&n.interface).copied().unwrap_or(0),
                    top_tx: self.top_tx.get(&n.interface).copied().unwrap_or(0),
                    total_rx: self.total_rx.get(&n.interface).copied().unwrap_or(0),
                    total_tx: self.total_tx.get(&n.interface).copied().unwrap_or(0),
                }
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

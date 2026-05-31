# rtop

**A terminal-based Linux system monitor, written in Rust.**

Inspired by htop, rtop provides real-time monitoring of system resources directly in your terminal. It reads data straight from `/proc` and `/sys` — no daemons, no GUI, no bloat.

## Features

- **Process list** — view all running processes with PID, user, CPU%, memory, state
- **CPU monitoring** — per-core utilization with delta-based calculation
- **Memory & swap** — real-time usage with bar visualization
- **Network stats** — per-interface RX/TX (basic)
- **Battery** — capacity, status, voltage (laptops)
- **GPU temperature** — basic readout (NVIDIA/AMD via DRM)
- **Keyboard-driven** — sort, search, kill processes, toggle views
- **Configurable** — TOML config for refresh rate, theme, layout
- **Low resource** — targets <20 MB RAM, <2% CPU idle

## Quick Start

```bash
cargo run --release
```

Keys: `q` quit, `↑/↓` navigate, `/` search, `k` kill, `t` tree toggle, `r` refresh.

## Status

**v0.1.0** — Early development. Core monitoring (CPU, memory, processes) works. Tree view, disk usage, full network stats, sorting, and tests are still being built.

## Tech Stack

| Component | Technology |
|---|---|
| Language | Rust 2021 |
| TUI | ratatui + crossterm |
| Async | tokio |
| Data sources | /proc, /sys, procfs crate |
| Config | TOML via serde |

## License

MIT

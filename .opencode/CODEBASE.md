# rtop Codebase Overview

**Language:** Rust (edition 2021)  
**Framework:** ratatui 0.29 + crossterm 0.28  
**Runtime:** tokio 1.x (async)

## Structure

```
src/
├── main.rs              Entry point, logging init, runtime start
├── app.rs               Central App: event loop, refresh, draw
├── config.rs            TOML config load/save
├── error.rs             RtopError enum
├── event.rs             Async EventLoop (key + tick mpsc channel)
├── input.rs             Action enum + key mapping
├── state.rs             AppState struct
├── models/
│   ├── process_info.rs  ProcessInfo data
│   └── system_stats.rs  SystemStats + history ring buffers
├── system/
│   ├── cpu.rs           /proc/stat parser, delta CPU%
│   ├── memory.rs        /proc/meminfo parser
│   ├── process.rs       /proc/[pid]/* parser + kill
│   ├── disk.rs          /proc/mounts (stub)
│   ├── network.rs       /proc/net/dev parser
│   ├── battery.rs       /sys/class/power_supply/BAT*
│   └── gpu.rs           /sys/class/drm/card* (stub)
├── services/
│   ├── metrics_service.rs   CPU + memory refresh wrapper
│   └── process_service.rs   Process refresh wrapper
├── ui/
│   ├── mod.rs           Layout assembly (CPU+mem row, process table, footer)
│   ├── theme.rs         Color theme
│   ├── process_table.rs Table widget rendering
│   ├── cpu_widget.rs    CPU gauge
│   ├── memory_widget.rs Memory gauge
│   ├── network_widget.rs Placeholder
│   ├── footer.rs        Keybindings bar
│   └── layout.rs        Unused placeholder
└── utils/
    ├── ring_buffer.rs   Generic RingBuffer<T> (cap 120)
    ├── formatting.rs    Bytes, percent, duration formatting
    └── linux.rs         read_u64/file helpers, loadavg, uptime
```

## Key Dependencies

- **ratatui** — terminal UI widgets (Table, Gauge, Paragraph)
- **crossterm** — raw mode, event input, alternate screen
- **sysinfo** — fallback system data (secondary to direct /proc reads)
- **procfs** — typed /proc filesystem access
- **serde + toml** — config serialization

## Build & Run

```bash
cargo run --release     # Build and run
cargo build --release   # Release binary only
```

Config: `~/.config/rtop/config.toml`

## No Tests Yet

The project has zero tests — no `#[test]` blocks or `tests/` directory.

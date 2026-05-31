# rtop Architecture

## Overview

rtop is a terminal-based Linux system monitor written in Rust. It follows a **single-binary, flat-layered** architecture with an async event loop driving synchronous data collection and UI rendering.

```
main.rs
  └─ App (app.rs) — orchestrator
       ├─ Config (config.rs)            TOML-based config from ~/.config/rtop/
       ├─ AppState (state.rs)           Shared mutable state
       ├─ EventLoop (event.rs)          Async mpsc channel (keyboard + tick)
       ├─ System Collectors (system/)   Linux /proc & /sys readers
       │   ├─ cpu.rs                    /proc/stat delta CPU%
       │   ├─ memory.rs                 /proc/meminfo
       │   ├─ process.rs                /proc/[pid]/stat, statm, status
       │   ├─ disk.rs                   /proc/mounts (stub)
       │   ├─ network.rs                /proc/net/dev
       │   ├─ battery.rs                /sys/class/power_supply/BAT*
       │   └─ gpu.rs                    /sys/class/drm/card*/device (stub)
       ├─ Services (services/)          Thin wrappers around collectors
       ├─ UI (ui/)                      ratatui widget rendering
       │   ├─ mod.rs                    Layout assembly (vertical split)
       │   ├─ theme.rs                  Color configuration
       │   ├─ process_table.rs          Scrollable process list
       │   ├─ cpu_widget.rs             Gauge widget
       │   ├─ memory_widget.rs          Gauge widget
       │   ├─ network_widget.rs         Stub
       │   └─ footer.rs                 Keybind legend
       └─ Utils (utils/)                Helpers, RingBuffer, formatting
```

## Execution Model

```
┌─────────────┐     mpsc::unbounded_channel     ┌───────────┐
│  Event Loop  │ ──── Event::Tick (1s) ────────▶│           │
│  (tokio)     │ ──── Event::Key (crossterm) ──▶│   App     │
└─────────────┘                                  │  (poll)   │
                                                 │           │
  Every tick:                                    └─────┬─────┘
  1. Refresh collectors                                 │
  2. Update AppState                                    │
  3. Render UI (ratatui)                                ▼
                                                  Terminal
```

## Key Design Decisions

- **Linux-only**: Reads `/proc` and `/sys` directly (no cross-platform abstraction).
- **Delta-based CPU**: CPU utilization computed as delta between two `/proc/stat` reads.
- **RingBuffer history**: Generic fixed-capacity (120) ring buffer for CPU/memory history graphs.
- **Async events, sync rendering**: Tokio event loop feeds into synchronous draw loop — no concurrent mutations.
- **ratatui + crossterm**: TUI framework with crossterm backend (no ncurses dependency).
- **procfs crate**: Type-safe parsing of `/proc` filesystem.

## Current Limitations (v0.1.0)

- Disk, network, GPU collectors are stubs/placeholder
- No tree view, process filtering, or column sorting
- No mouse support
- Only SIGTERM kill supported
- No tests

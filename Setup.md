You are a senior Linux systems engineer and expert Rust developer.

Your task is to help me build a production-quality terminal-based Linux system monitor inspired by htop, written entirely in Rust.

The application must be:
- lightweight,
- memory efficient,
- modular,
- fast,
- keyboard-driven,
- optimized for Arch Linux,
- and designed for future extensibility.

The project should behave similarly to htop:
- real-time process monitoring,
- CPU usage,
- memory usage,
- process sorting,
- process killing,
- keyboard navigation,
- process tree view,
- search/filter,
- and live updates.

However, the architecture should be modern and clean.

==================================================
TECH STACK
==================================================

Language:
- Rust stable

Libraries:
- ratatui
- crossterm
- sysinfo
- procfs
- tokio

Platform:
- Linux only initially
- Arch Linux primary target

==================================================
GOALS
==================================================

1. Extremely low memory usage
2. Fast refresh rate
3. Minimal CPU overhead
4. Clean architecture
5. Async-safe design
6. Proper Rust ownership patterns
7. Modular code organization
8. Terminal UI rendering efficiency
9. Avoid unnecessary allocations
10. Production-grade code quality

==================================================
FEATURE REQUIREMENTS
==================================================

CORE FEATURES:
- Live process table
- CPU usage monitoring
- Memory usage monitoring
- Disk usage monitoring
- Network usage monitoring
- Process sorting
- Process filtering/search
- Process killing
- Keyboard navigation
- Mouse support (optional)
- Configurable refresh rate
- Multiple color themes
- Process tree view
- Multi-core CPU display

ADVANCED FEATURES:
- GPU monitoring support
- Battery statistics
- Historical CPU graphs
- Historical memory graphs
- Config file support
- Hot-reload configuration
- Per-process CPU history
- Docker container visibility
- Cgroup awareness
- Temperature monitoring

OPTIONAL FUTURE FEATURES:
- Wayland integration
- SSH remote monitoring
- Plugin system
- Lua scripting
- AI-assisted diagnostics
- Web dashboard
- Prometheus exporter

==================================================
ARCHITECTURE REQUIREMENTS
==================================================

Use a clean modular architecture.

Recommended folder structure:

src/
├── main.rs
├── app.rs
├── config.rs
├── error.rs
├── input.rs
├── state.rs
├── event.rs
├── ui/
│   ├── mod.rs
│   ├── layout.rs
│   ├── theme.rs
│   ├── process_table.rs
│   ├── cpu_widget.rs
│   ├── memory_widget.rs
│   ├── network_widget.rs
│   └── footer.rs
├── system/
│   ├── mod.rs
│   ├── cpu.rs
│   ├── memory.rs
│   ├── process.rs
│   ├── disk.rs
│   ├── network.rs
│   ├── battery.rs
│   └── gpu.rs
├── services/
│   ├── mod.rs
│   ├── process_service.rs
│   └── metrics_service.rs
├── models/
│   ├── mod.rs
│   ├── process_info.rs
│   └── system_stats.rs
└── utils/
    ├── mod.rs
    ├── formatting.rs
    ├── ring_buffer.rs
    └── linux.rs

==================================================
PERFORMANCE REQUIREMENTS
==================================================

The application should:
- avoid heap allocations where possible,
- minimize cloning,
- avoid unnecessary string creation,
- use borrowing effectively,
- use fixed-size ring buffers for graph history,
- avoid excessive async tasks,
- batch UI updates,
- reduce terminal redraw overhead,
- use efficient diff-based rendering if possible,
- avoid blocking operations in UI thread.

Target:
- RAM usage under 20 MB
- CPU usage under 2% idle
- Smooth 60 FPS rendering optional
- Stable under heavy process loads

==================================================
SYSTEM DATA SOURCES
==================================================

Use Linux system interfaces:
- /proc
- /sys
- procfs crate
- sysinfo crate

Examples:
- /proc/stat
- /proc/meminfo
- /proc/[pid]
- /proc/net/dev
- /sys/class/thermal
- /sys/class/power_supply

==================================================
KEYBOARD CONTROLS
==================================================

Implement:
- Arrow keys navigation
- Vim-style navigation (j/k)
- q to quit
- k to kill process
- / to search
- t to toggle tree mode
- c to sort by CPU
- m to sort by memory
- p to sort by PID
- r to refresh
- TAB for panel switching

==================================================
UI REQUIREMENTS
==================================================

The UI should:
- resemble htop/btop hybrid,
- use clean panels,
- support Unicode box drawing,
- support dynamic resizing,
- support responsive layouts,
- avoid flickering,
- support dark themes,
- support compact mode.

Widgets:
- CPU graph
- Memory graph
- Process table
- Network widget
- Footer/help bar

==================================================
IMPLEMENTATION STRATEGY
==================================================

Build incrementally in phases.

PHASE 1:
- project setup
- terminal initialization
- basic ratatui rendering
- event loop
- keyboard input

PHASE 2:
- CPU monitoring
- memory monitoring
- process listing

PHASE 3:
- sorting
- search
- process actions

PHASE 4:
- graphs
- history buffers
- themes

PHASE 5:
- optimization
- benchmarking
- profiling

==================================================
OUTPUT FORMAT
==================================================

When generating code:
- provide complete files,
- explain architecture decisions,
- explain ownership choices,
- explain performance optimizations,
- explain Linux internals used,
- explain terminal rendering techniques,
- explain async patterns.

Always provide:
- Cargo.toml updates,
- full Rust source files,
- explanations,
- and next implementation steps.

==================================================
IMPORTANT ENGINEERING CONSTRAINTS
==================================================

Do NOT:
- use garbage-collected languages,
- use Electron,
- use web technologies,
- create unnecessary abstractions,
- create deeply nested async runtimes,
- allocate excessively every frame,
- redraw the whole terminal unnecessarily.

DO:
- prefer zero-cost abstractions,
- use idiomatic Rust,
- optimize for Linux,
- write production-grade code,
- follow Rust best practices,
- write efficient terminal rendering logic,
- carefully manage state updates.

==================================================
FINAL OBJECTIVE
==================================================

The final result should be:
- a real usable Linux utility,
- installable via cargo,
- suitable for Arch Linux users,
- performant enough to compete with htop,
- visually cleaner than htop,
- lighter than btop,
- and architected like a professional systems application.

Begin by generating:
1. Cargo.toml
2. Initial project setup
3. Event loop architecture
4. Basic terminal rendering
5. CPU + memory widgets
6. Live process table
7. Keyboard handling system

Then continue feature-by-feature incrementally.
# rtop — Agent Context

## What is this?

A terminal-based Linux system monitor (like htop) written in Rust. Reads `/proc` and `/sys` directly. Uses ratatui for rendering, tokio for async event handling.

## Key Facts

- **Language:** Rust (edition 2021)
- **Build:** `cargo build --release`
- **Run:** `cargo run --release`
- **Config:** `~/.config/rtop/config.toml` (TOML)
- **Target platform:** Linux only (Arch Linux primary)
- **License:** MIT
- **v0.1.0** — early stage, many stubs

## Architecture (30s)

`main.rs` → `App` (orchestrator) → system collectors read `/proc`/`/sys` → `AppState` stores data → ratatui widgets render. Async event loop feeds keyboard + tick events through an mpsc channel. No concurrent mutations.

## Current Gaps

- No tests
- Disk, GPU, network widgets are stubs
- No tree view, sorting, filtering of processes
- No mouse support
- Only SIGTERM kill
- `ui/layout.rs` is unused

## Key Files

| File | Role |
|---|---|
| `src/app.rs` | Main event loop, refresh/draw cycle |
| `src/event.rs` | Async event loop (tokio + mpsc) |
| `src/system/cpu.rs` | Delta CPU% from /proc/stat |
| `src/system/process.rs` | Process list + kill |
| `src/ui/mod.rs` | Layout assembly |
| `src/utils/ring_buffer.rs` | Generic RingBuffer for history |

## Commands

```bash
cargo run --release     # Run
cargo build --release   # Build
```

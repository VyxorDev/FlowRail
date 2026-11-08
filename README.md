# FlowRail

**Your desktop has a taskbar. It deserves a signal track.**

FlowRail is a free, privacy-first, open-source Windows activity rail written entirely in Rust. Its **vertical signal track** is an original Windows-first UI, not a Dynamic Island clone.

> **v0.1 prototype:** draggable borderless app, not yet an edge-docked background utility. Fullscreen detection, tray, media controls, notifications, and plugins are future milestones.

## Run on Windows

Prerequisites: Windows 11, Rust 1.95.0 (pinned in rust-toolchain.toml), MSVC Build Tools and the Windows SDK.

    git clone https://github.com/VyxorDev/FlowRail.git
    cd FlowRail
    cargo run

Build an optimized executable:

    cargo build --release

The release executable is target/release/flowrail.exe. No administrator rights or accounts are required. Settings live locally at %APPDATA%\FlowRail\settings.json.

## Current features

- Original graphite/mint signal-track interface
- Focus session start/pause/resume/reset
- Prioritized activity stream with Quiet filtering
- Real Win32 system uptime and local app uptime
- Explorer, Windows Settings and Task Manager quick actions
- Local Quiet and Compact preferences; unit tests and Windows CI

The application contains no webviews, JavaScript, cloud backend, tracking, or advertisements. Never disable Windows security protections to run FlowRail.

Developer checks: cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo test, cargo build --release.

See docs/ARCHITECTURE.md, docs/ROADMAP.md, SECURITY.md, and CONTRIBUTING.md.

MIT © 2026 VyxorDev. Independent project, not affiliated with Microsoft or Apple.
